use std::{convert::Infallible, sync::Arc, time::Instant};

use ab_glyph::FontRef;
use bytes::Bytes;
use dotenvy::dotenv;
use log::info;
use regex::Regex;
use skyblock_lore_renderer::utils::build_image_response;
use warp::{reject, reply};
use warp::{
    Filter, Rejection, Reply,
    http::StatusCode,
};
use serde_json::json as s_json;

use skyblock_lore_renderer::{
    errors::BadRequest,
    font::load_font,
    parser::parse_lore_lines,
    renderer::render_lore,
    utils::{
        bytes_to_string,
        decode_base64,
        to_png_bytes,
    },
};

// ========== Состояние ==========

#[derive(Debug, Clone)]
struct AppState {
    font: FontRef<'static>,
    enchant_regex: Regex,
}

fn build_state() -> anyhow::Result<Arc<AppState>> {
    let font = load_font()?;
    info!("Faithful font loaded successfully");

    let enchant_regex = Regex::new(r"^([A-Za-z ]+) ([IVX]+)")?;
    Ok(Arc::new(AppState { font, enchant_regex }))
}

// ========== Handlers ==========

async fn handle_create(state: Arc<AppState>, body: Bytes) -> Result<impl Reply, Rejection> {
    let decoded = decode_base64(&body)?;
    let input = bytes_to_string(decoded)?;

    let start = Instant::now();
    let lines: Vec<&str> = input.lines().collect();

    let parsed = parse_lore_lines(&lines);
    let image = render_lore(&parsed, &state.font, &state.enchant_regex);
    let content = to_png_bytes(&image);

    let elapsed = format!("{:.2?}", start.elapsed());
    Ok(build_image_response(content, elapsed))
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
        eprintln!("unhandled rejection: {:?}", err);
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

fn routes(state: Arc<AppState>) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    warp::path!("create")
        .and(warp::post())
        .and(with_state(state))
        .and(warp::body::bytes())
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
    warp::serve(routes(state))
        .run(([127, 0, 0, 1], 8080))
        .await;

    Ok(())
}