use std::io::{self, BufRead};

fn main() {
    let mut unique_lines = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        if !unique_lines.contains(&line) {
            unique_lines.push(line);
        }
    }
    println!("Unique lines:");
    for line in unique_lines {
        println!("{}", line);
    }
}
