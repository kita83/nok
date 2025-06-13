use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{stream::StreamExt, sink::SinkExt};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub r#type: String, // knock, message, join_room, leave_room, user_status
    pub user_id: Option<String>,
    pub target_user_id: Option<String>,
    pub room_id: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
    pub data: Option<serde_json::Value>,
}

pub struct WebSocketClient {
    sender: Option<mpsc::UnboundedSender<WebSocketMessage>>,
    receiver: Option<mpsc::UnboundedReceiver<WebSocketMessage>>,
}

impl std::fmt::Debug for WebSocketClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebSocketClient")
            .field("sender_active", &self.sender.is_some())
            .field("receiver_active", &self.receiver.is_some())
            .finish()
    }
}

impl WebSocketClient {
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
        }
    }

    pub async fn connect(&mut self, user_id: &str) -> Result<(), Box<dyn Error>> {
        let ws_url = format!("ws://localhost:8001/ws/{}", user_id);

        let (ws_stream, _) = connect_async(ws_url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let (tx_to_ws, mut rx_from_app) = mpsc::unbounded_channel::<WebSocketMessage>();
        let (tx_to_app, rx_to_app) = mpsc::unbounded_channel::<WebSocketMessage>();

        // WebSocketからアプリへの受信タスク
        tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage>(&text) {
                            if tx_to_app.send(ws_msg).is_err() {
                                break;
                            }
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(_) => break,
                    _ => {}
                }
            }
        });

        // アプリからWebSocketへの送信タスク
        tokio::spawn(async move {
            while let Some(msg) = rx_from_app.recv().await {
                if let Ok(text) = serde_json::to_string(&msg) {
                    if ws_sender.send(Message::Text(text)).await.is_err() {
                        break;
                    }
                }
            }
        });

        self.sender = Some(tx_to_ws);
        self.receiver = Some(rx_to_app);

        Ok(())
    }

    pub fn send_message(&self, message: WebSocketMessage) -> Result<(), Box<dyn Error>> {
        if let Some(ref sender) = self.sender {
            sender.send(message)?;
        }
        Ok(())
    }

    pub async fn receive_message(&mut self) -> Option<WebSocketMessage> {
        if let Some(ref mut receiver) = self.receiver {
            receiver.recv().await
        } else {
            None
        }
    }

    pub fn send_knock(&self, sender_id: &str, target_user_id: &str) -> Result<(), Box<dyn Error>> {
        let message = WebSocketMessage {
            r#type: "knock".to_string(),
            user_id: Some(sender_id.to_string()),
            target_user_id: Some(target_user_id.to_string()),
            room_id: None,
            content: Some("kon kon".to_string()),
            status: None,
            data: None,
        };
        self.send_message(message)
    }

    pub fn send_room_message(&self, sender_id: &str, room_id: &str, content: &str) -> Result<(), Box<dyn Error>> {
        let message = WebSocketMessage {
            r#type: "message".to_string(),
            user_id: Some(sender_id.to_string()),
            target_user_id: None,
            room_id: Some(room_id.to_string()),
            content: Some(content.to_string()),
            status: None,
            data: None,
        };
        self.send_message(message)
    }

    pub fn join_room(&self, user_id: &str, room_id: &str) -> Result<(), Box<dyn Error>> {
        let message = WebSocketMessage {
            r#type: "join_room".to_string(),
            user_id: Some(user_id.to_string()),
            target_user_id: None,
            room_id: Some(room_id.to_string()),
            content: None,
            status: None,
            data: None,
        };
        self.send_message(message)
    }

    pub fn leave_room(&self, user_id: &str, room_id: &str) -> Result<(), Box<dyn Error>> {
        let message = WebSocketMessage {
            r#type: "leave_room".to_string(),
            user_id: Some(user_id.to_string()),
            target_user_id: None,
            room_id: Some(room_id.to_string()),
            content: None,
            status: None,
            data: None,
        };
        self.send_message(message)
    }

    pub fn update_status(&self, user_id: &str, status: &str) -> Result<(), Box<dyn Error>> {
        let message = WebSocketMessage {
            r#type: "user_status".to_string(),
            user_id: Some(user_id.to_string()),
            target_user_id: None,
            room_id: None,
            content: None,
            status: Some(status.to_string()),
            data: None,
        };
        self.send_message(message)
    }

    pub async fn disconnect(&mut self) {
        // チャンネルをクローズして接続を切断
        self.sender = None;
        self.receiver = None;
    }
}