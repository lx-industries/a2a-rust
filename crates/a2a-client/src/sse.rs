// crates/a2a-client/src/sse.rs
//! Server-Sent Events (SSE) parser.

use bytes::Bytes;

#[derive(Debug, Clone, PartialEq)]
pub struct SseEvent {
    pub event: Option<String>,
    pub data: String,
    pub id: Option<String>,
}

pub struct SseParser {
    buffer: String,
}

impl SseParser {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    /// Feed bytes into the parser and return any complete events.
    pub fn feed(&mut self, chunk: &Bytes) -> Vec<SseEvent> {
        let text = String::from_utf8_lossy(chunk);
        self.buffer.push_str(&text);

        let mut events = Vec::new();

        // Split on double newlines (event boundaries)
        while let Some(pos) = self.buffer.find("\n\n") {
            let event_text = self.buffer[..pos].to_string();
            self.buffer = self.buffer[pos + 2..].to_string();

            if let Some(event) = Self::parse_event(&event_text) {
                events.push(event);
            }
        }

        events
    }

    fn parse_event(text: &str) -> Option<SseEvent> {
        let mut event = None;
        let mut data_lines = Vec::new();
        let mut id = None;

        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("event:") {
                event = Some(rest.trim().to_string());
            } else if let Some(rest) = line.strip_prefix("data:") {
                data_lines.push(rest.trim().to_string());
            } else if let Some(rest) = line.strip_prefix("id:") {
                id = Some(rest.trim().to_string());
            }
        }

        if data_lines.is_empty() {
            return None;
        }

        Some(SseEvent {
            event,
            data: data_lines.join("\n"),
            id,
        })
    }
}

impl Default for SseParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_event() {
        let mut parser = SseParser::new();
        let events = parser.feed(&Bytes::from("data: hello world\n\n"));
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].data, "hello world");
        assert!(events[0].event.is_none());
    }

    #[test]
    fn test_parse_event_with_type() {
        let mut parser = SseParser::new();
        let events = parser.feed(&Bytes::from("event: message\ndata: hello\n\n"));
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, Some("message".to_string()));
        assert_eq!(events[0].data, "hello");
    }

    #[test]
    fn test_parse_multiline_data() {
        let mut parser = SseParser::new();
        let events = parser.feed(&Bytes::from("data: line1\ndata: line2\n\n"));
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].data, "line1\nline2");
    }

    #[test]
    fn test_parse_multiple_events() {
        let mut parser = SseParser::new();
        let events = parser.feed(&Bytes::from("data: first\n\ndata: second\n\n"));
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].data, "first");
        assert_eq!(events[1].data, "second");
    }

    #[test]
    fn test_parse_chunked() {
        let mut parser = SseParser::new();
        let events1 = parser.feed(&Bytes::from("data: hel"));
        assert_eq!(events1.len(), 0);
        let events2 = parser.feed(&Bytes::from("lo\n\n"));
        assert_eq!(events2.len(), 1);
        assert_eq!(events2[0].data, "hello");
    }
}
