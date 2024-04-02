use clap::Parser;
use file::write_json::write_json;

use crate::{file::read_json::join, interface::cli::Cli};

mod file;
mod interface;
fn main() {
    let args = Cli::parse();
    let results = join(&args).unwrap();
    write_json(args.output, results).expect("Error");
}
