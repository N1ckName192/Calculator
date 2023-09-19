use std::io;

fn main() {
    println!("Выберите систему счисления (2 - двоичная, 8 - восьмеричная, 16 - 16-ричная):");

    let mut input_base = String::new();
    io::stdin().read_line(&mut input_base).expect("Не удалось прочитать строку");

    let base: u32 = input_base.trim().parse().expect("Введите корректное число");

    if base != 2 && base != 8 && base != 16 {
        println!("Неподдерживаемая система счисления");
        return;
    }

    println!("Введите первое число:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Не удалось прочитать строку");

    println!("Введите второе число:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Не удалось прочитать строку");

    println!("Выберите операцию (+ - сложение, - - вычитание):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Не удалось прочитать строку");

    let num1_decimal = match u64::from_str_radix(num1.trim(), base) {
        Ok(val) => val,
        Err(_) => {
            println!("Ошибка при конвертации первого числа");
            return;
        }
    };

    let num2_decimal = match u64::from_str_radix(num2.trim(), base) {
        Ok(val) => val,
        Err(_) => {
            println!("Ошибка при конвертации второго числа");
            return;
        }
    };

    let result_decimal = match operator.trim() {
        "+" => num1_decimal + num2_decimal,
        "-" => num1_decimal - num2_decimal,
        _ => {
            println!("Неподдерживаемая операция");
            return;
        }
    };

    let result = format!("{:x}", result_decimal);

    println!("Результат операции: {}", result);
}
