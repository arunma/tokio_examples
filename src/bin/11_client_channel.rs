use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};
use crate::Command::{Get, Set};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Get { key: "hello".to_string(), resp: resp_tx };
        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Set { key: "hello".to_string(), value: "world".into(), resp: resp_tx };
        tx2.send(cmd).await.unwrap();
    });


    let manager = tokio::spawn(async move {
        let mut client = client::connect("localhost:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Get { key, resp } => {
                    let result = client.get(&key).await;
                    resp.send(result);
                }
                Set { key, value, resp } => {
                    let res = client.set(&key, value).await;
                    resp.send(res);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}