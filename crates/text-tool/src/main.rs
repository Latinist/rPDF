use std::process;
use clap::Parser;
use shared::FileStats;

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

    let stats = match FileStats::from_file(&cli.file) {
        Ok(stats) => stats,
        Err(e) => {
            eprintln!("Ошибка: не удалось открыть файл \"{}\": {}", cli.file, e);
            process::exit(1);
        }
    };

    stats.print_stats(cli.count_lines);
}