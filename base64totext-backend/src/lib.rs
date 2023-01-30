use axum::http::StatusCode;
use axum::{
    body::{boxed, Body},
    response::Response,
    routing::get,
    Router,
};
use std::path::PathBuf;
use sync_wrapper::SyncWrapper;
use tokio::fs;
use tower::util::ServiceExt;
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(get, path="/api/hello", responses((status=200, description="receiving a hello message", body=String)))]
async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(OpenApi)]
#[openapi(paths(
hello_world
),
tags((name="base64totoxt", description="Test project for rust backend")))]
struct ApiDoc;

#[shuttle_service::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "public")] static_folder: PathBuf,
) -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
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
