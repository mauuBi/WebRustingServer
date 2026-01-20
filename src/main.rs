use rust_axum::running_server;

#[tokio::main]
async fn main() {
    running_server().await;
}
