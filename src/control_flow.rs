pub fn control_flow_example() {
    // if/else
    let number = 42;
    if number > 30 {
        println!("Число больше 30");
    } else {
        println!("Число меньше или равно 30");
    }

    // match (аналог switch)
    let skill_level = "Junior+";
    match skill_level {
        "Junior" => println!("Начинающий разработчик"),
        "Junior+" => println!("Продвинутый Junior"),
        "Middle" => println!("Средний разработчик"),
        _ => println!("Другой уровень"),
    }

    // Циклы
    for i in 1..=5 {
        println!("Итерация: {}", i);
    }

    let mut count = 0;
    while count < 3 {
        println!("Цикл while: {}", count);
        count += 1;
    }
}
