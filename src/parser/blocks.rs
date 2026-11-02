// src/parser/blocks.rs
use crate::lexer::{Token, TokenKind};
use crate::error::TypeBashError;
use super::{Statement, Condition, FunctionParam};
use super::statements::parse_statements;

/// Парсинг цикла for item in array { ... }
pub fn parse_for<I>(file_path: &str, iter: &mut std::iter::Peekable<I>, current_line: usize) -> Result<Statement, TypeBashError>
where
    I: Iterator<Item = Token>,
{
    iter.next(); // Потребляем 'for'
    
    let item_name = match iter.next().map(|t| t.kind) {
        Some(TokenKind::Identifier(id)) => id,
        _ => return Err(TypeBashError::new(360, current_line, file_path, "Expected variable name after 'for'", "Provide an iterator variable name.")),
    };

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::In) => {},
        _ => return Err(TypeBashError::new(361, current_line, file_path, "Expected keyword 'in' after variable", "Add 'in' keyword.")),
    };

    let array_name = match iter.next().map(|t| t.kind) {
        Some(TokenKind::Identifier(id)) => id,
        _ => return Err(TypeBashError::new(362, current_line, file_path, "Expected array name to iterate over", "Provide a valid array variable.")),
    };

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::OpenBrace) => {},
        _ => return Err(TypeBashError::new(363, current_line, file_path, "Expected '{' to start the 'for' loop body", "Add an open curly brace.")),
    };

    let body = parse_statements(file_path, iter)?;

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::CloseBrace) => {},
        _ => return Err(TypeBashError::new(364, current_line, file_path, "Expected '}' to close the 'for' loop body", "Add a closing curly brace.")),
    };

    Ok(Statement::ForLoop { item_name, array_name, body, line: current_line })
}

pub fn parse_function<I>(file_path: &str, iter: &mut std::iter::Peekable<I>, current_line: usize) -> Result<Statement, TypeBashError>
where
    I: Iterator<Item = Token>,
{
    iter.next(); 
    let name = match iter.next().map(|t| t.kind) {
        Some(TokenKind::Identifier(id)) => id,
        _ => return Err(TypeBashError::new(330, current_line, file_path, "Expected function name after 'fn'", "Provide a valid identifier.")),
    };

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::OpenParen) => {},
        _ => return Err(TypeBashError::new(331, current_line, file_path, "Expected '(' after function name", "Open parenthesis.")),
    };

    let mut params = Vec::new();
    if let Some(t) = iter.peek() {
        if t.kind != TokenKind::CloseParen {
            loop {
                let p_name = match iter.next().map(|t| t.kind) {
                    Some(TokenKind::Identifier(id)) => id,
                    _ => return Err(TypeBashError::new(332, current_line, file_path, "Expected parameter name", "Name your parameter.")),
                };
                match iter.next().map(|t| t.kind) {
                    Some(TokenKind::Colon) => {},
                    _ => return Err(TypeBashError::new(333, current_line, file_path, "Expected ':' after parameter name", "Specify a type.")),
                };
                let p_type = match iter.next().map(|t| t.kind) {
                    Some(TokenKind::Type(t)) => t,
                    _ => return Err(TypeBashError::new(334, current_line, file_path, "Expected parameter type", "Provide a valid type.")),
                };

                params.push(FunctionParam { name: p_name, var_type: p_type });

                if let Some(next_t) = iter.peek() {
                    if next_t.kind == TokenKind::Comma {
                        iter.next(); 
                        continue;
                    }
                }
                break;
            }
        }
    }

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::CloseParen) => {},
        _ => return Err(TypeBashError::new(335, current_line, file_path, "Expected ')' after parameters", "Close parenthesis.")),
    };

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::OpenBrace) => {},
        _ => return Err(TypeBashError::new(336, current_line, file_path, "Expected '{' to start function body", "Add an open brace.")),
    };

    let body = parse_statements(file_path, iter)?;

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::CloseBrace) => {},
        _ => return Err(TypeBashError::new(337, current_line, file_path, "Expected '}' to close function body", "Add a closing brace.")),
    };

    Ok(Statement::FunctionDeclaration { name, params, body, line: current_line })
}

pub fn parse_if<I>(file_path: &str, iter: &mut std::iter::Peekable<I>, current_line: usize) -> Result<Statement, TypeBashError>
where
    I: Iterator<Item = Token>,
{
    iter.next(); 
    let mut conditions = Vec::new();
    let mut logical_ops = Vec::new();

    loop {
        let left_var = match iter.next().map(|t| t.kind) {
            Some(TokenKind::Identifier(id)) => id,
            _ => return Err(TypeBashError::new(310, current_line, file_path, "Expected variable name in condition", "Provide a declared variable.")),
        };

        let op = match iter.next().map(|t| t.kind) {
            Some(val @ TokenKind::Equal) |
            Some(val @ TokenKind::NotEqual) |
            Some(val @ TokenKind::LessThan) |
            Some(val @ TokenKind::GreaterThan) => val,
            _ => return Err(TypeBashError::new(311, current_line, file_path, "Expected comparison operator (==, !=, <, >)", "Use a valid operator.")),
        };

        let right_value = match iter.next().map(|t| t.kind) {
            Some(val @ TokenKind::StringLiteral(_)) | 
            Some(val @ TokenKind::NumberLiteral(_)) | 
            Some(val @ TokenKind::BooleanLiteral(_)) |
            Some(val @ TokenKind::Identifier(_)) => val, 
            _ => return Err(TypeBashError::new(312, current_line, file_path, "Expected literal value or variable name to compare with", "Provide string, number, boolean, or variable.")),
        };

        conditions.push(Condition { left_var, op, right_value });

        if let Some(t) = iter.peek() {
            if t.kind == TokenKind::And || t.kind == TokenKind::Or {
                logical_ops.push(t.kind.clone());
                iter.next(); 
                continue;
            }
        }
        break;
    }

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::OpenBrace) => {},
        _ => return Err(TypeBashError::new(313, current_line, file_path, "Expected '{'", "Add an open curly brace.")),
    };

    let body = parse_statements(file_path, iter)?;

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::CloseBrace) => {},
        _ => return Err(TypeBashError::new(314, current_line, file_path, "Expected '}'", "Add a closing curly brace.")),
    };

    let mut else_body = None;
    if let Some(t) = iter.peek() {
        if t.kind == TokenKind::Else {
            iter.next(); 
            
            match iter.next().map(|t| t.kind) {
                Some(TokenKind::OpenBrace) => {},
                _ => return Err(TypeBashError::new(315, current_line, file_path, "Expected '{' after 'else'", "Add an open curly brace.")),
            };

            let e_body = parse_statements(file_path, iter)?;

            match iter.next().map(|t| t.kind) {
                Some(TokenKind::CloseBrace) => {},
                _ => return Err(TypeBashError::new(316, current_line, file_path, "Expected '}' to close 'else'", "Add a closing curly brace.")),
            };
            
            else_body = Some(e_body);
        }
    }

    Ok(Statement::If { conditions, logical_ops, body, else_body, line: current_line })
}
