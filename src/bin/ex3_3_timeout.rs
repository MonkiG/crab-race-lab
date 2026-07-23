async fn sleep(time: u32) {
    println!("Hello, before sleep");
    for i in 1..=time {
        println!("Sec: {}", i);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    println!("Hello, after sleep!")
}

#[tokio::main]
async fn main(){
    match tokio::time::timeout(tokio::time::Duration::from_secs(2), sleep(5)).await {
        Ok(_) => println!("La operación terminó a tiempo"),
        Err(_) => println!("¡Tiempo de espera agotado (timeout)!"),
    };
}