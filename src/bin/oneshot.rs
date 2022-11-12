use tokio::sync::oneshot::{self, Sender};

#[derive(Debug, Clone)]
struct OneShotRequest {
    req: String,
    res: Option<String>,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = oneshot::channel();

    tokio::spawn(async move { send_request(tx).await });

    if let Ok(message) = rx.await {
        println!("Message: {:?}", message);
    }
}

async fn send_request(sender: Sender<OneShotRequest>) {
    sender
        .send(OneShotRequest {
            req: "From User 1".into(),
            res: None,
        })
        .unwrap();
}
