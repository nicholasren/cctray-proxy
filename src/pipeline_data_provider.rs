use std::fs;
use serde_json::Value;
use crate::pipeline::Pipeline;
use crate::pipeline_parser::parse;

pub fn one(file_path: &str) -> Pipeline {
    let content = fs::read_to_string(file_path).expect("Could not read file");
    let data: Value = serde_json::from_str(&content).expect("JSON was not well-formatted");
    Pipeline { data }
}

pub fn from_fake_response(file_path: &str) -> Vec<Pipeline> {
    let content = fs::read_to_string(file_path).expect("Could not read file");
    parse(&content)
}
