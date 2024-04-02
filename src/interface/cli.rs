use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub source: PathBuf,
    #[arg(short, long)]
    pub target: PathBuf,
    #[arg(short, long)]
    pub column: String,
    #[arg(short, long)]
    pub output: PathBuf,
}
