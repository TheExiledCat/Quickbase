use axum::http::HeaderMap;
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

pub async fn get(OriginalUri(uri): OriginalUri, headers: HeaderMap) -> impl IntoResponse {
    let path = uri
        .path()
        .trim_start_matches("/webui")
        .trim_start_matches('/');
    let accept_gzip = headers
        .get(header::ACCEPT_ENCODING)
        .map(|v| v.to_str().unwrap_or("").contains("gzip"))
        .unwrap_or(false);

    let gz_path = format!("{}.gz", path);

    if accept_gzip {
        if let Some(asset) = WebAssets::get(&gz_path) {
            let mime = from_path(path).first_or_octet_stream();
            return (
                [
                    (header::CONTENT_TYPE, mime.to_string()),
                    (header::CONTENT_ENCODING, "gzip".to_string()),
                ],
                asset.data.into_owned(),
            )
                .into_response();
        }
    }

    if let Some(asset) = WebAssets::get(path) {
        let mime = from_path(path).first_or_octet_stream();
        return (
            [(header::CONTENT_TYPE, mime.to_string())],
            asset.data.into_owned(),
        )
            .into_response();
    }

    // Fallback to SPA routing
    if let Some(asset) = WebAssets::get("index.html.gz") {
        return (
            [("Content-Type", "text/html"), ("Content-Encoding", "gzip")],
            asset.data.into_owned(),
        )
            .into_response();
    }

    StatusCode::NOT_FOUND.into_response()
}
