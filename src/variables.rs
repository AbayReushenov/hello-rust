pub fn variables_example() {
    // Неизменяемые переменные
    let name = "Абай";
    let age: u32 = 58;

    // Изменяемые переменные
    let mut counter = 0;
    counter += 1;

    // Константы
    const MAX_POINTS: u32 = 100_000;

    println!("Имя: {}, Возраст: {}", name, age);
    println!("Счетчик: {}, Максимум: {}", counter, MAX_POINTS);
}
