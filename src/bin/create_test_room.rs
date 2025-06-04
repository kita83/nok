use std::io::{self, Write};
use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏠 nok Test Room Creator");
    println!("========================");

    // ユーザー認証情報入力
    print!("Enter username (creator): ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim();

    print!("Enter password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    print!("Enter room name: ");
    io::stdout().flush()?;
    let mut room_name = String::new();
    io::stdin().read_line(&mut room_name)?;
    let room_name = room_name.trim();

    print!("Enter room alias (e.g., 'general'): ");
    io::stdout().flush()?;
    let mut room_alias = String::new();
    io::stdin().read_line(&mut room_alias)?;
    let room_alias = room_alias.trim();

    let client = reqwest::Client::new();
    let homeserver = "http://localhost:6167";

    // まずログインしてアクセストークンを取得
    println!("\n🔐 Logging in...");
    let user_id = format!("@{}:nok.local", username);
    let login_data = json!({
        "type": "m.login.password",
        "user": user_id,
        "password": password
    });

    let login_response = client
        .post(&format!("{}/_matrix/client/r0/login", homeserver))
        .json(&login_data)
        .send()
        .await?;

    if !login_response.status().is_success() {
        println!("❌ Login failed: {}", login_response.text().await?);
        return Ok(());
    }

    let login_json: serde_json::Value = serde_json::from_str(&login_response.text().await?)?;
    let access_token = login_json["access_token"].as_str()
        .ok_or("Access token not found")?;

    println!("✅ Login successful!");

    // ルーム作成
    println!("\n🏗️  Creating room '{}'...", room_name);

    let room_alias_full = format!("#{}:nok.local", room_alias);
    let create_room_data = json!({
        "room_alias_name": room_alias,
        "name": room_name,
        "topic": format!("Test room for nok Matrix migration - {}", room_name),
        "preset": "public_chat",
        "room_version": "10",
        "visibility": "public"
    });

    let create_response = client
        .post(&format!("{}/_matrix/client/r0/createRoom", homeserver))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&create_room_data)
        .send()
        .await?;

    let status = create_response.status();
    let response_text = create_response.text().await?;

    if status.is_success() {
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        if let Some(room_id) = response_json["room_id"].as_str() {
            println!("✅ Room created successfully!");
            println!("🆔 Room ID: {}", room_id);
            println!("🔗 Room Alias: {}", room_alias_full);
            println!("📛 Room Name: {}", room_name);

            // ウェルカムメッセージを送信
            println!("\n📝 Sending welcome message...");
            let welcome_message = format!("🎉 Welcome to {}! This room was created for nok Matrix migration testing.", room_name);
            let message_data = json!({
                "msgtype": "m.text",
                "body": welcome_message
            });

            let message_response = client
                .put(&format!("{}/_matrix/client/r0/rooms/{}/send/m.room.message/{}",
                            homeserver, room_id, chrono::Utc::now().timestamp_millis()))
                .header("Authorization", format!("Bearer {}", access_token))
                .json(&message_data)
                .send()
                .await?;

            if message_response.status().is_success() {
                println!("✅ Welcome message sent!");
            } else {
                println!("⚠️  Warning: Failed to send welcome message");
            }
        }
    } else {
        println!("❌ Room creation failed!");
        println!("Status: {}", status);
        println!("Response: {}", response_text);

        if response_text.contains("M_ROOM_IN_USE") {
            println!("💡 This room alias is already in use. Try a different alias.");
        }
    }

    Ok(())
}