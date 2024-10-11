use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task;

#[tokio::main]
async fn main() {
    let map: Arc<Mutex<HashMap<i32, String>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let map_clone = Arc::clone(&map);
        let handle = task::spawn(async move {
            let mut map = map_clone.lock().unwrap();
            map.insert(i, format!("Value: {}", i));
            println!("Inserted: {} - {}", i, map.get(&i).unwrap());
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let map = map.lock().unwrap();
    for (key, value) in map.iter() {
        println!("Key: {}, {}", key, value);
    }
}
