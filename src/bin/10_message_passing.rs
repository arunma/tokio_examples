use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2=tx.clone();

    tokio::spawn(async move {
       tx.send("Sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("Sending from second handle").await;
    });

    println!("whatever");
    while let Some(message)=rx.recv().await{
        println!("Got message {}", message)
    }


}