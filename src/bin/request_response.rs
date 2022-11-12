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

type Payload<Req, Res> = (Req, Responder<Res>);

pub struct Responder<Res> {
    sender: oneshot::Sender<Res>,
}

pub struct RequestReceiver<Req, Res> {
    receiver: mpsc::Receiver<Payload<Req, Res>>,
}

#[derive(Clone, Debug)]
pub struct RequestSender<Req, Res> {
    sender: mpsc::Sender<Payload<Req, Res>>,
}

impl<Res> Responder<Res> {
    pub async fn send(self, response: Res) -> Result<(), Res> {
        self.sender.send(response)
    }
}

impl<Req, Res> RequestReceiver<Req, Res> {
    pub async fn recv(&mut self) -> Result<Payload<Req, Res>, AppError> {
        match self.receiver.recv().await {
            Some(payload) => Ok(payload),
            None => Err(AppError::ReceiveError),
        }
    }
}

pub struct ResponseReceiver<Res> {
    os_rx: Option<oneshot::Receiver<Res>>,
}

impl<Res> ResponseReceiver<Res> {
    pub async fn recv(&mut self) -> Result<Res, AppError> {
        match self.os_rx.take() {
            Some(rx) => Ok(rx.await?),
            None => Err(AppError::ReceiveError),
        }
    }
}

impl<Req, Res> RequestSender<Req, Res> {
    pub async fn send(&self, req: Req) -> Result<ResponseReceiver<Res>, AppError> {
        let (os_tx, os_rx) = oneshot::channel::<Res>();
        let responder = Responder { sender: os_tx };
        let payload: Payload<Req, Res> = (req, responder);
        self.sender
            .send(payload)
            .await
            .map_err(|err| AppError::SendError(err.to_string()))?;
        let receiver = ResponseReceiver { os_rx: Some(os_rx) };
        Ok(receiver)
    }

    pub async fn send_and_receive(&self, req: Req) -> Result<Res, AppError> {
        let mut receiver = self.send(req).await?;

        receiver
            .recv()
            .await
            .map_err(|err| AppError::UnknownError(err.to_string()))
    }
}

pub fn channel<Req, Res>() -> (RequestSender<Req, Res>, RequestReceiver<Req, Res>) {
    let (tx, rx) = mpsc::channel::<Payload<Req, Res>>(100);
    let sender = RequestSender { sender: tx };
    let receiver = RequestReceiver { receiver: rx };
    (sender, receiver)
}

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = channel::<String, String>();
    tokio::spawn(async move {
        while let Ok((request, responder)) = receiver.recv().await {
            println!("Received request from client {:?}", &request);
            let response = get_response(request).await;
            responder.send(response).await;
        }
    });

    let inputs = ["Foo", "Bar", "Baz"];
    for each in inputs {
        if let Ok(response) = sender.send_and_receive(each.into()).await {
            println!("Response: {:?}", response);
        }
    }
}

async fn get_response(req: String) -> String {
    req.to_ascii_uppercase()
}
