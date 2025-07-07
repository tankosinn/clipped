use std::path::PathBuf;

use clap::Parser;
use serde::Serialize;

use crate::diagnostics::Level;

#[derive(Debug, Parser, Serialize)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long = "config")]
    pub config_path: Option<PathBuf>,
    #[arg(long, value_enum)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[arg(long, short = 'v')]
    pub verbose: bool,
    #[arg()]
    pub files: Vec<PathBuf>,
    #[arg(last = true)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clippy_args: Vec<String>,
}
