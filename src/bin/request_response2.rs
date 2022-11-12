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

type Payload<Req, Res> = (Req, oneshot::Sender<Res>);

pub async fn send_and_receive<Req, Res>(
    mp_tx: mpsc::Sender<Payload<Req, Res>>,
    req: Req,
) -> Result<Res, AppError> {
    let (os_tx, os_rx) = oneshot::channel::<Res>();
    let payload: Payload<Req, Res> = (req, os_tx);
    mp_tx
        .send(payload)
        .await
        .map_err(|err| AppError::SendError(err.to_string()))?;

    os_rx
        .await
        .map_err(|err| AppError::UnknownError(err.to_string()))
}

pub fn channel<Req, Res>() -> (
    mpsc::Sender<Payload<Req, Res>>,
    mpsc::Receiver<Payload<Req, Res>>,
) {
    let (tx, rx) = mpsc::channel::<Payload<Req, Res>>(100);
    (tx, rx)
}

#[tokio::main]
async fn main() {
    let (mp_tx, mut mp_rx) = channel::<String, String>();
    tokio::spawn(async move {
        while let Some((request, os_tx)) = mp_rx.recv().await {
            println!("Received request from client {:?}", &request);
            let response = get_response(request).await;
            os_tx.send(response).unwrap();
        }
    });

    let inputs = ["Foo", "Bar", "Baz"];
    for each in inputs {
        let mp_tx = mp_tx.clone();
        if let Ok(response) = send_and_receive(mp_tx, each.into()).await {
            println!("Response: {:?}", response);
        }
    }
}

async fn get_response(req: String) -> String {
    req.to_ascii_uppercase()
}
