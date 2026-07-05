use std::fs;
use std::io;

const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

pub struct FileStats {
    pub file_path: String,
    pub content: String,
    pub byte_count: usize,
    pub line_count: usize,
}

impl FileStats {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let content = fs::read_to_string(path)?;

        let byte_count = content.len();
        let line_count = content.lines().count();

        Ok(Self {
            file_path: path.to_string(),
            content,
            byte_count,
            line_count,
        })
    }

    pub fn print_stats(&self, count_lines: bool) {
        println!("{}Содержимое файла:{}", GREEN, RESET);
        println!("---");
        print!("{}", self.content);
        println!("---");

        if count_lines {
            println!(
                "{}Статистика:{} прочитано {} строк(и).",
                YELLOW, RESET, self.line_count
            );
        } else {
            println!(
                "{}Статистика:{} прочитано {} байт.",
                YELLOW, RESET, self.byte_count
            );
        }
    }
}