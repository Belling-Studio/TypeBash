// src/parser/statements.rs
use crate::lexer::{Token, TokenKind};
use crate::error::TypeBashError;
use super::{Statement, blocks, calls};

pub fn parse_statements<I>(file_path: &str, iter: &mut std::iter::Peekable<I>) -> Result<Vec<Statement>, TypeBashError>
where
    I: Iterator<Item = Token>,
{
    let mut ast = Vec::new();

    while let Some(token) = iter.peek() {
        let current_line = token.line; 

        match &token.kind {
            TokenKind::CloseBrace => break,

            TokenKind::Let => {
                iter.next(); 
                let name = match iter.next().map(|t| (t.kind, t.line)) {
                    Some((TokenKind::Identifier(id), _)) => id,
                    _ => return Err(TypeBashError::new(301, current_line, file_path, "Expected variable name", "Provide a valid identifier.")),
                };

                match iter.next().map(|t| t.kind) {
                    Some(TokenKind::Colon) => {},
                    _ => return Err(TypeBashError::new(302, current_line, file_path, "Expected ':'", "Add a colon.")),
                };

                let mut var_type = match iter.next().map(|t| t.kind) {
                    Some(TokenKind::Type(t)) => t,
                    _ => return Err(TypeBashError::new(303, current_line, file_path, "Expected data type", "Specify a valid type.")),
                };

                let mut is_array = false;
                let mut element_type = String::new();

                if var_type == "Array" {
                    is_array = true;
                    match iter.next().map(|t| t.kind) {
                        Some(TokenKind::LessThan) => {},
                        _ => return Err(TypeBashError::new(350, current_line, file_path, "Expected '<' after 'Array'", "Use Array<type>.")),
                    };
                    element_type = match iter.next().map(|t| t.kind) {
                        Some(TokenKind::Type(t)) => t,
                        _ => return Err(TypeBashError::new(351, current_line, file_path, "Expected array element type", "Provide valid type.")),
                    };
                    match iter.next().map(|t| t.kind) {
                        Some(TokenKind::GreaterThan) => {},
                        _ => return Err(TypeBashError::new(352, current_line, file_path, "Expected '>'", "Close type.")),
                    };
                    var_type = format!("Array<{}>", element_type);
                }

                match iter.next().map(|t| t.kind) {
                    Some(TokenKind::Assign) => {},
                    _ => return Err(TypeBashError::new(304, current_line, file_path, "Expected '='", "Add '='.")),
                };

                if is_array {
                    match iter.next().map(|t| t.kind) {
                        Some(TokenKind::OpenBracket) => {},
                        _ => return Err(TypeBashError::new(353, current_line, file_path, "Expected '['", "Initialize with [...]")),
                    };

                    let mut elements = Vec::new();
                    if let Some(t) = iter.peek() {
                        if t.kind != TokenKind::CloseBracket {
                            loop {
                                let el = match iter.next().map(|t| t.kind) {
                                    Some(val @ TokenKind::StringLiteral(_)) |
                                    Some(val @ TokenKind::NumberLiteral(_)) |
                                    Some(val @ TokenKind::BooleanLiteral(_)) => val,
                                    _ => return Err(TypeBashError::new(354, current_line, file_path, "Expected literal value inside array", "Fix literal.")),
                                };
                                elements.push(el);

                                if let Some(next_t) = iter.peek() {
                                    if next_t.kind == TokenKind::Comma { iter.next(); continue; }
                                }
                                break;
                            }
                        }
                    }

                    match iter.next().map(|t| t.kind) {
                        Some(TokenKind::CloseBracket) => {},
                        _ => return Err(TypeBashError::new(355, current_line, file_path, "Expected ']'", "Close array.")),
                    };

                    ast.push(Statement::ArrayDeclaration { name, element_type, elements, line: current_line });
                } else {
                    let value = match iter.next().map(|t| t.kind) {
                        Some(val @ TokenKind::StringLiteral(_)) | 
                        Some(val @ TokenKind::NumberLiteral(_)) | 
                        Some(val @ TokenKind::BooleanLiteral(_)) => val,
                        _ => return Err(TypeBashError::new(305, current_line, file_path, "Expected literal value", "Provide value.")),
                    };
                    ast.push(Statement::VariableDeclaration { name, var_type, value, line: current_line });
                }
            }

            TokenKind::Exec => {
                iter.next(); 
                let command = match iter.next().map(|t| t.kind) {
                    Some(TokenKind::StringLiteral(cmd)) => cmd,
                    _ => return Err(TypeBashError::new(320, current_line, file_path, "Expected command string", "Provide string.")),
                };
                ast.push(Statement::ExecCall { command, line: current_line });
            }

            TokenKind::Comment(text) => {
                ast.push(Statement::Comment { text: text.clone() });
                iter.next();
            }

            TokenKind::Print => {
                let print_node = calls::parse_print(file_path, iter, current_line)?;
                ast.push(print_node);
            }

            TokenKind::Fn => {
                let func_node = blocks::parse_function(file_path, iter, current_line)?;
                ast.push(func_node);
            }

            TokenKind::For => {
                let for_node = blocks::parse_for(file_path, iter, current_line)?;
                ast.push(for_node);
            }

            TokenKind::If => {
                let if_node = blocks::parse_if(file_path, iter, current_line)?;
                ast.push(if_node);
            }

            // ИСПРАВЛЕНО: сначала извлекаем имя через клонирование, закрывая неизменяемое заимствование peek
            TokenKind::Identifier(name) => {
                let name_clone = name.clone();
                let call_node = calls::parse_ident_call(file_path, iter, name_clone, current_line)?;
                ast.push(call_node);
            }

            _ => {
                return Err(TypeBashError::new(300, current_line, file_path, "Unexpected token outside statement", "Check your code."));
            }
        }
    }

    Ok(ast)
}
