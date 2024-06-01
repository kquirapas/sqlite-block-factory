use anyhow::Result;
// use sha256::digest;
// use std::hash;
use serde::Deserialize;
use tokio::time;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    #[serde(with = "serde_bytes")]
    pub instruction: Vec<u8>,
}

pub struct Block {
    pub transactions: Vec<String>,
}

pub struct Node {
    pub tx_pool: Vec<Transaction>,
}

impl Node {
    pub fn new() -> Self {
        Self { tx_pool: vec![] }
    }

    pub async fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
