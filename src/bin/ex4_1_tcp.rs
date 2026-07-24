use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await?;

    let client_counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    
    loop {
        let (mut socket, _) = listener.accept().await?;

        let counter_clone = Arc::clone(&client_counter);

        tokio::spawn(async move {
            let client_number = {
                let mut counter = counter_clone.lock().await;
                *counter += 1;
                *counter
            };

            socket.write_all(format!("You are the client: {}", client_number).as_bytes()).await.unwrap();
        });
    }
}