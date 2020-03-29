use quicli::prelude::*;
use structopt::StructOpt;

///
/// List contents of directories in a tree-like format
#[derive(Debug, StructOpt)]
pub struct Cli {
    /*
     * Flags
     */
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    pub verbosity: Verbosity,

    /// Shows hidden files
    #[structopt(long = "hidden", short = "h")]
    pub hidden: bool,

    /// Hides dev files
    #[structopt(long = "hide-dev", short = "d")]
    pub hide_dev: bool,

    /*
     * Options
     */
    /// Number of levels to explore.
    /// If it is not specifed all the content will be listed.
    #[structopt(long = "levels", short = "l", default_value = "0")]
    pub levels: u8,

    /// Number of spaces used to indent the branches of the tree
    #[structopt(long = "tab", short = "t", default_value = "2")]
    pub tab: u8,

    /// Server port
    #[structopt(long = "exclude", short = "e", default_value = "")]
    pub exclude: String,

    /// Select the database to query.
    /// If no database is specified the tool will search in all the available databases.
    #[structopt(long = "include", short = "i", default_value = "")]
    pub include: String,

    /// The tree node format.
    #[structopt(long = "format", short = "f", default_value = "name")]
    pub format: String,

    /*
     * Required Parameters
     */
    /// The path of the root folder
    pub path: String,
}
