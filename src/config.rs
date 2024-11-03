use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub id: String,
    pub bearer_token: String,
}

pub fn load(file_path: &str) -> Vec<Config> {
    println!("Loading config from {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    serde_json::from_str(&contents)
        .expect("JSON was not well-formatted")
}
