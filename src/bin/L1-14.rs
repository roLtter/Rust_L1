use std::any::Any;
use std::io::{self, Write};
use std::str::FromStr;

fn print_type_of(value: &dyn Any) {
    match value.type_id() {
        id if id == std::any::TypeId::of::<i32>() => println!("Тип переменной: i32"),
        id if id == std::any::TypeId::of::<f64>() => println!("Тип переменной: f64"),
        id if id == std::any::TypeId::of::<String>() => println!("Тип переменной: String"),
        id if id == std::any::TypeId::of::<bool>() => println!("Тип переменной: bool"),
        _ => println!("Неизвестный тип"),
    }
}

fn main() {
    let mut input = String::new();
    print!("Введите данные: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");

    let input = input.trim();
    match (
        i32::from_str(input),
        f64::from_str(input),
        bool::from_str(input),
    ) {
        (Ok(i), _, _) => print_type_of(&i as &dyn Any),
        (_, Ok(f), _) => print_type_of(&f as &dyn Any),
        (_, _, Ok(b)) => print_type_of(&b as &dyn Any),
        _ => {
            let s = input.to_string();
            print_type_of(&s as &dyn Any);
        }
    }
}
