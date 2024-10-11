use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let cancellation_token = CancellationToken::new();
    let task = {
        let cancellation_token = cancellation_token.clone();
        tokio::spawn(async move {
            for i in 0..5 {
                if cancellation_token.is_cancelled() {
                    println!("Task cancelled.");
                    return;
                }
                println!("Working... {}", i);
                sleep(Duration::from_secs(1)).await;
            }
            println!("Task completed.");
        })
    };

    sleep(Duration::from_secs(3)).await;
    cancellation_token.cancel();

    let _ = task.await;
}
