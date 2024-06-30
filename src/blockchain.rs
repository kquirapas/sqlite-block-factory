use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time;
use uuid::Uuid;

use crate::persistence::{BlockData, NodePersistency, Persistence, TransactionData};
use crate::utils::get_random_nonce;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    // #[serde(with = "serde_bytes")]
    pub instruction: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn from(tx_pool: Vec<Transaction>) -> Self {
        Self {
            transactions: tx_pool,
        }
    }

    pub fn get_genesis() -> Self {
        Self {
            transactions: vec![Transaction {
                from: String::from("Foundation"),
                to: String::from("Earth"),
                instruction: "LET'S FUCKING GOOOO!!!!".as_bytes().to_vec(),
            }],
        }
    }
}

pub struct Chain {
    pub tx_pool: Arc<Mutex<Vec<Transaction>>>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            tx_pool: Arc::new(Mutex::new(vec![])),
        }
    }

    /// Creates a [`Block`] from the transaction pool,
    /// empties the pool, and returns the block
    pub async fn create_block_from_pool(&self) -> Result<Block> {
        let arc_mutex_pool: Arc<Mutex<Vec<Transaction>>> = Arc::clone(&self.tx_pool);
        let mut pool = arc_mutex_pool.lock().await;
        let block = Block::from(pool.to_vec());
        // clear tx pool
        *pool = vec![];
        Ok(block)
    }
}

pub struct Node {
    persistence: Persistence,
}

impl Node {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            persistence: Persistence::new().await?,
        })
    }

    // pub async fn create_genesis(&self) -> Result<BlockData> {
    //     let block_data = Block::get_genesis();
    //     self.persistence.create_block_data(block_data)
    // }

    // chain runner
    pub async fn run(&self, chain: &Chain, block_time: u32) -> Result<()> {
        let pool = Arc::clone(&chain.tx_pool);
        let mut interval = time::interval(time::Duration::from_secs(block_time as u64));
        // deal with the first initial tick
        interval.tick().await;
        loop {
            interval.tick().await;
            println!("End of block epoch. Processing...");

            {
                // limit lock to smallest possible block scope
                let pool_size = pool.lock().await.len();
                if pool_size == 0 {
                    println!("No transaction in pool. Skipping...");
                    continue;
                }
            } // lock drops here

            // create new block
            let block = chain.create_block_from_pool().await?;

            println!("{:?}", block);

            // store_block
            self.store_block(block).await?;
        }
        // Ok(())
    }

    //----- PRIVATE -----
    /// Consumes a [`Block`] and returns its calculated hash
    async fn prev_block_hash(&self) -> Result<String> {
        // retrieve latest block_data
        let latest_block_data = self.persistence.read_latest_block_data().await?;
        let prev_block_hash = latest_block_data.prev_block_hash;

        Ok(prev_block_hash)
    }

    /// Consumes a [`Block`] and returns its calculated hash
    async fn store_block(&self, block: Block) -> Result<()> {
        let tx = self.persistence.pool.begin().await?;

        // retrieve latest block_data
        let latest_block_data = self.persistence.read_latest_block_data().await?;

        // create block_data
        let uuid = Uuid::now_v7().to_string();
        let nonce = get_random_nonce();
        let height = latest_block_data.height;
        let prev_block_hash = latest_block_data.prev_block_hash;

        // calculate hash
        let hash = BlockData::get_sha256_hash(&uuid, &nonce, &height, &prev_block_hash);

        let block_data = BlockData {
            hash,
            uuid,
            nonce,
            height,
            prev_block_hash,
        };

        // store BlockData
        self.persistence.store_block_data(block_data).await?;

        // store transactions
        for t in block.transactions.into_iter() {
            let tx_data = TransactionData::try_from(t)?;
            self.persistence.store_transaction_data(tx_data).await?;
        }

        Ok(tx.commit().await?)
    }
}
