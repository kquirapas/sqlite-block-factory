use crate::blockchain::Node;
use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum Mode {
    Full,
    FactoryOnly,
    QueryOnly,
}

pub struct Configuration {
    pub port: u32,
    pub block_time: u32,
    pub mode: Mode,
    pub node: Node,
}
