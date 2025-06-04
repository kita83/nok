use std::io::{self, Write};
use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎭 nok Test User Registration");
    println!("=============================");

    // ユーザー情報入力
    print!("Enter username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim();

    print!("Enter password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    print!("Enter display name (optional): ");
    io::stdout().flush()?;
    let mut display_name = String::new();
    io::stdin().read_line(&mut display_name)?;
    let display_name = display_name.trim();

    // Conduitサーバーへのリクエスト
    let client = reqwest::Client::new();
    let homeserver = "http://localhost:6167";

    println!("\n🔐 Registering user...");

    // Matrix Client-Server APIのregisterエンドポイントを使用
    let register_data = json!({
        "username": username,
        "password": password,
        "auth": {
            "type": "m.login.registration_token",
            "token": "nokdev_registration_token"
        },
        "initial_device_display_name": "nok test client"
    });

    let response = client
        .post(&format!("{}/_matrix/client/r0/register", homeserver))
        .json(&register_data)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if status.is_success() {
        println!("✅ User registration successful!");
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        if let Some(user_id) = response_json["user_id"].as_str() {
            println!("👤 Registered user ID: {}", user_id);
        }

        // display nameを設定（オプション）
        if !display_name.is_empty() {
            println!("🏷️  Setting display name...");

            // まずアクセストークンを取得してログイン
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

            if login_response.status().is_success() {
                let login_json: serde_json::Value = serde_json::from_str(&login_response.text().await?)?;
                if let Some(access_token) = login_json["access_token"].as_str() {
                    // display name設定
                    let display_data = json!({
                        "displayname": display_name
                    });

                    let display_response = client
                        .put(&format!("{}/_matrix/client/r0/profile/{}/displayname", homeserver, username))
                        .header("Authorization", format!("Bearer {}", access_token))
                        .json(&display_data)
                        .send()
                        .await?;

                    if display_response.status().is_success() {
                        println!("✅ Display name set successfully!");
                    } else {
                        println!("⚠️  Warning: Failed to set display name");
                    }
                }
            }
        }

        println!("\n🎉 Registration complete!");
        println!("📝 You can now use these credentials:");
        println!("   Username: {}", username);
        println!("   User ID: @{}:nok.local", username);

    } else {
        println!("❌ Registration failed!");
        println!("Status: {}", status);
        println!("Response: {}", response_text);

        // よくあるエラーの説明
        if response_text.contains("M_USER_IN_USE") {
            println!("💡 This username is already taken. Try a different username.");
        } else if response_text.contains("M_FORBIDDEN") {
            println!("💡 Registration might be disabled on this server.");
        }
    }

    Ok(())
}