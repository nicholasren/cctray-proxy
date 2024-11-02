use std::fs;
use serde_json::Value;
use crate::pipeline::Pipeline;
use itertools::Itertools;

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
        .max_by_key(|pipeline: &&Pipeline| pipeline.build_time())
        .unwrap();

    latest.clone()
}

#[cfg(test)]
mod tests {
    use crate::pipeline_parser;

    #[test]
    fn parse_from() {
        let pipelines = pipeline_parser::from_fake_response("fixture/pipelines.json");
        assert_eq!(5, pipelines.len());
        for pipeline in pipelines {
            println!("{:?}", pipeline.to_xml());
        }
    }
}
