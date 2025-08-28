mod control_flow;
mod functions;
mod variables;
mod shadowing_muts;

use control_flow::control_flow_example;
use functions::functions_example;
use variables::variables_example;
use shadowing_muts::shadowing_muts_functions;


fn main() {
    println!("=== День 1: Основы Rust ===\n");

    // Базовый Hello World
    println!("1. Hello World:");
    println!("Hello, World!");
    println!("Привет, мир!\n");

    // Примеры переменных
    println!("2. Переменные и типы:");
    variables_example();
    println!();

    // Примеры функций
    println!("3. Функции:");
    functions_example();
    println!();

    // Управляющие конструкции
    println!("4. Управляющие конструкции:");
    control_flow_example();
    println!();

    // Интерактивный пример
    println!("5. Интерактивный расчет:");
    let experience_years = 5;
    let is_web3_ready = experience_years >= 3;

    println!("Опыт разработки: {} лет", experience_years);
    println!("Готов к Web3: {}", is_web3_ready);

    if is_web3_ready {
        println!("Можно переходить к изучению блокчейн разработки!");
    }

    // Примеры Иммутабельности и shadowing
    println!("6.  Иммутабельности и shadowing");
    shadowing_muts_functions();
    println!();
}
