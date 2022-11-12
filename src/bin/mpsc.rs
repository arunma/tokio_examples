use std::iter;

use tokio::sync::mpsc::{self, Sender};

#[derive(Debug)]
struct User {
    id: String,
    name: String,
}

/* #[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    let mut print_users = vec![];

    let user_names = ["Foo", "Bar", "Baz"];

    let mut handles = vec![];
    for user_id in user_names {
        let tx = tx.clone();
        println!("Spawning for user {:?}", &user_id);
        handles.push(tokio::spawn(async move { lookup_user(user_id, tx).await }));
    }

    while let Some(user) = rx.recv().await {
        println!("Received user: {:?}", &user);
        print_users.push(user)
    }

    println!("Print Users: {:?}", &print_users);
} */

/* #[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    let mut print_users = vec![];

    let user_names = ["Foo", "Bar", "Baz"];

    let mut handles = vec![];
    for user_id in user_names {
        let tx = tx.clone();
        println!("Spawning for user {:?}", &user_id);
        handles.push(tokio::spawn(async move { lookup_user(user_id, tx).await }));
    }

    drop(tx);
    while let Some(user) = rx.recv().await {
        println!("Received user: {:?}", &user);
        print_users.push(user)
    }

    println!("Print Users: {:?}", &print_users);
} */

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    let mut print_users = vec![];

    let user_names = ["Foo", "Bar", "Baz"];

    /*   user_names
    .into_iter()
    .zip(std::iter::repeat(tx))
    .map(|(user_id, tx_clone)| {
        tokio::spawn(async move { lookup_user(user_id, tx_clone).await })
    }); */

    /*    for (user_id, tx_clone) in user_names.into_iter().zip(std::iter::repeat(tx)) {
        tokio::spawn(async move { lookup_user(user_id, tx_clone).await });
    } */

    for (user_id, tx_clone) in iter::zip(user_names, std::iter::repeat(tx)) {
        tokio::spawn(async move { lookup_user(user_id, tx_clone).await });
    }

    while let Some(user) = rx.recv().await {
        println!("Received user: {:?}", &user);
        print_users.push(user)
    }

    println!("Print Users: {:?}", &print_users);
}

async fn lookup_user(user_id: &str, sender: Sender<User>) {
    let user = User {
        id: user_id.into(),
        name: user_id.to_ascii_uppercase(),
    };

    println!("Sending back response for user : {:?}", &user);
    sender
        .send(user)
        .await
        .expect("Unable to send to the sender channel");

    println!("Message sent ")
}
