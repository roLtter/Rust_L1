use num_bigint::BigInt;
use num_traits::Zero;
use std::str::FromStr;

fn main() {
    let a = BigInt::from_str("2097153").unwrap();  // 2^21 + 1
    let b = BigInt::from_str("1048577").unwrap();  // 2^20 + 1


    let sum = &a + &b;
    println!("Сложение: {} + {} = {}", a, b, sum);

    let difference = &a - &b;
    println!("Вычитание: {} - {} = {}", a, b, difference);

    let product = &a * &b;
    println!("Умножение: {} * {} = {}", a, b, product);

    if !b.is_zero() {
        let quotient = &a / &b;
        println!("Деление: {} / {} = {}", a, b, quotient);
    } else {
        println!("Деление на ноль невозможно.");
    }
}
