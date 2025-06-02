use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::app::{User, Room, Message};

const BASE_URL: &str = "http://localhost:8001";

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUser {
    pub id: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRoom {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub created_at: String,
    pub member_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiMessage {
    pub id: String,
    pub content: String,
    pub message_type: String,
    pub sender_id: String,
    pub sender_name: Option<String>,
    pub room_id: Option<String>,
    pub target_user_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessage {
    pub content: String,
    pub message_type: String,
    pub room_id: Option<String>,
    pub target_user_id: Option<String>,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<ApiUser>, Box<dyn Error>> {
        let url = format!("{}/api/users/", self.base_url);
        let response = self.client.get(&url).send().await?;
        let users: Vec<ApiUser> = response.json().await?;
        Ok(users)
    }

    pub async fn find_user_by_name(&self, name: &str) -> Result<Option<ApiUser>, Box<dyn Error>> {
        let url = format!("{}/api/users/", self.base_url);
        let response = self.client.get(&url).send().await?;
        let users: Vec<ApiUser> = response.json().await?;
        Ok(users.into_iter().find(|u| u.name == name))
    }

    pub async fn get_rooms(&self) -> Result<Vec<ApiRoom>, Box<dyn Error>> {
        let url = format!("{}/api/rooms/", self.base_url);
        let response = self.client.get(&url).send().await?;
        let rooms: Vec<ApiRoom> = response.json().await?;
        Ok(rooms)
    }

    pub async fn get_messages(&self, room_id: Option<&str>) -> Result<Vec<ApiMessage>, Box<dyn Error>> {
        let mut url = format!("{}/api/messages/", self.base_url);
        if let Some(room_id) = room_id {
            url = format!("{}?room_id={}", url, room_id);
        }
        let response = self.client.get(&url).send().await?;
        let messages: Vec<ApiMessage> = response.json().await?;
        Ok(messages)
    }

    pub async fn create_user(&self, name: &str) -> Result<ApiUser, Box<dyn Error>> {
        let url = format!("{}/api/users/", self.base_url);
        let new_user = CreateUser {
            name: name.to_string(),
        };
        let response = self.client.post(&url).json(&new_user).send().await?;
        let user: ApiUser = response.json().await?;
        Ok(user)
    }

    pub async fn send_message(&self, sender_id: &str, content: &str, room_id: Option<&str>, target_user_id: Option<&str>) -> Result<ApiMessage, Box<dyn Error>> {
        let url = format!("{}/api/messages/?sender_id={}", self.base_url, sender_id);
        let new_message = CreateMessage {
            content: content.to_string(),
            message_type: "text".to_string(),
            room_id: room_id.map(|s| s.to_string()),
            target_user_id: target_user_id.map(|s| s.to_string()),
        };
        let response = self.client.post(&url).json(&new_message).send().await?;
        let message: ApiMessage = response.json().await?;
        Ok(message)
    }

    pub async fn send_knock(&self, sender_id: &str, target_user_id: &str) -> Result<ApiMessage, Box<dyn Error>> {
        let url = format!("{}/api/messages/?sender_id={}", self.base_url, sender_id);
        let knock_message = CreateMessage {
            content: "kon kon".to_string(),
            message_type: "knock".to_string(),
            room_id: None,
            target_user_id: Some(target_user_id.to_string()),
        };
        let response = self.client.post(&url).json(&knock_message).send().await?;
        let message: ApiMessage = response.json().await?;
        Ok(message)
    }

    pub async fn join_room(&self, user_id: &str, room_id: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/rooms/{}/members", self.base_url, room_id);
        let payload = serde_json::json!({ "user_id": user_id });
        let response = self.client.post(&url).json(&payload).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to join room: {}", response.status()).into())
        }
    }

    pub async fn get_room_members(&self, room_id: &str) -> Result<Vec<ApiUser>, Box<dyn Error>> {
        let url = format!("{}/api/rooms/{}/members", self.base_url, room_id);
        let response = self.client.get(&url).send().await?;
        let members: Vec<serde_json::Value> = response.json().await?;

        // レスポンスからApiUserに変換
        let api_users: Result<Vec<ApiUser>, _> = members.into_iter().map(|member| {
            Ok(ApiUser {
                id: member["id"].as_str().unwrap_or("").to_string(),
                name: member["name"].as_str().unwrap_or("").to_string(),
                status: member["status"].as_str().unwrap_or("offline").to_string(),
                created_at: member.get("created_at").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            })
        }).collect();

        api_users
    }

    pub async fn health_check(&self) -> Result<bool, Box<dyn Error>> {
        let url = format!("{}/", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}

// Convert API types to App types
impl From<ApiUser> for User {
    fn from(api_user: ApiUser) -> Self {
        let mut user = User::new(api_user.name);
        user.id = Some(api_user.id);
        user.status = match api_user.status.as_str() {
            "online" => crate::app::UserStatus::Online,
            "away" => crate::app::UserStatus::Away,
            "busy" => crate::app::UserStatus::Busy,
            _ => crate::app::UserStatus::Offline,
        };
        user
    }
}

impl From<ApiRoom> for Room {
    fn from(api_room: ApiRoom) -> Self {
        let mut room = Room::new(api_room.name);
        room.id = Some(api_room.id);
        room.description = api_room.description;
        room
    }
}

impl From<ApiMessage> for Message {
    fn from(api_message: ApiMessage) -> Self {
        let sender_name = api_message.sender_name.unwrap_or_else(|| "Unknown".to_string());
        let room_name = api_message.room_id.unwrap_or_else(|| "Direct Message".to_string());
        let mut message = Message::new(sender_name, api_message.content, room_name);
        message.id = Some(api_message.id);
        message.message_type = api_message.message_type;
        message
    }
}