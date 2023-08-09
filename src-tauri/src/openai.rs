use reqwest::{
    header::{self, HeaderMap},
    Client, StatusCode,
};
use serde::{de::DeserializeOwned, ser::Serialize};

const OPENAI_KEY: &str = "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";
const BASE_URL: &str = "https://api.openai.com/v1";

async fn request<T, B>(endpoint: String, data: Option<B>) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
    B: Serialize,
{
    let client = Client::new();

    let url = format!("{}{}", BASE_URL, endpoint);

    let mut header_map: HeaderMap = HeaderMap::new();
    header_map.insert(
        header::AUTHORIZATION,
        format!("Bearer {}", OPENAI_KEY).parse().unwrap(),
    );
    header_map.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    let mut request = client.post(url).headers(header_map);

    if let Some(data) = data {
        request = request.json(&data)
    }

    let response = request.send().await;

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    let content = response.unwrap().json::<T>().await;

    match content {
        Ok(c) => Ok(c),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub mod chat {
    use std::{process::exit, thread, time::Duration};

    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    use crate::database::Message as DB_MESSAGE;

    use super::request;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct FunctionCall {
        name: String,
        arguments: Value,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct Message {
        role: String,
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        function_call: Option<FunctionCall>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct FunctionParameters {
        #[serde(rename = "type")]
        object_type: String,
        properties: Value,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Function {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        parameters: FunctionParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ChatCompletionRequest {
        model: String,
        messages: Vec<Message>,
        #[serde(skip_serializing_if = "Option::is_none")]
        functions: Option<Vec<Function>>,
        function_call: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Choice {
        message: Message,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ChatCompletionResponse {
        choices: Vec<Choice>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChatCompletion {
        messages: Vec<Message>,
    }

    impl From<Vec<DB_MESSAGE>> for ChatCompletion {
        fn from(messages: Vec<DB_MESSAGE>) -> Self {
            let messages_mapped = messages
                .into_iter()
                .map(|message| Message {
                    role: message.role,
                    content: Some(message.content),
                    name: message.name,
                    function_call: None,
                })
                .collect();
            ChatCompletion {
                messages: messages_mapped,
            }
        }
    }

    impl ChatCompletion {
        pub fn new() -> Self {
            ChatCompletion {
                messages: Vec::new(),
            }
        }

        #[allow(dead_code)]
        pub async fn completion(&mut self, message: String) -> Result<String, StatusCode> {
            let endpoint = String::from("/chat/completions");

            self.messages.push(Message {
                role: String::from("user"),
                content: Some(message),
                name: None,
                function_call: None,
            });

            let data = ChatCompletionRequest {
                model: String::from("gpt-3.5-turbo"),
                messages: self.messages.to_vec(),
                functions: Some(vec![Function {
                    name: "end_chat".to_string(),
                    description: Some("end current chat".to_string()),
                    parameters: FunctionParameters {
                        object_type: "object".to_string(),
                        properties: json!({}),
                    },
                }]),
                function_call: Some(String::from("auto")),
            };

            let request =
                request::<ChatCompletionResponse, ChatCompletionRequest>(endpoint, Some(data));
            let response = request.await?;

            let message = response.choices.last().unwrap().message.to_owned();

            self.messages.push(message.to_owned());

            if let Some(function_to_call) = message.function_call {
                match function_to_call.name.as_str() {
                    "end_chat" => {
                        println!("Quitting chat...");
                        thread::sleep(Duration::from_millis(1));
                        exit(0);
                    }
                    _ => {
                        panic!("Function not found")
                    }
                }
            }

            Ok(message.content.unwrap())
        }
    }
}
