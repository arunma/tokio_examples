use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

// Does not work - Needs a hash function

type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:6379").await.unwrap();
    let db = new_sharded_db(4);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Spin off a task
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await
        });
    }
}

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

async fn process(socket: TcpStream, db: ShardedDb) {
    let mut connection = Connection::new(socket);
    let hasher = DefaultHasher::new();
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                let db = db[cmd.key().hash() % db.len()].lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                let db = db[cmd.key().hash() % db.len()].lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd)
        };

        connection.write_frame(&response).await.unwrap()
    }
}