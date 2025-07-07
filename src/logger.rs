use env_logger::Builder;

use crate::cli::Cli;

pub fn init_logger(cli: &Cli) {
    let mut logger = Builder::from_default_env();
    if cli.verbose {
        logger.filter_level(log::LevelFilter::Debug);
    }

    logger.init();
}
