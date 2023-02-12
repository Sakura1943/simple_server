#[path = "src/cli.rs"]
mod cli;
use clap_complete::{
    generate_to,
    Shell::{Bash, Fish, Zsh},
};
use cli::Cli;
use std::path::Path;
use std::{fs::create_dir, io::Result};

fn main() -> Result<()> {
    let dir = "completions";
    let dir_path = Path::new(dir);
    if !dir_path.exists() {
        create_dir(dir_path).unwrap();
    }
    let bin_name = "simple_server";
    let cmd = &mut Cli::cmds();
    generate_to(Bash, cmd, bin_name, dir)?;
    generate_to(Zsh, cmd, bin_name, dir)?;
    generate_to(Fish, cmd, bin_name, dir)?;
    Ok(())
}
