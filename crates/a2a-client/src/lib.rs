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

    /// Send a message to the agent.
    ///
    /// Uses the negotiated binding (JSON-RPC or REST).
    pub async fn send_message(
        &self,
        params: a2a_types::MessageSendParams,
    ) -> Result<a2a_types::SendMessageResponse> {
        match &self.binding {
            SelectedBinding::JsonRpc { url } => self.send_message_jsonrpc(url, params).await,
            SelectedBinding::Rest { url } => self.send_message_rest(url, params).await,
        }
    }

    async fn send_message_jsonrpc(
        &self,
        url: &str,
        params: a2a_types::MessageSendParams,
    ) -> Result<a2a_types::SendMessageResponse> {
        let request = jsonrpc::JsonRpcRequest::new(self.next_id(), "message/send", &params);
        let body = serde_json::to_vec(&request)?;

        let http_request = HttpRequest::post(url, body)
            .with_header("Content-Type", "application/json")
            .with_header("Accept", "application/json");

        let response = self
            .transport
            .request(http_request)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;

        let rpc_response: jsonrpc::JsonRpcResponse<a2a_types::SendMessageResponse> =
            serde_json::from_slice(&response.body)?;

        match rpc_response.result {
            jsonrpc::JsonRpcResult::Success { result } => Ok(result),
            jsonrpc::JsonRpcResult::Error { error } => Err(Error::Agent {
                message: error.message.clone(),
                source: ProtocolError::JsonRpc {
                    code: JsonRpcErrorCode::from_code(error.code),
                    message: error.message,
                    data: error.data,
                },
            }),
        }
    }

    async fn send_message_rest(
        &self,
        url: &str,
        params: a2a_types::MessageSendParams,
    ) -> Result<a2a_types::SendMessageResponse> {
        let body = serde_json::to_vec(&params)?;
        let http_request = rest::send_message_request(url, body);

        let response = self
            .transport
            .request(http_request)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;

        if response.status != 200 {
            let body: Option<serde_json::Value> = serde_json::from_slice(&response.body).ok();
            return Err(Error::Agent {
                message: format!("REST error {}", response.status),
                source: ProtocolError::Rest {
                    status: response.status,
                    body,
                },
            });
        }

        let result: a2a_types::SendMessageResponse = serde_json::from_slice(&response.body)?;
        Ok(result)
    }

    /// Get a task by ID.
    ///
    /// Uses the negotiated binding (JSON-RPC or REST).
    pub async fn get_task(
        &self,
        task_id: &a2a_types::TaskId,
        history_length: Option<u32>,
    ) -> Result<Option<a2a_types::Task>> {
        match &self.binding {
            SelectedBinding::JsonRpc { url } => {
                self.get_task_jsonrpc(url, task_id, history_length).await
            }
            SelectedBinding::Rest { url } => self.get_task_rest(url, task_id, history_length).await,
        }
    }

    async fn get_task_jsonrpc(
        &self,
        url: &str,
        task_id: &a2a_types::TaskId,
        history_length: Option<u32>,
    ) -> Result<Option<a2a_types::Task>> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            id: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            history_length: Option<u32>,
        }

        let request = jsonrpc::JsonRpcRequest::new(
            self.next_id(),
            "tasks/get",
            Params {
                id: task_id.as_str(),
                history_length,
            },
        );
        let body = serde_json::to_vec(&request)?;

        let http_request = HttpRequest::post(url, body)
            .with_header("Content-Type", "application/json")
            .with_header("Accept", "application/json");

        let response = self
            .transport
            .request(http_request)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;

        let rpc_response: jsonrpc::JsonRpcResponse<Option<a2a_types::Task>> =
            serde_json::from_slice(&response.body)?;

        match rpc_response.result {
            jsonrpc::JsonRpcResult::Success { result } => Ok(result),
            jsonrpc::JsonRpcResult::Error { error } => Err(Error::Agent {
                message: error.message.clone(),
                source: ProtocolError::JsonRpc {
                    code: JsonRpcErrorCode::from_code(error.code),
                    message: error.message,
                    data: error.data,
                },
            }),
        }
    }

    async fn get_task_rest(
        &self,
        url: &str,
        task_id: &a2a_types::TaskId,
        history_length: Option<u32>,
    ) -> Result<Option<a2a_types::Task>> {
        let http_request = match history_length {
            Some(len) => rest::get_task_with_history_request(url, task_id, len),
            None => rest::get_task_request(url, task_id),
        };

        let response = self
            .transport
            .request(http_request)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;

        if response.status == 404 {
            return Ok(None);
        }

        if response.status != 200 {
            let body: Option<serde_json::Value> = serde_json::from_slice(&response.body).ok();
            return Err(Error::Agent {
                message: format!("REST error {}", response.status),
                source: ProtocolError::Rest {
                    status: response.status,
                    body,
                },
            });
        }

        let task: a2a_types::Task = serde_json::from_slice(&response.body)?;
        Ok(Some(task))
    }

    /// Cancel a task by ID.
    ///
    /// Uses the negotiated binding (JSON-RPC or REST).
    pub async fn cancel_task(
        &self,
        task_id: &a2a_types::TaskId,
    ) -> Result<Option<a2a_types::Task>> {
        match &self.binding {
            SelectedBinding::JsonRpc { url } => self.cancel_task_jsonrpc(url, task_id).await,
            SelectedBinding::Rest { url } => self.cancel_task_rest(url, task_id).await,
        }
    }

    async fn cancel_task_jsonrpc(
        &self,
        url: &str,
        task_id: &a2a_types::TaskId,
    ) -> Result<Option<a2a_types::Task>> {
        #[derive(serde::Serialize)]
        struct Params<'a> {
            id: &'a str,
        }

        let request = jsonrpc::JsonRpcRequest::new(
            self.next_id(),
            "tasks/cancel",
            Params {
                id: task_id.as_str(),
            },
        );
        let body = serde_json::to_vec(&request)?;

        let http_request = HttpRequest::post(url, body)
            .with_header("Content-Type", "application/json")
            .with_header("Accept", "application/json");

        let response = self
            .transport
            .request(http_request)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;

        let rpc_response: jsonrpc::JsonRpcResponse<Option<a2a_types::Task>> =
            serde_json::from_slice(&response.body)?;

        match rpc_response.result {
            jsonrpc::JsonRpcResult::Success { result } => Ok(result),
            jsonrpc::JsonRpcResult::Error { error } => Err(Error::Agent {
                message: error.message.clone(),
                source: ProtocolError::JsonRpc {
                    code: JsonRpcErrorCode::from_code(error.code),
                    message: error.message,
                    data: error.data,
                },
            }),
        }
    }

    async fn cancel_task_rest(
        &self,
        url: &str,
        task_id: &a2a_types::TaskId,
    ) -> Result<Option<a2a_types::Task>> {
        let http_request = rest::cancel_task_request(url, task_id);

        let response = self
            .transport
            .request(http_request)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;

        if response.status == 404 {
            return Ok(None);
        }

        if response.status != 200 {
            let body: Option<serde_json::Value> = serde_json::from_slice(&response.body).ok();
            return Err(Error::Agent {
                message: format!("REST error {}", response.status),
                source: ProtocolError::Rest {
                    status: response.status,
                    body,
                },
            });
        }

        let task: a2a_types::Task = serde_json::from_slice(&response.body)?;
        Ok(Some(task))
    }
}
