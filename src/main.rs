mod config;
mod pipeline;
mod json;
mod proxy;
mod pipeline_fetcher;
mod pipeline_loader;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    config_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    // let args = Cli::parse();
    // println!("Using config path: {:?}", args.config_path);

    proxy::start("0.0.0.0:3000").await;
}
