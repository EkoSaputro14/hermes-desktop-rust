//! Asynchronous SSE streaming client for Hermes Agent (11+ LLM providers)
//! Uses `reqwest` + `futures::StreamExt` to stream Server-Sent Events directly into Iced state.

use std::time::Instant;
use futures::StreamExt;
use reqwest::Client;
use serde_json::json;

use crate::models::{AgentProfile, ChatMessage, CompletionReply, MemoryEntry};

pub async fn stream_agent_completion(
    profile: AgentProfile,
    history: Vec<ChatMessage>,
    memories: Vec<MemoryEntry>,
    user_prompt: String,
) -> Result<CompletionReply, String> {
    let started_at = Instant::now();
    let client = Client::builder()
        .use_rustls_tls()
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    // Synthesize Honcho & local SQLite memory context into system prompt
    let memory_context = memories
        .iter()
        .take(8)
        .map(|m| format!("- [{}] {}", m.category.as_str(), m.content))
        .collect::<Vec<_>>()
        .join("\n");

    let full_system_prompt = format!(
        "{}\n\n[Active Hermes Memory Context]\n{}",
        profile.system_prompt, memory_context
    );

    let mut messages_payload = vec![json!({
        "role": "system",
        "content": full_system_prompt
    })];

    for msg in history.iter().rev().take(16).rev() {
        messages_payload.push(json!({
            "role": msg.role,
            "content": msg.content
        }));
    }
    messages_payload.push(json!({
        "role": "user",
        "content": user_prompt
    }));

    let body = json!({
        "model": profile.model_name,
        "messages": messages_payload,
        "stream": true
    });

    let response = client
        .post(&profile.endpoint_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("SSE connection failed ({}): {}", profile.endpoint_url, e))?;

    let mut byte_stream = response.bytes_stream();
    let mut accumulated = String::new();
    let mut token_count = 0usize;

    while let Some(chunk_res) = byte_stream.next().await {
        let chunk = chunk_res.map_err(|e| format!("Stream read error: {}", e))?;
        let text = String::from_utf8_lossy(&chunk);

        for line in text.lines() {
            if let Some(data) = line.strip_prefix("data: ") {
                if data.trim() == "[DONE]" {
                    break;
                }
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(data) {
                    if let Some(delta) = parsed["choices"][0]["delta"]["content"].as_str() {
                        accumulated.push_str(delta);
                        token_count += 1;
                    }
                }
            }
        }
    }

    Ok(CompletionReply {
        content: accumulated,
        token_count,
        latency_ms: started_at.elapsed().as_millis() as u64,
    })
}
