use std::{sync::Arc, time::Instant};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use log::{info, warn};
use serde_json::json as s_json;
use warp::{
    http::StatusCode, reject::{self, Rejection}, reply::{self, Json, Reply, json}
};

use crate::{
    core::{
        parser::parse_lore_lines,
        renderer::render_lore
    },
    errors::BadRequest,
    models::{
        RenderOptions,
        RenderRequest,
        RenderResponse
    },
    state::AppState,
    utils::to_png_bytes
};

pub async fn handle_create(state: Arc<AppState>, request: RenderRequest) -> Result<Json, Rejection> {
    let start = Instant::now();

    let lore = request.lore;
    let lines: Vec<&str> = lore.lines().collect();
    info!("Rendering: {:?}", lines.first().unwrap_or(&""));

    let parsed = parse_lore_lines(&lines, &state.line_regex);
    let options = request.options.unwrap_or(RenderOptions::default());

    let image = render_lore(&parsed, &state, &options);
    let content = to_png_bytes(&image)
        .map_err(|_| reject::custom(BadRequest("PNG encode failed")))?;

    let elapsed = format!("{:.2?}", start.elapsed());
    info!("Rendered image in {}", elapsed);

    let response = RenderResponse {
        image: STANDARD.encode(&content),
        width: image.width() as u32,
        height: image.height() as u32,
        render_time_ms: start.elapsed().as_secs_f32() * 1000.0
    };
    Ok(json(&response))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else if let Some(e) = err.find::<BadRequest>() {
        (StatusCode::BAD_REQUEST, e.0)
    } else if err.find::<reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed")
    } else {
        warn!("unhandled rejection: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    };

    let reply = reply::json(&s_json!({
        "code": code.as_u16(),
        "message": message,
    }));

    Ok(reply::with_status(reply, code))
}