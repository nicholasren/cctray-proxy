use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub id: String,
    pub bearer_token: String,
}

pub fn load(file_path: PathBuf) -> Vec<Config> {
    println!("Loading config from {}", file_path.display());
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    serde_json::from_str(&contents)
        .expect("JSON was not well-formatted")
}
