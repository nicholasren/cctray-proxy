use axum::{routing::get, Router};
use crate::{config, pipeline};

pub async fn start() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> String {
    let configs = config::load();
    let config = configs.first().unwrap();
    let pipelines = pipeline::fetch(&config.id, &config.bearer_token).await;
    pipelines.first().unwrap().to_xml().to_string()
}
