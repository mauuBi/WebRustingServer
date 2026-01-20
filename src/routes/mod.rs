mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;
mod middleware_custom_header;

use axum::{
    Extension, Router, http::Method, routing::{get, post}
};

use hello_world::handle_root;
use mirror_body_json::handle_json;
use mirror_body_string::handle_post;
use path_variables::path_variables;
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use tower_http::cors::{CorsLayer, Any};
use middleware_message::middleware_message;
use middleware_custom_header::middleware_custom_header;

#[derive(Clone)]
pub struct ShareData{
	pub message: String,
}

pub fn create_routes() -> Router {
	let cors = CorsLayer::new()
	.allow_methods([Method::GET, Method::POST])
	.allow_origin(Any);

	let shared_data = ShareData{
		message: String::from("Hello from shared data en petard")
	};

	Router::new().route("/mirror_body_string", post(handle_post))
		.route("/", get(handle_root))
		.route("/mirror_body_json", post(handle_json))
		.route("/path_variables/{capture}", get(path_variables))
		.route("/query_params", get(query_params))
		.route("/mirror_user_agent", get(mirror_user_agent))
		.route("/mirror_custom_header", get(mirror_custom_header))
		.route("/middleware_message", get(middleware_message))
		.route("/middleware_custom_header", get(middleware_custom_header))
		.layer(cors)
		.layer(Extension(shared_data))
}

