use nok::migration::{legacy::LegacyDataLoader, converter::IdMappings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Testing legacy data migration...");

    // 1. Load legacy data
    println!("\n📖 Loading legacy data from database...");
    let loader = LegacyDataLoader::new("backend/nok.db");
    let legacy_data = loader.load_all().await?;

    println!("\n📊 Legacy Data Summary:");
    println!("  Users: {}", legacy_data.users.len());
    for user in &legacy_data.users {
        println!("    - {} ({}): {:?}", user.name, user.id, user.status);
    }

    println!("  Rooms: {}", legacy_data.rooms.len());
    for room in &legacy_data.rooms {
        println!("    - {} ({}): {:?}", room.name, room.id, room.description);
    }

    println!("  Messages: {}", legacy_data.messages.len());
    println!("  Room Memberships: {}", legacy_data.room_memberships.len());

    // 2. Generate ID mappings
    println!("\n🔄 Generating Matrix ID mappings...");
    let mappings = IdMappings::generate_from_legacy_data(
        &legacy_data.users,
        &legacy_data.rooms,
        "nok.local"
    );

    println!("\n🆔 User ID Mappings:");
    for (legacy_id, matrix_id) in &mappings.user_mappings {
        if let Some(user) = legacy_data.users.iter().find(|u| u.id == *legacy_id) {
            println!("  {} -> {}", user.name, matrix_id);
        }
    }

    println!("\n🏠 Room ID Mappings:");
    for (legacy_id, matrix_id) in &mappings.room_mappings {
        if let Some(room) = legacy_data.rooms.iter().find(|r| r.id == *legacy_id) {
            let alias = mappings.get_matrix_room_alias(legacy_id).cloned().unwrap_or_else(|| "unknown".to_string());
            println!("  {} -> {} (#{}:nok.local)", room.name, matrix_id, alias);
        }
    }

    // 3. Save mappings for reference
    println!("\n💾 Saving ID mappings...");
    mappings.save_to_file("id_mappings.json")?;

    // 4. Show room membership analysis
    println!("\n👥 Room Membership Analysis:");
    for (room_id, members) in &legacy_data.room_members {
        if let Some(room) = legacy_data.rooms.iter().find(|r| r.id == *room_id) {
            println!("  Room: {} ({} members)", room.name, members.len());
            for member_id in members {
                if let Some(user) = legacy_data.users.iter().find(|u| u.id == *member_id) {
                    println!("    - {}", user.name);
                }
            }
        }
    }

    // 5. Message type analysis
    println!("\n💬 Message Analysis:");
    let mut message_types = std::collections::HashMap::new();
    for message in &legacy_data.messages {
        let msg_type = message.message_type.as_deref().unwrap_or("text");
        *message_types.entry(msg_type).or_insert(0) += 1;
    }

    for (msg_type, count) in message_types {
        println!("  {}: {} messages", msg_type, count);
    }

    // Show sample knock messages
    println!("\n🚪 Sample Knock Messages:");
    for message in legacy_data.messages.iter().take(3) {
        if message.message_type.as_deref() == Some("knock") {
            let sender_name = legacy_data.users.iter()
                .find(|u| u.id == message.sender_id)
                .map(|u| &u.name)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());
            let target_name = message.target_user_id.as_ref()
                .and_then(|id| legacy_data.users.iter().find(|u| u.id == *id))
                .map(|u| &u.name)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());
            println!("  {} knocked on {}", sender_name, target_name);
        }
    }

    println!("\n✅ Migration test completed successfully!");
    println!("📝 Next steps:");
    println!("  1. Create Matrix accounts for users");
    println!("  2. Create Matrix rooms with appropriate settings");
    println!("  3. Invite users to their respective rooms");
    println!("  4. Set up room aliases and topics");

    Ok(())
}