// src/lexer.rs
use crate::error::TypeBashError;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Let, Print, If, Else, Exec, Fn, 
    For,        // <-- НОВОЕ: for
    In,         // <-- НОВОЕ: in
    OpenParen, CloseParen, OpenBrace, CloseBrace, OpenBracket, CloseBracket,
    Identifier(String),
    Colon,
    DoubleColon, // <-- НОВОЕ: :: для StdLib (например, file::exists)
    Comma,      
    Type(String),
    Assign, Equal, NotEqual, LessThan, GreaterThan, And, Or,         
    StringLiteral(String),
    NumberLiteral(i64),
    BooleanLiteral(bool), 
    Comment(String), 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

pub fn tokenize(file_path: &str, source: &str) -> Result<Vec<Token>, TypeBashError> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;

    while let Some(&ch) = chars.peek() {
        let mut add_token = |kind: TokenKind| {
            tokens.push(Token { kind, line });
        };

        match ch {
            '\n' => { line += 1; chars.next(); }
            ' ' | '\r' | '\t' => { chars.next(); }
            ':' => {
                chars.next();
                if let Some(&':') = chars.peek() {
                    chars.next();
                    add_token(TokenKind::DoubleColon);
                } else {
                    add_token(TokenKind::Colon);
                }
            }
            ',' => { add_token(TokenKind::Comma); chars.next(); }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    add_token(TokenKind::Equal);
                } else {
                    add_token(TokenKind::Assign);
                }
            }
            '!' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    add_token(TokenKind::NotEqual);
                } else {
                    return Err(TypeBashError::new(236, line, file_path, "Single '!' is not supported", "Use '!=' for not-equal comparison."));
                }
            }
            '<' => { add_token(TokenKind::LessThan); chars.next(); }
            '>' => { add_token(TokenKind::GreaterThan); chars.next(); }
            '&' => {
                chars.next();
                if let Some(&'&') = chars.peek() { chars.next(); add_token(TokenKind::And); }
                else { return Err(TypeBashError::new(237, line, file_path, "Single '&' is not supported", "Use '&&'.")); }
            }
            '|' => {
                chars.next();
                if let Some(&'|') = chars.peek() { chars.next(); add_token(TokenKind::Or); }
                else { return Err(TypeBashError::new(238, line, file_path, "Single '|' is not supported", "Use '||'.")); }
            }
            '(' => { add_token(TokenKind::OpenParen); chars.next(); }
            ')' => { add_token(TokenKind::CloseParen); chars.next(); }
            '{' => { add_token(TokenKind::OpenBrace); chars.next(); }
            '}' => { add_token(TokenKind::CloseBrace); chars.next(); }
            '[' => { add_token(TokenKind::OpenBracket); chars.next(); }
            ']' => { add_token(TokenKind::CloseBracket); chars.next(); }
            '/' => {
                chars.next(); 
                if let Some(&'/') = chars.peek() {
                    chars.next(); 
                    let mut comment_text = String::new();
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '\n' { break; }
                        comment_text.push(next_ch);
                        chars.next();
                    }
                    add_token(TokenKind::Comment(comment_text));
                } else {
                    return Err(TypeBashError::new(235, line, file_path, "Single '/' is not supported", "Use '//' for comments."));
                }
            }
            '"' => {
                chars.next(); 
                let mut string_val = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '"' { break; }
                    string_val.push(next_ch);
                    chars.next();
                }
                if chars.peek().is_none() {
                    return Err(TypeBashError::new(101, line, file_path, "Unclosed string literal", "Add a closing double quote (\")."));
                }
                chars.next(); 
                add_token(TokenKind::StringLiteral(string_val));
            }
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() { num_str.push(next_ch); chars.next(); }
                    else { break; }
                }
                let num = num_str.parse::<i64>().unwrap();
                add_token(TokenKind::NumberLiteral(num));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident_str = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_alphanumeric() || next_ch == '_' { ident_str.push(next_ch); chars.next(); }
                    else { break; }
                }
                
                match ident_str.as_str() {
                    "let" => add_token(TokenKind::Let),
                    "print" => add_token(TokenKind::Print),
                    "if" => add_token(TokenKind::If),
                    "else" => add_token(TokenKind::Else),
                    "exec" => add_token(TokenKind::Exec), 
                    "fn" => add_token(TokenKind::Fn), 
                    "for" => add_token(TokenKind::For), // <-- Ловим for
                    "in" => add_token(TokenKind::In),   // <-- Ловим in
                    "true" => add_token(TokenKind::BooleanLiteral(true)),  
                    "false" => add_token(TokenKind::BooleanLiteral(false)), 
                    "string" | "number" | "boolean" | "Path" | "Array" => add_token(TokenKind::Type(ident_str)),
                    _ => add_token(TokenKind::Identifier(ident_str)),
                }
            }
            _ => {
                return Err(TypeBashError::new(234, line, file_path, &format!("Unknown symbol \"{}\"", ch), "Use ASCII characters."));
            }
        }
    }

    Ok(tokens)
}
