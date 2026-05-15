use crate::errors::KeychainError;
use crate::models::ProviderId;

const SERVICE_NAME: &str = "com.tokentach.credentials";

/// Trait for secure credential storage (ISP: only credential operations).
pub trait SecretStore: Send + Sync {
    fn store(&self, provider: ProviderId, key: &str) -> Result<(), KeychainError>;
    fn retrieve(&self, provider: ProviderId) -> Result<String, KeychainError>;
    fn delete(&self, provider: ProviderId) -> Result<(), KeychainError>;
    fn exists(&self, provider: ProviderId) -> Result<bool, KeychainError>;
}

/// OS keychain implementation using the `keyring` crate.
pub struct OsSecretStore {
    service: String,
}

impl Default for OsSecretStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OsSecretStore {
    pub fn new() -> Self {
        Self {
            service: SERVICE_NAME.to_string(),
        }
    }

    #[cfg(test)]
    pub fn with_service(service: &str) -> Self {
        Self {
            service: service.to_string(),
        }
    }

    fn entry(&self, provider: ProviderId) -> Result<keyring::Entry, KeychainError> {
        let username = format!("{}", provider);
        keyring::Entry::new(&self.service, &username)
            .map_err(|e| KeychainError::StoreError(e.to_string()))
    }
}

impl SecretStore for OsSecretStore {
    fn store(&self, provider: ProviderId, key: &str) -> Result<(), KeychainError> {
        let entry = self.entry(provider)?;
        entry
            .set_password(key)
            .map_err(|e| KeychainError::StoreError(e.to_string()))
    }

    fn retrieve(&self, provider: ProviderId) -> Result<String, KeychainError> {
        let entry = self.entry(provider)?;
        entry.get_password().map_err(|e| match e {
            keyring::Error::NoEntry => KeychainError::NotFound(provider.to_string()),
            keyring::Error::Ambiguous(_) => KeychainError::NotFound(provider.to_string()),
            _ => KeychainError::AccessDenied,
        })
    }

    fn delete(&self, provider: ProviderId) -> Result<(), KeychainError> {
        let entry = self.entry(provider)?;
        entry.delete_credential().map_err(|e| match e {
            keyring::Error::NoEntry => KeychainError::NotFound(provider.to_string()),
            _ => KeychainError::StoreError(e.to_string()),
        })
    }

    fn exists(&self, provider: ProviderId) -> Result<bool, KeychainError> {
        match self.retrieve(provider) {
            Ok(_) => Ok(true),
            Err(KeychainError::NotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // In-memory mock for unit tests (no real keychain access)
    struct MockSecretStore {
        secrets: std::sync::Mutex<std::collections::HashMap<ProviderId, String>>,
    }

    impl MockSecretStore {
        fn new() -> Self {
            Self {
                secrets: std::sync::Mutex::new(std::collections::HashMap::new()),
            }
        }
    }

    impl SecretStore for MockSecretStore {
        fn store(&self, provider: ProviderId, key: &str) -> Result<(), KeychainError> {
            self.secrets
                .lock()
                .unwrap()
                .insert(provider, key.to_string());
            Ok(())
        }

        fn retrieve(&self, provider: ProviderId) -> Result<String, KeychainError> {
            self.secrets
                .lock()
                .unwrap()
                .get(&provider)
                .cloned()
                .ok_or_else(|| KeychainError::NotFound(provider.to_string()))
        }

        fn delete(&self, provider: ProviderId) -> Result<(), KeychainError> {
            self.secrets.lock().unwrap().remove(&provider);
            Ok(())
        }

        fn exists(&self, provider: ProviderId) -> Result<bool, KeychainError> {
            Ok(self.secrets.lock().unwrap().contains_key(&provider))
        }
    }

    #[test]
    fn test_store_and_retrieve() {
        let store = MockSecretStore::new();
        store
            .store(ProviderId::Anthropic, "sk-ant-test-key")
            .unwrap();
        let key = store.retrieve(ProviderId::Anthropic).unwrap();
        assert_eq!(key, "sk-ant-test-key");
    }

    #[test]
    fn test_retrieve_not_found() {
        let store = MockSecretStore::new();
        let result = store.retrieve(ProviderId::Cursor);
        assert!(matches!(result, Err(KeychainError::NotFound(_))));
    }

    #[test]
    fn test_store_overwrite() {
        let store = MockSecretStore::new();
        store.store(ProviderId::OpenAi, "key-1").unwrap();
        store.store(ProviderId::OpenAi, "key-2").unwrap();
        assert_eq!(store.retrieve(ProviderId::OpenAi).unwrap(), "key-2");
    }

    #[test]
    fn test_delete_existing() {
        let store = MockSecretStore::new();
        store.store(ProviderId::Cursor, "test-key").unwrap();
        store.delete(ProviderId::Cursor).unwrap();
        assert!(matches!(
            store.retrieve(ProviderId::Cursor),
            Err(KeychainError::NotFound(_))
        ));
    }

    #[test]
    fn test_delete_nonexistent_is_ok() {
        let store = MockSecretStore::new();
        // MockSecretStore is lenient — delete of missing key is fine
        assert!(store.delete(ProviderId::Kilo).is_ok());
    }

    #[test]
    fn test_exists_true() {
        let store = MockSecretStore::new();
        store.store(ProviderId::Windsurf, "key").unwrap();
        assert!(store.exists(ProviderId::Windsurf).unwrap());
    }

    #[test]
    fn test_exists_false() {
        let store = MockSecretStore::new();
        assert!(!store.exists(ProviderId::Windsurf).unwrap());
    }

    #[test]
    fn test_multiple_providers_isolated() {
        let store = MockSecretStore::new();
        store.store(ProviderId::Anthropic, "ant-key").unwrap();
        store.store(ProviderId::Cursor, "cursor-key").unwrap();
        store.store(ProviderId::OpenAi, "oai-key").unwrap();

        assert_eq!(store.retrieve(ProviderId::Anthropic).unwrap(), "ant-key");
        assert_eq!(store.retrieve(ProviderId::Cursor).unwrap(), "cursor-key");
        assert_eq!(store.retrieve(ProviderId::OpenAi).unwrap(), "oai-key");

        store.delete(ProviderId::Cursor).unwrap();
        assert!(store.exists(ProviderId::Anthropic).unwrap());
        assert!(!store.exists(ProviderId::Cursor).unwrap());
        assert!(store.exists(ProviderId::OpenAi).unwrap());
    }

    #[test]
    fn test_trait_object_usage() {
        let store: Box<dyn SecretStore> = Box::new(MockSecretStore::new());
        store.store(ProviderId::Anthropic, "key").unwrap();
        assert_eq!(store.retrieve(ProviderId::Anthropic).unwrap(), "key");
    }

    // Integration test: OsSecretStore struct can be created
    #[test]
    fn test_os_secret_store_construction() {
        let _store = OsSecretStore::new();
        let _store2 = OsSecretStore::with_service("com.tokentach.test");
    }
}
