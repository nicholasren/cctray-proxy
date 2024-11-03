use std::fs;
use serde_json::Value;
use crate::pipeline::model::Pipeline;
use crate::pipeline::parser::parse;

pub fn _one(file_path: &str) -> Pipeline {
    let content = fs::read_to_string(file_path).expect("Could not read file");
    let data: Value = serde_json::from_str(&content).expect("JSON was not well-formatted");
    Pipeline { data }
}
pub fn _from_fake_response(fake_response_file: &str) -> Vec<Pipeline> {
    let content = fs::read_to_string(fake_response_file).expect("Could not read file");
    parse(&content)
}
