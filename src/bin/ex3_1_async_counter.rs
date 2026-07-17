#[tokio::main]
async fn main(){
    const MAX_COUNT: u32 = 5;

    let counter = std::sync::Arc::new(tokio::sync::Mutex::new(0));
    let mut thread_handlers: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for _ in 1..=MAX_COUNT {
        let counter_clone = std::sync::Arc::clone(&counter);
        let thread_handler = tokio::spawn(async move {
            for _ in 1..=1000 {
                let mut num = counter_clone.lock().await;
                *num += 1;
                println!("Thread {:?} incremented counter to: {}", tokio::task::id(), num);
            }
        });

        thread_handlers.push(thread_handler);
    }

    for thread in thread_handlers {
        thread.await.unwrap();
    }

    println!("Final counter value: {}", *counter.lock().await);
}
