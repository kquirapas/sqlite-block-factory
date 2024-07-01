use super::config::Mode;
use clap::{
    builder::{EnumValueParser, RangedU64ValueParser},
    Arg, ArgMatches, Command,
};

pub struct Cli {}

impl Cli {
    pub fn get_matches() -> ArgMatches {
        Command::new("block-factory")
            .version("0.1.0")
            .about("Generate blocks from incoming transactions")
            .arg(
                Arg::new("PORT")
                    .help("Port to use in serving the factory")
                    .long("port")
                    .short('p')
                    .value_parser(RangedU64ValueParser::<u32>::new().range(1..))
                    .default_value("8080"),
            )
            .arg(
                Arg::new("BLOCKTIME")
                    .help("Amount of seconds to wait before creating a block")
                    .long("block-time")
                    .short('b')
                    .value_parser(RangedU64ValueParser::<u32>::new().range(1..))
                    .default_value("1"),
            )
            .arg(
                Arg::new("MODE")
                    .help("Mode for block factory")
                    .long("mode")
                    .short('m')
                    .value_parser(EnumValueParser::<Mode>::new())
                    .default_value("full"),
            )
            .get_matches()
    }
}
