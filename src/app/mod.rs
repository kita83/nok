mod state;
pub mod user;
mod room;
mod message;
mod config;

use crossterm::event::KeyEvent;
use crossterm::event::KeyCode;
pub use state::AppState;
pub use user::{User, UserStatus};
pub use room::Room;
pub use message::Message;
pub use config::Config;
use crate::ui::TabView;
use crate::api::{ApiClient, WebSocketClient};
use chrono;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PaneIdentifier {
    Rooms,
    Users,
    Messages,
    AsciiArt,
}

const AA_PANE_WIDTH: u16 = 60; // Approximate width, adjust as needed
const AA_PANE_HEIGHT: u16 = 20; // Approximate height, adjust as needed

pub struct App {
    pub state: AppState,
    pub users: Vec<User>,
    pub rooms: Vec<Room>,
    pub messages: Vec<Message>,
    pub current_user: User,
    pub current_room: usize,
    pub input: String,
    pub notification: Option<String>,
    pub selected_user: Option<usize>,
    pub selected_room_idx: usize,
    pub selected_message_idx: Option<usize>,
    pub error: Option<String>,
    pub view: TabView,
    pub focused_pane: PaneIdentifier,
    pub my_aa_position: (u16, u16), // (x, y) for player in ASCII Art pane
    // API連携用のフィールド
    pub api_client: ApiClient,
    pub websocket_client: WebSocketClient,
    pub connection_status: ConnectionStatus,
    // デバッグログ用
    pub debug_logs: Vec<String>,
    pub max_debug_logs: usize,
    // 設定機能
    pub config: Config,
    pub username_edit_buffer: String,
    // 設定画面用のログ
    pub settings_logs: Vec<String>,
    pub should_reconnect: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

impl App {
    pub fn new() -> Self {
        // 設定を読み込み
        let config = Config::load();

        // 設定からユーザー情報を作成
        let mut current_user = User::new(config.username.clone());
        // 永続化されたuser_idを使用
        current_user.id = Some(config.user_id.clone());

        let mut app = Self {
            state: AppState::Normal,
            users: Vec::new(),
            rooms: Vec::new(),
            messages: Vec::new(),
            current_user,
            current_room: 0,
            input: String::new(),
            notification: None,
            selected_user: None,
            selected_room_idx: 0,
            selected_message_idx: None,
            error: None,
            view: TabView::Rooms,
            focused_pane: PaneIdentifier::Rooms,
            my_aa_position: (5, 2), // Initial position for player's AA
            api_client: ApiClient::new(),
            websocket_client: WebSocketClient::new(),
            connection_status: ConnectionStatus::Disconnected,
            debug_logs: Vec::new(),
            max_debug_logs: 50,
            username_edit_buffer: config.username.clone(),
            config: config.clone(),
            settings_logs: Vec::new(),
            should_reconnect: false,
        };

        // 設定ファイル読み込み状況をデバッグログに追加
        app.add_debug_log(format!("Config loaded - user_id: {}, username: {}", config.user_id, config.username));

        app
    }

    // バックエンドとの接続を初期化
    pub async fn initialize_connection(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.connection_status = ConnectionStatus::Connecting;

        // まずヘルスチェック
        if let Err(e) = self.api_client.health_check().await {
            self.connection_status = ConnectionStatus::Error(format!("Backend connection failed: {}", e));
            return Err(e);
        }

        // ユーザーを作成または取得
        // 設定されたuser_idを優先的に使用し、なければユーザー名で検索
        let mut found_existing_user = false;

        // まず設定のuser_idで既存ユーザーを検索
        if let Ok(Some(existing_user)) = self.api_client.find_user_by_id(&self.config.user_id).await {
            // 既存ユーザーが見つかった場合、ユーザー名が変更されていれば更新
            if existing_user.name != self.current_user.name {
                if let Ok(updated_user) = self.api_client.update_user(&self.config.user_id, &self.current_user.name, None).await {
                    self.current_user.id = Some(updated_user.id.clone());
                    found_existing_user = true;
                }
            } else {
                self.current_user.id = Some(existing_user.id.clone());
                self.current_user.status = match existing_user.status.as_str() {
                    "online" => UserStatus::Online,
                    "away" => UserStatus::Away,
                    "busy" => UserStatus::Busy,
                    _ => UserStatus::Offline,
                };
                found_existing_user = true;
            }
        }

        // 既存ユーザーが見つからない場合は新規作成
        if !found_existing_user {
            match self.api_client.create_user(&self.current_user.name).await {
                Ok(api_user) => {
                    self.current_user.id = Some(api_user.id.clone());
                    // 新しいuser_idを設定に保存
                    self.config.user_id = api_user.id.clone();
                    self.config.save();
                }
                Err(e) => {
                    self.connection_status = ConnectionStatus::Error(format!("Failed to create user: {}", e));
                    return Err(e);
                }
            }
        }

        // データを初期読み込み
        self.refresh_data().await?;

        // WebSocket接続
        if let Some(ref user_id) = self.current_user.id {
            if let Err(e) = self.websocket_client.connect(user_id).await {
                self.connection_status = ConnectionStatus::Error(format!("WebSocket connection failed: {}", e));
                return Err(e);
            }
        }

        self.connection_status = ConnectionStatus::Connected;
        Ok(())
    }

    // 再接続機能
    pub async fn reconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.add_debug_log("Starting reconnection process".to_string());
        self.add_settings_log("Disconnecting from current session...".to_string());

        // 既存の接続を切断
        self.websocket_client.disconnect().await;
        self.connection_status = ConnectionStatus::Disconnected;

        self.add_settings_log("Reconnecting with new username...".to_string());

        // 新しいユーザー名で再接続
        match self.initialize_connection().await {
            Ok(_) => {
                self.add_settings_log("Reconnection completed successfully".to_string());
                Ok(())
            }
            Err(e) => {
                self.add_settings_log(format!("Reconnection failed: {}", e));
                Err(e)
            }
        }
    }

    // バックエンドからデータを更新
    pub async fn refresh_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // ユーザー一覧を取得
        match self.api_client.get_users().await {
            Ok(api_users) => {
                self.users = api_users.into_iter().map(|u| u.into()).collect();
            }
            Err(e) => {
                self.set_error(format!("Failed to load users: {}", e));
            }
        }

        // ルーム一覧を取得
        match self.api_client.get_rooms().await {
            Ok(api_rooms) => {
                self.rooms = api_rooms.into_iter().map(|r| r.into()).collect();
                // 現在のルームインデックスを調整
                if self.current_room >= self.rooms.len() {
                    self.current_room = 0;
                }
                // 選択されたルームインデックスも調整
                if self.selected_room_idx >= self.rooms.len() && !self.rooms.is_empty() {
                    self.selected_room_idx = 0;
                }

                // 各ルームのメンバー情報を取得してユーザーに関連付け
                self.update_user_room_associations().await;
            }
            Err(e) => {
                self.set_error(format!("Failed to load rooms: {}", e));
            }
        }

        // 現在のルームのメッセージを取得
        if let Some(room) = self.rooms.get(self.current_room) {
            if let Some(ref room_id) = room.id {
                match self.api_client.get_messages(Some(room_id)).await {
                    Ok(api_messages) => {
                        self.messages = api_messages.into_iter().map(|m| m.into()).collect();
                    }
                    Err(e) => {
                        self.set_error(format!("Failed to load messages: {}", e));
                    }
                }
            }
        }

        Ok(())
    }

    // WebSocketメッセージを処理
    pub async fn handle_websocket_message(&mut self) {
        if let Some(ws_message) = self.websocket_client.receive_message().await {
            self.add_debug_log(format!("Received WebSocket message: type={}", ws_message.r#type));
            match ws_message.r#type.as_str() {
                "knock" => {
                    if let Some(sender_name) = ws_message.user_id {
                        self.notification = Some(format!("🚪 {} is knocking!", sender_name));
                        // ノック音を再生（既存の機能）
                        self.knock(&sender_name);
                    }
                }
                "message" => {
                    if let (Some(content), Some(sender_id)) = (ws_message.content, ws_message.user_id) {
                        // 新しいメッセージを追加（実際の実装では、メッセージ履歴を再取得する方が良い）
                        let sender_name = self.users.iter()
                            .find(|u| u.id.as_ref() == Some(&sender_id))
                            .map(|u| u.name.clone())
                            .unwrap_or_else(|| "Unknown".to_string());

                        let room_name = if let Some(room_id) = ws_message.room_id {
                            self.rooms.iter()
                                .find(|r| r.id.as_ref() == Some(&room_id))
                                .map(|r| r.name.clone())
                                .unwrap_or_else(|| "Unknown Room".to_string())
                        } else {
                            "Direct Message".to_string()
                        };

                        let message = Message::new(sender_name, content, room_name);
                        self.messages.push(message);
                    }
                }
                "user_status" => {
                    if let (Some(user_id), Some(status)) = (ws_message.user_id, ws_message.status) {
                        // ユーザーステータスを更新
                        if let Some(user) = self.users.iter_mut().find(|u| u.id.as_ref() == Some(&user_id)) {
                            let new_status = match status.as_str() {
                                "online" => UserStatus::Online,
                                "away" => UserStatus::Away,
                                "busy" => UserStatus::Busy,
                                _ => UserStatus::Offline,
                            };
                            user.update_status(new_status);
                        }
                    }
                }
                _ => {
                    // 未知のメッセージタイプは無視
                }
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Normal => {
                // First, check for pane-specific keybindings if a certain pane is focused
                if self.focused_pane == PaneIdentifier::AsciiArt {
                    match key.code {
                        KeyCode::Char('i') => { // Move Up
                            self.my_aa_position.1 = self.my_aa_position.1.saturating_sub(1).max(1); // Min Y boundary
                            return; // Consume the key event
                        }
                        KeyCode::Char('k') => { // Move Down
                            self.my_aa_position.1 = (self.my_aa_position.1 + 1).min(AA_PANE_HEIGHT - 2); // Max Y boundary
                            return; // Consume the key event
                        }
                        KeyCode::Char('j') => { // Move Left
                            self.my_aa_position.0 = self.my_aa_position.0.saturating_sub(1).max(1); // Min X boundary
                            return;
                        }
                        KeyCode::Char('l') => { // Move Right
                            self.my_aa_position.0 = (self.my_aa_position.0 + 1).min(AA_PANE_WIDTH - 10); // Max X boundary (leave space for icon+name)
                            return;
                        }
                        _ => {} // Other keys will fall through to global keybindings
                    }
                }

                // Global keybindings (if not consumed by pane-specific logic above)
                match key.code {
                    KeyCode::Char('r') => self.focused_pane = PaneIdentifier::Rooms,
                    KeyCode::Char('u') => self.focused_pane = PaneIdentifier::Users,
                    KeyCode::Char('c') => self.focused_pane = PaneIdentifier::Messages,
                    KeyCode::Char('p') => self.focused_pane = PaneIdentifier::AsciiArt,
                    KeyCode::Char('1') => self.focused_pane = PaneIdentifier::Rooms,
                    KeyCode::Char('2') => self.focused_pane = PaneIdentifier::Users,
                    KeyCode::Char('3') => self.focused_pane = PaneIdentifier::Messages,
                    KeyCode::Char('4') => self.focused_pane = PaneIdentifier::AsciiArt,
                    KeyCode::Char('s') => {
                        // 設定画面への遷移
                        self.add_settings_log("Entering settings screen".to_string());
                        self.state = AppState::Settings;
                        self.username_edit_buffer = self.config.username.clone();
                        self.add_settings_log(format!("Current username: '{}'", self.config.username));
                    },
                    KeyCode::F(5) => {
                        // 再接続機能
                        self.notification = Some("Reconnecting...".to_string());
                        // 非同期処理なので実際の処理は別途必要
                    },
                    KeyCode::Tab => {
                        self.cycle_focus(false);
                    },
                    KeyCode::BackTab => {
                        self.cycle_focus(true);
                    },
                    KeyCode::Down | KeyCode::Char('j') => self.handle_down_key(),
                    KeyCode::Up | KeyCode::Char('k') => self.handle_up_key(),
                    KeyCode::Enter | KeyCode::Char(' ') => self.handle_confirm_key(),
                    KeyCode::Char('i') => {
                        if self.focused_pane == PaneIdentifier::Messages ||
                           self.focused_pane == PaneIdentifier::Rooms ||
                           self.focused_pane == PaneIdentifier::Users {
                            self.state = AppState::Input;
                            self.input.clear();
                            self.error = None;
                            self.notification = None;
                        }
                    },
                    KeyCode::Char('n') => {
                        // ノック機能
                        if self.focused_pane == PaneIdentifier::Users {
                            if let Some(user) = self.get_selected_user() {
                                if let (Some(sender_id), Some(target_id)) = (&self.current_user.id, &user.id) {
                                    if let Err(e) = self.websocket_client.send_knock(sender_id, target_id) {
                                        self.set_error(format!("Failed to send knock: {}", e));
                                    } else {
                                        self.notification = Some(format!("Knocked on {}", user.name));
                                    }
                                }
                            }
                        }
                    },
                    _ => {}
                }
            },
            AppState::Settings => {
                match key.code {
                    KeyCode::Enter => {
                        // ユーザー名を保存
                        if !self.username_edit_buffer.trim().is_empty() {
                            self.add_settings_log(format!("Attempting to save username: '{}'", self.username_edit_buffer.trim()));
                            let old_username = self.config.username.clone();
                            let new_username = self.username_edit_buffer.trim().to_string();

                            // ユーザー名が変更された場合のみリコネクト
                            let username_changed = old_username != new_username;

                            self.config.update_username(new_username);
                            self.current_user.name = self.config.username.clone();
                            self.add_settings_log(format!("Username changed from '{}' to '{}'", old_username, self.config.username));

                            if username_changed {
                                self.add_settings_log("Username changed, initiating automatic reconnection...".to_string());
                                self.notification = Some("Username updated! Reconnecting...".to_string());
                                // リコネクトフラグを設定（非同期処理のため、後で処理）
                                self.should_reconnect = true;
                            } else {
                                self.notification = Some("Username updated!".to_string());
                            }
                        } else {
                            self.add_settings_log("Username cannot be empty".to_string());
                        }
                        self.state = AppState::Normal;
                    },
                    KeyCode::Esc => {
                        // キャンセル
                        self.add_settings_log("Settings cancelled".to_string());
                        self.username_edit_buffer = self.config.username.clone();
                        self.state = AppState::Normal;
                    },
                    KeyCode::Char(c) => {
                        // 文字入力
                        if self.username_edit_buffer.len() < 20 { // 最大長制限
                            self.username_edit_buffer.push(c);
                            self.add_settings_log(format!("Editing username: '{}'", self.username_edit_buffer));
                        }
                    },
                    KeyCode::Backspace => {
                        // 文字削除
                        self.username_edit_buffer.pop();
                        self.add_settings_log(format!("Editing username: '{}'", self.username_edit_buffer));
                    },
                    _ => {}
                }
            },
            AppState::Input => {
                match key.code {
                    KeyCode::Enter => {
                        let input_clone = self.input.trim().to_string();
                        if !input_clone.is_empty() {
                            if self.focused_pane == PaneIdentifier::Messages ||
                               self.focused_pane == PaneIdentifier::Rooms ||
                               self.focused_pane == PaneIdentifier::Users {

                                if input_clone.starts_with("nok @") {
                                    self.handle_command(&input_clone);
                                } else {
                                    // メッセージを送信
                                    self.send_message(&input_clone);
                                }
                            }
                        }
                        self.input.clear();
                        self.state = AppState::Normal;
                    },
                    KeyCode::Esc => {
                        self.input.clear();
                        self.state = AppState::Normal;
                        self.error = None;
                        self.notification = None;
                    },
                    KeyCode::Char(c) => self.input.push(c),
                    KeyCode::Backspace => { self.input.pop(); },
                    _ => {}
                }
            },
        }
    }

    // メッセージ送信
    pub fn send_message(&mut self, content: &str) {
        if let Some(room) = self.rooms.get(self.current_room) {
            if let (Some(sender_id), Some(room_id)) = (&self.current_user.id, &room.id) {
                if let Err(e) = self.websocket_client.send_room_message(sender_id, room_id, content) {
                    self.set_error(format!("Failed to send message: {}", e));
                    return;
                }
                // メッセージ送信成功時は通知を表示
                self.notification = Some("Message sent!".to_string());
            }
        }
    }

    pub fn cycle_focus(&mut self, backward: bool) {
        let panes = [
            PaneIdentifier::Rooms,
            PaneIdentifier::Users,
            PaneIdentifier::Messages,
            PaneIdentifier::AsciiArt,
        ];
        let current_idx = panes.iter().position(|&p| p == self.focused_pane).unwrap_or(0);
        let next_idx = if backward {
            if current_idx == 0 { panes.len() - 1 } else { current_idx - 1 }
        } else {
            (current_idx + 1) % panes.len()
        };
        self.focused_pane = panes[next_idx];
    }

    pub fn handle_down_key(&mut self) {
        match self.focused_pane {
            PaneIdentifier::Rooms => {
                if !self.rooms.is_empty() {
                    self.selected_room_idx = (self.selected_room_idx + 1) % self.rooms.len();
                }
            }
            PaneIdentifier::Users => {
                if !self.users.is_empty() {
                    let current = self.selected_user.unwrap_or(0);
                    self.selected_user = Some((current + 1) % self.users.len());
                }
            }
            PaneIdentifier::Messages => {
                if !self.messages.is_empty() {
                    let current = self.selected_message_idx.unwrap_or(0);
                    self.selected_message_idx = Some((current + 1) % self.messages.len());
                }
            }
            _ => {}
        }
    }

    pub fn handle_up_key(&mut self) {
        match self.focused_pane {
            PaneIdentifier::Rooms => {
                if !self.rooms.is_empty() {
                    self.selected_room_idx = if self.selected_room_idx == 0 {
                        self.rooms.len() - 1
                    } else {
                        self.selected_room_idx - 1
                    };
                }
            }
            PaneIdentifier::Users => {
                if !self.users.is_empty() {
                    let current = self.selected_user.unwrap_or(0);
                    self.selected_user = Some(if current == 0 {
                        self.users.len() - 1
                    } else {
                        current - 1
                    });
                }
            }
            PaneIdentifier::Messages => {
                if !self.messages.is_empty() {
                    let current = self.selected_message_idx.unwrap_or(0);
                    self.selected_message_idx = Some(if current == 0 {
                        self.messages.len() - 1
                    } else {
                        current - 1
                    });
                }
            }
            _ => {}
        }
    }

    pub fn handle_confirm_key(&mut self) {
        match self.focused_pane {
            PaneIdentifier::Rooms => {
                // ルームを変更して、そのルームに参加
                self.current_room = self.selected_room_idx;
                if let Some(room) = self.rooms.get(self.current_room) {
                    if let (Some(user_id), Some(room_id)) = (&self.current_user.id, &room.id) {
                        if let Err(e) = self.websocket_client.join_room(user_id, room_id) {
                            self.set_error(format!("Failed to join room: {}", e));
                        } else {
                            self.notification = Some(format!("Joined {}", room.name));
                            // メッセージを再読み込み（実際の実装では非同期で行う）
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        // Placeholder for any periodic updates
    }

    pub fn knock(&mut self, target_user: &str) {
        self.notification = Some(format!("Knocked on {}", target_user));
        // Play knock sound
        if let Err(_) = crate::audio::play_knock_sound() {
            // Silently handle audio errors
        }
    }

    pub fn get_selected_user(&self) -> Option<&User> {
        self.selected_user.and_then(|idx| self.users.get(idx))
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn add_debug_log(&mut self, log: String) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        self.debug_logs.push(format!("[{}] {}", timestamp, log));
        if self.debug_logs.len() > self.max_debug_logs {
            self.debug_logs.remove(0);
        }
    }

    pub fn add_settings_log(&mut self, log: String) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        self.settings_logs.push(format!("[{}] {}", timestamp, log));
        if self.settings_logs.len() > 10 { // 設定ログは最大10件
            self.settings_logs.remove(0);
        }
    }

    pub fn select_next_user(&mut self) {
        if !self.users.is_empty() {
            let current = self.selected_user.unwrap_or(0);
            self.selected_user = Some((current + 1) % self.users.len());
        }
    }

    // ユーザーと部屋の関連付けを更新
    async fn update_user_room_associations(&mut self) {
        // まずすべてのユーザーの部屋リストをクリア
        for user in &mut self.users {
            user.rooms.clear();
        }

        // 各ルームのメンバーを取得してユーザーに関連付け
        for room in &self.rooms {
            if let Some(room_id) = &room.id {
                match self.api_client.get_room_members(room_id).await {
                    Ok(members) => {
                        for member in members {
                            // 該当するユーザーを見つけて部屋IDを追加
                            if let Some(user) = self.users.iter_mut().find(|u| u.id.as_ref() == Some(&member.id)) {
                                user.rooms.push(room_id.clone());
                            }
                        }
                    }
                    Err(e) => {
                        // エラーは記録するが処理は継続
                        eprintln!("Failed to get members for room {}: {}", room_id, e);
                    }
                }
            }
        }
    }

    pub fn handle_command(&mut self, input: &str) {
        if input.starts_with("nok @") {
            let target = input.trim_start_matches("nok @").trim();
            if let Some(user) = self.users.iter().find(|u| u.name == target) {
                if let (Some(sender_id), Some(target_id)) = (&self.current_user.id, &user.id) {
                    if let Err(e) = self.websocket_client.send_knock(sender_id, target_id) {
                        self.set_error(format!("Failed to send knock: {}", e));
                    } else {
                        self.notification = Some(format!("Knocked on {}", target));
                    }
                }
            } else {
                self.set_error(format!("User '{}' not found", target));
            }
        }
    }
}
