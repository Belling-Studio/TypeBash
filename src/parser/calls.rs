// src/parser/calls.rs
use crate::lexer::{Token, TokenKind};
use crate::error::TypeBashError;
use super::Statement;

/// Парсинг встроенного вызова print(variable)
pub fn parse_print<I>(file_path: &str, iter: &mut std::iter::Peekable<I>, current_line: usize) -> Result<Statement, TypeBashError>
where
    I: Iterator<Item = Token>,
{
    iter.next(); // Потребляем 'print'
    match iter.next().map(|t| t.kind) {
        Some(TokenKind::OpenParen) => {},
        _ => return Err(TypeBashError::new(306, current_line, file_path, "Expected '(' after 'print'", "Use parentheses.")),
    };

    let target_name = match iter.next().map(|t| t.kind) {
        Some(TokenKind::Identifier(id)) => id,
        _ => return Err(TypeBashError::new(307, current_line, file_path, "Expected variable name inside print(...)", "Pass a variable name.")),
    };

    match iter.next().map(|t| t.kind) {
        Some(TokenKind::CloseParen) => {},
        _ => return Err(TypeBashError::new(308, current_line, file_path, "Expected ')' after variable name", "Close parenthesis.")),
    };

    Ok(Statement::PrintCall { target_name, line: current_line })
}

/// Парсинг вызовов пользовательских функций или стандартной библиотеки (StdLib)
pub fn parse_ident_call<I>(file_path: &str, iter: &mut std::iter::Peekable<I>, first_ident: String, current_line: usize) -> Result<Statement, TypeBashError>
where
    I: Iterator<Item = Token>,
{
    iter.next(); // Потребляем само имя (первый идентификатор)

    // Вариант 1: Вызов стандартной библиотеки через двойное двоеточие (например, file::exists)
    if let Some(TokenKind::DoubleColon) = iter.peek().map(|t| &t.kind) {
        iter.next(); // Потребляем '::'
        let func_ident = match iter.next().map(|t| t.kind) {
            Some(TokenKind::Identifier(id)) => id,
            _ => return Err(TypeBashError::new(370, current_line, file_path, "Expected function name after '::'", "Provide a valid StdLib function.")),
        };

        match iter.next().map(|t| t.kind) {
            Some(TokenKind::OpenParen) => {},
            _ => return Err(TypeBashError::new(371, current_line, file_path, "Expected '(' to open StdLib arguments", "Open parenthesis.")),
        };

        let mut args = Vec::new();
        if let Some(t) = iter.peek() {
            if t.kind != TokenKind::CloseParen {
                loop {
                    let arg = match iter.next().map(|t| t.kind) {
                        Some(val @ TokenKind::StringLiteral(_)) |
                        Some(val @ TokenKind::Identifier(_)) => val,
                        _ => return Err(TypeBashError::new(372, current_line, file_path, "Expected string literal or variable in StdLib call", "Check arguments.")),
                    };
                    args.push(arg);
                    if let Some(next_t) = iter.peek() {
                        if next_t.kind == TokenKind::Comma { iter.next(); continue; }
                    }
                    break;
                }
            }
        }

        match iter.next().map(|t| t.kind) {
            Some(TokenKind::CloseParen) => {},
            _ => return Err(TypeBashError::new(373, current_line, file_path, "Expected ')' to close StdLib call", "Close parenthesis.")),
        };

        return Ok(Statement::StdLibCall { module: first_ident, function: func_ident, args, line: current_line });
    } 
    
    // Вариант 2: Обычный вызов пользовательской функции (например, deployApp(dir))
    if let Some(TokenKind::OpenParen) = iter.peek().map(|t| &t.kind) {
        iter.next(); // Потребляем '('
        let mut args = Vec::new();
        if let Some(t) = iter.peek() {
            if t.kind != TokenKind::CloseParen {
                loop {
                    let arg = match iter.next().map(|t| t.kind) {
                        Some(val @ TokenKind::StringLiteral(_)) |
                        Some(val @ TokenKind::NumberLiteral(_)) |
                        Some(val @ TokenKind::BooleanLiteral(_)) |
                        Some(val @ TokenKind::Identifier(_)) => val,
                        _ => return Err(TypeBashError::new(340, current_line, file_path, "Expected a valid argument (literal or variable) in function call", "Check arguments.")),
                    };
                    args.push(arg);
                    if let Some(next_t) = iter.peek() {
                        if next_t.kind == TokenKind::Comma { iter.next(); continue; }
                    }
                    break;
                }
            }
        }
        match iter.next().map(|t| t.kind) {
            Some(TokenKind::CloseParen) => {},
            _ => return Err(TypeBashError::new(341, current_line, file_path, "Expected ')' to close function arguments", "Close parenthesis.")),
        };
        
        return Ok(Statement::FunctionCall { name: first_ident, args, line: current_line });
    }

    Err(TypeBashError::new(300, current_line, file_path, &format!("Unexpected expression '{}'.", first_ident), "Check your code syntax."))
}
