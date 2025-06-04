use nok::matrix::{MatrixClient, MatrixConfig};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing Matrix SDK connection to Conduit...");

    // Matrix設定
    let config = MatrixConfig::default();
    println!("📡 Homeserver: {}", config.homeserver_url);
    println!("🏠 Server name: {}", config.server_name);

    // Matrix clientを作成
    println!("🔧 Creating Matrix client...");
    let client = MatrixClient::new(config).await?;
    println!("✅ Matrix client created successfully");

    // ログインをテスト
    println!("🔐 Testing login with testuser...");
    match client.login("testuser", "testpass").await {
        Ok(_) => {
            println!("✅ Login successful!");

            if let Some(user_id) = client.user_id() {
                println!("👤 Logged in as: {}", user_id);
            }

            // sync開始をテスト
            println!("🔄 Starting sync...");
            client.start_sync().await?;
            println!("✅ Sync started");

            // 少し待ってからstop
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            println!("⏹️  Stopping sync...");
            client.stop_sync().await;

            // ルーム一覧を取得
            let rooms = client.rooms();
            println!("🏠 Found {} rooms", rooms.len());
            for room in rooms {
                let room_name = room.display_name().await
                    .map(|name| name.to_string())
                    .unwrap_or_else(|_| "Unknown Room".to_string());
                println!("  - {}: {}", room.room_id(), room_name);
            }

        },
        Err(e) => {
            println!("❌ Login failed: {}", e);
            return Err(e.into());
        }
    }

    println!("🎉 Matrix connection test completed successfully!");
    Ok(())
}