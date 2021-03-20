use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await;
        })
    }

    Ok(())
}

async fn process(socket: TcpStream) {
    loop {
        socket.readable().await?
    }
}