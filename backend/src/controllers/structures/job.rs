use axum::debug_handler;
use loco_rs::prelude::*;


#[debug_handler]
async fn get_all() -> Result<Response> {
    format::text("Get all jobs")
}

#[debug_handler]
async fn get_specific(Path(id): Path<u32>) -> Result<Response> {
    format::text(&format!("Get job with id {id}"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1")
        .add("/jobs", get(get_all))
        .add("/jobs/{id}", get(get_specific))
}
