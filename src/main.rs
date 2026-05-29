use dotenvy::dotenv;
use log::info;

use skyblock_lore_renderer::{api::routes::make_routes, state::build_state};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    println!("Skyblock Lore Renderer - Starting up...");

    let state = build_state()?;
    let routes = make_routes(state);

    info!("server starting on http://localhost:8080");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;

    Ok(())
}
