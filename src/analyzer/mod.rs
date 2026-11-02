// src/analyzer/mod.rs
pub mod statements;
pub mod blocks;

use crate::parser::Statement;
use crate::error::TypeBashError;
use std::collections::HashMap;

/// Теперь возвращает вектор ВСЕХ найденных ошибок и варнингов
pub fn check_types(file_path: &str, ast: &[Statement]) -> Result<(), Vec<TypeBashError>> {
    let mut scopes = vec![HashMap::new()];
    let mut function_signatures = HashMap::new();
    let mut diagnostics = Vec::new(); // Сюда собираем всё
    
    statements::analyze_statements(file_path, ast, &mut scopes, &mut function_signatures, &mut diagnostics);

    // Если среди собранных диагностик есть хотя бы одна ОШИБКА — блокируем сборку
    let has_errors = diagnostics.iter().any(|d| matches!(d.kind, crate::error::DiagnosticKind::Error));
    
    // Но выводим мы ВСЁ (и ошибки, и ворнинги), что успели найти!
    if !diagnostics.is_empty() {
        for diag in &diagnostics {
            diag.render();
        }
    }

    if has_errors {
        Err(diagnostics)
    } else {
        Ok(())
    }
}
