use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::provider::ProviderId;

/// A single usage data point from a provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsageData {
    pub provider: ProviderId,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cost_usd: f64,
    pub model: String,
    pub timestamp: DateTime<Utc>,
}

impl UsageData {
    pub fn total_tokens(&self) -> u64 {
        self.input_tokens + self.output_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_usage() -> UsageData {
        UsageData {
            provider: ProviderId::Anthropic,
            input_tokens: 1500,
            output_tokens: 500,
            cost_usd: 0.042,
            model: "claude-opus-4-6".to_string(),
            timestamp: Utc.with_ymd_and_hms(2026, 5, 15, 12, 0, 0).unwrap(),
        }
    }

    #[test]
    fn test_usage_data_total_tokens() {
        let usage = sample_usage();
        assert_eq!(usage.total_tokens(), 2000);
    }

    #[test]
    fn test_usage_data_total_tokens_zero() {
        let usage = UsageData {
            input_tokens: 0,
            output_tokens: 0,
            ..sample_usage()
        };
        assert_eq!(usage.total_tokens(), 0);
    }

    #[test]
    fn test_usage_data_serde_roundtrip() {
        let usage = sample_usage();
        let json = serde_json::to_string(&usage).unwrap();
        let back: UsageData = serde_json::from_str(&json).unwrap();
        assert_eq!(back, usage);
    }

    #[test]
    fn test_usage_data_json_fields() {
        let usage = sample_usage();
        let val: serde_json::Value = serde_json::to_value(&usage).unwrap();
        assert_eq!(val["provider"], "anthropic");
        assert_eq!(val["input_tokens"], 1500);
        assert_eq!(val["output_tokens"], 500);
        assert_eq!(val["model"], "claude-opus-4-6");
    }

    #[test]
    fn test_usage_data_clone() {
        let usage = sample_usage();
        let cloned = usage.clone();
        assert_eq!(usage, cloned);
    }
}
