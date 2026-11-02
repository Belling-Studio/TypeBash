// src/main.rs
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;

mod error;
mod lexer;
mod parser;
mod analyzer;
mod codegen;

#[derive(Parser)]
#[command(name = "typebash")]
#[command(about = "TypeBash Compiler - TypeScript for Bash", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a .tb file into a secure .sh script
    Build {
        /// Path to the TypeBash file (e.g., main.tb)
        input: String,
        
        /// Bash version compatibility target (bash3, bash4, bash5)
        #[arg(long, default_value = "bash4")]
        target: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { input, target: _target } => {
            // Выводим только минималистичный статус компиляции в стиле Rust (Compiling...)
            println!("{} typebash v0.5.0-beta ({})", "Compiling".green().bold(), input);

            // 1. Читаем исходный код с диска
            let source_code = match fs::read_to_string(input) {
                Ok(content) => content,
                Err(err) => {
                    println!("❌ Failed to open file '{}': {}", input, err);
                    std::process::exit(1);
                }
            };

            // 2. Запускаем Лексер
            match lexer::tokenize(input, &source_code) {
                Ok(tokens) => {
                    // 3. Запускаем Парсер
                    match parser::parse(input, tokens) {
                        Ok(ast) => {
                            // 4. Запускаем Анализатор типов
                            match analyzer::check_types(input, &ast) {
                                Ok(_) => {
                                    // ВСЁ ИДЕАЛЬНО: Вот теперь запускаем красивый визуал генерации
                                    let path = Path::new(input);
                                    let output_filename = path.with_extension("sh").to_string_lossy().to_string();
                                    
                                    let pb = ProgressBar::new(100);
                                    pb.set_style(ProgressStyle::default_bar()
                                        .template("{msg} [{bar:25.cyan/blue}] {pos}%")
                                        .unwrap()
                                        .progress_chars("-> "));
                                    pb.set_message("Generating");

                                    for _ in 0..100 {
                                        pb.inc(1);
                                        sleep(Duration::from_millis(8));
                                    }
                                    pb.finish_and_clear();

                                    // 5. Запускаем Кодогенератор
                                    match codegen::generate(&output_filename, ast) {
                                        Ok(_) => {
                                            // Строгий и эстетичный финальный статус как в Cargo!
                                            println!("{} build [secure] target(s) in .sh format ✨", "Finished".green().bold());
                                        }
                                        Err(err) => {
                                            println!("❌ Error writing output file: {}", err);
                                            std::process::exit(1);
                                        }
                                    }
                                }
                                Err(_) => {
                                    // ЕСТЬ ОШИБКА ТИПОВ: Анализатор уже сам отрендерил ошибку.
                                    // Больше ничего не пишем, никаких лишних баров, просто выходим!
                                    std::process::exit(1);
                                }
                            }
                        }
                        Err(parser_error) => {
                            parser_error.render();
                            std::process::exit(1);
                        }
                    }
                }
                Err(lexer_error) => {
                    lexer_error.render();
                    std::process::exit(1);
                }
            }
        }
    }
}
