//! Run with
//!
//! ```not_rust
//! cargo run -p example-reqwest-response
//! ```
use std::time::Duration;

use super::sse_chat_enricher::{enriched_chat, GenerationEvent};
use crate::auth::Authentication;
use crate::CustomError;
use axum::response::{sse::Event, Sse};
use axum::Extension;
use db::{queries, Pool};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    RequestBuilder,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use super::api_chat_stream::{Completion, Message};
use super::UICompletions;

// Called from the front end to generate a streaming chat with the model
pub async fn chat_generate(
    UICompletions { chat_id }: UICompletions,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>>, CustomError> {
    let request = create_request(&pool, current_user, chat_id).await?;

    // Create a channel for sending SSE events
    let (sender, receiver) = mpsc::channel::<Result<GenerationEvent, axum::Error>>(10);

    // Spawn a task that generates SSE events and sends them into the channel
    tokio::spawn(async move {
        // Call your existing function to start generating events
        if let Err(e) = enriched_chat(request, sender, true).await {
            eprintln!("Error generating SSE stream: {:?}", e);
        }
    });

    let receiver_stream = ReceiverStream::new(receiver);

    let event_stream = receiver_stream.map(|item| {
        match item {
            Ok(event) => match event {
                GenerationEvent::Text(text) => Ok(Event::default().data(text)),
                GenerationEvent::End(text) => Ok(Event::default().data(text)),
            },
            Err(e) => {
                // Handle error without altering the accumulator
                Err(axum::Error::new(e))
            }
        }
    });

    Ok(Sse::new(event_stream))
}

// Create the request that we'll send to reqwest to create an SSE stream of incoming
// chat completions.
async fn create_request(
    pool: &Pool,
    current_user: Authentication,
    chat_id: i32,
) -> Result<RequestBuilder, CustomError> {
    let mut db_client = pool.get().await?;
    let transaction = db_client.transaction().await?;
    db::authz::set_row_level_security_user_id(&transaction, current_user.sub).await?;
    let model = queries::models::model_host_by_chat_id()
        .bind(&transaction, &chat_id)
        .one()
        .await?;
    let chat = queries::chats::chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;
    let conversation = queries::conversations::get_conversation_from_chat()
        .bind(&transaction, &chat_id)
        .one()
        .await?;
    let prompt = queries::prompts::prompt()
        .bind(&transaction, &chat.prompt_id, &conversation.team_id)
        .one()
        .await?;
    let chat_request = Message {
        role: "user".to_string(),
        content: chat.user_request,
    };
    let messages = super::prompt::execute_prompt(
        &transaction,
        prompt.id,
        conversation.team_id,
        Some(conversation.id),
        vec![chat_request],
    )
    .await?;
    let json_messages = serde_json::to_string(&messages)?;
    queries::chats::update_prompt()
        .bind(&transaction, &json_messages, &chat_id)
        .await?;
    transaction.commit().await?;
    let completion = Completion {
        model: model.name,
        stream: Some(true),
        max_tokens: Some(prompt.max_tokens),
        temperature: prompt.temperature,
        messages,
    };
    let completion_json = serde_json::to_string(&completion)?;
    let client = reqwest::Client::new();
    let request = if let Some(api_key) = model.api_key {
        client
            .post(format!("{}/chat/completions", model.base_url))
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .timeout(Duration::from_secs(5))
            .body(completion_json.to_string())
    } else {
        client
            .post(format!("{}/chat/completions", model.base_url))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .timeout(Duration::from_secs(5))
            .body(completion_json.to_string())
    };
    Ok(request)
}
