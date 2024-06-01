use anyhow::Result;
// use sha256::digest;
// use std::hash;
// use tokio::time;

struct BlockData {
    transactions: Vec<String>,
    prev_block_hash: String,
    timestamp: u32,
    nonce: u32,
}

struct TransactionData<'a> {
    from: String,
    to: String,
    instruction: &'a [u8],
}
