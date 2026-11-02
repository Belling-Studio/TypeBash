// src/codegen.rs
use crate::parser::Statement;
use crate::lexer::TokenKind;
use std::fs::File;
use std::io::Write;

pub fn generate(output_path: &str, ast: Vec<Statement>) -> std::io::Result<()> {
    let mut file = File::create(output_path)?;

    writeln!(file, "#!/usr/bin/env bash")?;
    writeln!(file, "set -euo pipefail")?;
    writeln!(file)?;
    writeln!(file, "# ──────────────────────────────────────────────────────────")?;
    writeln!(file, "# Generated professionally by TypeBash Compiler")?;
    writeln!(file, "# ──────────────────────────────────────────────────────────")?;
    writeln!(file)?;

    writeln!(file, "typebash_err_handler() {{")?;
    writeln!(file, "    local exit_code=$1")?;
    writeln!(file, "    local line_no=$2")?;
    writeln!(file, "    local bash_command=$3")?;
    writeln!(file, "    echo \"\" >&2")?;
    writeln!(file, "    echo \"==================================================\" >&2")?;
    writeln!(file, "    echo \"❌ [TypeBash Runtime Error] Script failed!\" >&2")?;
    writeln!(file, "    echo \"  • Exit Code : $exit_code\" >&2")?;
    writeln!(file, "    echo \"  • Line      : $line_no\" >&2")?;
    writeln!(file, "    echo \"  • Command   : $bash_command\" >&2")?;
    writeln!(file, "    echo \"==================================================\" >&2")?;
    writeln!(file, "}}")?;
    writeln!(file, "trap 'typebash_err_handler $? $LINENO \"$BASH_COMMAND\"' ERR")?;
    writeln!(file)?;

    let mut type_map = std::collections::HashMap::new();
    generate_statements(&mut file, ast, 0, &mut type_map)?;

    Ok(())
}

fn generate_statements(
    file: &mut File, 
    statements: Vec<Statement>, 
    indent_level: usize,
    type_map: &mut std::collections::HashMap<String, String>
) -> std::io::Result<()> {
    let indent = "    ".repeat(indent_level);

    for stmt in statements {
        match stmt {
            Statement::VariableDeclaration { name, var_type, value, .. } => {
                type_map.insert(name.clone(), var_type.clone()); 
                let formatted_value = match value {
                    TokenKind::StringLiteral(s) => format!("\"{}\"", s),
                    TokenKind::NumberLiteral(n) => n.to_string(),
                    TokenKind::BooleanLiteral(b) => format!("\"{}\"", b), 
                    _ => "\"\"".to_string(),
                };
                writeln!(file, "{}readonly {}={}", indent, name.to_uppercase(), formatted_value)?;
            }

            Statement::ArrayDeclaration { name, element_type, elements, .. } => {
                type_map.insert(name.clone(), format!("Array<{}>", element_type));
                let mut formatted_elements = Vec::new();
                for el in elements {
                    let s = match el {
                        TokenKind::StringLiteral(val) => format!("\"{}\"", val),
                        TokenKind::NumberLiteral(val) => val.to_string(),
                        TokenKind::BooleanLiteral(val) => format!("\"{}\"", val), 
                        _ => "\"\"".to_string(),
                    };
                    formatted_elements.push(s);
                }
                writeln!(file, "{}declare -a {}=({})", indent, name.to_uppercase(), formatted_elements.join(" "))?;
            }
            
            Statement::PrintCall { target_name, .. } => {
                let var_type = type_map.get(&target_name).cloned().unwrap_or_else(|| "string".to_string());
                if var_type.starts_with("Array<") {
                    writeln!(file, "{}echo \"${{{}[@]}}\"", indent, target_name.to_uppercase())?;
                } else {
                    writeln!(file, "{}echo \"${{{}}}\"", indent, target_name.to_uppercase())?;
                }
            }

            Statement::ExecCall { command, .. } => {
                let mut translated_command = command.clone();
                let words: Vec<&str> = command.split_whitespace().collect();
                for word in words {
                    if word.starts_with('$') {
                        let clean_var = word.trim_start_matches('$').trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_');
                        let target_bash_var = format!("${{{}}}", clean_var.to_uppercase());
                        translated_command = translated_command.replace(word, &target_bash_var);
                    }
                }
                writeln!(file, "{}{}", indent, translated_command)?;
            }

            Statement::Comment { text } => { writeln!(file, "{}#{}", indent, text)?; }

            Statement::FunctionDeclaration { name, params, body, .. } => {
                writeln!(file, "{}{} () {{", indent, name)?;
                for (index, param) in params.iter().enumerate() {
                    writeln!(file, "{}    local {}={}", indent, param.name.to_uppercase(), format!("${}", index + 1))?;
                    type_map.insert(param.name.clone(), param.var_type.clone());
                }
                generate_statements(file, body, indent_level + 1, type_map)?;
                writeln!(file, "{}}}", indent)?;
                writeln!(file)?;
            }

            Statement::FunctionCall { name, args, .. } => {
                let mut formatted_args = Vec::new();
                for arg in args {
                    let formatted = match arg {
                        TokenKind::StringLiteral(s) => format!("\"{}\"", s),
                        TokenKind::NumberLiteral(n) => n.to_string(),
                        TokenKind::BooleanLiteral(b) => format!("\"{}\"", b),
                        TokenKind::Identifier(id) => format!("\"${{{}}}\"", id.to_uppercase()),
                        _ => "\"\"".to_string(),
                    };
                    formatted_args.push(formatted);
                }
                writeln!(file, "{}{} {}", indent, name, formatted_args.join(" "))?;
            }

            Statement::ForLoop { item_name, array_name, body, .. } => {
                type_map.insert(item_name.clone(), "string".to_string());
                writeln!(file, "{}for {} in \"${{{}[@]}}\"; do", indent, item_name.to_uppercase(), array_name.to_uppercase())?;
                generate_statements(file, body, indent_level + 1, type_map)?;
                writeln!(file, "{}done", indent)?;
            }

            // ─── ИСПРАВЛЕНО: Безопасное извлечение аргументов из Vec<TokenKind> ───
            Statement::StdLibCall { module, function, args, .. } => {
                if let Some(first_arg) = args.first() {
                    let arg_raw = match first_arg {
                        TokenKind::StringLiteral(s) => format!("\"{}\"", s),
                        TokenKind::Identifier(id) => format!("\"${{{}}}\"", id.to_uppercase()),
                        _ => "\"\"".to_string(),
                    };

                    if module == "file" && function == "exists" {
                        writeln!(file, "{}if [ -f {} ]; then echo \"File exists\"; fi", indent, arg_raw)?;
                    } else if module == "env" && function == "get" {
                        writeln!(file, "{}echo ${}", indent, arg_raw.replace('"', ""))?;
                    }
                }
            }

            Statement::If { conditions, logical_ops, body, else_body, .. } => {
                let mut bash_condition = String::new();
                for (i, cond) in conditions.iter().enumerate() {
                    let raw_val = match &cond.right_value {
                        TokenKind::StringLiteral(s) => s.clone(),
                        TokenKind::NumberLiteral(n) => n.to_string(),
                        TokenKind::BooleanLiteral(b) => b.to_string(),
                        TokenKind::Identifier(id) => format!("${{{}}}", id.to_uppercase()),
                        _ => "".to_string(),
                    };
                    let var_type = type_map.get(&cond.left_var).cloned().unwrap_or_else(|| "string".to_string());
                    let bash_op = if var_type == "number" {
                        match cond.op {
                            TokenKind::Equal => "-eq", TokenKind::NotEqual => "-ne",
                            TokenKind::LessThan => "-lt", TokenKind::GreaterThan => "-gt", _ => "==",
                        }
                    } else {
                        match cond.op { TokenKind::Equal => "==", TokenKind::NotEqual => "!=", _ => "==", }
                    };
                    let cond_str = format!("[ \"${{{}}}\" {} \"{}\" ]", cond.left_var.to_uppercase(), bash_op, raw_val);
                    bash_condition.push_str(&cond_str);
                    if i < logical_ops.len() {
                        let op_str = match logical_ops[i] { TokenKind::And => " && ", TokenKind::Or => " || ", _ => " && ", };
                        bash_condition.push_str(op_str);
                    }
                }
                writeln!(file, "{}if {}; then", indent, bash_condition)?;
                generate_statements(file, body, indent_level + 1, type_map)?;
                if let Some(e_body) = else_body {
                    writeln!(file, "{}else", indent)?;
                    generate_statements(file, e_body, indent_level + 1, type_map)?;
                }
                writeln!(file, "{}fi", indent)?;
            }
        }
    }
    Ok(())
}
