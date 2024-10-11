use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    // Получаем аргументы из командной строки
    let args: Vec<String> = env::args().collect();

    // Проверяем, что передан хотя бы один аргумент (время работы программы)
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin L1-6 -- <duration_in_seconds>");
        return;
    }

    // Парсим аргумент в целое число
    let duration_in_secs: u64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number for the duration.");
            return;
        }
    };

    // Создаем канал
    let (tx, rx) = mpsc::channel();

    // Создаем поток для отправки сообщений
    let sender_thread = {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut value = 0;
            loop {
                value += 1;
                // Если не удается отправить (канал закрыт), прерываем поток
                if tx.send(value).is_err() {
                    break;
                }
                println!("Отправлено: {}", value);
                thread::sleep(Duration::from_millis(500)); // Отправляем сообщение каждые 500 мс
            }
        })
    };

    // Основной поток для чтения сообщений и завершения работы через N секунд
    let receiver_thread = thread::spawn(move || {
        let timeout = Duration::from_secs(duration_in_secs);
        let start_time = std::time::Instant::now();
        let mut has_no_messages = false;  // Флаг для отслеживания, выводили ли мы "Нет сообщений"

        while start_time.elapsed() < timeout {
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(msg) => {
                    println!("Принято: {}", msg);
                    has_no_messages = false; // Если пришло сообщение, сбрасываем флаг
                }
                Err(_) => {
                    if !has_no_messages {
                        println!("Нет сообщений, продолжаем ждать...");
                        has_no_messages = true; // Выводим сообщение только один раз
                    }
                }
            }
        }

        println!("Время вышло! Завершаем программу.");
    });

    // Ждем завершения принимающего потока
    receiver_thread.join().unwrap();
    // Закрываем канал, завершая поток отправки
    drop(tx);
    // Ждем завершения отправляющего потока
    sender_thread.join().unwrap();
}
