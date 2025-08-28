# День 1: Подробная инструкция по настройке среды разработки Rust

## Шаг 1: Установка Rust Toolchain через Rustup

### На macOS/Linux:

Откройте терминал и выполните команду:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


### Проверка установки:

После установки **перезапустите терминал** и проверьте:

```bash
rustc --version
cargo --version
rustup --version
```

Вы должны увидеть версии всех инструментов.

## Шаг 2: Настройка VS Code с rust-analyzer

### Установка необходимых расширений:

1. Откройте VS Code
2. Нажмите `Ctrl+Shift+X` (Extensions view)
3. Установите следующие расширения:

**Обязательные расширения**:

- **rust-analyzer** - основное расширение для Rust
- **Even Better TOML** - подсветка синтаксиса для Cargo.toml
- **Crates** - автодополнение версий crates в Cargo.toml

**Дополнительные расширения**:

- **Error Lens** - визуальное выделение ошибок
- **CodeLLDB** - отладчик для Rust


### Настройка горячих клавиш:

Добавьте в `keybindings.json` (File → Preferences → Keyboard Shortcuts → Open Keyboard Shortcuts JSON):

```json
[
    {
        "key": "shift+cmd+enter",
        "command": "rust-analyzer.run"
    }
]
```


## Шаг 3: Создание проекта hello-rust

### Создание проекта через Cargo:

```bash
cargo new hello-rust
cd hello-rust
```

## Шаг 5: Запуск и тестирование

### Запуск проекта:

```bash
cargo run
```


### Проверка кода:

```bash
cargo check
```


### Форматирование кода:

```bash
cargo fmt
```


### Линтинг (статический анализ):

```bash
cargo clippy
```


## Шаг 6: Создание README.md и коммит в GitHub

### Создайте README.md:

```markdown
# Hello Rust - День 1

Мой первый проект на Rust в рамках 90-дневного курса изучения Rust для Web3 разработки.

## Что изучено:
- Установка Rust toolchain (rustup, cargo)
- Настройка VS Code с rust-analyzer
- Базовый синтаксис Rust:
  - Переменные и константы
  - Функции и возвращаемые значения
  - Управляющие конструкции (if/else, match, циклы)
  - Модульная система

## Структура проекта:
```

src/
├── main.rs          \# Точка входа
├── variables.rs     \# Примеры переменных
├── functions.rs     \# Примеры функций
└── control_flow.rs  \# Управляющие конструкции

```

## Запуск:
```

cargo run

```

## Цель:
Стать Rust разработчиком для Web3 за 90 дней.

**День 1 завершен ✅**
```


### Инициализация Git и пуш в GitHub:

```bash
# Инициализация репозитория
git init

# Добавление файлов
git add .

# Первый коммит
git commit -m "День 1: Hello Rust - базовые примеры синтаксиса"

# Добавление remote (замените на ваш GitHub URL)
git remote add origin https://github.com/AbayReushenov/hello-rust.git

# Пуш в GitHub
git push -u origin main
```


## Дополнительные команды для изучения

### Полезные cargo команды:

```bash
cargo build          # Компиляция без запуска
cargo test            # Запуск тестов
cargo doc --open      # Генерация и открытие документации
cargo --version       # Версия cargo
```


### Информация о проекте:

```bash
cargo tree            # Дерево зависимостей
rustc --explain E0308 # Объяснение ошибок компилятора
```


## Что ожидать от rust-analyzer в VS Code:

- **Автодополнение кода** при наборе
- **Подсветка ошибок** в реальном времени
- **Inlay hints** - подсказки типов переменных
- **Go to definition** (F12) - переход к определению
- **Hover информация** - наведение мыши для справки
- **Рефакторинг** - переименование переменных/функций


# Пример запуска программы
```bash
$ cargo run
```

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/hello-rust`
=== День 1: Основы Rust ===

1. Hello World:
Hello, World!
Привет, мир!

2. Переменные и типы:
Имя: Абай, Возраст: 58
Счетчик: 1, Максимум: 100000

3. Функции:
Привет, Rust разработчик!
5 + 10 = 15

4. Управляющие конструкции:
Число больше 30
Продвинутый Junior
Итерация: 1
Итерация: 2
Итерация: 3
Итерация: 4
Итерация: 5
Цикл while: 0
Цикл while: 1
Цикл while: 2

5. Интерактивный расчет:
Опыт разработки: 5 лет
Готов к Web3: true
Можно переходить к изучению блокчейн разработки!
