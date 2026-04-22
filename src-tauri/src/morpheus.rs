use crate::error::{AppError, AppResult};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter};

pub const MORPHEUS_BASE: &str = "https://api.mor.org/api/v1";
pub const DEFAULT_MODEL: &str = "kimi-k2.6";
pub const DEFAULT_MODEL_WEB: &str = "kimi-k2.6:web";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub tee: bool,
    pub web: bool,
    pub description: String,
}

pub fn curated_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "kimi-k2.6".into(),
            name: "Kimi 2.6 (default)".into(),
            tee: false,
            web: false,
            description: "Moonshot Kimi 2.6 — high-capacity reasoning, recommended default.".into(),
        },
        ModelInfo {
            id: "kimi-k2.6:web".into(),
            name: "Kimi 2.6 + web".into(),
            tee: false,
            web: true,
            description: "Kimi 2.6 with web tool-use; useful for cross-referencing fresh literature.".into(),
        },
        ModelInfo {
            id: "mistral-31-24b:tee".into(),
            name: "Mistral 3.1 24B (TEE)".into(),
            tee: true,
            web: false,
            description: "Trusted-Execution-Environment inference. Prompts + genomic context stay inside an attested enclave end-to-end.".into(),
        },
        ModelInfo {
            id: "hermes-3-llama-3.1-405b".into(),
            name: "Hermes 3 405B".into(),
            tee: false,
            web: false,
            description: "Nous Hermes 3 (Llama 3.1 405B) — strong instruction-following, good agent backbone.".into(),
        },
        ModelInfo {
            id: "kimi-k2-thinking".into(),
            name: "Kimi K2 Thinking".into(),
            tee: false,
            web: false,
            description: "Extended chain-of-thought variant — slower but more deliberate.".into(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

pub async fn chat_once(api_key: &str, req: &ChatRequest) -> AppResult<String> {
    if api_key.is_empty() {
        return Err(AppError::MissingApiKey);
    }
    let client = reqwest::Client::builder()
        .user_agent("personal-genomics-tauri/0.1")
        .build()?;
    let resp = client
        .post(format!("{}/chat/completions", MORPHEUS_BASE))
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&ChatRequest { stream: false, ..req.clone() })
        .send()
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::Api(format!("{} — {}", status, body)));
    }
    let v: Value = resp.json().await?;
    let content = v
        .get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .unwrap_or("")
        .to_string();
    Ok(content)
}

pub async fn chat_stream(
    app: &AppHandle,
    event: &str,
    api_key: &str,
    req: &ChatRequest,
) -> AppResult<String> {
    if api_key.is_empty() {
        return Err(AppError::MissingApiKey);
    }
    let client = reqwest::Client::builder()
        .user_agent("personal-genomics-tauri/0.1")
        .build()?;
    let mut req = req.clone();
    req.stream = true;
    let resp = client
        .post(format!("{}/chat/completions", MORPHEUS_BASE))
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .header("Accept", "text/event-stream")
        .json(&req)
        .send()
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::Api(format!("{} — {}", status, body)));
    }
    let mut stream = resp.bytes_stream();
    let mut buf = String::new();
    let mut full = String::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        buf.push_str(&String::from_utf8_lossy(&chunk));
        while let Some(idx) = buf.find('\n') {
            let line = buf[..idx].to_string();
            buf.drain(..=idx);
            let line = line.trim_end_matches('\r');
            if let Some(data) = line.strip_prefix("data: ") {
                if data.trim() == "[DONE]" {
                    let _ = app.emit(event, json!({"delta": "", "done": true}));
                    return Ok(full);
                }
                if let Ok(v) = serde_json::from_str::<Value>(data) {
                    if let Some(delta) = v
                        .pointer("/choices/0/delta/content")
                        .and_then(|c| c.as_str())
                    {
                        full.push_str(delta);
                        let _ = app.emit(event, json!({"delta": delta, "done": false}));
                    }
                }
            }
        }
    }
    let _ = app.emit(event, json!({"delta": "", "done": true}));
    Ok(full)
}
