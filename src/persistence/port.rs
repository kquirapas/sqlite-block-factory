use super::models::{BlockData, TransactionData};
use anyhow::Result;

/// Trait for handling Node persistency
pub trait NodePersistency {
    /// Store [`BlockData`] into local DB
    async fn read_latest_block_data(&self) -> Result<BlockData>;
    /// Store [`BlockData`] into local DB
    async fn store_block_data(&self, block_data: BlockData) -> Result<()>;
    /// Store [`TransactionData`] into local DB
    async fn store_transaction_data(&self, tx_data: TransactionData) -> Result<()>;
}
