// crates/a2a-server/src/handler.rs
use futures_core::Stream;
use std::future::Future;

/// Context for a request.
#[derive(Debug, Clone, Default)]
pub struct RequestContext {
    pub task_id: Option<String>,
    pub context_id: Option<String>,
}

/// User-implemented handler for agent logic.
pub trait AgentHandler: Send + Sync {
    /// Error type for this handler.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Stream type for streaming responses.
    type EventStream: Stream<Item = serde_json::Value> + Send + Unpin;

    /// Handle a non-streaming message.
    fn handle_message(
        &self,
        message: serde_json::Value,
        context: RequestContext,
    ) -> impl Future<Output = Result<serde_json::Value, Self::Error>> + Send;

    /// Handle a streaming message.
    fn handle_message_stream(
        &self,
        message: serde_json::Value,
        context: RequestContext,
    ) -> impl Future<Output = Result<Self::EventStream, Self::Error>> + Send;

    /// Handle task cancellation.
    fn handle_cancel(
        &self,
        task_id: &str,
    ) -> impl Future<Output = Result<serde_json::Value, Self::Error>> + Send;
}
