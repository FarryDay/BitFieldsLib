use bitfieldslib::BitField;

// Определяем флаги
const READ: u32 = 1 << 0;
const WRITE: u32 = 1 << 1;
const EXECUTE: u32 = 1 << 1;

fn main() {
    // Создаем новые флаги
    let mut permissions = BitField::new();

    // Устанавливаем флаги
    permissions.set(READ | WRITE);

    // Проверяем флаги
    println!("{}", permissions.contains(READ));
    println!("{}", !permissions.contains(EXECUTE));

    // Переключаем флаг
    permissions.toggle(WRITE);
    println!("{}", !permissions.contains(WRITE));
}
