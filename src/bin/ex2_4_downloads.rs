use std::time::Instant;

async fn download_simulation(secs: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await;
    println!("Download: {}", secs);
}

#[tokio::main]
async fn main(){

    let start_time = Instant::now();
    tokio::join!(
        download_simulation(3),
        download_simulation(1),
        download_simulation(2)
    );
    
    let elapsed = start_time.elapsed();

    println!("Total time spent: {:?}", elapsed.as_secs_f32());
}
