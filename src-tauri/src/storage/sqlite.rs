use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::sync::Mutex;

use crate::errors::StorageError;
use crate::models::{BillingPeriod, Budget, ProviderId, Transaction};

use super::{BudgetStore, TransactionReader, TransactionWriter, UsageSummary};

pub struct SqliteRepository {
    conn: Mutex<Connection>,
}

impl SqliteRepository {
    /// Open a database at the given path (creates if missing).
    pub fn open(path: &str) -> Result<Self, StorageError> {
        let conn =
            Connection::open(path).map_err(|e| StorageError::ConnectionFailed(e.to_string()))?;
        let repo = Self {
            conn: Mutex::new(conn),
        };
        repo.run_migrations()?;
        Ok(repo)
    }

    /// Open an in-memory database (for testing).
    pub fn open_in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| StorageError::ConnectionFailed(e.to_string()))?;
        let repo = Self {
            conn: Mutex::new(conn),
        };
        repo.run_migrations()?;
        Ok(repo)
    }

    fn run_migrations(&self) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")
            .map_err(|e| StorageError::MigrationFailed(e.to_string()))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                provider TEXT NOT NULL,
                model TEXT NOT NULL,
                input_tokens INTEGER NOT NULL,
                output_tokens INTEGER NOT NULL,
                cost_usd REAL NOT NULL,
                timestamp TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_transactions_timestamp
                ON transactions(timestamp);

            CREATE INDEX IF NOT EXISTS idx_transactions_provider
                ON transactions(provider);

            CREATE TABLE IF NOT EXISTS budgets (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                monthly_limit_usd REAL NOT NULL,
                daily_limit_usd REAL,
                threshold_pcts TEXT NOT NULL,
                billing_period TEXT NOT NULL
            );",
        )
        .map_err(|e| StorageError::MigrationFailed(e.to_string()))?;

        Ok(())
    }
}

fn provider_to_str(p: ProviderId) -> &'static str {
    match p {
        ProviderId::Anthropic => "anthropic",
        ProviderId::Cursor => "cursor",
        ProviderId::OpenAi => "open_ai",
        ProviderId::Copilot => "copilot",
        ProviderId::Windsurf => "windsurf",
        ProviderId::Kilo => "kilo",
    }
}

fn str_to_provider(s: &str) -> Result<ProviderId, StorageError> {
    match s {
        "anthropic" => Ok(ProviderId::Anthropic),
        "cursor" => Ok(ProviderId::Cursor),
        "open_ai" => Ok(ProviderId::OpenAi),
        "copilot" => Ok(ProviderId::Copilot),
        "windsurf" => Ok(ProviderId::Windsurf),
        "kilo" => Ok(ProviderId::Kilo),
        other => Err(StorageError::QueryFailed(format!(
            "unknown provider: {other}"
        ))),
    }
}

fn row_to_transaction(row: &rusqlite::Row) -> rusqlite::Result<Transaction> {
    let provider_str: String = row.get(1)?;
    let timestamp_str: String = row.get(6)?;

    Ok(Transaction {
        id: row.get(0)?,
        provider: str_to_provider(&provider_str).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e))
        })?,
        model: row.get(2)?,
        input_tokens: row.get::<_, i64>(3)? as u64,
        output_tokens: row.get::<_, i64>(4)? as u64,
        cost_usd: row.get(5)?,
        timestamp: DateTime::parse_from_rfc3339(&timestamp_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    6,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?,
    })
}

impl TransactionWriter for SqliteRepository {
    fn insert_transaction(&self, txn: &Transaction) -> Result<i64, StorageError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO transactions (provider, model, input_tokens, output_tokens, cost_usd, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                provider_to_str(txn.provider),
                txn.model,
                txn.input_tokens as i64,
                txn.output_tokens as i64,
                txn.cost_usd,
                txn.timestamp.to_rfc3339(),
            ],
        )
        .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(conn.last_insert_rowid())
    }
}

impl TransactionReader for SqliteRepository {
    fn recent_transactions(
        &self,
        limit: usize,
        provider: Option<ProviderId>,
    ) -> Result<Vec<Transaction>, StorageError> {
        let conn = self.conn.lock().unwrap();

        let (sql, boxed_params): (String, Vec<Box<dyn rusqlite::types::ToSql>>) = match provider {
            Some(p) => (
                "SELECT id, provider, model, input_tokens, output_tokens, cost_usd, timestamp
                 FROM transactions WHERE provider = ?1 ORDER BY timestamp DESC LIMIT ?2"
                    .to_string(),
                vec![
                    Box::new(provider_to_str(p).to_string()),
                    Box::new(limit as i64),
                ],
            ),
            None => (
                "SELECT id, provider, model, input_tokens, output_tokens, cost_usd, timestamp
                 FROM transactions ORDER BY timestamp DESC LIMIT ?1"
                    .to_string(),
                vec![Box::new(limit as i64)],
            ),
        };

        let params_refs: Vec<&dyn rusqlite::types::ToSql> =
            boxed_params.iter().map(|b| b.as_ref()).collect();

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let rows = stmt
            .query_map(params_refs.as_slice(), row_to_transaction)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let mut transactions = Vec::new();
        for row in rows {
            transactions.push(row.map_err(|e| StorageError::QueryFailed(e.to_string()))?);
        }
        Ok(transactions)
    }

    fn transactions_since(
        &self,
        since: DateTime<Utc>,
        provider: Option<ProviderId>,
    ) -> Result<Vec<Transaction>, StorageError> {
        let conn = self.conn.lock().unwrap();

        let (sql, boxed_params): (String, Vec<Box<dyn rusqlite::types::ToSql>>) = match provider {
            Some(p) => (
                "SELECT id, provider, model, input_tokens, output_tokens, cost_usd, timestamp
                 FROM transactions WHERE timestamp >= ?1 AND provider = ?2 ORDER BY timestamp DESC"
                    .to_string(),
                vec![
                    Box::new(since.to_rfc3339()),
                    Box::new(provider_to_str(p).to_string()),
                ],
            ),
            None => (
                "SELECT id, provider, model, input_tokens, output_tokens, cost_usd, timestamp
                 FROM transactions WHERE timestamp >= ?1 ORDER BY timestamp DESC"
                    .to_string(),
                vec![Box::new(since.to_rfc3339())],
            ),
        };

        let params_refs: Vec<&dyn rusqlite::types::ToSql> =
            boxed_params.iter().map(|b| b.as_ref()).collect();

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let rows = stmt
            .query_map(params_refs.as_slice(), row_to_transaction)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let mut transactions = Vec::new();
        for row in rows {
            transactions.push(row.map_err(|e| StorageError::QueryFailed(e.to_string()))?);
        }
        Ok(transactions)
    }
}

impl UsageSummary for SqliteRepository {
    fn total_cost_since(
        &self,
        since: DateTime<Utc>,
        provider: Option<ProviderId>,
    ) -> Result<f64, StorageError> {
        let conn = self.conn.lock().unwrap();

        let (sql, boxed_params): (String, Vec<Box<dyn rusqlite::types::ToSql>>) = match provider {
            Some(p) => (
                "SELECT COALESCE(SUM(cost_usd), 0.0) FROM transactions
                 WHERE timestamp >= ?1 AND provider = ?2"
                    .to_string(),
                vec![
                    Box::new(since.to_rfc3339()),
                    Box::new(provider_to_str(p).to_string()),
                ],
            ),
            None => (
                "SELECT COALESCE(SUM(cost_usd), 0.0) FROM transactions WHERE timestamp >= ?1"
                    .to_string(),
                vec![Box::new(since.to_rfc3339())],
            ),
        };

        let params_refs: Vec<&dyn rusqlite::types::ToSql> =
            boxed_params.iter().map(|b| b.as_ref()).collect();

        conn.query_row(&sql, params_refs.as_slice(), |row| row.get(0))
            .map_err(|e| StorageError::QueryFailed(e.to_string()))
    }

    fn cost_by_provider_since(
        &self,
        since: DateTime<Utc>,
    ) -> Result<Vec<(ProviderId, f64)>, StorageError> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT provider, COALESCE(SUM(cost_usd), 0.0)
                 FROM transactions WHERE timestamp >= ?1
                 GROUP BY provider ORDER BY SUM(cost_usd) DESC",
            )
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let rows = stmt
            .query_map(params![since.to_rfc3339()], |row| {
                let provider_str: String = row.get(0)?;
                let cost: f64 = row.get(1)?;
                Ok((provider_str, cost))
            })
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let mut results = Vec::new();
        for row in rows {
            let (provider_str, cost) = row.map_err(|e| StorageError::QueryFailed(e.to_string()))?;
            let provider = str_to_provider(&provider_str)?;
            results.push((provider, cost));
        }
        Ok(results)
    }
}

impl BudgetStore for SqliteRepository {
    fn save_budget(&self, budget: &Budget) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        let thresholds_json = serde_json::to_string(&budget.threshold_pcts)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;
        let period_json = serde_json::to_string(&budget.billing_period)
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO budgets (id, monthly_limit_usd, daily_limit_usd, threshold_pcts, billing_period)
             VALUES (1, ?1, ?2, ?3, ?4)",
            params![
                budget.monthly_limit_usd,
                budget.daily_limit_usd,
                thresholds_json,
                period_json,
            ],
        )
        .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        Ok(())
    }

    fn load_budget(&self) -> Result<Option<Budget>, StorageError> {
        let conn = self.conn.lock().unwrap();

        let result = conn.query_row(
            "SELECT monthly_limit_usd, daily_limit_usd, threshold_pcts, billing_period FROM budgets WHERE id = 1",
            [],
            |row| {
                let monthly_limit_usd: f64 = row.get(0)?;
                let daily_limit_usd: Option<f64> = row.get(1)?;
                let thresholds_json: String = row.get(2)?;
                let period_json: String = row.get(3)?;
                Ok((monthly_limit_usd, daily_limit_usd, thresholds_json, period_json))
            },
        );

        match result {
            Ok((monthly, daily, thresholds_json, period_json)) => {
                let threshold_pcts: Vec<u8> = serde_json::from_str(&thresholds_json)
                    .map_err(|e| StorageError::QueryFailed(e.to_string()))?;
                let billing_period: BillingPeriod = serde_json::from_str(&period_json)
                    .map_err(|e| StorageError::QueryFailed(e.to_string()))?;

                Ok(Some(Budget {
                    monthly_limit_usd: monthly,
                    daily_limit_usd: daily,
                    threshold_pcts,
                    billing_period,
                }))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(StorageError::QueryFailed(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn make_repo() -> SqliteRepository {
        SqliteRepository::open_in_memory().unwrap()
    }

    fn make_transaction(provider: ProviderId, cost: f64, timestamp: DateTime<Utc>) -> Transaction {
        Transaction {
            id: 0, // will be assigned by DB
            provider,
            model: "test-model".to_string(),
            input_tokens: 1000,
            output_tokens: 500,
            cost_usd: cost,
            timestamp,
        }
    }

    // --- TransactionWriter tests ---

    #[test]
    fn test_insert_transaction_returns_id() {
        let repo = make_repo();
        let txn = make_transaction(ProviderId::Anthropic, 0.05, Utc::now());
        let id = repo.insert_transaction(&txn).unwrap();
        assert!(id > 0);
    }

    #[test]
    fn test_insert_multiple_transactions_sequential_ids() {
        let repo = make_repo();
        let id1 = repo
            .insert_transaction(&make_transaction(ProviderId::Anthropic, 0.05, Utc::now()))
            .unwrap();
        let id2 = repo
            .insert_transaction(&make_transaction(ProviderId::Cursor, 0.10, Utc::now()))
            .unwrap();
        assert_eq!(id2, id1 + 1);
    }

    // --- TransactionReader tests ---

    #[test]
    fn test_recent_transactions_empty() {
        let repo = make_repo();
        let result = repo.recent_transactions(10, None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_recent_transactions_ordered_by_timestamp_desc() {
        let repo = make_repo();
        let t1 = Utc.with_ymd_and_hms(2026, 5, 15, 10, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2026, 5, 15, 11, 0, 0).unwrap();
        let t3 = Utc.with_ymd_and_hms(2026, 5, 15, 12, 0, 0).unwrap();

        repo.insert_transaction(&make_transaction(ProviderId::Anthropic, 0.01, t1))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::Cursor, 0.02, t3))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::OpenAi, 0.03, t2))
            .unwrap();

        let txns = repo.recent_transactions(10, None).unwrap();
        assert_eq!(txns.len(), 3);
        assert_eq!(txns[0].timestamp, t3); // most recent first
        assert_eq!(txns[1].timestamp, t2);
        assert_eq!(txns[2].timestamp, t1);
    }

    #[test]
    fn test_recent_transactions_with_limit() {
        let repo = make_repo();
        for i in 0..5 {
            repo.insert_transaction(&make_transaction(
                ProviderId::Anthropic,
                0.01 * (i as f64),
                Utc::now(),
            ))
            .unwrap();
        }
        let txns = repo.recent_transactions(3, None).unwrap();
        assert_eq!(txns.len(), 3);
    }

    #[test]
    fn test_recent_transactions_filtered_by_provider() {
        let repo = make_repo();
        repo.insert_transaction(&make_transaction(ProviderId::Anthropic, 0.05, Utc::now()))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::Cursor, 0.10, Utc::now()))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::Anthropic, 0.03, Utc::now()))
            .unwrap();

        let anthropic_txns = repo
            .recent_transactions(10, Some(ProviderId::Anthropic))
            .unwrap();
        assert_eq!(anthropic_txns.len(), 2);
        assert!(anthropic_txns
            .iter()
            .all(|t| t.provider == ProviderId::Anthropic));

        let cursor_txns = repo
            .recent_transactions(10, Some(ProviderId::Cursor))
            .unwrap();
        assert_eq!(cursor_txns.len(), 1);
    }

    #[test]
    fn test_transactions_since() {
        let repo = make_repo();
        let old = Utc.with_ymd_and_hms(2026, 5, 1, 0, 0, 0).unwrap();
        let recent = Utc.with_ymd_and_hms(2026, 5, 15, 0, 0, 0).unwrap();
        let cutoff = Utc.with_ymd_and_hms(2026, 5, 10, 0, 0, 0).unwrap();

        repo.insert_transaction(&make_transaction(ProviderId::Anthropic, 0.05, old))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::Cursor, 0.10, recent))
            .unwrap();

        let txns = repo.transactions_since(cutoff, None).unwrap();
        assert_eq!(txns.len(), 1);
        assert_eq!(txns[0].provider, ProviderId::Cursor);
    }

    #[test]
    fn test_transactions_since_filtered_by_provider() {
        let repo = make_repo();
        let ts = Utc.with_ymd_and_hms(2026, 5, 15, 0, 0, 0).unwrap();
        let since = Utc.with_ymd_and_hms(2026, 5, 1, 0, 0, 0).unwrap();

        repo.insert_transaction(&make_transaction(ProviderId::Anthropic, 0.05, ts))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::Cursor, 0.10, ts))
            .unwrap();

        let txns = repo
            .transactions_since(since, Some(ProviderId::Anthropic))
            .unwrap();
        assert_eq!(txns.len(), 1);
        assert_eq!(txns[0].provider, ProviderId::Anthropic);
    }

    // --- UsageSummary tests ---

    #[test]
    fn test_total_cost_since_empty() {
        let repo = make_repo();
        let cost = repo.total_cost_since(Utc::now(), None).unwrap();
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_total_cost_since_sums_correctly() {
        let repo = make_repo();
        let ts = Utc.with_ymd_and_hms(2026, 5, 15, 0, 0, 0).unwrap();
        let since = Utc.with_ymd_and_hms(2026, 5, 1, 0, 0, 0).unwrap();

        repo.insert_transaction(&make_transaction(ProviderId::Anthropic, 10.50, ts))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::Cursor, 7.25, ts))
            .unwrap();

        let cost = repo.total_cost_since(since, None).unwrap();
        assert!((cost - 17.75).abs() < 0.001);
    }

    #[test]
    fn test_total_cost_since_filtered_by_provider() {
        let repo = make_repo();
        let ts = Utc.with_ymd_and_hms(2026, 5, 15, 0, 0, 0).unwrap();
        let since = Utc.with_ymd_and_hms(2026, 5, 1, 0, 0, 0).unwrap();

        repo.insert_transaction(&make_transaction(ProviderId::Anthropic, 10.50, ts))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::Cursor, 7.25, ts))
            .unwrap();

        let cost = repo
            .total_cost_since(since, Some(ProviderId::Anthropic))
            .unwrap();
        assert!((cost - 10.50).abs() < 0.001);
    }

    #[test]
    fn test_cost_by_provider_since() {
        let repo = make_repo();
        let ts = Utc.with_ymd_and_hms(2026, 5, 15, 0, 0, 0).unwrap();
        let since = Utc.with_ymd_and_hms(2026, 5, 1, 0, 0, 0).unwrap();

        repo.insert_transaction(&make_transaction(ProviderId::Anthropic, 10.0, ts))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::Cursor, 20.0, ts))
            .unwrap();
        repo.insert_transaction(&make_transaction(ProviderId::Anthropic, 5.0, ts))
            .unwrap();

        let costs = repo.cost_by_provider_since(since).unwrap();
        assert_eq!(costs.len(), 2);
        // Sorted by cost DESC
        assert_eq!(costs[0].0, ProviderId::Cursor);
        assert!((costs[0].1 - 20.0).abs() < 0.001);
        assert_eq!(costs[1].0, ProviderId::Anthropic);
        assert!((costs[1].1 - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_cost_by_provider_since_empty() {
        let repo = make_repo();
        let costs = repo.cost_by_provider_since(Utc::now()).unwrap();
        assert!(costs.is_empty());
    }

    // --- BudgetStore tests ---

    #[test]
    fn test_load_budget_empty() {
        let repo = make_repo();
        assert!(repo.load_budget().unwrap().is_none());
    }

    #[test]
    fn test_save_and_load_budget() {
        let repo = make_repo();
        let budget = Budget {
            monthly_limit_usd: 200.0,
            daily_limit_usd: Some(20.0),
            threshold_pcts: vec![50, 75, 90, 100],
            billing_period: BillingPeriod::CalendarMonth,
        };

        repo.save_budget(&budget).unwrap();
        let loaded = repo.load_budget().unwrap().unwrap();
        assert_eq!(loaded, budget);
    }

    #[test]
    fn test_save_budget_overwrites() {
        let repo = make_repo();
        let budget1 = Budget {
            monthly_limit_usd: 100.0,
            daily_limit_usd: None,
            threshold_pcts: vec![50, 100],
            billing_period: BillingPeriod::Rolling30,
        };
        let budget2 = Budget {
            monthly_limit_usd: 300.0,
            daily_limit_usd: Some(30.0),
            threshold_pcts: vec![80, 100],
            billing_period: BillingPeriod::Custom(15),
        };

        repo.save_budget(&budget1).unwrap();
        repo.save_budget(&budget2).unwrap();

        let loaded = repo.load_budget().unwrap().unwrap();
        assert_eq!(loaded, budget2);
    }

    #[test]
    fn test_budget_with_daily_limit_none() {
        let repo = make_repo();
        let budget = Budget {
            monthly_limit_usd: 200.0,
            daily_limit_usd: None,
            threshold_pcts: vec![50, 75, 90, 100],
            billing_period: BillingPeriod::CalendarMonth,
        };

        repo.save_budget(&budget).unwrap();
        let loaded = repo.load_budget().unwrap().unwrap();
        assert_eq!(loaded.daily_limit_usd, None);
    }

    // --- Round-trip: write then read ---

    #[test]
    fn test_transaction_roundtrip_preserves_all_fields() {
        let repo = make_repo();
        let ts = Utc.with_ymd_and_hms(2026, 5, 15, 14, 30, 45).unwrap();
        let txn = Transaction {
            id: 0,
            provider: ProviderId::Cursor,
            model: "gpt-4o".to_string(),
            input_tokens: 3000,
            output_tokens: 1200,
            cost_usd: 0.085,
            timestamp: ts,
        };

        repo.insert_transaction(&txn).unwrap();
        let loaded = repo.recent_transactions(1, None).unwrap();
        assert_eq!(loaded.len(), 1);

        let loaded_txn = &loaded[0];
        assert_eq!(loaded_txn.provider, ProviderId::Cursor);
        assert_eq!(loaded_txn.model, "gpt-4o");
        assert_eq!(loaded_txn.input_tokens, 3000);
        assert_eq!(loaded_txn.output_tokens, 1200);
        assert!((loaded_txn.cost_usd - 0.085).abs() < 0.0001);
        assert_eq!(loaded_txn.timestamp, ts);
    }
}
