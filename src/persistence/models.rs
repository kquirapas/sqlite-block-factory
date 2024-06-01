use anyhow::Result;
// use sha256::digest;
// use std::hash;
// use tokio::time;

struct BlockData {
    uuid: String,
    transactions: Vec<String>,
    height: u32,
    prev_block_hash: String,
    timestamp: u32,
    nonce: u32,
}

struct TransactionData<'a> {
    uuid: String,
    from: String,
    to: String,
    instruction: &'a [u8],
}
