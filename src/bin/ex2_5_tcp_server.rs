
use tokio::io::AsyncWriteExt;

fn get_response(socket: &tokio::net::TcpStream) -> String{
    println!("{:?}", *socket);
    let client_address = socket.peer_addr().unwrap();
    format!("Hello client with addr: {}", client_address)
    
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let response = get_response(&socket);
            socket.write_all(response.as_bytes()).await.unwrap();
        });
    }
}