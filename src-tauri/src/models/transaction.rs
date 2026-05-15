use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::provider::ProviderId;

/// A single recorded API interaction with cost.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub provider: ProviderId,
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cost_usd: f64,
    pub timestamp: DateTime<Utc>,
}

impl Transaction {
    pub fn total_tokens(&self) -> u64 {
        self.input_tokens + self.output_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_transaction() -> Transaction {
        Transaction {
            id: 1,
            provider: ProviderId::Cursor,
            model: "gpt-4o".to_string(),
            input_tokens: 3000,
            output_tokens: 1200,
            cost_usd: 0.085,
            timestamp: Utc.with_ymd_and_hms(2026, 5, 15, 14, 30, 0).unwrap(),
        }
    }

    #[test]
    fn test_transaction_total_tokens() {
        assert_eq!(sample_transaction().total_tokens(), 4200);
    }

    #[test]
    fn test_transaction_serde_roundtrip() {
        let txn = sample_transaction();
        let json = serde_json::to_string(&txn).unwrap();
        let back: Transaction = serde_json::from_str(&json).unwrap();
        assert_eq!(back, txn);
    }

    #[test]
    fn test_transaction_json_fields() {
        let txn = sample_transaction();
        let val: serde_json::Value = serde_json::to_value(&txn).unwrap();
        assert_eq!(val["id"], 1);
        assert_eq!(val["provider"], "cursor");
        assert_eq!(val["model"], "gpt-4o");
        assert_eq!(val["cost_usd"], 0.085);
    }

    #[test]
    fn test_transaction_zero_tokens() {
        let txn = Transaction {
            input_tokens: 0,
            output_tokens: 0,
            ..sample_transaction()
        };
        assert_eq!(txn.total_tokens(), 0);
    }
}
