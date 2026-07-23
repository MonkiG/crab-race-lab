#[tokio::main]
async fn main(){
    const TOTAL_CLIENTS: u64 = 10;

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(3));
    let mut handlers = vec![];

    for client in 1..= TOTAL_CLIENTS {
        let sem_clone = std::sync::Arc::clone(&semaphore);

        let handle = tokio::spawn(async move {

            let _permit = sem_clone.acquire_owned().await.unwrap();
            
            println!("Client {} started", client);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("Client {} finished", client);
        });

        handlers.push(handle);
    }

    for handle in handlers {
        handle.await.unwrap();
    }
}