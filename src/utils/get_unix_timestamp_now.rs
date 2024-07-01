use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_unix_timestamp_now() -> Result<u64> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let unix_timestamp = duration.as_secs();
    Ok(unix_timestamp)
}
