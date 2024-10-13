use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));

    let num_threads = 100;

    let increments_per_thread = 1000;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Итоговое значение счетчика: {}", counter.load(Ordering::SeqCst));
}
