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
    pub uuid: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub instruction: Vec<u8>,
}

impl TryFrom<Transaction> for TransactionData {
    type Error = anyhow::Error;

    fn try_from(tx: Transaction) -> Result<Self> {
        let uuid = Uuid::now_v7().to_string();
        let from = tx.from;
        let to = tx.to;
        let instruction = tx.instruction.clone();
        let instr_string = str::from_utf8(&tx.instruction[..])?.to_string();
        // generate hash
        let hash = digest(format!("{uuid}{from}{to}{instr_string}"));

        Ok(TransactionData {
            hash,
            uuid,
            from,
            to,
            instruction,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct BlockData {
    // uuidv7 with timestamp
    pub uuid: String,
    pub hash: String,
    pub nonce: u32,
    pub height: u32,
    pub prev_block_hash: String,
}

impl BlockData {
    pub fn get_sha256_hash(
        uuid: &String,
        nonce: &u32,
        height: &u32,
        prev_block_hash: &String,
    ) -> String {
        digest(format!(
            "{}{}{}{}",
            *uuid, *nonce, *height, *prev_block_hash
        ))
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
        let db_folder = env::var("DATABASE_FOLDER")?;
        let db_name = env::var("DATABASE_NAME")?;
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

// create table block_data (
// 	id varchar(36) not null,
// 	hash varchar(32) not null unique,
// 	block_timestamp bigint not null,
// 	nonce bigint not null,
// 	height bigint not null unique,
// 	prev_block_hash varchar(32) not null unique,
// 	primary key (id)
// );
//
// create table transaction_data (
// 	id varchar(36) not null,
// 	hash varchar(32) not null unique,
// 	tx_timestamp bigint not null,
// 	from_address varchar(32) not null,
// 	to_address varchar(32) not null,
// 	instruction varchar(32) not null,
// 	primary key (id)
// );

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
        let option_block_data: Option<BlockData> =
            sqlx::query_as("SELECT prev_block_hash FROM block_data ORDER BY block_timestamp desc")
                .fetch_optional(&self.pool)
                .await?;

        match option_block_data {
            Some(block_data) => Ok(block_data),
            None => bail!(BlockFactoryError::MissingGenesis),
        }
    }

    async fn store_block_data(&self, block_data: BlockData) -> Result<()> {
        sqlx::query(
            "INSERT INTO block_data (id, hash, block_timestamp, height, prev_block_hash, nonce) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(block_data.uuid)
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
            "INSERT INTO transaction_data (id, hash, from, to, instruction) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(tx_data.uuid)
        .bind(tx_data.hash)
        .bind(tx_data.from)
        .bind(tx_data.to)
        .bind(tx_data.instruction)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
