use std::{
    convert::Infallible,
    sync::Arc,
    time::Instant
};

use base64::{Engine, engine::general_purpose::STANDARD};
use dotenvy::dotenv;
use log::{info, warn};
use warp::{
    Filter, Rejection, Reply,
    http::StatusCode,
    reject,
    reply::{self, Json, json}
};
use serde_json::json as s_json;

use skyblock_lore_renderer::{
    errors::BadRequest,
    models::{
        RenderOptions,
        RenderRequest,
        RenderResponse
    },
    parser::parse_lore_lines,
    renderer::render_lore,
    state::{AppState, build_state},
    utils::to_png_bytes
};

// ========== Handlers ==========

async fn handle_create(state: Arc<AppState>, request: RenderRequest) -> Result<Json, Rejection> {
    let start = Instant::now();

    let lore = request.lore;
    let lines: Vec<&str> = lore.lines().collect();
    info!("Rendering: {:?}", lines.first().unwrap_or(&""));

    let parsed = parse_lore_lines(&lines);
    let options = request.options.unwrap_or(RenderOptions::default());

    let image = render_lore(&parsed, state, &options);
    let content = to_png_bytes(&image);

    let elapsed = format!("{:.2?}", start.elapsed());
    info!("Rendered image in {}", elapsed);

    let response = RenderResponse {
        image: STANDARD.encode(&content),
        width: image.width() as u32,
        height: image.height() as u32,
        render_time_ms: start.elapsed().as_millis()
    };
    Ok(json(&response))
}

// ========== Error handler ==========

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
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

// ========== Filters ==========

fn with_state(state: Arc<AppState>) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn make_routes(state: Arc<AppState>) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    warp::path!("render")
        .and(warp::post())
        .and(with_state(state))
        .and(warp::body::content_length_limit(1024 * 10)) // 10KB max
        .and(warp::body::json::<RenderRequest>())
        .and_then(handle_create)
        .recover(handle_rejection)
}

// ========== Main ==========

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    println!("Skyblock Lore Renderer - Starting up...");

    let state = build_state()?;
    let routes = make_routes(state);
    info!("routes configured");

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;

    Ok(())
}