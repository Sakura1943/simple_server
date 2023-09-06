use super::Result;
use poem::{
    endpoint::StaticFilesEndpoint,
    listener::TcpListener,
    middleware::{Cors, Tracing},
    EndpointExt, Route, Server,
};
use std::path::PathBuf;

pub async fn build_server(bind: &str, port: u16, dir: PathBuf, index_file: String) -> Result<()> {
    let cors = Cors::default();
    let app = Route::new()
        .nest(
            "/",
            StaticFilesEndpoint::new(dir)
                .index_file(index_file)
                .show_files_listing()
                .fallback_to_index()
                .redirect_to_slash_directory(),
        )
        .with(Tracing)
        .with(cors);
    Server::new(TcpListener::bind(format!("{bind}:{port}")))
        .run(app)
        .await?;
    Ok(())
}
