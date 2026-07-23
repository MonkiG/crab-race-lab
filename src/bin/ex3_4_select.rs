async fn servidor_a() -> &'static str {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    "Respuesta de A"
}

async fn servidor_b() -> &'static str {
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    "Respuesta de B"
}
#[tokio::main]
async fn main(){

    tokio::select! {
        r = servidor_a() => println!("Respuesta de Server A: {}", r),
        r = servidor_b() => println!("Respuesta de Server B: {}", r),
    };
}