use dashmap::DashMap;
use tokio::task;

#[tokio::main]
async fn main() {
    let map: DashMap<i32, String> = DashMap::new();
    let mut handles = vec![];

    for i in 0..10 {
        let map_clone = map.clone(); // Клонируем DashMap для каждого потока
        let handle = task::spawn(async move {
            map_clone.insert(i, format!("Value {}", i));
            // Получаем значение по ключу и распечатываем его
            if let Some(value) = map_clone.get(&i) {
                println!("Inserted: {} - Value {}", i, *value);
            }
        });
        handles.push(handle);
    }

    // Ждем завершения всех задач
    for handle in handles {
        let _ = handle.await;
    }

    // Выводим все данные из DashMap
    println!("Содержимое DashMap после вставки:");
    for r in map.iter() {
        println!("Ключ: {}, Значение: {}", r.key(), r.value());
    }
}
