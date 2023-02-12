use std::path::PathBuf;

use clap::{Parser, CommandFactory, Command};

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    /// Server port
    #[arg(short, long, default_value_t = 8000)]
    pub port: u16,
    /// Server bind address
    #[arg(short, long, value_name = "HOST", default_value = "0.0.0.0")]
    pub bind: String,
    /// Server bind directory
    #[arg(short, long, default_value = "./")]
    pub dir: PathBuf,
    /// Server index file
    #[arg(short, long, value_name = "INDEX", default_value = "index.html")]
    pub index_file: String
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}

impl Cli {
    pub fn cmds() -> Command {
        Self::command()
    }
}
