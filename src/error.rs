use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockFactoryError {
    #[error("genesis block missing")]
    MissingGenesis,
}
