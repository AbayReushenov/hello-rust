pub fn greet(name: &str) -> String {
    format!("Привет, {}!", name)
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b // без точки с запятой - возвращает значение
}

pub fn functions_example() {
    let greeting = greet("Rust разработчик");
    let sum = add(5, 10);

    println!("{}", greeting);
    println!("5 + 10 = {}", sum);
}
