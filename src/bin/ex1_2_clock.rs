#[tokio::main]
async fn main() {
    const MAX_COUNT: u32 = 5;

    for i in 1..=MAX_COUNT {
        println!("Tick: {}", i);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}