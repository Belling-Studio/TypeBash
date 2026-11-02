// src/analyzer/statements.rs
use crate::parser::Statement;
use crate::lexer::TokenKind;
use crate::error::TypeBashError;
use super::blocks;
use std::collections::HashMap;

pub fn analyze_statements(
    file_path: &str,
    statements: &[Statement],
    scopes: &mut Vec<HashMap<String, String>>,
    function_signatures: &mut HashMap<String, Vec<crate::parser::FunctionParam>>,
    diagnostics: &mut Vec<TypeBashError>,
) {
    // РЕКОМЕНДОВАНО: Расширили белый список базовых DevOps-команд, которые гарантированно есть в Bash
    let bash_builtins = vec![
        "echo", "cd", "pwd", "exit", "logout", "return", 
        "local", "export", "alias", "read", 
        "mkdir", "rm", "touch", "clear"
    ];

    for stmt in statements {
        match stmt {
            Statement::VariableDeclaration { name, var_type, value, line } => {
                if name.as_str() == name.to_uppercase().as_str() && name.len() > 2 {
                    diagnostics.push(TypeBashError::warning(
                        101, *line, file_path,
                        &format!("Variable '{}' is in UPPERCASE.", name),
                        "TypeBash automatically handles UPPERCASE for Bash. Use camelCase instead."
                    ));
                }

                let value_type = match value {
                    TokenKind::StringLiteral(_) => "string",
                    TokenKind::NumberLiteral(_) => "number",
                    TokenKind::BooleanLiteral(_) => "boolean", 
                    _ => "unknown",
                };

                if var_type == "Path" {
                    if let TokenKind::StringLiteral(s) = value {
                        if !s.starts_with('/') && !s.starts_with("./") && !s.starts_with("../") {
                            diagnostics.push(TypeBashError::new(402, *line, file_path, "Invalid Path format.", "Fix path."));
                            continue;
                        }
                    }
                }

                if var_type != value_type && var_type != "Path" {
                    diagnostics.push(TypeBashError::new(401, *line, file_path, "Type mismatch.", "Fix type."));
                }

                if let Some(current_scope) = scopes.last_mut() { 
                    current_scope.insert(name.clone(), var_type.clone()); 
                }
            }

            Statement::ArrayDeclaration { name, element_type, elements, line } => {
                for el in elements {
                    let actual_el_type = match el {
                        TokenKind::StringLiteral(_) => "string",
                        TokenKind::NumberLiteral(_) => "number",
                        TokenKind::BooleanLiteral(_) => "boolean",
                        _ => "unknown",
                    };
                    if element_type != actual_el_type {
                        diagnostics.push(TypeBashError::new(460, *line, file_path, "Type mismatch inside Array.", "Fix elements."));
                    }
                }
                if let Some(current_scope) = scopes.last_mut() { 
                    current_scope.insert(name.clone(), format!("Array<{}>", element_type)); 
                }
            }

            Statement::PrintCall { target_name, line } => {
                let mut found = false;
                for scope in scopes.iter().rev() { 
                    if scope.contains_key(target_name) { found = true; break; } 
                }
                if !found { 
                    diagnostics.push(TypeBashError::new(403, *line, file_path, "Cannot find name.", "Fix print.")); 
                }
            }

            Statement::ExecCall { command, line } => {
                let mut words = command.split_whitespace();
                if let Some(first_word) = words.next() {
                    if first_word.starts_with('$') {
                        let var_name = first_word.trim_start_matches('$').trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_');
                        let mut found = false;
                        for scope in scopes.iter().rev() { 
                            if scope.contains_key(var_name) { found = true; break; } 
                        }
                        if !found { 
                            diagnostics.push(TypeBashError::new(407, *line, file_path, "Variable not found.", "Fix variable.")); 
                        }
                    } else if !bash_builtins.contains(&first_word) {
                        let cmd_check = if cfg!(target_os = "windows") { 
                            std::process::Command::new("where").arg(first_word).output() 
                        } else { 
                            std::process::Command::new("which").arg(first_word).output() 
                        };
                        if let Ok(output) = cmd_check {
                            if !output.status.success() { 
                                diagnostics.push(TypeBashError::new(450, *line, file_path, "System utility not found.", "Fix spelling.")); 
                            }
                        }
                    }
                }
            }

            Statement::FunctionDeclaration { .. } => { 
                blocks::analyze_function(file_path, stmt, scopes, function_signatures, diagnostics); 
            }
            Statement::FunctionCall { .. } => { 
                blocks::analyze_function_call(file_path, stmt, scopes, function_signatures, diagnostics); 
            }
            Statement::If { .. } => { 
                blocks::analyze_if(file_path, stmt, scopes, function_signatures, diagnostics); 
            }
            Statement::ForLoop { .. } => {
                blocks::analyze_for(file_path, stmt, scopes, function_signatures, diagnostics);
            }
            Statement::Comment { .. } => {}
            Statement::StdLibCall { .. } => {}
        }
    }
}
