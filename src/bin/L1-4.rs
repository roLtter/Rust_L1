use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_workers: usize = args.get(1).unwrap_or(&"4".to_string()).parse().unwrap_or(4);

    let (tx, rx) = mpsc::channel();

    let rx = Arc::new(Mutex::new(rx));

    for i in 0..num_workers {
        let rx = Arc::clone(&rx);

        thread::spawn(move || {
            loop {
                let received = rx.lock().unwrap().recv();
                match received {
                    Ok(data) => {
                        println!("Worker {} received: {}", i, data);
                    }
                    Err(_) => {
                        println!("Worker {}: channel closed", i);
                        break;
                    }
                }
            }
        });
    }

    thread::spawn(move || {
        let mut count = 0;
        loop {
            count += 1;
            let message = format!("Data {}", count);

            if tx.send(message).is_err() {
                break;
            }

            thread::sleep(Duration::from_millis(500));
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
