use std::sync::mpsc;
use std::{env, thread};
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin L1-10 -- <n>");
        return;
    }
    let n: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number.");
            return;
        }
    };

    let numbers: Vec<i32> = (1..=n).collect();

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let producer = thread::spawn(move || {
        for &num in &numbers {
            println!("Отправка числа: {}", num);
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let processor = thread::spawn(move || {
        for num in rx1 {
            let square = num * num;
            println!("Число {} возведено в квадрат: {}", num, square);
            tx2.send(square).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        for square in rx2 {
            println!("Получен результат: {}", square);
        }
    });

    producer.join().unwrap();
    processor.join().unwrap();
    consumer.join().unwrap();
}
