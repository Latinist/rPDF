use std::fs;
use std::process;
use clap::Parser;

const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

#[derive(Parser)]
#[command(name = "text-tool")]
#[command(about = "Читает текстовый файл и выводит статистику")]
struct Cli {
    /// Путь к текстовому файлу
    file: String,

    /// Считать строки вместо байт
    #[arg(short = 'l', long = "count-lines")]
    count_lines: bool,
}

fn main() {
    let cli = Cli::parse();

    let content = match fs::read_to_string(&cli.file) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Ошибка: не удалось открыть файл \"{}\": {}", cli.file, e);
            process::exit(1);
        }
    };

    println!("{}Содержимое файла:{}", GREEN, RESET);
    println!("---");
    print!("{}", content);
    println!("---");

    if cli.count_lines {
        let line_count = content.lines().count();
        println!("{}Статистика:{} прочитано {} строк(и).", YELLOW, RESET, line_count);
    } else {
        let byte_count = content.len();
        println!("{}Статистика:{} прочитано {} байт.", YELLOW, RESET, byte_count);
    }
}