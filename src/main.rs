use std::collections::HashSet;
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
                               name     -- show just the name of the item in  a tree view
                               relative -- show the relative path of the item
                               absolute -- show the absolute path of the item
    hidden=BOOLEAN,h=BOOLEAN   if true we will show hidden item (dafault: false)
    dev=BOOLEAN,d=BOOLEAN      if true standard development folders and files are discarded
                               (.git, target, node_modules, build, bin)
    tab=NUMBER,t=NUMBER        the number of spaces used to indent the tree (default is 2)
";

fn parse_arg(arg: &String, arg_prefix: &'static str, abbreviation: &'static str) -> String {
    if arg.starts_with(abbreviation) {
        arg[abbreviation.len()..].to_string()
    } else {
        arg[arg_prefix.len()..].to_string()
    }
}

#[derive(Debug, Clone)]
struct Arguments {
    path: String,
    levels: u32,
    style: String,
    hidden: bool,
    development: bool,
    tab: u32,
}

impl Arguments {
    fn new(args: &Vec<String>) -> Result<Arguments, Box<dyn Error>> {
        let mut path = String::from("");
        let mut levels = 0;
        let mut style = String::from("");
        let mut hidden = false;
        let mut development = false;
        let mut tab: u32 = 0;
        if args.len() < 1 {
            eprintln!("{}", HELP);
            process::exit(1);
        }
        for arg in args {
            if arg.starts_with("levels=") || arg.starts_with("l=") {
                levels = match parse_arg(arg, "levels=", "l=").parse() {
                    Ok(s) => Ok(s),
                    Err(_e) => Err("\"levels\" must be a valid number (u32)."),
                }?
            } else if arg.starts_with("style=") || arg.starts_with("s=") {
                style = parse_arg(arg, "style=", "s=");
            } else if arg.starts_with("hidden=") || arg.starts_with("h=") {
                hidden = match parse_arg(arg, "hidden=", "h=").parse() {
                    Ok(s) => Ok(s),
                    Err(_e) => Err("\"hidden\" must be \"true\" or \"false\"."),
                }?
            } else if arg.starts_with("development=") || arg.starts_with("d=") {
                development = match parse_arg(arg, "development=", "d=").parse() {
                    Ok(s) => Ok(s),
                    Err(_e) => Err("\"development\" must be \"true\" or \"false\"."),
                }?
            } else if arg.starts_with("tab=") || arg.starts_with("t=") {
                tab = match parse_arg(arg, "tab=", "t=").parse() {
                    Ok(s) => Ok(s),
                    Err(_e) => Err("\"tab\" must be a positive integer number (u32)."),
                }?
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
            development,
            tab,
        })
    }
}

#[derive(Debug)]
struct Tree {
    options: Arguments,
    dev_excludes: HashSet<String>, //HashSet::from_iter
}

impl Tree {
    fn new(options: &Arguments) -> Tree {
        let mut dev_excludes = HashSet::new();
        dev_excludes.insert(".git".to_string());
        dev_excludes.insert("target".to_string());
        dev_excludes.insert("node_modules".to_string());
        dev_excludes.insert("build".to_string());
        dev_excludes.insert("bin".to_string());
        Tree {
            options: options.clone(),
            dev_excludes,
        }
    }

    fn get_name(&self, path: &Path) -> Result<(String), Box<dyn Error>> {
        match path.file_name() {
            Some(name) => Ok(name.to_str().ok_or("")?.to_string()),
            None => match path.canonicalize()?.file_name() {
                Some(name) => Ok(name.to_str().ok_or("")?.to_string()),
                None => Ok(path.canonicalize()?.to_str().ok_or("")?.to_string()),
            },
        }
    }

    fn get_relative_path(&self, path: &Path) -> Result<(String), Box<dyn Error>> {
        Ok(path.to_str().ok_or("-??-")?.to_string())
    }

    fn get_absolute_path(&self, path: &Path) -> Result<(String), Box<dyn Error>> {
        Ok(path.canonicalize()?.to_str().ok_or("-??-")?.to_string())
    }

    fn visit_path(&self, path: &Path, level: u32) -> Result<(), Box<dyn Error>> {
        let Arguments {
            path: _path,
            levels: max_level,
            style,
            hidden,
            development,
            tab,
        } = &self.options;

        let name = self.get_name(&path)?;
        if !hidden && name.starts_with('.') {
            return Ok(());
        }
        if *development && self.dev_excludes.contains(&name) {
            return Ok(());
        }
        self.print_item(&path, level, &style, *tab)?;
        if *max_level != 0 as u32 && level >= *max_level {
            return Ok(());
        }
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                self.visit_path(&entry?.path(), level + 1)?;
            }
        }
        Ok(())
    }

    fn print_item(
        &self,
        path: &Path,
        level: u32,
        style: &String,
        tab: u32,
    ) -> Result<(), Box<dyn Error>> {
        let item = if style == "absolute" {
            self.get_absolute_path(path)?
        } else if style == "relative" {
            self.get_relative_path(path)?
        } else {
            let spaces = if tab > 0 { tab } else { 2 };
            let mut pad: String = format!("{: >level$}* ", "", level = (level * spaces) as usize);
            pad.push_str(&self.get_name(path)?);
            pad
        };
        println!("{}", &item);
        Ok(())
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::new(&env::args().skip(1).collect())?;
    let path = Path::new(&arguments.path);
    if !path.exists() {
        Err("the specified path does not reference to any file or directory.")?
    }
    let tree = Tree::new(&arguments);
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
