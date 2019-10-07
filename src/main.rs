use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

static HELP: &'static str = "\
Usage:
    minitree <path> [...options]

Params:
    path                       the path to explore
    levels=NUMBER,l=NUMBER     the number of levels to explore
    style=TEXT,s=TEXT          the node style. Available style are:
                               name     -- show just the name of the item
                               relative -- show the relative path of the item
                               absolute -- show the absolute path of the item
    hidden=BOOLEAN,h=BOOLEAN   if true we will show hidden item (dafault: false)
";

struct Arguments {
    path: String,
    levels: u32,
    style: String,
    hidden: bool,
}
impl Arguments {
    fn new(args: &Vec<String>) -> Result<Arguments, Box<dyn Error>> {
        let mut path = String::from("");
        let mut levels = 0;
        let mut style = String::from("");
        let mut hidden = false;
        if args.len() < 1 {
            eprintln!("{}", HELP);
            process::exit(1);
        }
        for arg in args {
            if arg.starts_with("levels=") || arg.starts_with("l=") {
                let value = if arg.starts_with("l=") {
                    &arg[2..]
                } else {
                    &arg[7..]
                };
                levels = value
                    .parse()
                    .expect("\"levels\" must be a valid number (u32).");
            } else if arg.starts_with("style=") || arg.starts_with("s=") {
                style = if arg.starts_with("s=") {
                    arg[2..].to_string()
                } else {
                    arg[6..].to_string()
                };
            } else if arg.starts_with("hidden=") || arg.starts_with("h=") {
                let value = if arg.starts_with("h=") {
                    &arg[2..]
                } else {
                    &arg[7..]
                };
                hidden = value
                    .parse()
                    .expect("\"hidden\" must be \"true\" or \"false\".");
            } else if path.is_empty() {
                path = arg.to_string();
            } else {
                eprintln!("{}", HELP);
                process::exit(1);
            }
        }
        Ok(Arguments {
            path,
            levels,
            style,
            hidden,
        })
    }
}

fn get_name(path: &Path) -> String {
    match path.file_name() {
        Some(name) => name.to_str().unwrap().to_string(),
        None => match path.canonicalize().unwrap().file_name() {
            Some(name) => name.to_str().unwrap().to_string(),
            None => path.canonicalize().unwrap().to_str().unwrap().to_string(),
        },
    }
}

fn visit_path(
    path: &Path,
    level: u32,
    max_level: u32,
    style: &String,
    hidden: bool,
) -> std::io::Result<()> {
    if !hidden && get_name(&path).starts_with('.') {
        return Ok(());
    }
    print_item(&path, level, style);
    if max_level != 0 && level >= max_level {
        return Ok(());
    }
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            visit_path(&entry.unwrap().path(), level + 1, max_level, style, hidden)?;
        }
    }
    Ok(())
}

fn print_item(path: &Path, level: u32, style: &String) {
    let item = if style == "absolute" {
        path.canonicalize().unwrap().to_str().unwrap().to_string()
    } else if style == "relative" {
        path.to_str().unwrap().to_string()
    } else {
        let mut pad: String = format!("{: >level$}* ", "", level = (level * 2) as usize);
        pad.push_str(&get_name(path));
        pad
    };
    println!("{}", &item);
}

fn main() -> Result<(), Box<dyn Error>> {
    let Arguments {
        path: path_str,
        levels,
        style,
        hidden,
    } = Arguments::new(&env::args().skip(1).collect())?;
    let path = Path::new(&path_str);
    if !path.exists() {
        eprintln!("\nERROR: the specified path does not reference to any file or directory.\n");
        eprintln!("{}", HELP);
        process::exit(1);
    }
    visit_path(&path, 0, levels, &style, hidden).unwrap();

    Ok(())
}
