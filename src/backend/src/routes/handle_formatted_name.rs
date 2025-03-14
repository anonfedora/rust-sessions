use axum::Json;
use serde_json::{json, Value};

pub async fn formatted_name() -> Json<Value> {
    Json(json!({
        "status": 200,
        "name": "Yo"
    }))
}
