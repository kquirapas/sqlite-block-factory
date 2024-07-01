use crate::blockchain::Transaction;
use anyhow::Result;
use sha256::digest;
use sqlx::FromRow;
use std::str;
use uuid::Uuid;

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

#[derive(Debug, FromRow)]
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
