// crates/a2a-client/src/lib.rs
//! A2A protocol client.

pub mod error;
pub mod jsonrpc;
pub mod sse;

pub use error::{Error, Result};

use a2a_transport::{HttpClient, HttpRequest};
use jsonrpc::{JsonRpcRequest, JsonRpcResponse, JsonRpcResult};
use std::sync::atomic::{AtomicU64, Ordering};

/// A2A client for communicating with A2A agents.
pub struct Client<T: HttpClient> {
    transport: T,
    base_url: String,
    request_id: AtomicU64,
}

impl<T: HttpClient> Client<T> {
    /// Create a new client with the given transport and base URL.
    pub fn new(transport: T, base_url: impl Into<String>) -> Self {
        Self {
            transport,
            base_url: base_url.into(),
            request_id: AtomicU64::new(1),
        }
    }

    /// Get the next request ID.
    fn next_id(&self) -> String {
        self.request_id.fetch_add(1, Ordering::SeqCst).to_string()
    }

    /// Discover an agent by fetching its agent card.
    pub async fn discover(&self) -> Result<serde_json::Value> {
        let url = format!("{}/.well-known/agent.json", self.base_url.trim_end_matches('/'));
        let request = HttpRequest::get(&url)
            .with_header("Accept", "application/json");

        let response = self.transport.request(request).await
            .map_err(|e| Error::Transport(e.to_string()))?;

        if response.status != 200 {
            return Err(Error::AgentNotFound(url));
        }

        let agent_card: serde_json::Value = serde_json::from_slice(&response.body)?;
        Ok(agent_card)
    }

    /// Send a JSON-RPC request to the agent.
    pub async fn rpc<P, R>(&self, method: &str, params: P) -> Result<R>
    where
        P: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let request = JsonRpcRequest::new(self.next_id(), method, params);
        let body = serde_json::to_vec(&request)?;

        let url = format!("{}/", self.base_url.trim_end_matches('/'));
        let http_request = HttpRequest::post(&url, body)
            .with_header("Content-Type", "application/json")
            .with_header("Accept", "application/json");

        let response = self.transport.request(http_request).await
            .map_err(|e| Error::Transport(e.to_string()))?;

        let rpc_response: JsonRpcResponse<R> = serde_json::from_slice(&response.body)?;

        match rpc_response.result {
            JsonRpcResult::Success { result } => Ok(result),
            JsonRpcResult::Error { error } => Err(Error::JsonRpc {
                code: error.code,
                message: error.message,
                data: error.data,
            }),
        }
    }
}
