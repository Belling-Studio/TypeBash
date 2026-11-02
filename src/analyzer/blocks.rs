// src/analyzer/blocks.rs
use crate::parser::{Statement, FunctionParam};
use crate::lexer::TokenKind;
use crate::error::TypeBashError;
use super::statements::analyze_statements;
use std::collections::HashMap;

pub fn analyze_for(
    file_path: &str,
    stmt: &Statement,
    scopes: &mut Vec<HashMap<String, String>>,
    function_signatures: &mut HashMap<String, Vec<FunctionParam>>,
    diagnostics: &mut Vec<TypeBashError>, // Принимаем вектор диагностик
) {
    if let Statement::ForLoop { item_name, array_name, body, line } = stmt {
        let mut array_type = None;
        for scope in scopes.iter().rev() {
            if let Some(t) = scope.get(array_name) {
                array_type = Some(t.clone());
                break;
            }
        }

        let array_type = match array_type {
            Some(t) => t,
            None => {
                diagnostics.push(TypeBashError::new(461, *line, file_path, &format!("Cannot find array '{}' to iterate over.", array_name), "Declare the array variable first."));
                return;
            }
        };

        if !array_type.starts_with("Array<") {
            diagnostics.push(TypeBashError::new(462, *line, file_path, &format!("Variable '{}' is of type '{}', but 'for' loop expects an Array.", array_name, array_type), "You can only iterate over Array types."));
            return;
        }

        let element_type = array_type.trim_start_matches("Array<").trim_end_matches('>');

        let mut loop_scope = HashMap::new();
        loop_scope.insert(item_name.clone(), element_type.to_string());

        scopes.push(loop_scope);
        // ИСПРАВЛЕНО: передаем diagnostics и убираем оператор '?'
        analyze_statements(file_path, body, scopes, function_signatures, diagnostics);
        scopes.pop();
    }
}

pub fn analyze_function(
    file_path: &str,
    stmt: &Statement,
    scopes: &mut Vec<HashMap<String, String>>,
    function_signatures: &mut HashMap<String, Vec<FunctionParam>>,
    diagnostics: &mut Vec<TypeBashError>, // Принимаем вектор диагностик
) {
    if let Statement::FunctionDeclaration { name, params, body, .. } = stmt {
        function_signatures.insert(name.clone(), params.clone());

        let mut function_scope = HashMap::new();
        for param in params {
            function_scope.insert(param.name.clone(), param.var_type.clone());
        }

        scopes.push(function_scope);
        // ИСПРАВЛЕНО: передаем diagnostics и убираем оператор '?'
        analyze_statements(file_path, body, scopes, function_signatures, diagnostics);
        scopes.pop();
    }
}

pub fn analyze_function_call(
    file_path: &str,
    stmt: &Statement,
    scopes: &mut Vec<HashMap<String, String>>,
    function_signatures: &HashMap<String, Vec<FunctionParam>>,
    diagnostics: &mut Vec<TypeBashError>, // Принимаем вектор диагностик
) {
    if let Statement::FunctionCall { name, args, line } = stmt {
        let expected_params = match function_signatures.get(name) {
            Some(params) => params.clone(),
            None => {
                diagnostics.push(TypeBashError::new(420, *line, file_path, &format!("Cannot find function '{}'.", name), "Declare the function."));
                return;
            }
        };

        if expected_params.len() != args.len() {
            diagnostics.push(TypeBashError::new(421, *line, file_path, &format!("Expected {} arguments for function '{}'.", expected_params.len(), name), "Pass exact arguments."));
            return;
        }

        for (i, arg) in args.iter().enumerate() {
            let expected_type = &expected_params[i].var_type;
            let actual_type = match arg {
                TokenKind::StringLiteral(_) => "string".to_string(),
                TokenKind::NumberLiteral(_) => "number".to_string(),
                TokenKind::BooleanLiteral(_) => "boolean".to_string(),
                TokenKind::Identifier(var_name) => {
                    let mut v_type = None;
                    for scope in scopes.iter().rev() {
                        if let Some(t) = scope.get(var_name) { v_type = Some(t.clone()); break; }
                    }
                    match v_type {
                        Some(t) => t,
                        None => {
                            diagnostics.push(TypeBashError::new(422, *line, file_path, &format!("Undefined variable '{}'.", var_name), "Declare variable."));
                            continue;
                        }
                    }
                }
                _ => "unknown".to_string(),
            };

            if expected_type != &actual_type && !(expected_type == "Path" && actual_type == "string") {
                diagnostics.push(TypeBashError::new(423, *line, file_path, &format!("Argument {} of '{}' mismatch.", i + 1, name), "Fix types."));
            }
        }
    }
}

pub fn analyze_if(
    file_path: &str,
    stmt: &Statement,
    scopes: &mut Vec<HashMap<String, String>>,
    function_signatures: &mut HashMap<String, Vec<FunctionParam>>,
    diagnostics: &mut Vec<TypeBashError>, // Принимаем вектор диагностик
) {
    if let Statement::If { conditions, body, else_body, line, .. } = stmt {
        for cond in conditions {
            let mut left_type = None;
            for scope in scopes.iter().rev() {
                if let Some(t) = scope.get(&cond.left_var) { left_type = Some(t.clone()); break; }
            }
            let left_type = match left_type {
                Some(t) => t,
                None => {
                    diagnostics.push(TypeBashError::new(404, *line, file_path, &format!("Undefined variable '{}'.", cond.left_var), "Fix variable."));
                    return;
                }
            };

            let right_type = match &cond.right_value {
                TokenKind::StringLiteral(_) => "string".to_string(),
                TokenKind::NumberLiteral(_) => "number".to_string(),
                TokenKind::BooleanLiteral(_) => "boolean".to_string(),
                TokenKind::Identifier(right_var_name) => {
                    let mut r_type = None;
                    for scope in scopes.iter().rev() {
                        if let Some(t) = scope.get(right_var_name) { r_type = Some(t.clone()); break; }
                    }
                    match r_type {
                        Some(t) => t,
                        None => {
                            diagnostics.push(TypeBashError::new(411, *line, file_path, &format!("Cannot find '{}'.", right_var_name), "Fix variable."));
                            return;
                        }
                    }
                }
                _ => "unknown".to_string(),
            };

            if left_type != right_type && !(left_type == "Path" && right_type == "string") {
                diagnostics.push(TypeBashError::new(412, *line, file_path, "Type mismatch in 'if'.", "Fix types."));
            }
        }

        scopes.push(HashMap::new());
        // ИСПРАВЛЕНО: передаем diagnostics и убираем оператор '?'
        analyze_statements(file_path, body, scopes, function_signatures, diagnostics);
        scopes.pop();

        if let Some(e_body) = else_body {
            scopes.push(HashMap::new());
            // ИСПРАВЛЕНО: передаем diagnostics и убираем оператор '?'
            analyze_statements(file_path, e_body, scopes, function_signatures, diagnostics);
            scopes.pop();
        }
    }
}
