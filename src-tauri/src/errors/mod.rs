use std::time::Duration;

/// Errors from provider API interactions.
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("rate limited, retry after {retry_after:?}")]
    RateLimit { retry_after: Option<Duration> },

    #[error("authentication failed: invalid or expired API key")]
    AuthFailed,

    #[error("network error: {0}")]
    NetworkError(String),

    #[error("failed to parse response: {0}")]
    ParseError(String),

    #[error("operation not supported by this provider")]
    NotSupported,
}

/// Errors from the local SQLite storage layer.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("database connection failed: {0}")]
    ConnectionFailed(String),

    #[error("query failed: {0}")]
    QueryFailed(String),

    #[error("migration failed: {0}")]
    MigrationFailed(String),
}

/// Errors from OS keychain operations.
#[derive(Debug, thiserror::Error)]
pub enum KeychainError {
    #[error("credential not found for {0}")]
    NotFound(String),

    #[error("access denied to keychain")]
    AccessDenied,

    #[error("failed to store credential: {0}")]
    StoreError(String),
}

/// Top-level application error wrapping all subsystem errors.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("provider error: {0}")]
    Provider(#[from] ProviderError),

    #[error("storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("keychain error: {0}")]
    Keychain(#[from] KeychainError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_error_rate_limit_display() {
        let err = ProviderError::RateLimit {
            retry_after: Some(Duration::from_secs(30)),
        };
        assert!(err.to_string().contains("rate limited"));
        assert!(err.to_string().contains("30"));
    }

    #[test]
    fn test_provider_error_rate_limit_none_display() {
        let err = ProviderError::RateLimit { retry_after: None };
        assert!(err.to_string().contains("rate limited"));
    }

    #[test]
    fn test_provider_error_auth_failed_display() {
        let err = ProviderError::AuthFailed;
        assert!(err.to_string().contains("authentication failed"));
    }

    #[test]
    fn test_provider_error_network_display() {
        let err = ProviderError::NetworkError("connection refused".to_string());
        assert!(err.to_string().contains("connection refused"));
    }

    #[test]
    fn test_provider_error_parse_display() {
        let err = ProviderError::ParseError("unexpected field".to_string());
        assert!(err.to_string().contains("unexpected field"));
    }

    #[test]
    fn test_provider_error_not_supported_display() {
        let err = ProviderError::NotSupported;
        assert!(err.to_string().contains("not supported"));
    }

    #[test]
    fn test_storage_error_variants_display() {
        let errors = [
            (
                StorageError::ConnectionFailed("no file".to_string()),
                "connection failed",
            ),
            (
                StorageError::QueryFailed("syntax error".to_string()),
                "query failed",
            ),
            (
                StorageError::MigrationFailed("version mismatch".to_string()),
                "migration failed",
            ),
        ];
        for (err, expected_substring) in errors {
            assert!(
                err.to_string().contains(expected_substring),
                "Expected '{}' to contain '{}'",
                err,
                expected_substring
            );
        }
    }

    #[test]
    fn test_keychain_error_variants_display() {
        let errors = [
            (
                KeychainError::NotFound("anthropic".to_string()),
                "not found",
            ),
            (KeychainError::AccessDenied, "access denied"),
            (
                KeychainError::StoreError("OS error".to_string()),
                "failed to store",
            ),
        ];
        for (err, expected_substring) in errors {
            assert!(
                err.to_string().contains(expected_substring),
                "Expected '{}' to contain '{}'",
                err,
                expected_substring
            );
        }
    }

    #[test]
    fn test_app_error_from_provider_error() {
        let provider_err = ProviderError::AuthFailed;
        let app_err: AppError = provider_err.into();
        assert!(app_err.to_string().contains("authentication failed"));
    }

    #[test]
    fn test_app_error_from_storage_error() {
        let storage_err = StorageError::QueryFailed("bad sql".to_string());
        let app_err: AppError = storage_err.into();
        assert!(app_err.to_string().contains("bad sql"));
    }

    #[test]
    fn test_app_error_from_keychain_error() {
        let keychain_err = KeychainError::AccessDenied;
        let app_err: AppError = keychain_err.into();
        assert!(app_err.to_string().contains("access denied"));
    }
}
