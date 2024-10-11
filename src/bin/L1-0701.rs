use std::sync::mpsc::{self};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        while let Ok(message) = rx.recv() {
            println!("Received: {}", message);
            if message == "stop" {
                println!("Stopping thread.");
                break;
            }
        }
    });

    tx.send("I").unwrap();
    tx.send("am Steve").unwrap();
    tx.send("stop").unwrap();

    handle.join().unwrap();
}
