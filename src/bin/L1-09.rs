use std::env;

fn flip(num: i64, i: usize) -> i64 {
    let mask = 1 << (i-1);
    num ^ mask
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run --bin L1-09 -- <num> <i>");
        return;
    }

    let num: i64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number.");
            return;
        }
    };

    let i: usize = match args[2].parse() {
        Ok(index) => index,
        Err(_) => {
            eprintln!("Please provide a valid number.");
            return;
        }
    };

    println!("{:b} - ({})", num, num);
    let result = flip(num, i);
    println!("{:b} - ({})", result, result);
}