use simple_server::{cli::Cli, server::build_server, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "poem=debug");
    tracing_subscriber::fmt::init();

    let cli = Cli::default();
    let Cli { bind, port, dir, index_file } = cli;
    build_server(&bind, port, dir, index_file).await?;
    Ok(())
}
