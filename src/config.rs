use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub id: String,
    pub bearer_token: String,
}

pub fn load() -> Vec<Config> {
    let contents = fs::read_to_string("config/repos.json").unwrap();
    serde_json::from_str(&contents).expect("JSON was not well-formatted")
}
