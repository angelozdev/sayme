use std::{
    env,
    error::Error,
    process::{Command, Output},
};

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use question::{Answer, Question};
use sayme::client;
use spinners;

#[derive(Debug, Parser)]
#[clap(name = "sayme", version = "0.1.0", author = "Angelo Zambrano")]
struct Cli {
    prompt: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut spinner = spinners::Spinner::new(spinners::Spinners::Hamburger, "Loading...".into());
    let prompt = build_prompt(cli.prompt.join(" "));
    let response = client::make_request(prompt).await;

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

    let code = response
        .choices
        .into_iter()
        .map(|c| c.text)
        .collect::<Vec<String>>()
        .join("\n");

    spinner.stop_with_newline();

    PrettyPrinter::new()
        .input_from_bytes(code.as_bytes())
        .language("bash")
        .grid(true)
        .print()
        .unwrap();

    let answer = Question::new("Do you want to run the code?")
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    if answer != Answer::YES {
        return Ok(());
    }

    spinner = spinners::Spinner::new(spinners::Spinners::Hamburger, "Running...".into());
    let output = run_code(code);

    match output {
        Err(e) => {
            spinner.stop_and_persist(
                "x".red().to_string().as_str(),
                "It was not possible to run the code.".to_string(),
            );
            panic!("Error: {}", e);
        }
        Ok(output) => {
            spinner.stop_and_persist(
                "✔".green().to_string().as_str(),
                String::from_utf8_lossy(&output.stdout).to_string(),
            );
        }
    };

    Ok(())
}

fn build_prompt(prompt: String) -> String {
    let os = env::consts::OS;
    let os_hint = match os {
        "linux" => "(on Linux)",
        "macos" => "(on macOS)",
        "windows" => "(on Windows)",
        _ => "",
    };

    format!("{} {}:\n```bash\n#!/bin/bash\n", prompt, os_hint)
}

fn run_code(code: String) -> Result<Output, Box<dyn Error>> {
    let output = Command::new("bash").arg("-c").arg(code).output()?;

    if !output.status.success() {
        return Err("Error: Execution failed.".into());
    }

    Ok(output)
}
