use std::fs;
use std::process;
use clap::{Parser, Subcommand};
use shared::FileStats;
use docx_core::DocxContent;

#[derive(Parser)]
#[command(name = "text-tool")]
#[command(about = "Утилита для работы с текстовыми файлами и документами")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Читает текстовый файл и выводит статистику
    Read {
        /// Путь к текстовому файлу
        file: String,

        /// Считать строки вместо байт
        #[arg(short = 'l', long = "count-lines")]
        count_lines: bool,
    },
    /// Анализирует DOCX-файл и находит теги
    Docx {
        /// Путь к DOCX-файлу
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { file, count_lines } => {
            let stats = match FileStats::from_file(&file) {
                Ok(stats) => stats,
                Err(e) => {
                    eprintln!("Ошибка: не удалось открыть файл \"{}\": {}", file, e);
                    process::exit(1);
                }
            };
            stats.print_stats(count_lines);
        }
        Commands::Docx { file } => {
            // Читаем файл как байты
            let bytes = match fs::read(&file) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Ошибка: не удалось прочитать файл \"{}\": {}", file, e);
                    process::exit(1);
                }
            };

            // Парсим DOCX
            let docx = match DocxContent::from_bytes(&bytes) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Ошибка при обработке DOCX: {}", e);
                    process::exit(1);
                }
            };

            // Выводим результаты
            println!("Анализ DOCX-файла: {}\n", file);
            println!("Параграфов: {}\n", docx.paragraph_count);
            println!("Содержимое:");
            println!("---");
            println!("{}", docx.full_text);
            println!("---\n");

            if docx.tags.is_empty() {
                println!("Найдены теги: отсутствуют");
            } else {
                println!("Найдены теги:");
                for tag in &docx.tags {
                    println!("  - {}", tag);
                }
            }
        }
    }
}