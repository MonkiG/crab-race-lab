use tokio::sync::broadcast;

#[tokio::main]
async fn main(){
    const MAX_CLIENTS: u32 = 10;
    let (tx, _rx1) = broadcast::channel(16);

    for i in 1..=MAX_CLIENTS {
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let mut rx = tx_clone.subscribe();

            while let Ok(msg) = rx.recv().await {
                println!("Client {}, received: \"{}\"", i, msg)
            }
        });
    }

    let mut counter = 1;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let message = format!("Tick num: {}", counter);
        tx.send(message).unwrap();
        counter += 1;
        println!("----------------------------------")
    }
}