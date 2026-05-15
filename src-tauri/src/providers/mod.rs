use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::errors::ProviderError;
use crate::models::{Balance, ProviderId, Transaction, UsageData};

/// Fetches usage data (tokens + cost) from a provider.
#[async_trait]
pub trait UsageFetcher: Send + Sync {
    async fn fetch_usage(&self, since: DateTime<Utc>) -> Result<Vec<UsageData>, ProviderError>;
}

/// Checks the credit balance / billing status for a provider.
#[async_trait]
pub trait BalanceChecker: Send + Sync {
    async fn check_balance(&self) -> Result<Balance, ProviderError>;
}

/// Fetches individual transaction records from a provider.
#[async_trait]
pub trait TransactionFetcher: Send + Sync {
    async fn recent_transactions(&self, limit: usize) -> Result<Vec<Transaction>, ProviderError>;
}

/// Static metadata about a provider (does not require network).
pub trait ProviderInfo: Send + Sync {
    fn id(&self) -> ProviderId;
    fn display_name(&self) -> &str;
    fn is_configured(&self) -> bool;
}

/// Tests whether the provider's API key is valid and the service is reachable.
#[async_trait]
pub trait ConnectionTester: Send + Sync {
    async fn test_connection(&self) -> Result<(), ProviderError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ProviderStatus;

    // Mock provider implementing all traits — proves the interfaces are implementable.
    struct MockProvider {
        configured: bool,
    }

    impl ProviderInfo for MockProvider {
        fn id(&self) -> ProviderId {
            ProviderId::Anthropic
        }

        fn display_name(&self) -> &str {
            "Mock Anthropic"
        }

        fn is_configured(&self) -> bool {
            self.configured
        }
    }

    #[async_trait]
    impl UsageFetcher for MockProvider {
        async fn fetch_usage(
            &self,
            _since: DateTime<Utc>,
        ) -> Result<Vec<UsageData>, ProviderError> {
            Ok(vec![])
        }
    }

    #[async_trait]
    impl BalanceChecker for MockProvider {
        async fn check_balance(&self) -> Result<Balance, ProviderError> {
            Err(ProviderError::NotSupported)
        }
    }

    #[async_trait]
    impl TransactionFetcher for MockProvider {
        async fn recent_transactions(
            &self,
            _limit: usize,
        ) -> Result<Vec<Transaction>, ProviderError> {
            Ok(vec![])
        }
    }

    #[async_trait]
    impl ConnectionTester for MockProvider {
        async fn test_connection(&self) -> Result<(), ProviderError> {
            if self.configured {
                Ok(())
            } else {
                Err(ProviderError::AuthFailed)
            }
        }
    }

    #[test]
    fn test_provider_info_id() {
        let provider = MockProvider { configured: true };
        assert_eq!(provider.id(), ProviderId::Anthropic);
    }

    #[test]
    fn test_provider_info_display_name() {
        let provider = MockProvider { configured: true };
        assert_eq!(provider.display_name(), "Mock Anthropic");
    }

    #[test]
    fn test_provider_info_is_configured() {
        let configured = MockProvider { configured: true };
        let unconfigured = MockProvider { configured: false };
        assert!(configured.is_configured());
        assert!(!unconfigured.is_configured());
    }

    #[tokio::test]
    async fn test_usage_fetcher_returns_empty() {
        let provider = MockProvider { configured: true };
        let result = provider.fetch_usage(Utc::now()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_balance_checker_not_supported() {
        let provider = MockProvider { configured: true };
        let result = provider.check_balance().await;
        assert!(matches!(result, Err(ProviderError::NotSupported)));
    }

    #[tokio::test]
    async fn test_transaction_fetcher_returns_empty() {
        let provider = MockProvider { configured: true };
        let result = provider.recent_transactions(10).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_connection_tester_configured_ok() {
        let provider = MockProvider { configured: true };
        assert!(provider.test_connection().await.is_ok());
    }

    #[tokio::test]
    async fn test_connection_tester_unconfigured_fails() {
        let provider = MockProvider { configured: false };
        assert!(matches!(
            provider.test_connection().await,
            Err(ProviderError::AuthFailed)
        ));
    }

    // Prove traits work with trait objects (dependency inversion)
    #[tokio::test]
    async fn test_usage_fetcher_as_trait_object() {
        let provider: Box<dyn UsageFetcher> = Box::new(MockProvider { configured: true });
        let result = provider.fetch_usage(Utc::now()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_connection_tester_as_trait_object() {
        let provider: Box<dyn ConnectionTester> = Box::new(MockProvider { configured: true });
        assert!(provider.test_connection().await.is_ok());
    }

    // ISP test: a struct can implement only some traits
    struct PartialProvider;

    impl ProviderInfo for PartialProvider {
        fn id(&self) -> ProviderId {
            ProviderId::Copilot
        }
        fn display_name(&self) -> &str {
            "GitHub Copilot"
        }
        fn is_configured(&self) -> bool {
            true
        }
    }

    // PartialProvider only implements ProviderInfo — not forced to implement
    // UsageFetcher, BalanceChecker, etc. This IS the ISP test.
    #[test]
    fn test_isp_partial_provider_only_implements_info() {
        let _provider: &dyn ProviderInfo = &PartialProvider;
        // Compiles — proves ISP works. No forced implementations.
    }

    // Suppress unused warning for ProviderStatus import
    #[test]
    fn test_provider_status_exists() {
        let _status = ProviderStatus::Active;
    }
}
