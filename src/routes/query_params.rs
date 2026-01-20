use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Querying{
	message: String,
	id: i32,
}

pub async fn query_params(Query(query): Query<Querying>) -> Json<Querying> {
	Json(query)
}