use std::fs;
use serde_json::Value;

#[derive(Debug)]
pub struct Pipeline {
    data: Value,
}

impl Pipeline {
    pub fn from_response(path: &str) -> Vec<Pipeline> {
        let content = fs::read_to_string(path).expect("Could not read file");
        let parsed_body: Value = serde_json::from_str(&content).expect("JSON was not well-formatted");
        let values = parsed_body.get("values");
        values
            .and_then(|v| v.as_array())
            .map(|items| {
                items
                    .iter()
                    .map(|value| Pipeline { data: value.clone() })
                    .collect()
            })
            .unwrap()
    }
    pub fn from(path: &str) -> Pipeline {
        let content = fs::read_to_string(path).expect("Could not read file");
        let data: Value = serde_json::from_str(&content).expect("JSON was not well-formatted");
        Pipeline { data }
    }

    pub fn pipeline_type(self: &Pipeline) -> &str {
        let target = &self.data["target"];
        if crate::json::string_value_of(target, "ref_type") == "branch"
            && crate::json::string_value_of(target, "ref_name") == "main"
            && crate::json::string_values_of(target, "selector", "type") == "branches"
        {
            "main"
        } else if crate::json::string_values_of(target, "selector", "type") == "custom" {
            "custom"
        } else {
            "unknown"
        }
    }
    pub fn repo_name(self: &Pipeline) -> &str {
        crate::json::string_values_of(&self.data, "repository", "name")
    }

    pub fn name(self: &Pipeline) -> String {
        let target = &self.data["target"];
        if self.pipeline_type() == "main" {
            format!("[{repo_name}] pipeline:main", repo_name = self.repo_name()).clone()
        } else if self.pipeline_type() == "custom" {
            let pipeline_name = crate::json::string_values_of(target, "selector", "pattern");
            format!("[{repo_name}] custom pipeline:{custom_pipeline_name}", repo_name = self.repo_name(), custom_pipeline_name = pipeline_name).clone()
        } else {
            String::from("unknown")
        }
    }

    pub fn activity(self: &Pipeline) -> &str {
        let state = crate::json::string_values_of(&self.data, "state", "name");
        if state == "IN_PROGRESS" {
            "building"
        } else if state == "COMPLETED" {
            "sleeping"
        } else {
            "checking_modification"
        }
    }
    pub fn status(self: &Pipeline) -> &str {
        if self.activity() == "sleeping" {
            let state = &self.data["state"];
            let state_result = crate::json::string_values_of(state, "result", "name");

            if state_result == "SUCCESSFUL" {
                "success"
            } else if state_result == "FAILED" {
                "failed"
            } else {
                "unknown"
            }
        } else if self.activity() == "building" {
            "building"
        } else {
            "unknown"
        }
    }
}

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

    let parsed_body: Value = serde_json::from_str(&body).expect("JSON was not well-formatted");
    let values = parsed_body.get("values");
    values.unwrap().as_array().unwrap().iter()
        .map(|value| Pipeline { data: value.clone() })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from() {
        let pipelines = Pipeline::from_response("fixture/pipelines.json");
        assert_eq!(100, pipelines.len());
    }

    //BEGIN of main pipeline
    #[test]
    fn pipeline_type() {
        let pipeline = Pipeline::from("fixture/bitbucket_pipeline_main.json");

        assert_eq!(pipeline.pipeline_type(), "main");
    }

    #[test]
    fn name() {
        let pipeline = Pipeline::from("fixture/bitbucket_pipeline_main.json");
        assert_eq!(pipeline.name(), "[adr-backup-monitoring] pipeline:main");
    }

    #[test]
    fn repo_name() {
        let pipeline = Pipeline::from("fixture/bitbucket_pipeline_main.json");
        assert_eq!(pipeline.repo_name(), "adr-backup-monitoring");
    }

    #[test]
    fn status() {
        let pipeline = Pipeline::from("fixture/bitbucket_pipeline_main.json");
        assert_eq!(pipeline.status(), "success");
    }

    //END of main pipeline
}
