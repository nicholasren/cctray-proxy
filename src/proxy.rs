use axum::{routing::get, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum_response_cache::CacheLayer;
use tokio::time::Instant;
use crate::pipeline::fetcher;
use crate::pipeline::model::Pipeline;
use crate::config::Config;

pub struct ProxyServer {
    pub addr: &'static str,
    pub configs: Vec<Config>,
}
impl ProxyServer {
    pub async fn start(self: &ProxyServer) {
        let app = Router::new()
            .route(
                "/cctray.xml",
                get(handler).layer(CacheLayer::with_lifespan(60)),
            ).with_state(self.configs.clone());

        let listener = tokio::net::TcpListener::bind(&self.addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
async fn handler(configs: State<Vec<Config>>) -> impl IntoResponse {
    let start = Instant::now();
    let all = fetch_concurrently(configs.to_vec()).await;
    let duration = start.elapsed();
    println!("Fetched pipeline status in {} seconds", duration.as_secs());

    let pipelines_in_xml = all
        .iter()
        .map(|pipeline| pipeline.to_xml())
        .collect::<Vec<_>>().join("\n");

    let body = format!("<Projects>{}</Projects>", pipelines_in_xml);

    let headers = [("Content-Type", "application/xml; charset=UTF-8")];
    (headers, body)
}

async fn fetch_concurrently(configs: Vec<Config>) -> Vec<Pipeline> {
    let handles = configs
        .into_iter() // into_iter do not require static lifetime.
        .map(|config| {
            tokio::spawn(async move {
                let repo_id = config.id.clone();
                let bearer_token = config.bearer_token.clone();
                println!("working on pipeline status for repo: {}", &config.id);
                fetcher::fetch(&repo_id, &bearer_token).await.clone()
            })
        });

    futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|result| result.unwrap_or(vec![]))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::config;
    use super::*;
    #[tokio::test]
    async fn test_fetch_concurrently() {
        let configs = config::load("/tmp/repos.json");
        fetch_concurrently(configs).await;
    }
}

