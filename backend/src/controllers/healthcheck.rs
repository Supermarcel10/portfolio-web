use axum::debug_handler;
use chrono::Utc;
use loco_rs::prelude::*;
use serde_json::json;


#[debug_handler]
async fn healthcheck() -> Result<Response> {
    format::json(json!({
        "message": "Status is healthy!",
        "status": 200,
        "timestamp": Utc::now().to_rfc3339(),
    }))
}

pub fn routes() -> Routes {
    Routes::new().add("/api/healthcheck", get(healthcheck))
}
