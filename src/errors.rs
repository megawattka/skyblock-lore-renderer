use warp::reject;

#[derive(Debug)]
pub struct BadRequest(pub &'static str);
impl reject::Reject for BadRequest {}