use anyhow::{bail, Result};
use sha256::digest;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::{env, str};
use uuid::Uuid;

use crate::blockchain::Transaction;
use crate::error::BlockFactoryError;

//----- MODELS -----
//
pub struct TransactionData {
    // uuidv7 with timestamp
    pub id: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub instruction: Vec<u8>,
}

impl TryFrom<Transaction> for TransactionData {
    type Error = anyhow::Error;

    fn try_from(tx: Transaction) -> Result<Self> {
        let id = Uuid::now_v7().to_string();
        let from = tx.from;
        let to = tx.to;
        let instruction = tx.instruction.clone();
        let instr_string = str::from_utf8(&tx.instruction[..])?.to_string();
        // generate hash
        let hash = digest(format!("{id}{from}{to}{instr_string}"));

        Ok(TransactionData {
            hash,
            id,
            from,
            to,
            instruction,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct BlockData {
    // uuidv7 with timestamp
    pub id: String,
    pub hash: String,
    pub nonce: u32,
    pub height: u32,
    pub prev_block_hash: String,
}

impl BlockData {
    pub fn get_sha256_hash(
        id: &String,
        nonce: &u32,
        height: &u32,
        prev_block_hash: &String,
    ) -> String {
        digest(format!("{}{}{}{}", *id, *nonce, *height, *prev_block_hash))
    }
}

// TODO: Make this support multiple DB services
/// Manages and maintains persistence data
/// and operations
pub struct Persistence {
    // only SQLite for now
    pub pool: Pool<Sqlite>,
}

impl Persistence {
    pub async fn new() -> Result<Self> {
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

/// Trait for handling Node persistency
pub trait NodePersistency {
    /// Store [`BlockData`] into local DB
    async fn read_latest_block_data(&self) -> Result<BlockData>;
    /// Store [`BlockData`] into local DB
    async fn store_block_data(&self, block_data: BlockData) -> Result<()>;
    /// Store [`TransactionData`] into local DB
    async fn store_transaction_data(&self, tx_data: TransactionData) -> Result<()>;
}

impl NodePersistency for Persistence {
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
