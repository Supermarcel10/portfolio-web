use axum::debug_handler;
use loco_rs::prelude::*;


#[debug_handler]
async fn get_all() -> Result<Response> {
    format::text("Get all projects")
}

#[debug_handler]
async fn get_specific(Path(id): Path<u32>) -> Result<Response> {
    format::text(&format!("Get project with id {id}"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1")
        .add("/projects", get(get_all))
        .add("/projects/{id}", get(get_specific))
}
