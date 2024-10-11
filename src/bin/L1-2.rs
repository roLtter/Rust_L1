use std::{env, thread};
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin L1-2 -- <amount of numbers>");
        return;
    }

    let n: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number for the duration.");
            return;
        }
    };

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
