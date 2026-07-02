use std::env;
use std::fs;
use std::process;

fn main() {
    // 1. Собираем аргументы командной строки
    let args: Vec<String> = env::args().collect();

    // 2. Ищем аргумент --file
    let file_path = match args.iter().position(|a| a == "--file") {
        Some(index) => {
            // --file найден, берём следующий аргумент
            if index + 1 < args.len() {
                args[index + 1].clone()
            } else {
                eprintln!("Ошибка: укажите путь к файлу после --file");
                process::exit(1);
            }
        }
        None => {
            eprintln!("Ошибка: укажите --file <путь>");
            process::exit(1);
        }
    };

    // 3. Читаем файл
    let content = match fs::read_to_string(&file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Ошибка: не удалось открыть файл \"{}\": {}", file_path, e);
            process::exit(1);
        }
    };

    // 4. Выводим содержимое
    println!("Содержимое файла:");
    println!("---");
    print!("{}", content);
    println!("---");

    // 5. Считаем байты
    let byte_count = content.len();
    println!("Прочитано {} байт.", byte_count);
}