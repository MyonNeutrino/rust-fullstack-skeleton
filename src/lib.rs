use axum::{routing::get, Router};
use axum_extra::routing::SpaRouter;
use sync_wrapper::SyncWrapper;
use std::path::PathBuf;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn axum(#[shuttle_static_folder::StaticFolder(folder = "public")] static_folder: PathBuf) -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/hello", get(hello_world))
      .merge(SpaRouter::new("/public", static_folder).index_file("index.html"))
      ;


    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
