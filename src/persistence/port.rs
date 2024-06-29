use anyhow::{bail, Result};
use sha256::digest;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::env;
use uuid::Uuid;
// use tokio::time;

use crate::blockchain::Block;
use crate::error::BlockFactoryError;
use crate::utils::get_unix_timestamp_now;

//----- MODELS -----

pub struct TransactionData<'a> {
    // uuidv7 with timestamp
    pub uuid: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub instruction: &'a [u8],
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
    fn get_sha256_hash(uuid: String, nonce: u32, height: u32, prev_block_hash: String) -> String {
        digest(format!("{uuid}{nonce}{height}{prev_block_hash}"))
    }
}

// convert Block into BlockData with automated necessary values
// impl From<Block> for BlockData {
//     fn from() -> BlockData {
//         let uuid = Uuid::now_v7();
//         BlockData {}
//     }
// }

// TODO: Make this support multiple DB services
/// Manages and maintains persistence data
/// and operations
pub struct Persistence {
    // only SQLite for now
    pool: SqlitePool,
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
    async fn create_block_data(&self, block_data: BlockData) -> Result<()>;
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

    async fn create_block_data(&self, block_data: BlockData) -> Result<()> {
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
}
