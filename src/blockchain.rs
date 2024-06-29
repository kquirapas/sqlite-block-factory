use anyhow::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
// use sha256::digest;
// use std::hash;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time;
use uuid::Uuid;

use crate::persistence::{BlockData, Persistence};

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
    pub fn new(tx_pool: Vec<Transaction>) -> Self {
        // [kristian] TODO: Ensure genesis block is created
        Self {
            transactions: tx_pool,
        }
    }
}

// Block to BlockData conversion
// impl TryFrom<Block> for BlockData {
//     type Error =  &'static str;
//
//     // [kristian] TODO: finish try_from conversion from [`Block`] to `BlockData`]
//     fn try_from(block: Block) -> Result<Self, Self::Error> {
//         let uuid = Uuid::new_v4();
//
//         let unix_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).into()?;
//
//         let block_hash =
//
//         Ok(BlockData {
//             uuid: uuid.to_string(),
//             hash: "".to_string(),
//             timestamp: unix_timestamp,
//             nonce: 0,
//             prev_block_hash:
//         })
//     }
// }

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
        let block = Block::new(pool.to_vec());
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
                // limit lock to smallet possible block scope
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
        }
        // Ok(())
    }

    //----- PRIVATE -----
    /// Consumes a [`Block`] and returns its calculated hash
    async fn prev_block_hash() -> Result<String> {
        todo!()
    }

    /// Consumes a [`Block`] and returns its calculated hash
    async fn store_block(block: Block) -> Result<String> {
        todo!()
    }
}
