use serde::Deserialize;
use std::{fmt, time::Duration};
use tokio::{
    sync::{mpsc, watch},
    time::sleep,
};

#[derive(Deserialize)]
struct Data {
    #[serde(rename(deserialize = "value"))]
    val: u32,
}

struct Reader {
    id: u32,
    token: u32,
}

impl Reader {
    fn new(id: u32, token: u32) -> Self {
        Self { id, token }
    }

    async fn read(
        &self,
        token_receiver: watch::Receiver<u32>,
        message_sender: mpsc::Sender<Message>,
    ) {
        let file = format!("data/{}.csv", self.id);
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(file)
            .expect("Error: Failed to read from file");

        let mut data = rdr.deserialize::<Data>();
        let mut token_receiver = token_receiver.clone();

        while let Some(result) = data.next() {
            while *token_receiver.borrow() != self.token {
                token_receiver.changed().await.unwrap();
            }

            match result {
                Ok(Data { val }) => {
                    if let Err(e) = message_sender.send(Message { id: self.id, val }).await {
                        eprintln!("Error: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("Error: Failed to deserialize {e}");
                    continue;
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    }
}

struct Message {
    id: u32,
    val: u32,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reader {} emmit value {}", self.id, self.val)
    }
}

struct DataCollector {
    receiver: mpsc::Receiver<Message>,
}

impl DataCollector {
    fn new(receiver: mpsc::Receiver<Message>) -> Self {
        Self { receiver }
    }

    async fn collect(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            println!("{message}");
        }
    }
}

struct TokenController {
    total_readers: u32,
    sender: watch::Sender<u32>,
}

impl TokenController {
    fn new(total_readers: u32, sender: watch::Sender<u32>) -> Self {
        Self {
            total_readers,
            sender,
        }
    }

    async fn run(&self) {
        let mut current_token = 0;
        loop {
            self.sender.send(current_token).unwrap();

            sleep(Duration::from_secs(3)).await;

            current_token = (current_token + 1) % self.total_readers;
        }
    }
}

#[tokio::main]
async fn main() {
    let readers: Vec<Reader> = (0..=4).map(|i| Reader::new(i, i)).collect();

    let (message_tx, message_rx) = mpsc::channel(96);
    let (token_tx, token_rx) = watch::channel(0);

    let mut data_collector = DataCollector::new(message_rx);
    tokio::spawn(async move {
        data_collector.collect().await;
    });

    let token_controller = TokenController::new(readers.len() as u32, token_tx);
    tokio::spawn(async move {
        token_controller.run().await;
    });

    let mut handles = vec![];

    for reader in readers {
        let message_sender = message_tx.clone();
        let token_receiver = token_rx.clone();

        let handle = tokio::spawn(async move {
            reader.read(token_receiver, message_sender).await;
        });

        handles.push(handle);
    }

    for h in handles {
        let _ = h.await;
    }
}
