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
    #[serde(with = "serde_bytes")]
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
        let pool = arc_mutex_pool.lock().await;
        let block = Block::from(pool.to_vec());
        // clear tx pool
        Ok(block)
    }

    pub async fn clear_pool(&self) -> Result<()> {
        let arc_mutex_pool: Arc<Mutex<Vec<Transaction>>> = Arc::clone(&self.tx_pool);
        let mut pool = arc_mutex_pool.lock().await;
        // clear tx pool
        *pool = vec![];
        Ok(())
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

    pub async fn store_genesis_block(&self) -> Result<()> {
        let tx = self.persistence.pool.begin().await?;

        // create block_data
        let id = Uuid::now_v7().to_string();
        let nonce = get_random_nonce(100); // upper_limit = 100
        let height = 1; // genesis is at height 1
        let prev_block_hash = String::from("");

        // calculate hash
        let hash = BlockData::get_sha256_hash(&id, &nonce, &height, &prev_block_hash);

        let block_data = BlockData {
            hash,
            id,
            nonce,
            height,
            prev_block_hash,
        };

        // store BlockData
        self.persistence.store_block_data(block_data).await?;

        Ok(tx.commit().await?)
    }

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
            // empty transaction pool to []
            chain.clear_pool().await?;

            // store_block
            self.store_block(block).await?;
        }
        // Ok(())
    }

    /// Consumes a [`Block`] and returns its calculated hash
    async fn store_block(&self, block: Block) -> Result<()> {
        println!("begin tx");
        let tx = self.persistence.pool.begin().await?;

        // retrieve latest block_data
        println!("read latest block data");
        let latest_block_data = self.persistence.read_latest_block_data().await?;

        println!("latest block: {:?}", latest_block_data);

        // create block_data
        println!("create block data");
        let id = Uuid::now_v7().to_string();
        let nonce = get_random_nonce(100); // uppet_limit = 100

        // increment height
        let height = latest_block_data.height + 1;
        // retrieve prev block's hash
        let prev_block_hash = latest_block_data.hash;

        // calculate hash
        let hash = BlockData::get_sha256_hash(&id, &nonce, &height, &prev_block_hash);

        println!("creating block data struct");
        let block_data = BlockData {
            hash,
            id,
            nonce,
            height,
            prev_block_hash,
        };

        println!("created block data: {:?}", block_data);

        // store BlockData
        println!("storing block data");
        self.persistence.store_block_data(block_data).await?;

        // store transactions
        for t in block.transactions.into_iter() {
            let tx_data = TransactionData::try_from(t)?;
            println!("storing transaction data");
            self.persistence.store_transaction_data(tx_data).await?;
        }

        Ok(tx.commit().await?)
    }
}
