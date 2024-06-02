use anyhow::Result;
// use sha256::digest;
// use std::hash;
use serde::{Deserialize, Serialize};
use tokio::time;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    #[serde(with = "serde_bytes")]
    pub instruction: Vec<u8>,
}

pub struct Block {
    pub transactions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub tx_pool: Vec<Transaction>,
}

impl Node {
    pub fn new() -> Self {
        Self { tx_pool: vec![] }
    }

    pub async fn run(&self, block_time: u32) -> Result<()> {
        let mut interval = time::interval(time::Duration::from_secs(block_time as u64));
        loop {
            interval.tick().await;
            println!("ticked");
        }
        Ok(())
    }
}
