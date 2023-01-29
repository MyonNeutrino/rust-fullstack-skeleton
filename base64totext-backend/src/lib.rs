use axum::{routing::get, Router, body::{Body, boxed}, response::Response};
use axum::http::StatusCode;
use tokio::fs;
use std::path::PathBuf;
use sync_wrapper::SyncWrapper;
use tower_http::services::ServeDir;
use tower::util::ServiceExt;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "public")] static_folder: PathBuf,
) -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/api/hello", get(hello_world))
        .fallback_service(get(|req| async move {
            match ServeDir::new(&static_folder).oneshot(req).await {
                Ok(res) => {
                    let status = res.status();
                    match status {
                        StatusCode::NOT_FOUND => {
                            let index_path = PathBuf::from(&static_folder).join("index.html");
                            let index_content = match fs::read_to_string(index_path).await {
                                Err(_) => {
                                    return Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(boxed(Body::from("index file not found")))
                                        .unwrap()
                                }
                                Ok(index_content) => index_content,
                            };

                            Response::builder()
                                .status(StatusCode::OK)
                                .body(boxed(Body::from(index_content)))
                                .unwrap()
                        }
                        _ => res.map(boxed),
                    }
                }
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }));

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
