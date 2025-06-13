use std::io::{self, Write};
use nok::matrix::{MatrixClient, MatrixConfig};
// use nok::app::user::UserStatus; // 現在は使用していない

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 nok Matrix Integration Test");
    println!("================================");

    // テスト用の一時的な状態ストアパスを生成
    let timestamp = chrono::Utc::now().timestamp_millis();
    let state_store_path = format!("/tmp/nok_test_store_{}", timestamp);

    // Matrix設定
    let config = MatrixConfig {
        homeserver_url: "http://localhost:6167".to_string(),
        server_name: "nok.local".to_string(),
        device_name: "test-client".to_string(),
        state_store_path: state_store_path.clone(),
        store_path: state_store_path,
    };

    // Matrixクライアント作成
    println!("📡 Creating Matrix client...");
    let store_path_clone = config.state_store_path.clone();
    let client = MatrixClient::new(config).await?;

    // テスト用ユーザー情報
    println!("\n🔑 Test User Setup");
    print!("Enter username (e.g., 'test_user'): ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim();

    print!("Enter password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    // ログインテスト
    println!("\n🔐 Testing login...");
    match client.login(username, password).await {
        Ok(_) => println!("✅ Login successful!"),
        Err(e) => {
            println!("❌ Login failed: {}", e);
            println!("💡 Try registering the user first with Element or another Matrix client");
            return Ok(());
        }
    }

    // 基本情報取得
    if let Some(user_id) = client.user_id() {
        println!("👤 Logged in as: {}", user_id);
    }

    // シンク開始
    println!("\n🔄 Starting Matrix sync...");
    client.start_sync().await?;

    // ルーム一覧表示
    println!("\n🏠 Current rooms:");
    let rooms = client.rooms();
    if rooms.is_empty() {
        println!("   No rooms joined yet");
    } else {
        for room in &rooms {
            if let Ok(display_name) = room.display_name().await {
                println!("   - {} ({})", display_name, room.room_id());
            }
        }
    }

    // インタラクティブテストメニュー
    loop {
        println!("\n🧪 Test Menu:");
        println!("1. Join room");
        println!("2. Send message");
        println!("3. Send knock");
        println!("4. List rooms");
        println!("5. Exit");
        print!("Choose option (1-5): ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                print!("Enter room ID or alias (e.g., #general:nok.local): ");
                io::stdout().flush()?;
                let mut room_input = String::new();
                io::stdin().read_line(&mut room_input)?;
                let room_input = room_input.trim();

                match client.join_room(room_input).await {
                    Ok(room) => {
                        if let Ok(name) = room.display_name().await {
                            println!("✅ Joined room: {}", name);
                        }
                    }
                    Err(e) => println!("❌ Failed to join room: {}", e),
                }
            }
            "2" => {
                if rooms.is_empty() {
                    println!("❌ No rooms available. Join a room first.");
                    continue;
                }

                println!("Available rooms:");
                for (i, room) in rooms.iter().enumerate() {
                    if let Ok(name) = room.display_name().await {
                        println!("{}. {} ({})", i + 1, name, room.room_id());
                    }
                }

                print!("Select room number: ");
                io::stdout().flush()?;
                let mut room_num = String::new();
                io::stdin().read_line(&mut room_num)?;

                if let Ok(num) = room_num.trim().parse::<usize>() {
                    if num > 0 && num <= rooms.len() {
                        print!("Enter message: ");
                        io::stdout().flush()?;
                        let mut message = String::new();
                        io::stdin().read_line(&mut message)?;
                        let message = message.trim();

                        let room = &rooms[num - 1];
                        let room_id = room.room_id().to_owned();
                        match client.send_message(&room_id, message).await {
                            Ok(_) => println!("✅ Message sent!"),
                            Err(e) => println!("❌ Failed to send message: {}", e),
                        }
                    }
                }
            }
            "3" => {
                if rooms.is_empty() {
                    println!("❌ No rooms available. Join a room first.");
                    continue;
                }

                print!("Enter target user ID (e.g., @user:nok.local): ");
                io::stdout().flush()?;
                let mut target_user = String::new();
                io::stdin().read_line(&mut target_user)?;
                let target_user = target_user.trim();

                if let Ok(user_id) = matrix_sdk::ruma::UserId::parse(target_user) {
                    let owned_user_id = user_id.to_owned();
                    if let Some(room) = rooms.first() {
                        let room_id = room.room_id().to_owned();
                        match client.send_knock(&room_id, &owned_user_id).await {
                            Ok(_) => println!("✅ Knock sent to {}!", target_user),
                            Err(e) => println!("❌ Failed to send knock: {}", e),
                        }
                    }
                } else {
                    println!("❌ Invalid user ID format");
                }
            }
            "4" => {
                let updated_rooms = client.rooms();
                println!("🏠 Current rooms ({}):", updated_rooms.len());
                for room in &updated_rooms {
                    if let Ok(name) = room.display_name().await {
                        println!("   - {} ({})", name, room.room_id());
                    }
                }
            }
            "5" => {
                println!("👋 Stopping sync and exiting...");
                client.stop_sync().await;
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }

    println!("✨ Test completed!");

    // テスト用ストレージをクリーンアップ
    if let Err(_) = std::fs::remove_dir_all(&store_path_clone) {
        // エラーは無視（ディレクトリが存在しない場合など）
    }

    Ok(())
}