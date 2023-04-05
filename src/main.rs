use std::collections::HashMap;
use std::collections::hash_map;
use std::env;
use std::fs;
use json;
use json::object;
use serde_json::{Result, Value};
use reqwest::header::HeaderMap;
use toml;
use std::*;
use toml::*;
use serde::{Deserialize,Serialize};

#[derive(Deserialize)]
struct Config {
    openai_config: OpenaiConfig
}

#[derive(Deserialize)]
struct OpenaiConfig {
    key: String
}

struct Message {
    role: String,
    content: String
}

enum JsonData<'T> {
    model(&'T str),
    messages(Vec<Message>)
}

fn json_to_hashmap(json: &str, keys: Vec<&str>) -> Result<HashMap<String, Value>> {
    let mut lookup: HashMap<String, Value> = serde_json::from_str(json).unwrap();
    let mut map = HashMap::new();
    for key in keys {
        let (k, v) = lookup.remove_entry (key).unwrap();
        map.insert(k, v);
    }
    Ok(map)
}

#[tokio::main]
async fn main() {

    let current_path = env::current_dir().unwrap().into_os_string().into_string().unwrap();

    let read_file = fs::read_to_string(current_path + "\\src\\config.toml").unwrap();
    let cfg: Config = toml::from_str(&read_file).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", ["Bearer", &cfg.openai_config.key].join(" ").parse().unwrap());
    let req_client = reqwest::Client::new();

    let data = r#"

    {
        'model': 'gpt-3.5-turbo',
        'messages': [{'role': 'user', 'content': 'Hello!'}]
      }
      

    "#;

    let res = req_client.post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .send()
        .await
        .unwrap();

    std::println!("{}", res.text().await.unwrap());
} 
