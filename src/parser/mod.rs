// src/parser/mod.rs
pub mod statements;
pub mod blocks;
pub mod calls;

use crate::lexer::{Token, TokenKind};
use crate::error::TypeBashError;

#[derive(Debug, Clone)]
pub struct Condition {
    pub left_var: String,
    pub op: TokenKind,
    pub right_value: TokenKind,
}

#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub name: String,
    pub var_type: String,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        var_type: String,
        value: TokenKind, 
        line: usize,
    },
    ArrayDeclaration {
        name: String,
        element_type: String, 
        elements: Vec<TokenKind>,
        line: usize,
    },
    PrintCall {
        target_name: String,
        line: usize,
    },
    ExecCall {
        command: String,  
        line: usize,
    },
    Comment {
        text: String,
    },
    If {
        conditions: Vec<Condition>, 
        logical_ops: Vec<TokenKind>, 
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>, 
        line: usize,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<FunctionParam>,
        body: Vec<Statement>,
        line: usize,
    },
    FunctionCall {
        name: String,
        args: Vec<TokenKind>, 
        line: usize,
    },
    // Восстанавливаем варианты для циклов и StdLib
    ForLoop {
        item_name: String,
        array_name: String,
        body: Vec<Statement>,
        line: usize,
    },
    StdLibCall {
        module: String,
        function: String,
        args: Vec<TokenKind>,
        line: usize,
    },
}

pub fn parse(file_path: &str, tokens: Vec<Token>) -> Result<Vec<Statement>, TypeBashError> {
    let mut iter = tokens.into_iter().peekable();
    statements::parse_statements(file_path, &mut iter)
}
