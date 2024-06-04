use anyhow::Result;
// use sha256::digest;
// use std::hash;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    #[serde(with = "serde_bytes")]
    pub instruction: Vec<u8>,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Block {
//     pub transactions: Vec<String>,
// }
//
// impl Block {
//     pub fn new() -> Self {
//         Self {
//             transactions: vec![],
//         }
//     }
// }

pub struct Node {
    pub tx_pool: Arc<Mutex<Vec<Transaction>>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            tx_pool: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn run(&self, block_time: u32) -> Result<()> {
        let mut interval = time::interval(time::Duration::from_secs(block_time as u64));
        // deal with the first initial tick
        interval.tick().await;
        loop {
            interval.tick().await;
            println!("ticked");
        }
        // Ok(())
    }
}
