mod hello_world;
mod mirror_body_string;
mod mirror_body_json;

use axum::{
    Router, routing::{get, post}
};

use crate::routes::{hello_world::handle_root, mirror_body_json::handle_json, mirror_body_string::handle_post};

pub fn create_routes() -> Router {
	Router::new().route("/mirror_body_string", post(handle_post))
		.route("/", get(handle_root))
		.route("/mirror_body_json", post(handle_json))
}

