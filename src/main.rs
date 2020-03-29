use quicli::prelude::*;
use std::path::Path;
use structopt::StructOpt;
use std::str::FromStr;

mod util;
use crate::util::cli::Cli;
use crate::util::core::Core;

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("head")?;

    let path = Path::new(&args.path);
    if !path.exists() {
        error!("the specified path does not reference to any file or directory.")
    }
    let tree = Core::new(&args);
    // tree.visit_path(path, 0)?;
    Ok(())
}
