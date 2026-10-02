use reqwest::{
    StatusCode,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct OpenAI {
    http: reqwest::Client,
    base_url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum OpenAIError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("api error {status}: {body}")]
    Api { status: StatusCode, body: String },
}

pub type OpenAIResult<T> = std::result::Result<T, OpenAIError>;

impl OpenAI {
    pub fn new(api_key: &str) -> OpenAIResult<Self> {
        let mut headers = HeaderMap::new();

        let mut auth = HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap();
        auth.set_sensitive(true);

        headers.insert(AUTHORIZATION, auth);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self {
            http,
            base_url: "https://api.openai.com/v1".into(),
        })
    }

    async fn send<T: for<'de> Deserialize<'de>>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> OpenAIResult<T> {
        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(OpenAIError::Api { status, body });
        }

        Ok(resp.json().await?)
    }

    pub async fn create_response(
        &self,
        req: &CreateModelResponseRequest,
    ) -> OpenAIResult<ModelResponse> {
        self.send(
            self.http
                .post(format!("{}/responses", self.base_url))
                .json(req),
        )
        .await
    }
}

// Reponses API
// https://developers.openai.com/api/reference/resources/responses/methods/create

// Reponses API - Request
#[derive(Clone, Debug, Serialize)]
pub struct CreateModelResponseRequest {
    pub model: String,
    pub input: Vec<Message>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: Vec<InputTextMessageContent>,
}

#[derive(Clone, Debug, Serialize)]
pub struct InputTextMessageContent {
    #[serde(rename = "type")]
    pub type_: String,
    pub text: String,
}

// Reponses API - Response
#[derive(Clone, Debug, Deserialize)]
pub struct ModelResponse {
    pub id: String,
    pub status: String,
    pub output: Vec<ModelResponseOutput>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelResponseOutput {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub status: Option<String>,
    pub content: Vec<ModelResponseOutputContent>,
    pub role: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModelResponseOutputContent {
    #[serde(rename = "type")]
    pub type_: String,
    pub text: String,
}
