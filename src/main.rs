mod config;
mod pipeline;
mod json;
mod proxy;
mod pipeline_fetcher;
mod pipeline_parser;
mod pipeline_data_provider;

use clap::Parser;
use crate::proxy::ProxyServer;

#[derive(Parser)]
pub struct Cli {
    config_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    println!("Using config path: {:?}", args.config_path);
    let configs = config::load();

    let server = ProxyServer {
        addr: "0.0.0.0:3000",
        configs,
    };

    server.start().await;
}
