use std::cmp::max_by_key;
use std::fs;
use serde_json::Value;
use crate::pipeline::Pipeline;
use itertools::Itertools;

pub fn one(file_path: &str) -> Pipeline {
    let content = fs::read_to_string(file_path).expect("Could not read file");
    let data: Value = serde_json::from_str(&content).expect("JSON was not well-formatted");
    Pipeline { data }
}

pub fn from_fake_response(file_path: &str) -> Vec<Pipeline> {
    let content = fs::read_to_string(file_path).expect("Could not read file");
    parse(&content)
}

pub fn parse(content: &String) -> Vec<Pipeline> {
    let parsed: Value = serde_json::from_str(&content).expect("JSON was not well-formatted");

    let values = parsed.get("values");
    values
        .and_then(|v| v.as_array())
        .map(|items| {
            let pipelines = items
                .iter()
                .map(|value| Pipeline { data: value.clone() })
                .filter(|pipeline| pipeline.supported())
                .collect::<Vec<Pipeline>>();

            return pipelines
                .into_iter()
                .into_group_map_by(|pipeline: &Pipeline| pipeline.name())
                .into_iter()
                .map(|(_, group)| latest_build_of(group))
                .collect::<Vec<Pipeline>>();
        })
        .unwrap()
}

fn latest_build_of(pipelines: Vec<Pipeline>) -> Pipeline {
    let latest = pipelines
        .iter()
        .max_by_key(|pipeline: &&Pipeline| pipeline.name())
        .unwrap();

    let x: Pipeline = latest.clone();
    x
}

#[cfg(test)]
mod tests {
    use crate::pipeline_loader;

    #[test]
    fn parse_from() {
        let pipelines = pipeline_loader::from_fake_response("fixture/pipelines.json");
        assert_eq!(5, pipelines.len());
        for pipeline in pipelines {
            println!("{:?}", pipeline.to_xml());
        }
    }
}
