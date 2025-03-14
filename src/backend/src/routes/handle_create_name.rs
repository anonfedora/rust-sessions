use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Username {
    name: String,
}

// create name route
pub async fn create_name(Json(payload): Json<Username>) -> Json<Value> {
    println!("create payload here____{:#?}", payload);
    Json(json!({
        "status": 200,
        "name": payload.name
    }))
}
