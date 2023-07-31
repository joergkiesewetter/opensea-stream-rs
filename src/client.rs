use core::fmt::Display;
use futures_util::{SinkExt, StreamExt};
use tokio::{sync::mpsc, time::Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::protocol::{Collection, Network};

pub struct Client {
    send_tx: mpsc::Sender<PhoenixMessage>,
    read_rx: mpsc::Receiver<String>,
}

impl Client {
    pub async fn new(network: Network, api_key: &str) -> Self {
        let url = url::Url::parse(&format!("{}?token={}", network, api_key)).unwrap();

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");
        let (mut ws_write, mut ws_read) = ws_stream.split();

        let (send_tx, mut send_rx) = mpsc::channel::<PhoenixMessage>(4);
        let (read_tx, read_rx) = mpsc::channel::<String>(1024);

        // handler to send messages to the websocket
        tokio::spawn(async move {
            while let Some(message) = send_rx.recv().await {
                let payload = message.to_string();
                ws_write.send(Message::binary(payload)).await.unwrap();
            }
        });

        // handler to read messages from the websocket
        tokio::spawn(async move {
            while let Some(message) = ws_read.next().await {
                let message = message.unwrap();
                // println!("RECEIVED = {:?}", message);
                let payload = match message {
                    Message::Text(payload) => payload,
                    // Message::Binary(payload) => payload,
                    _ => panic!("unexpected message"),
                };
                read_tx.send(payload).await.unwrap();
            }
        });

        let send_heartbeat = send_tx.clone();

        tokio::spawn(async move {
            loop {
                let _ = send_heartbeat.send(PhoenixMessage::Heartbeat).await;
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });
        println!("connected to {}", network);

        Self { send_tx, read_rx }
    }

    pub async fn subscribe(&mut self, collection: Collection) {
        self.send_tx
            .clone()
            .send(PhoenixMessage::Subscribe(collection))
            .await
            .unwrap();
    }

    pub async fn read_event(&mut self) -> String {
        let message = self.read_rx.recv().await.unwrap();

        let event = serde_json::from_str::<serde_json::Value>(&message).unwrap();

        println!("{:#?}", event);
        message
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum PhoenixMessage {
    Heartbeat,
    Subscribe(Collection),
}

impl Display for PhoenixMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoenixMessage::Heartbeat => write!(
                f,
                "{{\"topic\": \"phoenix\", \"event\": \"heartbeat\", \"payload\": {{}}, \"ref\": 0}}"
            ),
            PhoenixMessage::Subscribe(collection) => write!(
                f,
                "{{\"topic\": \"{}\", \"event\": \"phx_join\", \"payload\": {{}}, \"ref\": 0}}",
                collection
            ),
        }
    }
}