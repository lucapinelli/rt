use regex::Regex;
use std::error::Error;
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
    exclude=REGEX,e=REGEX      the items to show doesn't have to match the 'exclude' pattern
";
// incude=REGEX,i=REGEX       the items to show (or one of their descendant) have to match
//                            the 'include' pattern

fn parse_arg(arg: &String, arg_prefix: &'static str, abbreviation: &'static str) -> String {
    if arg.starts_with(abbreviation) {
        arg[abbreviation.len()..].to_string()
    } else {
        arg[arg_prefix.len()..].to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub levels: u32,
    pub path: String,
    pub style: String,
    pub hidden: bool,
    pub development: bool,
    pub tab: u32,
    pub exclude: Option<Regex>,
}

impl Arguments {
    pub fn new(args: &Vec<String>) -> Result<Arguments, Box<dyn Error>> {
        let mut path = String::from("");
        let mut levels = 0;
        let mut style = String::from("");
        let mut hidden = false;
        let mut development = false;
        let mut tab: u32 = 0;
        let mut exclude = Option::None;
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
            } else if arg.starts_with("exclude=") || arg.starts_with("e=") {
                exclude = Option::Some(match Regex::new(&parse_arg(arg, "exclude=", "e=")) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(format!(
                        "\"exclude\" must be a valid regular expression: {}",
                        e
                    )),
                }?)
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
            exclude,
        })
    }
}
