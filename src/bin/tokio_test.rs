use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    loop {
        let (socket, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move { process_client(socket).await });
    }
}

#[allow(clippy::unused_io_amount)]
async fn process_client(mut socket: TcpStream) {
    loop {
        let mut buff = vec![0; 1024];
        socket.read(&mut buff).await.unwrap();

        println!("Read string {:?}", String::from_utf8_lossy(&buff));
        socket.write(&buff).await.unwrap();
    }
}
