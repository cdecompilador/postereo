use std::path::PathBuf;

use axum::http::header::HeaderValue;
use axum::http::StatusCode;
use axum::response::Response;
use tower::ServiceBuilder;
use tower_http::services::{fs, ServeDir};
use anyhow::*;

use crate::app_state::AppState;

type ServeRequest = Response<fs::ServeFileSystemResponseBody>;

pub fn router(
    www_dir: &PathBuf
) -> Result<axum::Router<AppState>> {
    log::info!("Serving files at {:?}", 
            www_dir.canonicalize()?);

    let serve_dir = ServeDir::new(www_dir);

    Ok(axum::Router::new()
        .fallback_service(
            ServiceBuilder::new()
                // .map_response(|mut res: ServeRequest| {
                //     res.headers_mut()
                //         .insert(
                //             "Location", 
                //             HeaderValue::from_str("/").unwrap()
                //         );
                //     *res.status_mut() = StatusCode::NOT_MODIFIED;
                //     res
                // })
                .service(serve_dir)
        )
    )
}
