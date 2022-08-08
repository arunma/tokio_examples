use std::collections::HashMap;
use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:6379").await.unwrap();
    loop{
        let (socket, _) =listener.accept().await.unwrap();
        // Spin off a task
        tokio::spawn(async move {
            process(socket).await
        });

    }
}

async fn process(socket:TcpStream){
    let mut connection = Connection::new(socket);
    let mut db = HashMap::new();
    while let Some(frame)=connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                println!("Inserted {}", cmd.key());
                Frame::Simple("OK".to_string())

            }
            Command::Get(cmd) => {
                println!("Getting value for {}", cmd.key());
                if let Some(value) = db.get(cmd.key()){
                    Frame::Bulk(value.clone().into())
                }
                else{
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd)
        };

        connection.write_frame(&response).await.unwrap()
    }
}