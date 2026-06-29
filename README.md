# 🦀 Rust PDF Service — Диссертация

Путь Java-разработчика в Rust через три акта:

1. **Акт I: `text-tool`** — Консольный обработчик данных.  
   Изучаем владение, модули, трейты.

2. **Акт II: `doc-service`** — REST CRUD API.  
   Изучаем асинхронность, веб-фреймворки, работу с БД.

3. **Акт III: `pdf-engine`** — Сервис генерации PDF.  
   Строим сложный, системный микросервис с фоновыми задачами.


## Локальный запуск проверок
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release