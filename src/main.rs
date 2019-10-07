use std::env;
use std::fs;
use std::process;
use std::path::Path;
use std::error::Error;

static HELP: &'static str = "\
Usage:
    minitree <path> [...options]

Params:
    path             the path to explore
    levels=NUMBER    the number of levels to explore
    absolute=BOOLEAN if true the program will print the absolute path instead of the relative path
";

struct Arguments {
    path: String,
    levels: u32,
    absolute: bool,
}
impl Arguments {
    fn new(args: &Vec<String>) -> Result<Arguments, Box<dyn Error>> {
        let mut path = "".to_string();
        let mut levels = 0;
        let mut absolute = false;
        if args.len() < 1 {
            eprintln!("{}", HELP);
            process::exit(1);
        }
        for arg in args {
            if arg.starts_with("levels=") {
                levels = arg[7..].parse().expect("\"levels\" must be a valid number (i32).");
            } else if arg.starts_with("absolute=") {
                absolute = arg[9..].parse().expect("\"absolute\" must be \"true\" or \"false\".");
            } else if !args.contains(&String::from("=")) && path.is_empty() {
                path = arg.to_string();
            } else {
                eprintln!("{}", HELP);
                process::exit(1);
            }
        }
        Ok(Arguments { path, levels, absolute })
    }
}

fn visit_path(path: &Path, level: u32, max_level: u32, absolute: bool) -> std::io::Result<()> {
    print_path(&path, level, absolute);
    if max_level != 0 && level >= max_level {
        return Ok(())
    }
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            visit_path(&entry.unwrap().path(), level + 1, max_level, absolute)?;
        }
    }
    Ok(())
}

fn print_path(path: &Path, level: u32, absolute: bool) {
    let path_str = if absolute {
        fs::canonicalize(path).unwrap().to_str().unwrap().to_string()
    } else {
        path.to_str().unwrap().to_string()
    };
    let pad = format!("{: >level$}*", "", level = (level * 2) as usize);
    println!("{} {}", pad, &path_str);
}

fn main() -> Result<(), Box<dyn Error>> {
    let Arguments { path: path_str, levels, absolute } = Arguments::new(&env::args().skip(1).collect())?;
    let path = Path::new(&path_str);
    if !path.exists() {
        eprintln!("\nERROR: the specified path does not reference to any file or directory.\n");
        eprintln!("{}", HELP);
        process::exit(1);
    }
    visit_path(&path, 0, levels, absolute).unwrap();

    Ok(())
}
