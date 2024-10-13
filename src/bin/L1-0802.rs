use dashmap::DashMap;
use tokio::task;

#[tokio::main]
async fn main() {
    let map: DashMap<i32, String> = DashMap::new();
    let mut handles = vec![];

    for i in 0..10 {
        let map_clone = map.clone();
        let handle = task::spawn(async move {
            map_clone.insert(i, format!("Value {}", i));
            if let Some(value) = map_clone.get(&i) {
                println!("Inserted: {} - Value {}", i, *value);
            }
        });
        handles.push(handle);
    }


    for handle in handles {
        let _ = handle.await;
    }

    println!("Содержимое DashMap после вставки:");
    for r in map.iter() {
        println!("Ключ: {}, Значение: {}", r.key(), r.value());
    }
}
