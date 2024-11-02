mod config;
mod pipeline;
mod json;
mod proxy;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    config_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    println!("Using config path: {:?}", args.config_path);

    proxy::start().await;
}
