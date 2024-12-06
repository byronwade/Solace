use crate::ipc::{IPCCommand, IPCHandler};
use axum::{
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub fn router<H: IPCHandler + 'static>(handler: Arc<H>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/command", post(handle_command::<H>))
        .route("/api/subscribe", get(subscribe::<H>))
        .layer(cors)
        .with_state(handler)
}

async fn handle_command<H: IPCHandler>(
    State(handler): State<Arc<H>>,
    Json(command): Json<IPCCommand>,
) -> Json<serde_json::Value> {
    match handler.handle_command(command).await {
        Ok(response) => Json(serde_json::to_value(response).unwrap()),
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": {
                "code": "INTERNAL_ERROR",
                "message": e.to_string(),
            }
        })),
    }
}

async fn subscribe<H: IPCHandler>(
    State(handler): State<Arc<H>>,
) -> Sse<impl futures::Stream<Item = Result<Event, std::convert::Infallible>>> {
    let mut rx = handler.subscribe().await;
    
    let stream = async_stream::stream! {
        while let Ok(event) = rx.recv().await {
            let json = serde_json::to_string(&event).unwrap();
            yield Ok(Event::default().data(json));
        }
    };
    
    Sse::new(stream)
} 