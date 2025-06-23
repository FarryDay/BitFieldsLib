# bitfieldslib - Библиотека для работы с битовыми флагами на Rust.

### Есть возможность так-же использовать библиотеку в #[no_std] моде!

## Установка

Добавьте в Cargo.toml:

```toml
[dependencies]
bitfieldslib = "1.0.0"
```

## Использование

```rust
use bitfieldslib::BitField;

// Определяем флаги
const READ: u32 = 1 << 0;
const WRITE: u32 = 1 << 1;
const EXECUTE: u32 = 1 << 1;

fn main() {
    // Создаем новые флаги
    let mut flags = BitField::new();

    // Устанавливаем флаги
    flags.set(READ | WRITE);

    // Проверяем флаги
    println!("{}", flags.contains(READ));
    println!("{}", !flags.contains(EXECUTE));

    // Переключаем флаг
    flags.toggle(WRITE);
    println!("{}", !flags.contains(WRITE));
}
```
