use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    EmptyUsername,
    EmptyPassword,
    UsernameTooLong,
    UsernameTooShort,
    PasswordTooShort,
    InvalidUsernameCharacters,
    InvalidMatrixUsername,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyUsername => write!(f, "ユーザー名が空です"),
            ValidationError::EmptyPassword => write!(f, "パスワードが空です"),
            ValidationError::UsernameTooLong => write!(f, "ユーザー名が長すぎます（最大255文字）"),
            ValidationError::UsernameTooShort => write!(f, "ユーザー名が短すぎます（最小1文字）"),
            ValidationError::PasswordTooShort => write!(f, "パスワードが短すぎます（最小8文字）"),
            ValidationError::InvalidUsernameCharacters => write!(f, "ユーザー名に無効な文字が含まれています"),
            ValidationError::InvalidMatrixUsername => write!(f, "Matrix形式のユーザー名ではありません（例: @user:server.com）"),
        }
    }
}

impl std::error::Error for ValidationError {}

pub struct LoginValidator;

impl LoginValidator {
    /// Matrix username validation
    /// Validates that username follows Matrix format: @localpart:domain
    /// For local-only usernames, domain part is optional
    pub fn validate_username(username: &str) -> Result<String, ValidationError> {
        let trimmed = username.trim();
        
        if trimmed.is_empty() {
            return Err(ValidationError::EmptyUsername);
        }
        
        if trimmed.len() > 255 {
            return Err(ValidationError::UsernameTooLong);
        }
        
        if trimmed.len() < 1 {
            return Err(ValidationError::UsernameTooShort);
        }
        
        // Check for basic invalid characters
        if trimmed.contains('\n') || trimmed.contains('\r') || trimmed.contains('\0') {
            return Err(ValidationError::InvalidUsernameCharacters);
        }
        
        // For Matrix usernames, if it starts with @, validate Matrix format
        if trimmed.starts_with('@') {
            if !Self::is_valid_matrix_format(trimmed) {
                return Err(ValidationError::InvalidMatrixUsername);
            }
        }
        
        Ok(trimmed.to_string())
    }
    
    /// Password validation
    pub fn validate_password(password: &str) -> Result<String, ValidationError> {
        if password.is_empty() {
            return Err(ValidationError::EmptyPassword);
        }
        
        if password.len() < 8 {
            return Err(ValidationError::PasswordTooShort);
        }
        
        Ok(password.to_string())
    }
    
    /// Check if string follows Matrix username format: @localpart:domain
    fn is_valid_matrix_format(username: &str) -> bool {
        if !username.starts_with('@') {
            return false;
        }
        
        let without_at = &username[1..];
        let parts: Vec<&str> = without_at.split(':').collect();
        
        if parts.len() != 2 {
            return false;
        }
        
        let localpart = parts[0];
        let domain = parts[1];
        
        // Localpart validation (basic)
        if localpart.is_empty() || localpart.len() > 255 {
            return false;
        }
        
        // Domain validation (basic)
        if domain.is_empty() || domain.len() > 255 {
            return false;
        }
        
        // Check for basic invalid characters in localpart
        for c in localpart.chars() {
            if !c.is_ascii_alphanumeric() && c != '.' && c != '_' && c != '-' {
                return false;
            }
        }
        
        true
    }
    
    /// Validate both username and password
    pub fn validate_login_credentials(username: &str, password: &str) -> Result<(String, String), ValidationError> {
        let valid_username = Self::validate_username(username)?;
        let valid_password = Self::validate_password(password)?;
        Ok((valid_username, valid_password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_username_validation() {
        // Valid cases
        assert!(LoginValidator::validate_username("testuser").is_ok());
        assert!(LoginValidator::validate_username("@testuser:nok.local").is_ok());
        assert!(LoginValidator::validate_username("user123").is_ok());
        assert!(LoginValidator::validate_username("user_name").is_ok());
        assert!(LoginValidator::validate_username("user-name").is_ok());
        
        // Invalid cases
        assert!(LoginValidator::validate_username("").is_err());
        assert!(LoginValidator::validate_username("   ").is_err());
        assert!(LoginValidator::validate_username("user\nname").is_err());
        assert!(LoginValidator::validate_username("@").is_err());
        assert!(LoginValidator::validate_username("@user").is_err());
        assert!(LoginValidator::validate_username("@:domain").is_err());
        assert!(LoginValidator::validate_username("@user:").is_err());
    }
    
    #[test]
    fn test_password_validation() {
        // Valid cases
        assert!(LoginValidator::validate_password("password123").is_ok());
        assert!(LoginValidator::validate_password("SuperSecure!").is_ok());
        
        // Invalid cases
        assert!(LoginValidator::validate_password("").is_err());
        assert!(LoginValidator::validate_password("short").is_err());
        assert!(LoginValidator::validate_password("1234567").is_err());
    }
    
    #[test]
    fn test_matrix_format_validation() {
        assert!(LoginValidator::is_valid_matrix_format("@user:example.com"));
        assert!(LoginValidator::is_valid_matrix_format("@test123:nok.local"));
        assert!(LoginValidator::is_valid_matrix_format("@user_name:server.org"));
        
        assert!(!LoginValidator::is_valid_matrix_format("user:example.com"));
        assert!(!LoginValidator::is_valid_matrix_format("@user"));
        assert!(!LoginValidator::is_valid_matrix_format("@:example.com"));
        assert!(!LoginValidator::is_valid_matrix_format("@user:"));
        assert!(!LoginValidator::is_valid_matrix_format("@"));
    }
}