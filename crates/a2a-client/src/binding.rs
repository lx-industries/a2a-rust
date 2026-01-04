// crates/a2a-client/src/binding.rs
//! Protocol binding selection and configuration.

use a2a_types::{AgentCard, Binding, TransportProtocol};

/// Selected binding for client communication.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectedBinding {
    JsonRpc { url: String },
    Rest { url: String },
}

impl SelectedBinding {
    pub fn binding(&self) -> Binding {
        match self {
            Self::JsonRpc { .. } => Binding::JsonRpc,
            Self::Rest { .. } => Binding::Rest,
        }
    }

    pub fn url(&self) -> &str {
        match self {
            Self::JsonRpc { url } | Self::Rest { url } => url,
        }
    }
}

/// Default binding preference order.
pub const DEFAULT_PREFERENCE: &[Binding] = &[Binding::JsonRpc, Binding::Rest];

fn transport_to_binding(transport: &TransportProtocol) -> Option<Binding> {
    match transport {
        TransportProtocol::Jsonrpc => Some(Binding::JsonRpc),
        TransportProtocol::HttpJson => Some(Binding::Rest),
        TransportProtocol::Grpc => None, // Not supported
    }
}

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
    for iface in &card.additional_interfaces {
        if let Some(binding) = transport_to_binding(&iface.transport) {
            interfaces.push((iface.url.clone(), binding));
        }
    }

    interfaces
}

/// Select a binding from available interfaces based on preference order.
///
/// Returns the first interface that matches a binding in the preference list,
/// in preference order (not interface order).
pub fn select_binding(
    interfaces: &[(String, Binding)],
    preference: &[Binding],
) -> Option<SelectedBinding> {
    for preferred in preference {
        for (url, binding) in interfaces {
            if binding == preferred {
                return match binding {
                    Binding::JsonRpc => Some(SelectedBinding::JsonRpc { url: url.clone() }),
                    Binding::Rest => Some(SelectedBinding::Rest { url: url.clone() }),
                };
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_binding_prefers_jsonrpc() {
        let interfaces = vec![
            ("https://example.com/v1".to_string(), Binding::Rest),
            ("https://example.com/".to_string(), Binding::JsonRpc),
        ];
        let result = select_binding(&interfaces, &[Binding::JsonRpc, Binding::Rest]);
        assert_eq!(
            result,
            Some(SelectedBinding::JsonRpc {
                url: "https://example.com/".to_string()
            })
        );
    }

    #[test]
    fn test_select_binding_respects_preference() {
        let interfaces = vec![
            ("https://example.com/v1".to_string(), Binding::Rest),
            ("https://example.com/".to_string(), Binding::JsonRpc),
        ];
        let result = select_binding(&interfaces, &[Binding::Rest, Binding::JsonRpc]);
        assert_eq!(
            result,
            Some(SelectedBinding::Rest {
                url: "https://example.com/v1".to_string()
            })
        );
    }

    #[test]
    fn test_select_binding_no_match() {
        let interfaces: Vec<(String, Binding)> = vec![];
        let result = select_binding(&interfaces, &[Binding::JsonRpc]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_interfaces_from_agent_card() {
        // Construct AgentCard via JSON to work around internal type visibility
        let json = r#"{
            "name": "test",
            "description": "test",
            "version": "1.0",
            "url": "https://example.com/",
            "skills": [],
            "capabilities": {},
            "additionalInterfaces": [
                {
                    "url": "https://example.com/v1",
                    "transport": "http+json"
                }
            ]
        }"#;
        let card: AgentCard = serde_json::from_str(json).unwrap();

        let interfaces = extract_interfaces(&card);
        assert_eq!(interfaces.len(), 2); // main URL + additional
        assert!(interfaces.iter().any(|(url, b)| url == "https://example.com/" && *b == Binding::JsonRpc));
        assert!(interfaces.iter().any(|(url, b)| url == "https://example.com/v1" && *b == Binding::Rest));
    }
}
