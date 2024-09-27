// Задача
// Реализуем настройку фильтра по аналогии с консольной утилитой (man grep — см. описание и основные параметры).

// Реализовать поддержку утилиты следующими ключами:

// -A — «после» печатать +N строки после совпадения

// -B — «до» печатать +N строки до совпадения

// -C — «контекст» (A+B) печатать ±N строку вокруг совпадения

// -c — «счет» (количество строк)

// -i — «игнорировать регистр» (игнорировать регистр)

// -v — «инвертировать» (вместо совпадения, проверить)

// -F — «фиксированное», точное совпадение со строкой, а не по шаблону

// -n — «номер строки», напечатать номер строки


use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

// Структура для хранения опций командной строки
#[derive(Default)]
struct Options {
    after: usize,   // Количество строк после совпадения
    before: usize,  // Количество строк перед совпадением
    context: usize, // Количество строк до и после совпадения (если используется)
    count: bool,    // Флаг для подсчета совпадений
    ignore_case: bool, // Флаг для игнорирования регистра
    invert: bool,   // Флаг для инвертирования поиска
    fixed: bool,    // Флаг для фиксированного (точного) поиска
    number: bool,   // Флаг для вывода номеров строк
}

// Функция для выполнения поиска в файле
fn grep(file_path: &str, pattern: &str, options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    // Открытие файла для чтения
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut matching_lines = Vec::new();
    let regex = if options.fixed {
        None // Если фиксированный поиск, регулярное выражение не создается
    } else {
        // Создание регулярного выражения с учетом игнорирования регистра
        let flags = if options.ignore_case { "(?i)" } else { "" };
        Some(Regex::new(&format!("{}{}", flags, regex::escape(pattern)))?) // Создание регулярного выражения
    };

    // Чтение файла построчно
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?; // Чтение строки
        // Проверка на совпадение
        let matched = if options.fixed {
            line.contains(pattern) // Если фиксированный поиск, проверяем на вхождение
        } else {
            regex.as_ref().map_or(false, |r| r.is_match(&line)) // Используем регулярное выражение
        };

        // Добавление совпадений в вектор в зависимости от инверсии
        if options.invert {
            if !matched {
                matching_lines.push((line_number, line)); // Если не совпадает и инверсия активна
            }
        } else {
            if matched {
                matching_lines.push((line_number, line)); // Если совпадает
            }
        }
    }

    // Вывод результатов
    if options.count {
        println!("{}", matching_lines.len()); // Вывод количества совпадений
    } else {
        for (line_number, line) in matching_lines.iter() {
            if options.number {
                println!("{}: {}", line_number + 1, line); // Вывод номера строки и содержимого
            } else {
                println!("{}", line); // Вывод только содержимого
            }

            // Печать контекста
            if options.after > 0 || options.before > 0 {
                let start = line_number.saturating_sub(options.before); // Начальная строка
                let end = line_number + 1 + options.after; // Конечная строка

                // Вывод контекстных строк
                for context_line in &matching_lines[start..end] {
                    if context_line.0 != *line_number { // Проверка, чтобы не выводить само совпадение
                        if options.number {
                            println!("{}: {}", context_line.0 + 1, context_line.1); // Вывод номера строки
                        } else {
                            println!("{}", context_line.1); // Вывод только содержимого
                        }
                    }
                }
            }
        }
    }

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect(); // Получение аргументов командной строки
    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <file> [options]", args[0]); // Проверка на количество аргументов
        return; // Выход из программы
    }

    let pattern = &args[1]; // Шаблон для поиска
    let file_path = &args[2]; // Путь к файлу

    let mut options = Options::default(); // Инициализация опций

    // Обработка аргументов опций
    for arg in &args[3..] {
        match arg.as_str() {
            arg if arg.starts_with("-A") => {
                options.after = arg[2..].parse().unwrap_or(0); // Установка количества строк после совпадения
            }
            arg if arg.starts_with("-B") => {
                options.before = arg[2..].parse().unwrap_or(0); // Установка количества строк перед совпадением
            }
            arg if arg.starts_with("-C") => {
                options.context = arg[2..].parse().unwrap_or(0); // Установка количества контекстных строк
            }
            "-c" => {
                options.count = true; // Установка флага подсчета
            }
            "-i" => {
                options.ignore_case = true; // Установка флага игнорирования регистра
            }
            "-v" => {
                options.invert = true; // Установка флага инверсии поиска
            }
            "-F" => {
                options.fixed = true; // Установка флага фиксированного поиска
            }
            "-n" => {
                options.number = true; // Установка флага для вывода номеров строк
            }
            _ => {
                eprintln!("Неизвестная опция: {}", arg); // Сообщение об ошибке для неизвестной опции
            }
        }
    }

    // Выполнение функции grep и обработка ошибок
    if let Err(e) = grep(file_path, pattern, &options) {
        eprintln!("Error: {}", e); // Вывод ошибки, если она возникла
    }
}
