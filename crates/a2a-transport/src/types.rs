// crates/a2a-transport/src/types.rs
use bytes::Bytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: Method,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Bytes>,
}

impl HttpRequest {
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: Method::Get,
            url: url.into(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn post(url: impl Into<String>, body: impl Into<Bytes>) -> Self {
        Self {
            method: Method::Post,
            url: url.into(),
            headers: Vec::new(),
            body: Some(body.into()),
        }
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Bytes,
}

impl HttpResponse {
    pub fn ok(body: impl Into<Bytes>) -> Self {
        Self {
            status: 200,
            headers: Vec::new(),
            body: body.into(),
        }
    }

    pub fn with_status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_get() {
        let req = HttpRequest::get("https://example.com");
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.url, "https://example.com");
        assert!(req.body.is_none());
    }

    #[test]
    fn test_http_request_post() {
        let req = HttpRequest::post("https://example.com", b"hello".as_slice());
        assert_eq!(req.method, Method::Post);
        assert!(req.body.is_some());
    }

    #[test]
    fn test_http_request_with_header() {
        let req =
            HttpRequest::get("https://example.com").with_header("Content-Type", "application/json");
        assert_eq!(req.headers.len(), 1);
        assert_eq!(
            req.headers[0],
            ("Content-Type".to_string(), "application/json".to_string())
        );
    }

    #[test]
    fn test_http_response_header_lookup() {
        let resp = HttpResponse::ok(b"test".as_slice())
            .with_header("Content-Type", "application/json")
            .with_header("X-Custom", "value");

        assert_eq!(resp.header("content-type"), Some("application/json"));
        assert_eq!(resp.header("Content-Type"), Some("application/json"));
        assert_eq!(resp.header("x-custom"), Some("value"));
        assert_eq!(resp.header("missing"), None);
    }

    #[test]
    fn test_method_as_str() {
        assert_eq!(Method::Get.as_str(), "GET");
        assert_eq!(Method::Post.as_str(), "POST");
        assert_eq!(Method::Put.as_str(), "PUT");
        assert_eq!(Method::Delete.as_str(), "DELETE");
    }
}
