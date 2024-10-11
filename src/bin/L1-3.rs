use std::thread;
use std::sync::mpsc;
use std::iter::Iterator;

fn main() {
    let n = 100;
    let num_threads = 4;
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
