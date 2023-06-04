use std::env;

use reqwest::{
    blocking::{Client, RequestBuilder},
    header::HeaderMap,
};
use serde_json::json;

fn get_headers(token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers
}

pub fn make_request(prompt: String) -> RequestBuilder {
    const OPENAI_URL_BASE: &str = "	https://api.openai.com/v1";
    let openai_key = env::var("OPENAI_KEY");

    if openai_key.is_err() {
        panic!("OPENAI_KEY is not set");
    }

    let headers = get_headers(openai_key.unwrap());
    let client = Client::new()
        .post(format!("{}/completions", OPENAI_URL_BASE))
        .headers(headers)
        .json(&json!({
            "top_p": 1,
            "stop": "```",
            "temperature": 0,
            "suffix": "\n```",
            "max_tokens": 256,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "model": "text-davinci-003",
            "prompt": prompt
        }));

    client
}
