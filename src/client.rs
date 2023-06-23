use async_openai::{
    error::OpenAIError,
    types::{CreateCompletionRequestArgs, CreateCompletionResponse},
    Client as OpenAiClient,
};

pub async fn make_request(prompt: String) -> Result<CreateCompletionResponse, OpenAIError> {
    let client = OpenAiClient::new();
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(prompt)
        .max_tokens(1000_u16)
        .n(1_u8)
        .stop("```")
        .suffix("\n```")
        .build()?;

    let response = client.completions().create(request).await?;

    Ok(response)
}
