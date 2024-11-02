use axum::{routing::get, Router};
use crate::{config, pipeline_fetcher};

pub async fn start(addr: &str) {
    let app = Router::new()
        .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> String {
    let configs = config::load();
    let config = configs.first().unwrap();
    let pipelines = pipeline_fetcher::fetch(&config.id, &config.bearer_token).await;

    let pipelines_in_xml = pipelines.iter()
        .map(|pipeline| pipeline.to_xml())
        .collect::<Vec<_>>().join("\n");

    format!(r#"<Projects>
    {}
    </Projects>"#, pipelines_in_xml)
}
