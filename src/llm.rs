use crate::openai::{
    CreateModelResponseRequest, InputTextMessageContent, Message, ModelResponse, OpenAI,
    OpenAIResult,
};

pub struct LLM {
    model: String,
    provider: OpenAI,
}

impl LLM {
    pub fn new(api_key: &str, model: &str) -> Self {
        let provider = OpenAI::new(api_key).expect("Failed to create OpenAI provider client");

        LLM {
            model: model.to_string(),
            provider,
        }
    }

    pub async fn invoke(
        &self,
        messages: &CreateModelResponseRequest,
    ) -> OpenAIResult<ModelResponse> {
        self.provider.create_response(messages).await
    }

    pub fn messages(&self, messages: Vec<Message>) -> CreateModelResponseRequest {
        CreateModelResponseRequest {
            model: self.model.clone(),
            input: messages,
        }
    }

    pub fn user_message(&self, text: String) -> Message {
        self.message("user", text)
    }

    pub fn system_message(&self, text: String) -> Message {
        self.message("system", text)
    }

    fn message(&self, role: &str, text: String) -> Message {
        Message {
            role: role.into(),
            content: vec![InputTextMessageContent {
                type_: "input_text".into(),
                text,
            }],
        }
    }
}
