use serde::{Deserialize, Serialize};
use std::fmt;

/// Identifier for each supported AI coding tool provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderId {
    Anthropic,
    Cursor,
    OpenAi,
    Copilot,
    Windsurf,
    Kilo,
}

impl fmt::Display for ProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anthropic => write!(f, "Anthropic"),
            Self::Cursor => write!(f, "Cursor"),
            Self::OpenAi => write!(f, "OpenAI"),
            Self::Copilot => write!(f, "GitHub Copilot"),
            Self::Windsurf => write!(f, "Windsurf"),
            Self::Kilo => write!(f, "Kilo Code"),
        }
    }
}

/// Current operational status of a provider connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderStatus {
    Active,
    Idle,
    Error,
    Disconnected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_id_display_anthropic() {
        assert_eq!(ProviderId::Anthropic.to_string(), "Anthropic");
    }

    #[test]
    fn test_provider_id_display_openai() {
        assert_eq!(ProviderId::OpenAi.to_string(), "OpenAI");
    }

    #[test]
    fn test_provider_id_display_copilot() {
        assert_eq!(ProviderId::Copilot.to_string(), "GitHub Copilot");
    }

    #[test]
    fn test_provider_id_display_all_variants() {
        let variants = [
            (ProviderId::Anthropic, "Anthropic"),
            (ProviderId::Cursor, "Cursor"),
            (ProviderId::OpenAi, "OpenAI"),
            (ProviderId::Copilot, "GitHub Copilot"),
            (ProviderId::Windsurf, "Windsurf"),
            (ProviderId::Kilo, "Kilo Code"),
        ];
        for (id, expected) in variants {
            assert_eq!(id.to_string(), expected);
        }
    }

    #[test]
    fn test_provider_id_serde_roundtrip() {
        let id = ProviderId::Anthropic;
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"anthropic\"");
        let deserialized: ProviderId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_provider_id_serde_all_variants() {
        let variants = [
            (ProviderId::Anthropic, "\"anthropic\""),
            (ProviderId::Cursor, "\"cursor\""),
            (ProviderId::OpenAi, "\"open_ai\""),
            (ProviderId::Copilot, "\"copilot\""),
            (ProviderId::Windsurf, "\"windsurf\""),
            (ProviderId::Kilo, "\"kilo\""),
        ];
        for (id, expected_json) in variants {
            let json = serde_json::to_string(&id).unwrap();
            assert_eq!(json, expected_json);
            let back: ProviderId = serde_json::from_str(&json).unwrap();
            assert_eq!(back, id);
        }
    }

    #[test]
    fn test_provider_status_serde_roundtrip() {
        let statuses = [
            ProviderStatus::Active,
            ProviderStatus::Idle,
            ProviderStatus::Error,
            ProviderStatus::Disconnected,
        ];
        for status in statuses {
            let json = serde_json::to_string(&status).unwrap();
            let back: ProviderStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(back, status);
        }
    }

    #[test]
    fn test_provider_id_is_copy() {
        let id = ProviderId::Cursor;
        let copy = id;
        assert_eq!(id, copy); // both still valid — Copy trait
    }

    #[test]
    fn test_provider_id_hash_usable_in_hashmap() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(ProviderId::Anthropic, 42.0);
        map.insert(ProviderId::Cursor, 18.5);
        assert_eq!(map.get(&ProviderId::Anthropic), Some(&42.0));
        assert_eq!(map.get(&ProviderId::Cursor), Some(&18.5));
        assert_eq!(map.get(&ProviderId::Kilo), None);
    }
}
