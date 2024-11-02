use axum::{routing::get, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use crate::pipeline_fetcher;
use crate::pipeline::Pipeline;
use axum_response_cache::CacheLayer;
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
    let mut all: Vec<Pipeline> = vec![];

    for config in configs.iter() {
        let pipelines = pipeline_fetcher::fetch(&config.id, &config.bearer_token).await;
        all.extend(pipelines);
    }

    let pipelines_in_xml = all
        .iter()
        .map(|pipeline| pipeline.to_xml())
        .collect::<Vec<_>>().join("\n");

    let body = format!("<Projects>{}</Projects>", pipelines_in_xml);

    let headers = [("Content-Type", "application/xml; charset=UTF-8")];
    (headers, body)
}

