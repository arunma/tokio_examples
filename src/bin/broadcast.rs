use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Debug, Clone)]
struct ChatMessage {
    id: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(100);
    let users = vec!["Foo", "Bar", "Baz"];
    for user in users {
        let rx_clone = tx.subscribe();
        tokio::spawn(async move { spin_client(user, tx.clone(), rx_clone) });
    }

    while let Some(message) = rx.recv().await.ok() {
        println!("Message: {:?}", message);
    }
}

async fn spin_client(user: &str, sender: Sender<ChatMessage>, receiver: Receiver<ChatMessage>) {}
