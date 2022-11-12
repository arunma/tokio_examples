use std::io;

use futures::TryFutureExt;
use thiserror::Error;
use tokio::sync::{
    mpsc::{self},
    oneshot::{self, error::RecvError},
};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Receive error")]
    ReceiveError,
    #[error("Send error: {0}")]
    SendError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
    #[error("Converted error")]
    ConvertedError(#[from] io::Error),
    #[error("Tokio error")]
    TokioRecvErrorError(#[from] oneshot::error::RecvError),
}

type Payload<i32, String> = (i32, oneshot::Sender<String>);

#[tokio::main]
async fn main() {
    let (mp_tx, mut mp_rx) = mpsc::channel::<Payload<u32, String>>(100);
    tokio::spawn(async move {
        while let Some((request, os_tx)) = mp_rx.recv().await {
            println!("Received request from client {:?}", &request);
            let response = get_response(request).await;
            os_tx.send(response).unwrap();
        }
    });

    let inputs = [1, 2, 3, 4];
    for each in inputs {
        let mp_tx = mp_tx.clone();
        let (os_tx, os_rx) = oneshot::channel::<String>();
        let payload = (each, os_tx);
        mp_tx.send(payload).await.unwrap();
        let response = os_rx.await;
        println!("Response: {:?}", response);
    }
}

async fn get_response(req: u32) -> String {
    match req {
        1 => "ONE".into(),
        2 => "TWO".into(),
        3 => "THREE".into(),
        _ => "UNKNOWN".into(),
    }
}
