use std::time::Duration;
use tokio::task::spawn_blocking;

async fn hello_delay(task: u64, time: u64) {
    println!("[hello_delay started] {}", task);
    let _ = spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(time));
    }).await;
    // tokio::time::sleep(Duration::from_millis(time)).await;
    println!("[hello_delay finished] {}", task);
}


#[tokio::main]
async fn main() {
    tokio::join!(
        hello_delay(1, 200),
        hello_delay(2, 500),
        hello_delay(3, 1000),
    );
}
