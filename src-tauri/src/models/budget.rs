use serde::{Deserialize, Serialize};

/// How billing periods are calculated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingPeriod {
    /// Standard calendar month (1st to last day).
    CalendarMonth,
    /// Rolling 30-day window from today.
    Rolling30,
    /// Custom cycle starting on a specific day of month (1-28).
    Custom(u8),
}

/// User-configured spending limits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Budget {
    pub monthly_limit_usd: f64,
    pub daily_limit_usd: Option<f64>,
    /// Percentage thresholds at which to send notifications (e.g., [50, 75, 90, 100]).
    pub threshold_pcts: Vec<u8>,
    pub billing_period: BillingPeriod,
}

impl Budget {
    /// Returns the budget utilization percentage given current spend.
    pub fn utilization_pct(&self, current_spend: f64) -> f64 {
        if self.monthly_limit_usd <= 0.0 {
            return 0.0;
        }
        (current_spend / self.monthly_limit_usd) * 100.0
    }

    /// Returns which thresholds have been crossed given current spend.
    pub fn crossed_thresholds(&self, current_spend: f64) -> Vec<u8> {
        let pct = self.utilization_pct(current_spend);
        self.threshold_pcts
            .iter()
            .filter(|&&t| pct >= t as f64)
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_budget() -> Budget {
        Budget {
            monthly_limit_usd: 200.0,
            daily_limit_usd: Some(20.0),
            threshold_pcts: vec![50, 75, 90, 100],
            billing_period: BillingPeriod::CalendarMonth,
        }
    }

    #[test]
    fn test_utilization_pct_half() {
        let budget = sample_budget();
        assert_eq!(budget.utilization_pct(100.0), 50.0);
    }

    #[test]
    fn test_utilization_pct_zero_spend() {
        let budget = sample_budget();
        assert_eq!(budget.utilization_pct(0.0), 0.0);
    }

    #[test]
    fn test_utilization_pct_over_budget() {
        let budget = sample_budget();
        assert_eq!(budget.utilization_pct(250.0), 125.0);
    }

    #[test]
    fn test_utilization_pct_zero_limit() {
        let budget = Budget {
            monthly_limit_usd: 0.0,
            ..sample_budget()
        };
        assert_eq!(budget.utilization_pct(50.0), 0.0);
    }

    #[test]
    fn test_crossed_thresholds_none() {
        let budget = sample_budget();
        assert!(budget.crossed_thresholds(10.0).is_empty());
    }

    #[test]
    fn test_crossed_thresholds_partial() {
        let budget = sample_budget();
        // 60% = crosses 50 only
        assert_eq!(budget.crossed_thresholds(120.0), vec![50]);
    }

    #[test]
    fn test_crossed_thresholds_multiple() {
        let budget = sample_budget();
        // 92% = crosses 50, 75, 90
        assert_eq!(budget.crossed_thresholds(184.0), vec![50, 75, 90]);
    }

    #[test]
    fn test_crossed_thresholds_all() {
        let budget = sample_budget();
        assert_eq!(budget.crossed_thresholds(200.0), vec![50, 75, 90, 100]);
    }

    #[test]
    fn test_billing_period_serde_calendar_month() {
        let period = BillingPeriod::CalendarMonth;
        let json = serde_json::to_string(&period).unwrap();
        assert_eq!(json, "\"calendar_month\"");
        let back: BillingPeriod = serde_json::from_str(&json).unwrap();
        assert_eq!(back, period);
    }

    #[test]
    fn test_billing_period_serde_custom() {
        let period = BillingPeriod::Custom(15);
        let json = serde_json::to_string(&period).unwrap();
        let back: BillingPeriod = serde_json::from_str(&json).unwrap();
        assert_eq!(back, period);
    }

    #[test]
    fn test_budget_serde_roundtrip() {
        let budget = sample_budget();
        let json = serde_json::to_string(&budget).unwrap();
        let back: Budget = serde_json::from_str(&json).unwrap();
        assert_eq!(back, budget);
    }

    #[test]
    fn test_budget_daily_limit_none() {
        let budget = Budget {
            daily_limit_usd: None,
            ..sample_budget()
        };
        let json = serde_json::to_string(&budget).unwrap();
        let val: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(val["daily_limit_usd"].is_null());
    }
}
