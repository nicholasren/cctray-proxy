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
        if crate::json::has_value_by_key(target, "ref_type", "branch")
            && crate::json::has_value_by_key(target, "ref_name", "main")
            && crate::json::has_value_by_keys(target, "selector", "type", "branches")
        {
            "main"
        } else if crate::json::has_value_by_keys(target, "selector", "type", "custom") {
            "custom"
        } else {
            "unknown"
        }
    }
    pub fn repo_name(self: &Pipeline) -> &str {
        crate::json::string_value_by_keys(&self.data, "repository", "name")
    }

    pub fn name(self: &Pipeline) -> String {
        let target = &self.data["target"];
        if self.pipeline_type() == "main" {
            format!("[{repo_name}] pipeline:main", repo_name = self.repo_name()).clone()
        } else if self.pipeline_type() == "custom" {
            let pipeline_name = crate::json::string_value_by_keys(target, "selector", "pattern");
            format!("[{repo_name}] custom pipeline:{custom_pipeline_name}", repo_name = self.repo_name(), custom_pipeline_name = pipeline_name).clone()
        } else {
            String::from("unknown")
        }
    }

    pub fn duration(self: &Pipeline) -> String {
        //duration_in_seconds
        let target = self.data["duration_in_seconds"].as_u64().unwrap();

        format!("{} min {} sec", target / 60, target % 60)
    }

    pub fn creator(self: &Pipeline) -> &str {
        crate::json::string_value_by_keys(&self.data, "creator", "display_name")
    }

    pub fn activity(self: &Pipeline) -> &str {
        let state_name = crate::json::string_value_by_keys(&self.data, "state", "name");
        if state_name == "IN_PROGRESS" {
            "Building"
        } else if state_name == "COMPLETED" {
            "Sleeping"
        } else {
            "CheckingModifications"
        }
    }
    pub fn last_build_status(self: &Pipeline) -> &str {
        if self.activity() == "Building" {
            let state = &self.data["state"];
            let state_result = crate::json::string_value_by_keys(state, "result", "name");

            if state_result == "SUCCESSFUL" {
                "Success"
            } else if state_result == "FAILED" {
                "Failure"
            } else {
                "Unknown"
            }
        } else if self.activity() == "Sleeping" {
            "Building"
        } else {
            "unknown"
        }
    }
    pub fn last_build_label(self: &Pipeline) -> String {
        format!("by:{}, duration,{}", self.creator(), self.duration())
    }
    pub fn last_build_time(self: &Pipeline) -> &str {
        self.data["created_on"].as_str().unwrap()
    }

    pub fn web_url(self: &Pipeline) -> &str {
        ""
    }


    pub fn to_xml(self: &Pipeline) -> String {
        format!(r#"<Project
            lastBuildLabel="{}"
            lastBuildTime="{}"
            name="{}"
            webUrl="{}"
            activity="{}"
            lastBuildStatus="{}"
            />"#,
                self.last_build_label(),
                self.last_build_time(),
                self.name(),
                self.web_url(),
                self.activity(),
                self.last_build_status()
        )
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
        assert_eq!(pipeline.last_build_status(), "Building");
    }

    #[test]
    fn to_xml() {
        let pipeline = Pipeline::from("fixture/bitbucket_pipeline_main.json");
        println!("{}", pipeline.to_xml());
    }

    //END of main pipeline
}
