use axum::{
    extract::OriginalUri,
    http::{StatusCode, header},
    response::IntoResponse,
};
use mime_guess::from_path;
use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "webui/dist/"]
struct WebAssets;

pub async fn get(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
    let path = uri
        .path()
        .strip_prefix("/webui/")
        .unwrap_or("")
        .trim_end_matches('/');

    // Try to get a static asset (like js/css/img)
    if let Some(file) = WebAssets::get(path) {
        let mime = from_path(path).first_or_octet_stream();
        return (
            [(header::CONTENT_TYPE, mime.to_string())],
            file.data.into_owned(),
        )
            .into_response();
    }

    // Fallback to index.html for Vue Router to handle the route
    match WebAssets::get("index.html") {
        Some(index) => {
            let mime = from_path("index.html").first_or_octet_stream();
            (
                [(header::CONTENT_TYPE, mime.to_string())],
                index.data.into_owned(),
            )
                .into_response()
        }
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
