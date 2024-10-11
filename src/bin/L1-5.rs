use tokio::signal;
use flume::{Sender, Receiver};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let num_workers: usize = 4;
    let (tx, rx): (Sender<String>, Receiver<String>) = flume::unbounded();

    let running = Arc::new(AtomicBool::new(true));

    for i in 0..num_workers {
        let rx = rx.clone();
        let running = Arc::clone(&running);

        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                // Пытаемся получить сообщение из канала
                if let Ok(data) = rx.recv() {
                    println!("Worker {} received: {}", i, data);
                }
            }
            println!("Worker {} is shutting down", i);
        });
    }

    let tx_clone = tx.clone();
    {
        let running = Arc::clone(&running);
        thread::spawn(move || {
            let mut count = 0;
            while running.load(Ordering::SeqCst) {
                count += 1;
                let message = format!("Data {}", count);
                if tx_clone.send(message).is_err() {
                    break;
                }
                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    running.store(false, Ordering::SeqCst);

    drop(tx);

    tokio::time::sleep(Duration::from_secs(1)).await;

    println!("Program is shutting down.");
}
