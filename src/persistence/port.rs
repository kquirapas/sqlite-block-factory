use anyhow::Result;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::env;
// use sha256::digest;
// use std::hash;
// use tokio::time;

//----- MODELS -----

pub struct TransactionData<'a> {
    pub uuid: String,
    pub hash: String,
    pub timestamp: u32,
    pub from: String,
    pub to: String,
    pub instruction: &'a [u8],
}

pub struct BlockData {
    pub uuid: String,
    pub hash: String,
    /// specific time block was persisted into local DB
    /// in Unix timestamp
    pub timestamp: u32,
    pub nonce: u32,
    pub height: u32,
    pub prev_block_hash: String,
}

/// Trait for handling Node persistency
pub trait NodePersistency {
    /// Store [`BlockData`] into local DB
    async fn read_latest_block_data(&self) -> Result<BlockData>;
    async fn create_block_data(&self, block_data: BlockData) -> Result<()>;
}

/// Manages and maintains persistence data
/// and operations
pub struct Persistence {
    pool: Pool<Sqlite>,
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

impl NodePersistency for Persistence {
    async fn read_latest_block_data(&self) -> Result<BlockData> {
        let block_data =
            sqlx::query!("SELECT prev_block_hash FROM block_data ORDER BY block_timestamp desc")
                .fetch_one(&self.pool)
                .await?;
        Ok(block_data)
    }

    async fn create_block_data(&self, block_data: BlockData) -> Result<()> {
        sqlx::query(
            "INSERT INTO block_data (id, hash, block_timestamp, height, prev_block_hash, nonce) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(block_data.uuid)
        .bind(block_data.hash)
        .bind(block_data.timestamp)
        .bind(block_data.height)
        .bind(block_data.prev_block_hash)
        .bind(block_data.nonce)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
