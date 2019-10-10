use std::env;
use std::error::Error;
use std::path::Path;
use std::process;

mod util;
use crate::util::arguments::Arguments;
use crate::util::arguments::HELP;
use crate::util::core::Core;

fn run() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::new(&env::args().skip(1).collect())?;
    let path = Path::new(&arguments.path);
    if !path.exists() {
        Err("the specified path does not reference to any file or directory.")?
    }
    let tree = Core::new(&arguments);
    tree.visit_path(path, 0)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(some) => some,
        Err(e) => {
            eprintln!("\nERROR: {}\n", e);
            eprintln!("{}", HELP);
            process::exit(1);
        },
    }
}
