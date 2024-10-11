use tokio::time::{self, Duration};
use flume::{Sender, Receiver};
use std::env;

#[tokio::main]
async fn main() {
    // Получаем время выполнения программы из аргументов командной строки
    let args: Vec<String> = env::args().collect();
    let duration_secs: u64 = args.get(1).unwrap_or(&"5".to_string()).parse().unwrap_or(5);

    // Создаем канал flume
    let (tx, rx): (Sender<String>, Receiver<String>) = flume::unbounded();

    // Запускаем асинхронную задачу для отправки данных
    let sender = tokio::spawn({
        let tx = tx.clone(); // Клонируем Sender для передачи в асинхронную задачу
        async move {
            for i in 0.. {
                let message = format!("Message {}", i);
                if tx.send(message).is_err() {
                    println!("Receiver has been dropped. Exiting sender.");
                    break;
                }
                // Имитируем задержку между отправками
                time::sleep(Duration::from_millis(500)).await;
            }
        }
    });

    // Запускаем асинхронную задачу для чтения данных
    let receiver = tokio::spawn(async move {
        while let Ok(data) = rx.recv() { // Убираем await
            println!("Received: {}", data);
        }
    });

    // Ждем N секунд перед завершением программы
    time::sleep(Duration::from_secs(duration_secs)).await;

    // Закрываем отправителя
    drop(tx); // Это закроет канал и завершит воркера

    // Ждем завершения задач
    let _ = sender.await;
    let _ = receiver.await;

    println!("Program has finished executing.");
}
