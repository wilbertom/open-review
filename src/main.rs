use std::env;

use open_review::openai::{CreateModelResponseRequest, InputTextMessageContent, Message, OpenAI};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is missing");

    let open_ai_default_model = "gpt-6-luna".into();
    let open_ai_model = env::var("OPENAI_MODEL").unwrap_or_else(|_| open_ai_default_model);

    let request = CreateModelResponseRequest {
        model: open_ai_model.into(),
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
