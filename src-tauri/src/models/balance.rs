use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::provider::ProviderId;

/// Credit balance and billing period for a provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    pub provider: ProviderId,
    /// Remaining credits in USD, if the provider exposes this.
    pub remaining_usd: Option<f64>,
    /// Total credits used in the current billing period.
    pub credits_used: f64,
    pub billing_period_start: DateTime<Utc>,
    pub billing_period_end: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_balance() -> Balance {
        Balance {
            provider: ProviderId::OpenAi,
            remaining_usd: Some(42.50),
            credits_used: 57.50,
            billing_period_start: Utc.with_ymd_and_hms(2026, 5, 1, 0, 0, 0).unwrap(),
            billing_period_end: Utc.with_ymd_and_hms(2026, 5, 31, 23, 59, 59).unwrap(),
        }
    }

    #[test]
    fn test_balance_serde_roundtrip() {
        let balance = sample_balance();
        let json = serde_json::to_string(&balance).unwrap();
        let back: Balance = serde_json::from_str(&json).unwrap();
        assert_eq!(back, balance);
    }

    #[test]
    fn test_balance_remaining_none() {
        let balance = Balance {
            remaining_usd: None,
            ..sample_balance()
        };
        let json = serde_json::to_string(&balance).unwrap();
        let val: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(val["remaining_usd"].is_null());
    }

    #[test]
    fn test_balance_json_fields() {
        let balance = sample_balance();
        let val: serde_json::Value = serde_json::to_value(&balance).unwrap();
        assert_eq!(val["provider"], "open_ai");
        assert_eq!(val["remaining_usd"], 42.50);
        assert_eq!(val["credits_used"], 57.50);
    }
}
