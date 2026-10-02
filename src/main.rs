use std::env;

use open_review::llm::LLM;

const DEFAULT_OPENAI_MODEL: &str = "gpt-6-luna";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is missing");
    let openai_model =
        env::var("OPENAI_MODEL").unwrap_or_else(|_| DEFAULT_OPENAI_MODEL.to_string());

    let llm = LLM::new(&openai_api_key, &openai_model);

    let messages = llm.messages(vec![llm.user_message("Say hello in one sentence.".into())]);
    let response = llm.invoke(&messages).await?;

    println!("{response:#?}");

    Ok(())
}
