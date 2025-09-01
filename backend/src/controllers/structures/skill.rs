use axum::debug_handler;
use loco_rs::prelude::*;


#[debug_handler]
async fn get_all() -> Result<Response> {
    format::text("Get all skills")
}

#[debug_handler]
async fn get_specific(Path(name): Path<u32>) -> Result<Response> {
    format::text(&format!("Get skill with name {name}"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1")
        .add("/skills", get(get_all))
        .add("/skills/{name}", get(get_specific))
}
