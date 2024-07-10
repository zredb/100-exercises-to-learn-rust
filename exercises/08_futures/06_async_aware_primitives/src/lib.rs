/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
use tokio::sync::mpsc as mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Some(msg) = receiver.recv().await {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel(100);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .await;
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use tokio::sync::mpsc as mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel(10);
        let (response_sender,mut response_receiver) = mpsc::channel(10);
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .await;

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
