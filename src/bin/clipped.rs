use clipped::{
    cli::Cli, context::Context, diagnostics::run_diagnostics, error::Result, logger::init_logger,
};

use clap::Parser;
use log::debug;

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logger(&cli);
    debug!("cli args: {cli:?}");

    let ctx = Context::new(&cli)?;

    let success = run_diagnostics(&ctx)?;
    if !success {
        std::process::exit(1);
    }

    Ok(())
}
