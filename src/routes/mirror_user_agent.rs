use axum_extra::{TypedHeader, headers::{UserAgent}};

pub async fn mirror_user_agent(TypedHeader(header): TypedHeader<UserAgent>) -> String{
	header.to_string()
}