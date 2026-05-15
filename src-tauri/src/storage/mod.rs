mod sqlite;

use chrono::{DateTime, Utc};

use crate::errors::StorageError;
use crate::models::{Budget, ProviderId, Transaction};

pub use sqlite::SqliteRepository;

/// Writes transactions to storage.
pub trait TransactionWriter: Send + Sync {
    fn insert_transaction(&self, txn: &Transaction) -> Result<i64, StorageError>;
}

/// Reads transactions from storage.
pub trait TransactionReader: Send + Sync {
    fn recent_transactions(
        &self,
        limit: usize,
        provider: Option<ProviderId>,
    ) -> Result<Vec<Transaction>, StorageError>;

    fn transactions_since(
        &self,
        since: DateTime<Utc>,
        provider: Option<ProviderId>,
    ) -> Result<Vec<Transaction>, StorageError>;
}

/// Aggregated spend summaries.
pub trait UsageSummary: Send + Sync {
    fn total_cost_since(
        &self,
        since: DateTime<Utc>,
        provider: Option<ProviderId>,
    ) -> Result<f64, StorageError>;

    fn cost_by_provider_since(
        &self,
        since: DateTime<Utc>,
    ) -> Result<Vec<(ProviderId, f64)>, StorageError>;
}

/// Budget persistence.
pub trait BudgetStore: Send + Sync {
    fn save_budget(&self, budget: &Budget) -> Result<(), StorageError>;
    fn load_budget(&self) -> Result<Option<Budget>, StorageError>;
}
