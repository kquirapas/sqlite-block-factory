use crate::config::Mode;
use comfy_table::{presets::UTF8_FULL, *};

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
