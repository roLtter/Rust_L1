use std::sync::mpsc::{self, Sender};
use std::thread;

fn main() {
    // Создаем канал
    let (tx, rx) = mpsc::channel();

    // Создаем поток
    let handle = thread::spawn(move || {
        while let Ok(message) = rx.recv() {
            println!("Received: {}", message);
            if message == "stop" {
                println!("Stopping thread.");
                break;
            }
        }
    });

    // Отправляем сообщения
    tx.send("I").unwrap();
    tx.send("am Steve").unwrap();
    tx.send("stop").unwrap();

    // Ждем завершения потока
    handle.join().unwrap();
}
