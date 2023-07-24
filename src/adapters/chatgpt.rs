use async_trait::async_trait;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::{debug, trace};
use uuid::Uuid;

use super::Adapter;

pub struct Chatgpt {
    pub access_token: String,
    pub url: String,
    pub model_name: String,
    pub debug: bool,
    pub headers: HeaderMap<HeaderValue>,
    pub last_message_id: Option<String>,
    pub conversation_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Event {
    pub message: Message,
    pub conversation_id: String,
    pub error: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct Message {
    pub id: String,
    pub content: Content,
    pub author: Author,
}

#[derive(Debug, Deserialize)]
struct Author {
    pub role: AuthorRole,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum AuthorRole {
    Assistant,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
struct Content {
    pub parts: Vec<String>,
}

impl Chatgpt {
    pub fn new(access_token: String, url: Option<String>) -> Self {
        Self {
            access_token,
            url: url.unwrap_or_else(|| "https://ai.fakeopen.com/api/conversation".to_string()),
            model_name: "text-davinci-002-render-sha".to_string(),
            debug: false,
            headers: HeaderMap::new(),
            last_message_id: None,
            conversation_id: None,
        }
    }
}

#[async_trait]
impl Adapter for Chatgpt {
    async fn generate(
        &mut self,
        prompt: &str,
        progress_hook: &mut (impl FnMut(String) + Send),
    ) -> Result<String, String> {
        let parent_message_id = self
            .last_message_id
            .to_owned()
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let message_id = Uuid::new_v4().to_string();
        let mut body = json!({
            "action": "next",
            "messages": [
                {
                    "id": message_id,
                    "role": "user",
                    "content": {
                        "content_type": "text",
                        "parts": [prompt]
                    }
                }
            ],
            "model": self.model_name,
            "parent_message_id": parent_message_id,
        });

        if let Some(conv_id) = self.conversation_id.to_owned() {
            body.as_object_mut()
                .unwrap()
                .insert("conversation_id".to_string(), json!(conv_id));
        }

        let client = reqwest::Client::new();
        let mut stream = client
            .post(&self.url)
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token),
            )
            .header(header::ACCEPT, "text/event-stream")
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await
            .unwrap()
            .bytes_stream()
            .eventsource();

        let mut message: Option<Vec<String>> = None;
        let mut id: Option<String> = None;
        let mut conversation_id: Option<String> = None;
        while let Some(event) = stream.next().await {
            let event = event.unwrap();
            trace!("Event: {event:?}");
            if event.data == "[DONE]" {
                debug!("Generation done");
                continue;
            }
            let Ok(json) = serde_json::from_str::<Event>(&event.data) else {
                debug!("Failed to parse event data, ignoring");
                continue;
            };
            if let Some(err) = json.error {
                debug!("Recieved error field, stopping");
                return Err(err.to_string());
            }
            if json.message.author.role != AuthorRole::Assistant {
                continue;
            }
            progress_hook(json.message.content.parts.join(" "));
            message = Some(json.message.content.parts);
            id = Some(json.message.id);
            conversation_id = Some(json.conversation_id);
        }

        debug!("Connection ended");

        self.last_message_id = id;
        self.conversation_id = conversation_id;

        Ok(message.unwrap().join(" "))
    }
}
