use std::{env, thread};
use std::sync::mpsc;
use std::iter::Iterator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin L1-2 -- <amount of numbers>");
        return;
    }

    let n: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number.");
            return;
        }
    };
    let num_threads: u32 = 4;
    let chunk_size = n / num_threads;
    let (tx, rx) = mpsc::channel();

    let numbers: Vec<u32> = (1..=n).collect();

    for chunk in numbers.chunks(chunk_size as usize) {
        let tx = tx.clone();
        let chunk: Vec<u32> = chunk.to_vec();

        thread::spawn(move || {
            let sum_of_squares: u32 = chunk.iter().map(|&x| x * x).sum();
            tx.send(sum_of_squares).unwrap();
        });
    }
    drop(tx);
    let total_sum: u32 = rx.iter().sum();

    println!("Сумма квадратов чисел от 1 до {} равна {}", n, total_sum);
}
