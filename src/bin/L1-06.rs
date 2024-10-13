use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin L1-60 -- <duration_in_seconds>");
        return;
    }

    let duration_in_secs: u64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number for the duration.");
            return;
        }
    };

    let (tx, rx) = mpsc::channel();

    let sender_thread = {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut value = 0;
            loop {
                value += 1;
                if tx.send(value).is_err() {
                    break;
                }
                println!("Отправлено: {}", value);
                thread::sleep(Duration::from_millis(500));
            }
        })
    };

    let receiver_thread = thread::spawn(move || {
        let timeout = Duration::from_secs(duration_in_secs);
        let start_time = std::time::Instant::now();
        let mut has_no_messages = false;

        while start_time.elapsed() < timeout {
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(msg) => {
                    println!("Принято: {}", msg);
                    has_no_messages = false;
                }
                Err(_) => {
                    if !has_no_messages {
                        println!("Нет сообщений, продолжаем ждать...");
                        has_no_messages = true;
                    }
                }
            }
        }

        println!("Время вышло! Завершаем программу.");
    });

    receiver_thread.join().unwrap();
    drop(tx);
    sender_thread.join().unwrap();
}
