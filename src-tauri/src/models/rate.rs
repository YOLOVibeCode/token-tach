use serde::{Deserialize, Serialize};

/// Current burn rate — how fast money is being spent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurnRate {
    /// Dollars per hour at current velocity.
    pub dollars_per_hour: f64,
    /// Window size in seconds over which this rate was calculated.
    pub window_seconds: u64,
    /// Number of data points used to calculate this rate.
    pub sample_count: u32,
}

impl BurnRate {
    /// A zero burn rate (idle state).
    pub fn zero() -> Self {
        Self {
            dollars_per_hour: 0.0,
            window_seconds: 0,
            sample_count: 0,
        }
    }

    /// Whether the burn rate indicates idle (no recent spend).
    pub fn is_idle(&self) -> bool {
        self.dollars_per_hour == 0.0 || self.sample_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burn_rate_zero() {
        let rate = BurnRate::zero();
        assert_eq!(rate.dollars_per_hour, 0.0);
        assert_eq!(rate.window_seconds, 0);
        assert_eq!(rate.sample_count, 0);
    }

    #[test]
    fn test_burn_rate_is_idle_when_zero() {
        assert!(BurnRate::zero().is_idle());
    }

    #[test]
    fn test_burn_rate_is_idle_when_zero_samples() {
        let rate = BurnRate {
            dollars_per_hour: 5.0,
            window_seconds: 3600,
            sample_count: 0,
        };
        assert!(rate.is_idle());
    }

    #[test]
    fn test_burn_rate_is_not_idle_when_active() {
        let rate = BurnRate {
            dollars_per_hour: 3.41,
            window_seconds: 3600,
            sample_count: 15,
        };
        assert!(!rate.is_idle());
    }

    #[test]
    fn test_burn_rate_serde_roundtrip() {
        let rate = BurnRate {
            dollars_per_hour: 3.41,
            window_seconds: 3600,
            sample_count: 15,
        };
        let json = serde_json::to_string(&rate).unwrap();
        let back: BurnRate = serde_json::from_str(&json).unwrap();
        assert_eq!(back, rate);
    }
}
