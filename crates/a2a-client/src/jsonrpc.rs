// crates/a2a-client/src/jsonrpc.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest<T> {
    pub jsonrpc: &'static str,
    pub id: String,
    pub method: String,
    pub params: T,
}

impl<T> JsonRpcRequest<T> {
    pub fn new(id: impl Into<String>, method: impl Into<String>, params: T) -> Self {
        Self {
            jsonrpc: "2.0",
            id: id.into(),
            method: method.into(),
            params,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: String,
    #[serde(flatten)]
    pub result: JsonRpcResult<T>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResult<T> {
    Success { result: T },
    Error { error: JsonRpcError },
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = JsonRpcRequest::new("1", "SendMessage", serde_json::json!({"content": "hello"}));
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"method\":\"SendMessage\""));
    }

    #[test]
    fn test_deserialize_success_response() {
        let json = r#"{"jsonrpc":"2.0","id":"1","result":{"status":"ok"}}"#;
        let resp: JsonRpcResponse<Value> = serde_json::from_str(json).unwrap();
        assert!(matches!(resp.result, JsonRpcResult::Success { .. }));
    }

    #[test]
    fn test_deserialize_error_response() {
        let json =
            r#"{"jsonrpc":"2.0","id":"1","error":{"code":-32001,"message":"Task not found"}}"#;
        let resp: JsonRpcResponse<Value> = serde_json::from_str(json).unwrap();
        assert!(matches!(resp.result, JsonRpcResult::Error { .. }));
    }
}
