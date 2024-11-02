use axum::{routing::get, Router};
use axum::response::IntoResponse;
use crate::{config, pipeline_fetcher};
use crate::pipeline::Pipeline;
use axum_response_cache::CacheLayer;

pub async fn start(addr: &str) {
    let app = Router::new()
        .route(
            "/cctray.xml",
            get(handler).layer(CacheLayer::with_lifespan(60)),
        );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let configs = config::load();
    let mut all: Vec<Pipeline> = vec![];

    for config in &configs {
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
