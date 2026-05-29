use std::{convert::Infallible, sync::Arc};

use warp::{Filter, reply::Reply};

use crate::{
    api::handlers::{
        handle_create,
        handle_rejection,
    },
    models::RenderRequest,
    state::AppState,
};

pub fn with_state(state: Arc<AppState>) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn make_routes(state: Arc<AppState>) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    warp::path!("render")
        .and(warp::post())
        .and(with_state(state))
        .and(warp::body::content_length_limit(1024 * 10)) // 10KB max
        .and(warp::body::json::<RenderRequest>())
        .and_then(handle_create)
        .recover(handle_rejection)
}