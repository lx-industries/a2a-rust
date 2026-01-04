# Client Auto-Negotiation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add auto-negotiating unified client API that discovers agents and selects protocol binding automatically.

**Architecture:** Client fetches agent card on construction, parses `additionalInterfaces` to find supported bindings, selects based on preference order (JSON-RPC > REST by default). Unified methods dispatch to the selected binding internally.

**Tech Stack:** Rust, serde, a2a-transport, a2a-types (generated AgentCard type)

---

## Task 1: Add negotiation function to binding module

**Files:**
- Modify: `crates/a2a-client/src/binding.rs`
- Test: inline `#[cfg(test)]` module

**Step 1: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use a2a_types::Binding;

    #[test]
    fn test_select_binding_prefers_jsonrpc() {
        let interfaces = vec![
            ("https://example.com/v1".to_string(), Binding::Rest),
            ("https://example.com/".to_string(), Binding::JsonRpc),
        ];
        let result = select_binding(&interfaces, &[Binding::JsonRpc, Binding::Rest]);
        assert_eq!(result, Some(SelectedBinding::JsonRpc { url: "https://example.com/".to_string() }));
    }

    #[test]
    fn test_select_binding_respects_preference() {
        let interfaces = vec![
            ("https://example.com/v1".to_string(), Binding::Rest),
            ("https://example.com/".to_string(), Binding::JsonRpc),
        ];
        let result = select_binding(&interfaces, &[Binding::Rest, Binding::JsonRpc]);
        assert_eq!(result, Some(SelectedBinding::Rest { url: "https://example.com/v1".to_string() }));
    }

    #[test]
    fn test_select_binding_no_match() {
        let interfaces = vec![
            ("https://example.com/grpc".to_string(), Binding::JsonRpc), // Pretend only gRPC
        ];
        // Empty preference means no match
        let result = select_binding(&interfaces, &[]);
        assert_eq!(result, None);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p a2a-client binding::tests --no-run 2>&1 | head -20`
Expected: Compile error - `select_binding` not found

**Step 3: Write minimal implementation**

Add to `crates/a2a-client/src/binding.rs`:

```rust
use a2a_types::Binding;

/// Select a binding from available interfaces based on preference order.
///
/// Returns the first interface that matches a binding in the preference list,
/// in preference order (not interface order).
pub fn select_binding(
    interfaces: &[(String, Binding)],
    preference: &[Binding],
) -> Option<SelectedBinding> {
    for pref in preference {
        for (url, binding) in interfaces {
            if binding == pref {
                return Some(match binding {
                    Binding::JsonRpc => SelectedBinding::JsonRpc { url: url.clone() },
                    Binding::Rest => SelectedBinding::Rest { url: url.clone() },
                });
            }
        }
    }
    None
}

/// Default binding preference order.
pub const DEFAULT_PREFERENCE: &[Binding] = &[Binding::JsonRpc, Binding::Rest];
```

Also add `#[derive(PartialEq)]` to `SelectedBinding`.

**Step 4: Run test to verify it passes**

Run: `cargo test -p a2a-client binding::tests -- --nocapture`
Expected: 3 tests pass

**Step 5: Commit**

```bash
git add crates/a2a-client/src/binding.rs
git commit -m "feat(a2a-client): add binding selection logic"
```

---

## Task 2: Add interface extraction from AgentCard

**Files:**
- Modify: `crates/a2a-client/src/binding.rs`
- Test: inline `#[cfg(test)]` module

The generated `AgentCard` has:
- `url: String` - main endpoint (assumed JSON-RPC per spec)
- `preferred_transport: Option<TransportProtocol>`
- `additional_interfaces: Option<Vec<AgentInterface>>`

We need to extract all available bindings.

**Step 1: Write the failing test**

```rust
#[test]
fn test_extract_interfaces_from_agent_card() {
    let card = a2a_types::AgentCard {
        name: "test".to_string(),
        description: "test".to_string(),
        version: "1.0".to_string(),
        url: "https://example.com/".to_string(),
        skills: vec![],
        capabilities: a2a_types::AgentCapabilities {
            streaming: None,
            push_notifications: None,
            state_transition_history: None,
        },
        preferred_transport: None,
        default_input_modes: None,
        default_output_modes: None,
        provider: None,
        security_schemes: None,
        security: None,
        additional_interfaces: Some(vec![
            a2a_types::AgentInterface {
                url: "https://example.com/v1".to_string(),
                transport: a2a_types::TransportProtocol::HttpJson,
            },
        ]),
        documentation_url: None,
        icon_url: None,
        protocol_version: None,
        supports_authenticated_extended_card: None,
        signatures: None,
    };

    let interfaces = extract_interfaces(&card);
    assert_eq!(interfaces.len(), 2); // main URL + additional
    assert!(interfaces.iter().any(|(_, b)| *b == Binding::JsonRpc));
    assert!(interfaces.iter().any(|(_, b)| *b == Binding::Rest));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p a2a-client binding::tests::test_extract --no-run 2>&1 | head -20`
Expected: Compile error - `extract_interfaces` not found

**Step 3: Write minimal implementation**

```rust
use a2a_types::{AgentCard, TransportProtocol};

/// Extract all available interfaces from an agent card.
///
/// The main `url` is treated as JSON-RPC unless `preferred_transport` says otherwise.
/// Additional interfaces are added from `additional_interfaces`.
pub fn extract_interfaces(card: &AgentCard) -> Vec<(String, Binding)> {
    let mut interfaces = Vec::new();

    // Main URL - default to JSON-RPC, or use preferred_transport
    let main_binding = card.preferred_transport
        .as_ref()
        .and_then(transport_to_binding)
        .unwrap_or(Binding::JsonRpc);
    interfaces.push((card.url.clone(), main_binding));

    // Additional interfaces
    if let Some(additional) = &card.additional_interfaces {
        for iface in additional {
            if let Some(binding) = transport_to_binding(&iface.transport) {
                interfaces.push((iface.url.clone(), binding));
            }
        }
    }

    interfaces
}

fn transport_to_binding(transport: &TransportProtocol) -> Option<Binding> {
    match transport {
        TransportProtocol::Jsonrpc => Some(Binding::JsonRpc),
        TransportProtocol::HttpJson => Some(Binding::Rest),
        TransportProtocol::Grpc => None, // Not supported
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p a2a-client binding::tests -- --nocapture`
Expected: All tests pass

**Step 5: Commit**

```bash
git add crates/a2a-client/src/binding.rs
git commit -m "feat(a2a-client): extract interfaces from AgentCard"
```

---

## Task 3: Add ClientBuilder struct

**Files:**
- Create: `crates/a2a-client/src/builder.rs`
- Modify: `crates/a2a-client/src/lib.rs` (add module)

**Step 1: Write the failing test**

```rust
// In crates/a2a-client/src/builder.rs
#[cfg(test)]
mod tests {
    use super::*;
    use a2a_types::Binding;

    // Mock transport for testing
    struct MockTransport;

    impl a2a_transport::HttpClient for MockTransport {
        fn request(&self, _req: a2a_transport::HttpRequest)
            -> impl std::future::Future<Output = Result<a2a_transport::HttpResponse, a2a_transport::Error>> + Send
        {
            async { Err(a2a_transport::Error::new("mock")) }
        }
    }

    #[test]
    fn test_builder_default_preference() {
        let builder = ClientBuilder::new(MockTransport, "https://example.com");
        assert_eq!(builder.preference, None); // Uses default
    }

    #[test]
    fn test_builder_custom_preference() {
        let builder = ClientBuilder::new(MockTransport, "https://example.com")
            .prefer(&[Binding::Rest, Binding::JsonRpc]);
        assert_eq!(builder.preference, Some(vec![Binding::Rest, Binding::JsonRpc]));
    }

    #[test]
    fn test_builder_forced_binding() {
        let builder = ClientBuilder::new(MockTransport, "https://example.com")
            .binding(Binding::Rest);
        assert_eq!(builder.forced_binding, Some(Binding::Rest));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p a2a-client builder::tests --no-run 2>&1 | head -20`
Expected: Module not found

**Step 3: Write minimal implementation**

Create `crates/a2a-client/src/builder.rs`:

```rust
//! Client builder for configuration.

use a2a_transport::HttpClient;
use a2a_types::Binding;

/// Builder for configuring client behavior.
pub struct ClientBuilder<T: HttpClient> {
    pub(crate) transport: T,
    pub(crate) base_url: String,
    pub(crate) preference: Option<Vec<Binding>>,
    pub(crate) forced_binding: Option<Binding>,
}

impl<T: HttpClient> ClientBuilder<T> {
    /// Create a new builder.
    pub fn new(transport: T, base_url: impl Into<String>) -> Self {
        Self {
            transport,
            base_url: base_url.into(),
            preference: None,
            forced_binding: None,
        }
    }

    /// Set binding preference order.
    ///
    /// The client will select the first available binding in this order.
    pub fn prefer(mut self, preference: &[Binding]) -> Self {
        self.preference = Some(preference.to_vec());
        self
    }

    /// Force a specific binding (skip negotiation).
    ///
    /// The client will fail if the agent doesn't support this binding.
    pub fn binding(mut self, binding: Binding) -> Self {
        self.forced_binding = Some(binding);
        self
    }
}
```

Add to `crates/a2a-client/src/lib.rs`:
```rust
pub mod builder;
pub use builder::ClientBuilder;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p a2a-client builder::tests -- --nocapture`
Expected: All tests pass

**Step 5: Commit**

```bash
git add crates/a2a-client/src/builder.rs crates/a2a-client/src/lib.rs
git commit -m "feat(a2a-client): add ClientBuilder struct"
```

---

## Task 4: Refactor Client struct to hold AgentCard and SelectedBinding

**Files:**
- Modify: `crates/a2a-client/src/lib.rs`

This is the breaking change - Client now requires discovery before use.

**Step 1: Update Client struct**

```rust
use a2a_types::AgentCard;
use binding::SelectedBinding;

/// A2A client for communicating with A2A agents.
pub struct Client<T: HttpClient> {
    transport: T,
    agent_card: AgentCard,
    binding: SelectedBinding,
    request_id: AtomicU64,
}

impl<T: HttpClient> Client<T> {
    /// Get the cached agent card.
    pub fn agent_card(&self) -> &AgentCard {
        &self.agent_card
    }

    /// Get the selected binding.
    pub fn binding(&self) -> &SelectedBinding {
        &self.binding
    }

    /// Get the next request ID (for JSON-RPC).
    fn next_id(&self) -> String {
        self.request_id.fetch_add(1, Ordering::SeqCst).to_string()
    }
}
```

**Step 2: Add async connect method to ClientBuilder**

```rust
impl<T: HttpClient> ClientBuilder<T> {
    /// Build the client by discovering the agent and selecting a binding.
    pub async fn build(self) -> Result<Client<T>> {
        // Fetch agent card
        let agent_card = discover_agent(&self.transport, &self.base_url).await?;

        // Extract available interfaces
        let interfaces = binding::extract_interfaces(&agent_card);

        // Select binding
        let binding = if let Some(forced) = self.forced_binding {
            // Forced binding - must be available
            interfaces.iter()
                .find(|(_, b)| *b == forced)
                .map(|(url, b)| match b {
                    Binding::JsonRpc => SelectedBinding::JsonRpc { url: url.clone() },
                    Binding::Rest => SelectedBinding::Rest { url: url.clone() },
                })
                .ok_or_else(|| Error::NoCompatibleBinding {
                    available: interfaces.iter().map(|(_, b)| *b).collect(),
                })?
        } else {
            let pref = self.preference.as_deref()
                .unwrap_or(binding::DEFAULT_PREFERENCE);
            binding::select_binding(&interfaces, pref)
                .ok_or_else(|| Error::NoCompatibleBinding {
                    available: interfaces.iter().map(|(_, b)| *b).collect(),
                })?
        };

        Ok(Client {
            transport: self.transport,
            agent_card,
            binding,
            request_id: AtomicU64::new(1),
        })
    }
}

async fn discover_agent<T: HttpClient>(transport: &T, base_url: &str) -> Result<AgentCard> {
    let url = format!(
        "{}/.well-known/agent-card.json",
        base_url.trim_end_matches('/')
    );
    let request = HttpRequest::get(&url).with_header("Accept", "application/json");

    let response = transport
        .request(request)
        .await
        .map_err(|e| Error::Transport(e.to_string()))?;

    if response.status != 200 {
        return Err(Error::AgentNotFound(url));
    }

    let agent_card: AgentCard = serde_json::from_slice(&response.body)?;
    Ok(agent_card)
}
```

**Step 3: Add convenience constructor**

```rust
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
}
```

**Step 4: Run tests**

Run: `cargo test -p a2a-client -- --nocapture`
Expected: Tests compile (existing tests may need updates)

**Step 5: Commit**

```bash
git add crates/a2a-client/src/lib.rs crates/a2a-client/src/builder.rs
git commit -m "refactor(a2a-client): Client requires discovery on construction"
```

---

## Task 5: Add unified send_message method

**Files:**
- Modify: `crates/a2a-client/src/lib.rs`

**Step 1: Add send_message that dispatches based on binding**

```rust
impl<T: HttpClient> Client<T> {
    /// Send a message to the agent.
    ///
    /// Uses the negotiated binding (JSON-RPC or REST).
    pub async fn send_message(
        &self,
        params: a2a_types::MessageSendParams,
    ) -> Result<a2a_types::SendMessageResponse> {
        match &self.binding {
            SelectedBinding::JsonRpc { url } => {
                self.send_message_jsonrpc(url, params).await
            }
            SelectedBinding::Rest { url } => {
                self.send_message_rest(url, params).await
            }
        }
    }

    async fn send_message_jsonrpc(
        &self,
        url: &str,
        params: a2a_types::MessageSendParams,
    ) -> Result<a2a_types::SendMessageResponse> {
        let request = jsonrpc::JsonRpcRequest::new(
            self.next_id(),
            "message/send",
            params,
        );
        let body = serde_json::to_vec(&request)?;

        let http_request = HttpRequest::post(url, body)
            .with_header("Content-Type", "application/json")
            .with_header("Accept", "application/json");

        let response = self.transport
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

        let response = self.transport
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
}
```

**Step 2: Run tests**

Run: `cargo test -p a2a-client -- --nocapture`

**Step 3: Commit**

```bash
git add crates/a2a-client/src/lib.rs
git commit -m "feat(a2a-client): add unified send_message method"
```

---

## Task 6: Add unified get_task method

**Files:**
- Modify: `crates/a2a-client/src/lib.rs`

**Step 1: Add get_task that dispatches based on binding**

```rust
impl<T: HttpClient> Client<T> {
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
            SelectedBinding::Rest { url } => {
                self.get_task_rest(url, task_id, history_length).await
            }
        }
    }

    async fn get_task_jsonrpc(
        &self,
        url: &str,
        task_id: &a2a_types::TaskId,
        history_length: Option<u32>,
    ) -> Result<Option<a2a_types::Task>> {
        #[derive(serde::Serialize)]
        struct Params<'a> {
            id: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            history_length: Option<u32>,
        }

        let request = jsonrpc::JsonRpcRequest::new(
            self.next_id(),
            "tasks/get",
            Params { id: task_id.as_str(), history_length },
        );
        let body = serde_json::to_vec(&request)?;

        let http_request = HttpRequest::post(url, body)
            .with_header("Content-Type", "application/json")
            .with_header("Accept", "application/json");

        let response = self.transport
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

        let response = self.transport
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
```

**Step 2: Run tests**

Run: `cargo test -p a2a-client -- --nocapture`

**Step 3: Commit**

```bash
git add crates/a2a-client/src/lib.rs
git commit -m "feat(a2a-client): add unified get_task method"
```

---

## Task 7: Add unified cancel_task method

**Files:**
- Modify: `crates/a2a-client/src/lib.rs`

**Step 1: Add cancel_task that dispatches based on binding**

```rust
impl<T: HttpClient> Client<T> {
    /// Cancel a task by ID.
    ///
    /// Uses the negotiated binding (JSON-RPC or REST).
    pub async fn cancel_task(
        &self,
        task_id: &a2a_types::TaskId,
    ) -> Result<Option<a2a_types::Task>> {
        match &self.binding {
            SelectedBinding::JsonRpc { url } => {
                self.cancel_task_jsonrpc(url, task_id).await
            }
            SelectedBinding::Rest { url } => {
                self.cancel_task_rest(url, task_id).await
            }
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
            Params { id: task_id.as_str() },
        );
        let body = serde_json::to_vec(&request)?;

        let http_request = HttpRequest::post(url, body)
            .with_header("Content-Type", "application/json")
            .with_header("Accept", "application/json");

        let response = self.transport
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

        let response = self.transport
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
```

**Step 2: Run tests**

Run: `cargo test -p a2a-client -- --nocapture`

**Step 3: Commit**

```bash
git add crates/a2a-client/src/lib.rs
git commit -m "feat(a2a-client): add unified cancel_task method"
```

---

## Task 8: Update integration tests

**Files:**
- Modify: `crates/a2a-wasm-component/tests/integration_test.rs`

The integration tests use the old `Client::new` sync API. Update to use `Client::connect`.

**Step 1: Check what tests exist and update imports**

Review existing tests and update:
- `Client::new(transport, url)` → `Client::connect(transport, url).await?`
- Or use `Client::builder(transport, url).binding(Binding::JsonRpc).build().await?` to force JSON-RPC

**Step 2: Run integration tests**

Run: `cargo test -p a2a-wasm-component --test integration_test -- --nocapture`

**Step 3: Fix any failures**

The helloworld agent may need to serve `/.well-known/agent-card.json` properly. Check fixtures if tests fail.

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/tests/
git commit -m "test(a2a-wasm-component): update integration tests for new client API"
```

---

## Task 9: Clean up old discover method and rpc method

**Files:**
- Modify: `crates/a2a-client/src/lib.rs`

**Step 1: Deprecate or remove old methods**

The old `discover()` and `rpc()` methods are now internal. Either:
1. Make them private
2. Mark deprecated with `#[deprecated]`
3. Keep `rpc()` public for advanced use cases

Recommendation: Keep `rpc()` public for flexibility, remove standalone `discover()`.

**Step 2: Run all tests**

Run: `cargo test --workspace`

**Step 3: Commit**

```bash
git add crates/a2a-client/src/lib.rs
git commit -m "refactor(a2a-client): clean up old client API"
```

---

## Task 10: Run full test suite and clippy

**Files:** None (verification only)

**Step 1: Run tests**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 2: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings

**Step 3: Run fmt**

Run: `cargo fmt`

**Step 4: Commit any fixes**

```bash
git add -A
git commit -m "style: apply formatting and fix clippy warnings"
```

---

## Summary

After completing all tasks:

1. **Client construction** now discovers the agent:
   ```rust
   let client = Client::connect(transport, "https://agent.example.com").await?;
   ```

2. **Builder pattern** for customization:
   ```rust
   let client = Client::builder(transport, url)
       .prefer(&[Binding::Rest, Binding::JsonRpc])
       .build()
       .await?;
   ```

3. **Unified API** abstracts protocol:
   ```rust
   let response = client.send_message(params).await?;
   let task = client.get_task(&task_id, None).await?;
   let task = client.cancel_task(&task_id).await?;
   let card = client.agent_card();
   ```

4. **Breaking change**: `Client::new()` is replaced by `Client::connect()` (async).
