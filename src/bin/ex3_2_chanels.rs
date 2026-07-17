fn get_message_channel(value: usize) -> (tokio::sync::mpsc::Sender<usize>, tokio::sync::mpsc::Receiver<usize>) {
    tokio::sync::mpsc::channel(value)
}

#[tokio::main]
async fn main(){
    let (tx, mut rx) = get_message_channel(100);

    let mut handlers = Vec::new();
    for _ in 1..=3 {
        let tx_clone = tx.clone();
        let task_handler = tokio::spawn(async move {
            for i in 1..=10 {
                if let Err(e) = tx_clone.send(i).await {
                    println!("Reveiver dropped: {:?}", e);
                    ()
                }
            }
        });
        handlers.push(task_handler);
    }

    let mut counter = 0;
    let rx_task_handler = tokio::spawn(async move {

        while let Some(i) = rx.recv().await {
            println!("got = {}", i);
            counter+=i;
        }

        println!("Total count: {}", counter);
    });

    handlers.push(rx_task_handler);

    drop(tx);
    for handler in handlers {
        handler.await.unwrap();
    }
}