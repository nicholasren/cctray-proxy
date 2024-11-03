mod config;
mod pipeline;
mod json;
mod proxy;

use clap::Parser;
use crate::proxy::ProxyServer;

#[derive(Parser)]
pub struct Cli {
    repo_config_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let configs = config::load(args.repo_config_path);

    let addr = "0.0.0.0:3000";
    let server = ProxyServer {
        addr,
        configs,
    };
    println!("CCTray feed is available via http://{}/cctray.xml", addr);
    server.start().await;
}
