use std::fmt;
use crate::util::ValidationError;

#[derive(Debug)]
pub enum NokError {
    // Matrix関連エラー
    MatrixError(matrix_sdk::Error),
    MatrixClientNotInitialized,
    MatrixSyncError(String),
    MatrixLoginFailed(String),
    
    // 認証・検証エラー
    ValidationError(ValidationError),
    AuthenticationFailed(String),
    
    // ネットワーク関連エラー
    NetworkError(reqwest::Error),
    ConnectionTimeout,
    ConnectionFailed(String),
    
    // 設定関連エラー
    ConfigError(String),
    ConfigFileNotFound,
    ConfigParseError(String),
    
    // データベース関連エラー
    DatabaseError(rusqlite::Error),
    DataMigrationError(String),
    
    // ファイルシステム関連エラー
    FileSystemError(std::io::Error),
    FileNotFound(String),
    PermissionDenied(String),
    
    // UI関連エラー
    UIError(String),
    InvalidInput(String),
    
    // 汎用エラー
    InternalError(String),
    NotImplemented(String),
}

impl fmt::Display for NokError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Matrix関連
            NokError::MatrixError(e) => write!(f, "Matrixエラー: {}", e),
            NokError::MatrixClientNotInitialized => write!(f, "Matrixクライアントが初期化されていません"),
            NokError::MatrixSyncError(msg) => write!(f, "Matrix同期エラー: {}", msg),
            NokError::MatrixLoginFailed(msg) => write!(f, "Matrixログイン失敗: {}", msg),
            
            // 認証・検証
            NokError::ValidationError(e) => write!(f, "入力検証エラー: {}", e),
            NokError::AuthenticationFailed(msg) => write!(f, "認証失敗: {}", msg),
            
            // ネットワーク
            NokError::NetworkError(e) => write!(f, "ネットワークエラー: {}", e),
            NokError::ConnectionTimeout => write!(f, "接続タイムアウト"),
            NokError::ConnectionFailed(msg) => write!(f, "接続失敗: {}", msg),
            
            // 設定
            NokError::ConfigError(msg) => write!(f, "設定エラー: {}", msg),
            NokError::ConfigFileNotFound => write!(f, "設定ファイルが見つかりません"),
            NokError::ConfigParseError(msg) => write!(f, "設定ファイル解析エラー: {}", msg),
            
            // データベース
            NokError::DatabaseError(e) => write!(f, "データベースエラー: {}", e),
            NokError::DataMigrationError(msg) => write!(f, "データ移行エラー: {}", msg),
            
            // ファイルシステム
            NokError::FileSystemError(e) => write!(f, "ファイルシステムエラー: {}", e),
            NokError::FileNotFound(path) => write!(f, "ファイルが見つかりません: {}", path),
            NokError::PermissionDenied(path) => write!(f, "アクセス権限がありません: {}", path),
            
            // UI
            NokError::UIError(msg) => write!(f, "UIエラー: {}", msg),
            NokError::InvalidInput(msg) => write!(f, "無効な入力: {}", msg),
            
            // 汎用
            NokError::InternalError(msg) => write!(f, "内部エラー: {}", msg),
            NokError::NotImplemented(feature) => write!(f, "未実装の機能: {}", feature),
        }
    }
}

impl std::error::Error for NokError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NokError::MatrixError(e) => Some(e),
            NokError::ValidationError(e) => Some(e),
            NokError::NetworkError(e) => Some(e),
            NokError::DatabaseError(e) => Some(e),
            NokError::FileSystemError(e) => Some(e),
            _ => None,
        }
    }
}

// From implementations for automatic conversion
impl From<matrix_sdk::Error> for NokError {
    fn from(err: matrix_sdk::Error) -> Self {
        NokError::MatrixError(err)
    }
}

impl From<ValidationError> for NokError {
    fn from(err: ValidationError) -> Self {
        NokError::ValidationError(err)
    }
}

impl From<reqwest::Error> for NokError {
    fn from(err: reqwest::Error) -> Self {
        NokError::NetworkError(err)
    }
}

impl From<rusqlite::Error> for NokError {
    fn from(err: rusqlite::Error) -> Self {
        NokError::DatabaseError(err)
    }
}

impl From<std::io::Error> for NokError {
    fn from(err: std::io::Error) -> Self {
        NokError::FileSystemError(err)
    }
}

impl From<serde_json::Error> for NokError {
    fn from(err: serde_json::Error) -> Self {
        NokError::ConfigParseError(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for NokError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        NokError::InternalError(err.to_string())
    }
}

// Result type alias for convenience
pub type NokResult<T> = Result<T, NokError>;

impl NokError {
    /// Check if error is recoverable (e.g., network issues, temporary problems)
    pub fn is_recoverable(&self) -> bool {
        match self {
            NokError::NetworkError(_) 
            | NokError::ConnectionTimeout 
            | NokError::ConnectionFailed(_)
            | NokError::MatrixSyncError(_) => true,
            _ => false,
        }
    }
    
    /// Check if error should be retried automatically
    pub fn should_retry(&self) -> bool {
        match self {
            NokError::NetworkError(_) 
            | NokError::ConnectionTimeout => true,
            _ => false,
        }
    }
    
    /// Get user-friendly error message for UI display
    pub fn user_message(&self) -> String {
        match self {
            NokError::MatrixLoginFailed(_) => "ログインに失敗しました。ユーザー名とパスワードを確認してください。".to_string(),
            NokError::NetworkError(_) | NokError::ConnectionTimeout => "ネットワーク接続に問題があります。インターネット接続を確認してください。".to_string(),
            NokError::ValidationError(e) => e.to_string(),
            NokError::ConfigFileNotFound => "設定ファイルが見つかりません。初回起動時は自動作成されます。".to_string(),
            _ => "予期しないエラーが発生しました。".to_string(),
        }
    }
    
    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            NokError::ValidationError(_) | NokError::InvalidInput(_) => ErrorSeverity::Warning,
            NokError::NetworkError(_) | NokError::ConnectionTimeout => ErrorSeverity::Warning,
            NokError::MatrixLoginFailed(_) | NokError::AuthenticationFailed(_) => ErrorSeverity::Error,
            NokError::InternalError(_) | NokError::DatabaseError(_) => ErrorSeverity::Critical,
            _ => ErrorSeverity::Error,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARN"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}