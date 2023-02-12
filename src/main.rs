use colored::Colorize;
use simple_server::{cli::Cli, server::build_server, Result};
use std::{env, fs, path::PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::default();
    let Cli {
        bind,
        port,
        dir,
        index_file,
    } = cli;

    display_info(&bind, port, &dir, &index_file)?;

    env::set_var("RUST_LOG", "debug=debug");
    tracing_subscriber::fmt::init();

    build_server(&bind, port, dir, index_file).await?;
    Ok(())
}

fn display_info(bind: &str, port: u16, dir: &PathBuf, index_file: &str) -> Result<()> {
    let _path = fs::canonicalize(dir)?;

    let path = _path.as_path().to_string_lossy();

    #[allow(unused_assignments)]
    let mut bound = String::new();

    if bind == "0.0.0.0" {
        bound = format!("[::]:{port}, {bind}:{port}");
    } else {
        bound = format!("{bind}:{port}");
    }

    let content = format!(
        "{3} Bound: {}
{3} Serving path: {}
{3} Index file: {}",
        bound.bright_green().bold(),
        path.bright_yellow().bold(),
        index_file.bright_green().bold(),
        ">".bright_cyan().bold()
    );
    println!("{content}");

    Ok(())
}
