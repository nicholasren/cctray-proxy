mod config;
mod pipeline;
mod json;

#[tokio::main]
async fn main() {
    println!("================================================================");
    let configs = config::load();
    let config = configs.get(0).unwrap();
    let pipeline = pipeline::fetch(&config.id, &config.bearer_token).await;

    println!("{:#?}", pipeline);
}
