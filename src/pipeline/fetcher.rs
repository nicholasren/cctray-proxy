use crate::pipeline::model::Pipeline;

pub async fn fetch(repo_id: &str, token: &str) -> Vec<Pipeline> {
    let base_url = "https://api.bitbucket.org/2.0/repositories";

    let request_url = format!("{base_url}/{repo_id}/pipelines/?pagelen={page_length}&&sort=-created_on",
                              base_url = base_url,
                              repo_id = repo_id,
                              page_length = 100);

    let client = reqwest::Client::new();

    let body = client.get(request_url).bearer_auth(token).send()
        .await.unwrap().text()
        .await.unwrap();

    crate::pipeline::parser::parse(&body)
}

