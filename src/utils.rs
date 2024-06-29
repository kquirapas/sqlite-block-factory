use anyhow::Result;
use comfy_table::{presets::UTF8_FULL, *};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::Mode;

pub fn display_configuration(port: &u32, block_time: &u32, mode: &Mode) {
    // display configuration from flags
    let mut table = Table::new();
    // resolve temporary borrow error
    let table = table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(80)
        .set_header(vec![
            Cell::new("Configuration").add_attribute(Attribute::Bold)
        ])
        .set_header(vec![
            Cell::new("Name").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ]);

    table.add_row(vec![Cell::new("Port"), Cell::new(port)]);
    table.add_row(vec![Cell::new("Block Time"), Cell::new(block_time)]);
    table.add_row(vec![
        Cell::new("Mode"),
        Cell::new(match mode {
            Mode::Full => "Full",
            Mode::FactoryOnly => "Factory Only",
            Mode::QueryOnly => "Query Only",
        }),
    ]);

    println!("{table}");
}

pub fn get_unix_timestamp_now() -> Result<u64> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let unix_timestamp = duration.as_secs();
    Ok(unix_timestamp)
}

// pub fn get_random_nonce()
