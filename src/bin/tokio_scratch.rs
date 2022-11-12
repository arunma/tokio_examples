use futures::FutureExt;
use anyhow::Result;
use tokio::sync::mpsc;

/*#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::unbounded_channel::<Message>();
    let message = Message {
        from: Address::Peer("node2".into()),
        to: Address::Peer("node1".into()),
        event: RPCMessage::AppendEntriesResponse {
            term: 1,
            success: true,
        },
    };
    tokio::spawn({
        async move {
            sender.send(message).unwrap();
        }
    });

    // while let Some(Some(message)) = receiver.recv().now_or_never() {
    //     println!("Received: {:?}", message);
    // }
    let ret_msg = receiver.recv().now_or_never().unwrap().unwrap();
    println!("{:?}", ret_msg)
}
*/

struct TokioScratch {
    sender: MessageSender,
}

impl TokioScratch {
    pub fn on_message(mut self, message: Message) -> Result<Self> {
        self.sender.send(message).unwrap();
        Ok(self)
    }
}


type MessageSender = mpsc::UnboundedSender<Message>;
type MessageReceiver = mpsc::UnboundedReceiver<Message>;

#[derive(Debug)]
pub struct Message {
    pub from: Address,
    pub to: Address,
    pub event: RPCMessage,
}

#[derive(Debug)]
pub enum Address {
    Peers,
    Peer(String),
    Local,
}

#[derive(Debug)]
pub enum RPCMessage {
    AppendEntries {
        term: u64,
        leader_id: String,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<String>,
        leader_commit_index: u64,
    },
    AppendEntriesResponse {
        term: u64,
        success: bool,
    },
    RequestVote {
        term: u64,
        candidate_id: String,
        last_log_index: u64,
        last_log_term: u64,
    },
    RequestVoteResponse {
        term: u64,
        vote_granted: bool,
    },
}


#[cfg(test)]
mod tests {
    use super::*;

    //#[tokio::test]
    #[test]
    fn test_send_receive_message() {
        let (sender, mut receiver) = mpsc::unbounded_channel::<Message>();

        let message = Message {
            from: Address::Peer("node2".into()),
            to: Address::Peer("node1".into()),
            event: RPCMessage::AppendEntriesResponse {
                term: 1,
                success: true,
            },
        };

        let scratch = TokioScratch {sender};
        scratch.on_message(message);

        /* tokio::spawn({
             async move {
                 sender.send(message).unwrap();
             }
         });
 */
        // while let Some(Some(message)) = receiver.recv().now_or_never() {
        //     println!("Received: {:?}", message);
        // }


        let ret_msg = receiver.recv().now_or_never().unwrap().unwrap();
        println!("{:?}", ret_msg)
    }
}
