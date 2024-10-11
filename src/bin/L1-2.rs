use std::thread;
use std::io;

fn main() {
    let mut input = String::new();
    println!("Введите число n:");
    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

    let n = input.trim().parse().expect("Пожалуйста, введите корректное число!");
    let numbers: Vec<i32> = (1..=n).collect();

    let mut threads = Vec::new();

    for &num in &numbers {
        let handle = thread::spawn(move || {
            let square = num*num;
            println!("{}^2 = {}", num, square);
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}
