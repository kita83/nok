use crate::matrix::{MatrixClient, MatrixConfig};
use crate::util::{ValidationError, NokError, NokResult};

/// Matrix-specific state management
#[derive(Debug)]
pub struct MatrixState {
    pub client: Option<MatrixClient>,
    pub config: MatrixConfig,
    pub enabled: bool,
    pub login: LoginState,
}

/// Login form state and validation
#[derive(Debug)]
pub struct LoginState {
    pub username: String,
    pub password: String,
    pub error: Option<String>,
    pub field_focus: LoginField,
    pub is_logging_in: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LoginField {
    Username,
    Password,
}

impl MatrixState {
    pub fn new(config: MatrixConfig) -> Self {
        Self {
            client: None,
            config,
            enabled: false,
            login: LoginState::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.client = None;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_initialized(&self) -> bool {
        self.client.is_some()
    }

    pub fn is_logged_in(&self) -> bool {
        self.client.as_ref()
            .map(|client| client.user_id().is_some())
            .unwrap_or(false)
    }

    pub fn set_client(&mut self, client: MatrixClient) {
        self.client = Some(client);
    }

    pub fn get_client(&self) -> Option<&MatrixClient> {
        self.client.as_ref()
    }

    pub fn clear_client(&mut self) {
        self.client = None;
    }

    /// Initialize Matrix client
    pub async fn initialize_client(&mut self) -> NokResult<()> {
        if !self.enabled {
            return Err(NokError::InternalError("Matrix mode is disabled".to_string()));
        }

        let client = MatrixClient::new(self.config.clone()).await?;
        self.client = Some(client);
        Ok(())
    }

    /// Login to Matrix homeserver
    pub async fn login(&mut self, username: &str, password: &str) -> NokResult<()> {
        if !self.enabled {
            return Err(NokError::InternalError("Matrix mode is disabled".to_string()));
        }

        let client = self.client.as_ref()
            .ok_or(NokError::MatrixClientNotInitialized)?;

        self.login.set_logging_in(true);
        
        let result = client.login(username, password).await
            .map_err(|e| NokError::MatrixLoginFailed(e.to_string()));

        self.login.set_logging_in(false);

        if result.is_ok() {
            self.login.clear_credentials();
            self.login.clear_error();
        }

        result
    }

    /// Start Matrix sync
    pub async fn start_sync(&self) -> NokResult<()> {
        let client = self.client.as_ref()
            .ok_or(NokError::MatrixClientNotInitialized)?;

        client.start_sync().await
            .map_err(|e| NokError::MatrixSyncError(e.to_string()))
    }

    /// Stop Matrix sync
    pub async fn stop_sync(&self) {
        if let Some(client) = &self.client {
            client.stop_sync().await;
        }
    }

    /// Get Matrix user ID
    pub fn user_id(&self) -> Option<String> {
        self.client.as_ref()
            .and_then(|client| client.user_id())
            .map(|id| id.to_string())
    }

    /// Get Matrix rooms
    pub fn rooms(&self) -> Vec<matrix_sdk::Room> {
        self.client.as_ref()
            .map(|client| client.rooms())
            .unwrap_or_default()
    }
}

impl LoginState {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            error: None,
            field_focus: LoginField::Username,
            is_logging_in: false,
        }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
        self.clear_error();
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
        self.clear_error();
    }

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    pub fn clear_credentials(&mut self) {
        self.username.clear();
        self.password.clear();
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn set_field_focus(&mut self, field: LoginField) {
        self.field_focus = field;
    }

    pub fn next_field(&mut self) {
        self.field_focus = match self.field_focus {
            LoginField::Username => LoginField::Password,
            LoginField::Password => LoginField::Username,
        };
    }

    pub fn set_logging_in(&mut self, logging_in: bool) {
        self.is_logging_in = logging_in;
    }

    pub fn validate_input(&self) -> Option<ValidationError> {
        use crate::util::LoginValidator;

        // Check username first
        if let Err(e) = LoginValidator::validate_username(&self.username) {
            return Some(e);
        }
        
        // Only check password if username is valid and not empty
        if !self.password.is_empty() {
            if let Err(e) = LoginValidator::validate_password(&self.password) {
                return Some(e);
            }
        }
        
        None
    }

    pub fn is_form_valid(&self) -> bool {
        use crate::util::LoginValidator;
        LoginValidator::validate_login_credentials(&self.username, &self.password).is_ok()
    }

    pub fn can_submit(&self) -> bool {
        self.is_form_valid() && !self.is_logging_in
    }
}

impl Default for LoginState {
    fn default() -> Self {
        Self::new()
    }
}