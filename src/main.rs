use clap::Parser;
use colored::Colorize;
use sayme::{client, utils::format_text};
use spinners;
use std::env;

#[derive(Debug, Parser)]
#[clap(name = "sayme", version = "0.1.0", author = "Angelo Zambrano")]
struct Cli {
    prompt: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut spinner = spinners::Spinner::new(spinners::Spinners::Hamburger, "Loading...".into());
    let prompt = build_prompt(cli.prompt.join(" ").as_str());
    let response = client::make_request(prompt).send();

    let response = match response {
        Ok(response) => response,
        Err(e) => {
            spinner.stop_and_persist(
                "x".red().to_string().as_str(),
                "It was not possible to connect to the API.".to_string(),
            );
            panic!("Error: {}", e);
        }
    };

    let status_code = response.status();

    if status_code.is_client_error() || status_code.is_server_error() {
        spinner.stop_and_persist(
            "x".red().to_string().as_str(),
            "API is not available. Please try again later.".to_string(),
        );
        panic!("Error: {}", status_code);
    }

    let answer = format_text(
        response.json::<serde_json::Value>().unwrap()["choices"][0]["text"]
            .as_str()
            .unwrap()
            .to_string(),
    );

    spinner.stop_and_persist("✔".green().to_string().as_str(), answer);
}

fn build_prompt(prompt: &str) -> String {
    let mut prompt = prompt.to_string();
    let os = env::consts::OS;

    if os == "windows" {
        prompt.push_str(" (on Windows)");
    } else if os == "linux" {
        prompt.push_str(" (on Linux)");
    } else if os == "macos" {
        prompt.push_str(" (on macOS)");
    }

    prompt.push_str(":\n```bash\n#!/bin/bash\n");

    prompt
}
