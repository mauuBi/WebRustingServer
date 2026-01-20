use axum::{Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson{
	message: String,
	age: i64,
}

pub async fn handle_json(Json(body): Json<MirrorJson>) -> Json<MirrorJson>{
	Json(body)
}
