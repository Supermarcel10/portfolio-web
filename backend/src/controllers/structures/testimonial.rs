use axum::debug_handler;
use loco_rs::prelude::*;


#[debug_handler]
async fn get_all() -> Result<Response> {
    format::text("Get all testimonials")
}

#[debug_handler]
async fn get_specific(Path(id): Path<u32>) -> Result<Response> {
    format::text(&format!("Get testimonial with id {id}"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1")
        .add("/testimonials", get(get_all))
        .add("/testimonials/{id}", get(get_specific))
}
