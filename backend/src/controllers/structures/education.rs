use axum::debug_handler;
use loco_rs::prelude::*;


#[debug_handler]
async fn get_all() -> Result<Response> {
    format::text("Get all educations")
}

#[debug_handler]
async fn get_specific(Path(id): Path<u32>) -> Result<Response> {
    format::text(&format!("Get education with id {id}"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1")
        .add("/education", get(get_all))
        .add("/education/{id}", get(get_specific))
}

