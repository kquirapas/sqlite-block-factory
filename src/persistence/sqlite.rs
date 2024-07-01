use super::models::{BlockData, TransactionData};
use super::NodePersistency;
use crate::error::BlockFactoryError;
use anyhow::{bail, Result};
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::env;

pub struct SqlitePersistence {
    // only SQLite for now
    pub pool: Pool<Sqlite>,
}

impl SqlitePersistence {
    pub async fn from_env() -> Result<Self> {
        // load environmentt variables
        dotenvy::dotenv()?;

        let db_url = env::var("DATABASE_URL")?;

        // [kristian] TODO: configure optimal connection pooling options
        // create a connection pool
        // only SQLite for now
        let pool = SqlitePool::connect(db_url.as_str()).await?;
        // auto migrate tables
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}

impl NodePersistency for SqlitePersistence {
    async fn read_latest_block_data(&self) -> Result<BlockData> {
        let option_row = sqlx::query(
            "SELECT id, hash, height, prev_block_hash, nonce FROM block_data ORDER BY id DESC",
        )
        .fetch_optional(&self.pool)
        .await?;

        match option_row {
            Some(record) => Ok(BlockData {
                id: record.get(0),
                hash: record.get(1),
                height: record.get(2),
                prev_block_hash: record.get(3),
                nonce: record.get(4),
            }),

            None => bail!(BlockFactoryError::MissingGenesis),
        }
    }

    async fn store_block_data(&self, block_data: BlockData) -> Result<()> {
        sqlx::query( "INSERT INTO block_data (id, hash, height, prev_block_hash, nonce) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(block_data.id)
        .bind(block_data.hash)
        .bind(block_data.height)
        .bind(block_data.prev_block_hash)
        .bind(block_data.nonce)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn store_transaction_data(&self, tx_data: TransactionData) -> Result<()> {
        sqlx::query(
            "INSERT INTO transaction_data (id, hash, from_address, to_address, instruction) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(tx_data.id)
        .bind(tx_data.hash)
        .bind(tx_data.from)
        .bind(tx_data.to)
        .bind(tx_data.instruction)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
