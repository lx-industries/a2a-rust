// crates/a2a-client/src/lib.rs
//! A2A protocol client.

pub mod binding;
pub mod builder;
pub mod error;
pub mod jsonrpc;
pub mod rest;
pub mod sse;

pub use builder::ClientBuilder;
pub use error::{Error, JsonRpcErrorCode, ParamError, ProtocolError, Result};

use a2a_transport::{HttpClient, HttpRequest};
use a2a_types::AgentCard;
use binding::SelectedBinding;
use jsonrpc::{JsonRpcRequest, JsonRpcResponse, JsonRpcResult};
use std::sync::atomic::{AtomicU64, Ordering};

/// A2A client for communicating with A2A agents.
pub struct Client<T: HttpClient> {
    transport: T,
    agent_card: AgentCard,
    binding: SelectedBinding,
    request_id: AtomicU64,
}

impl<T: HttpClient> Client<T> {
    /// Create a new client by discovering the agent.
    ///
    /// Uses default binding preference (JSON-RPC > REST).
    pub async fn connect(transport: T, base_url: impl Into<String>) -> Result<Self> {
        ClientBuilder::new(transport, base_url).build().await
    }

    /// Create a builder for custom configuration.
    pub fn builder(transport: T, base_url: impl Into<String>) -> ClientBuilder<T> {
        ClientBuilder::new(transport, base_url)
    }

    /// Get the cached agent card.
    pub fn agent_card(&self) -> &AgentCard {
        &self.agent_card
    }

    /// Get the selected binding.
    pub fn binding(&self) -> &SelectedBinding {
        &self.binding
    }

    /// Get the next request ID.
    fn next_id(&self) -> String {
        self.request_id.fetch_add(1, Ordering::SeqCst).to_string()
    }

    /// Send a JSON-RPC request to the agent.
    pub async fn rpc<P, R>(&self, method: &str, params: P) -> Result<R>
    where
        P: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let request = JsonRpcRequest::new(self.next_id(), method, params);
        let body = serde_json::to_vec(&request)?;

        let url = format!("{}/", self.binding.url().trim_end_matches('/'));
        let http_request = HttpRequest::post(&url, body)
            .with_header("Content-Type", "application/json")
            .with_header("Accept", "application/json");

        let response = self
            .transport
            .request(http_request)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;

        let rpc_response: JsonRpcResponse<R> = serde_json::from_slice(&response.body)?;

        match rpc_response.result {
            JsonRpcResult::Success { result } => Ok(result),
            JsonRpcResult::Error { error } => Err(Error::Agent {
                message: error.message.clone(),
                source: ProtocolError::JsonRpc {
                    code: JsonRpcErrorCode::from_code(error.code),
                    message: error.message,
                    data: error.data,
                },
            }),
        }
    }
}
