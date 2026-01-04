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

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    // Empty stream type for testing
    struct EmptyStream;

    impl futures_core::Stream for EmptyStream {
        type Item = Result<bytes::Bytes, a2a_transport::Error>;

        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(None)
        }
    }

    // Mock transport for testing
    struct MockTransport;

    impl a2a_transport::HttpClient for MockTransport {
        type Error = a2a_transport::Error;

        fn request(
            &self,
            _req: a2a_transport::HttpRequest,
        ) -> impl std::future::Future<Output = Result<a2a_transport::HttpResponse, Self::Error>> + Send
        {
            async { Err(a2a_transport::Error::Connection("mock".to_string())) }
        }

        fn request_stream(
            &self,
            _req: a2a_transport::HttpRequest,
        ) -> impl std::future::Future<
            Output = Result<
                impl futures_core::Stream<Item = Result<bytes::Bytes, Self::Error>> + Send,
                Self::Error,
            >,
        > + Send {
            async { Err::<EmptyStream, _>(a2a_transport::Error::Connection("mock".to_string())) }
        }
    }

    #[test]
    fn test_builder_default_preference() {
        let builder = ClientBuilder::new(MockTransport, "https://example.com");
        assert!(builder.preference.is_none());
    }

    #[test]
    fn test_builder_custom_preference() {
        let builder = ClientBuilder::new(MockTransport, "https://example.com")
            .prefer(&[Binding::Rest, Binding::JsonRpc]);
        assert_eq!(
            builder.preference,
            Some(vec![Binding::Rest, Binding::JsonRpc])
        );
    }

    #[test]
    fn test_builder_forced_binding() {
        let builder =
            ClientBuilder::new(MockTransport, "https://example.com").binding(Binding::Rest);
        assert_eq!(builder.forced_binding, Some(Binding::Rest));
    }
}
