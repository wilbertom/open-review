use std::env;

use open_review::openai::{CreateModelResponseRequest, InputTextMessageContent, Message, OpenAI};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is missing");

    let request = CreateModelResponseRequest {
        model: "gpt-6-luna".into(),
        input: vec![Message {
            role: "user".into(),
            content: vec![InputTextMessageContent {
                type_: "input_text".into(),
                text: "Say hello in one sentence.".into(),
            }],
        }],
    };

    let openai = OpenAI::new(&openai_api_key).expect("Failed to create OpenAI client.");
    let response = openai.create_response(&request).await?;

    println!("{response:#?}");

    Ok(())
}
