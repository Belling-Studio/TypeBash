// src/error.rs
use colored::*;

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct TypeBashError {
    pub kind: DiagnosticKind,
    pub code: u32,
    pub line: usize,
    pub file_path: String,
    pub message: String,
    pub help: String,
}

impl TypeBashError {
    pub fn new(code: u32, line: usize, file_path: &str, message: &str, help: &str) -> Self {
        Self {
            kind: DiagnosticKind::Error,
            code,
            line,
            file_path: file_path.to_string(),
            message: message.to_string(),
            help: help.to_string(),
        }
    }

    pub fn warning(code: u32, line: usize, file_path: &str, message: &str, help: &str) -> Self {
        Self {
            kind: DiagnosticKind::Warning,
            code,
            line,
            file_path: file_path.to_string(),
            message: message.to_string(),
            help: help.to_string(),
        }
    }

    pub fn render(&self) {
        println!();
        match self.kind {
            DiagnosticKind::Error => {
                let err_tag = format!("error[E{:03}]:", self.code).red().bold();
                let line_info = format!("--> {}:{}", self.file_path, self.line).bright_black().italic();
                println!("{} {} {}", err_tag, self.message.white().bold(), line_info);
                println!("  {} {}", "• help:".green().bold(), self.help.bright_black());
                // ИСПРАВЛЕНО: Вернули вывод пути для ошибок
                println!("  {} {}", "• path:".blue().bold(), self.file_path.underline().bright_black());
            }
            DiagnosticKind::Warning => {
                let warn_tag = format!("warning[W{:03}]:", self.code).yellow().bold();
                let line_info = format!("--> {}:{}", self.file_path, self.line).bright_black().italic();
                println!("{} {} {}", warn_tag, self.message.white().bold(), line_info);
                println!("  {} {}", "• info:".blue().bold(), self.help.bright_black());
                // ИСПРАВЛЕНО: Вернули вывод пути для ворнингов
                println!("  {} {}", "• path:".blue().bold(), self.file_path.underline().bright_black());
            }
        }
        println!();
    }
}
