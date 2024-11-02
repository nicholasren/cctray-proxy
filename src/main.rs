mod config;
mod pipeline;
mod json;
mod proxy;

#[tokio::main]
async fn main() {
    proxy::start().await;
}
