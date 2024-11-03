use chrono::{DateTime, Utc};
use serde_json::Value;
#[derive(Debug, Clone)]
pub struct Pipeline {
    pub(crate) data: Value,
}

impl Pipeline {
    pub fn pipeline_type(self: &Pipeline) -> &str {
        let target = &self.data["target"];
        if crate::json::has_value_by_one_key(target, "ref_type", "branch")
            && crate::json::has_value_by_one_key(target, "ref_name", "main")
            && crate::json::has_value_by_two_keys(target, "selector", "type", "branches")
        {
            "main"
        } else if crate::json::has_value_by_two_keys(target, "selector", "type", "custom") {
            "custom"
        } else {
            "unknown"
        }
    }
    pub fn repo_name(self: &Pipeline) -> &str {
        crate::json::string_value_by_two_keys(&self.data, "repository", "name")
    }
    pub fn repo_full_name(self: &Pipeline) -> &str {
        crate::json::string_value_by_two_keys(&self.data, "repository", "full_name")
    }
    pub fn build_number(self: &Pipeline) -> String {
        let value = &self.data["build_number"].as_u64().unwrap();
        value.to_string()
    }
    pub fn name(self: &Pipeline) -> String {
        let target = &self.data["target"];
        if self.pipeline_type() == "main" {
            format!("[{repo_name}] pipeline:main", repo_name = self.repo_name()).clone()
        } else if self.pipeline_type() == "custom" {
            let pipeline_name = crate::json::string_value_by_two_keys(target, "selector", "pattern");
            format!("[{repo_name}] pipeline:custom:{custom_pipeline_name}", repo_name = self.repo_name(), custom_pipeline_name = pipeline_name).clone()
        } else {
            format!("[{repo_name}] pipeline:unknown", repo_name = self.repo_name())
        }
    }
    pub fn duration(self: &Pipeline) -> String {
        //duration_in_seconds
        let target = self.data["duration_in_seconds"].as_u64().unwrap();

        format!("{} min {} sec", target / 60, target % 60)
    }
    pub fn creator(self: &Pipeline) -> &str {
        crate::json::string_value_by_two_keys(&self.data, "creator", "display_name")
    }
    pub fn activity(self: &Pipeline) -> &str {
        if self.in_progress() {
            "Building"
        } else if self.completed() {
            "Sleeping"
        } else {
            "CheckingModifications"
        }
    }
    pub fn last_build_status(self: &Pipeline) -> &str {
        if self.completed() {
            if self.successful() {
                "Success"
            } else if self.failed() {
                "Failure"
            } else {
                "Unknown"
            }
        } else if self.in_progress() {
            "Building"
        } else {
            "unknown"
        }
    }
    pub fn build_label(self: &Pipeline) -> String {
        format!("by:{}, duration,{}", self.creator(), self.duration())
    }
    pub fn build_time_in_string(self: &Pipeline) -> &str {
        self.data["created_on"].as_str().unwrap()
    }
    pub fn web_url(self: &Pipeline) -> String {
        format!("https://bitbucket.org/{}/pipelines/results/{}", self.repo_full_name(), self.build_number())
    }
    pub fn supported(self: &Pipeline) -> bool {
        self.pipeline_type() != "unknown"
    }
    pub fn build_time(self: &Pipeline) -> DateTime<Utc> {
        let time = self.build_time_in_string();
        DateTime::parse_from_rfc3339(time)
            .expect(format!("unable to parse build time {}", time).as_str())
            .to_utc()
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
                self.build_label(),
                self.build_time_in_string(),
                self.name(),
                self.web_url(),
                self.activity(),
                self.last_build_status()
        )
    }
    fn in_progress(self: &Pipeline) -> bool {
        crate::json::has_value_by_two_keys(&self.data,
                                           "state",
                                           "name",
                                           "IN_PROGRESS")
    }
    fn completed(self: &Pipeline) -> bool {
        crate::json::has_value_by_two_keys(&self.data,
                                           "state",
                                           "name",
                                           "COMPLETED")
    }
    fn successful(self: &Pipeline) -> bool {
        crate::json::has_value_by_three_keys(&self.data,
                                             "state",
                                             "result",
                                             "name",
                                             "SUCCESSFUL")
    }
    fn failed(self: &Pipeline) -> bool {
        crate::json::has_value_by_three_keys(&self.data,
                                             "state",
                                             "result",
                                             "name",
                                             "FAILED")
    }
}

#[cfg(test)]
mod tests {
    use crate::pipeline::test_data_provider;

    #[test]
    fn parse_from() {
        let pipelines = test_data_provider::_from_fake_response("fixture/pipelines.json");
        assert_eq!(5, pipelines.len());
    }

    //BEGIN of main pipeline
    #[test]
    fn pipeline_type() {
        let pipeline = test_data_provider::_one("fixture/bitbucket_pipeline_main.json");

        assert_eq!(pipeline.pipeline_type(), "main");
    }

    #[test]
    fn name() {
        let pipeline = test_data_provider::_one("fixture/bitbucket_pipeline_main.json");
        assert_eq!(pipeline.name(), "[adr-backup-monitoring] pipeline:main");
    }

    #[test]
    fn repo_name() {
        let pipeline = test_data_provider::_one("fixture/bitbucket_pipeline_main.json");
        assert_eq!(pipeline.repo_name(), "adr-backup-monitoring");
    }

    #[test]
    fn status() {
        let pipeline = test_data_provider::_one("fixture/bitbucket_pipeline_main.json");
        assert_eq!(pipeline.last_build_status(), "Success");
    }

    #[test]
    fn to_xml() {
        let pipeline = test_data_provider::_one("fixture/bitbucket_pipeline_main.json");
        println!("{}", pipeline.to_xml());
    }

    #[test]
    fn supports() {
        let pipeline = test_data_provider::_one("fixture/bitbucket_pipeline_main.json");
        assert!(pipeline.supported())
    }
    #[test]
    fn web_url() {
        let pipeline = test_data_provider::_one("fixture/bitbucket_pipeline_main.json");
        assert_eq!(pipeline.web_url(), "https://bitbucket.org/atlassian/adr-backup-monitoring/pipelines/results/19442");
    }
}
