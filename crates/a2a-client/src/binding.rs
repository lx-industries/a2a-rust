// crates/a2a-client/src/binding.rs
//! Protocol binding selection and configuration.

use a2a_types::{AgentCard, Binding, ProtocolBinding};

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

/// Convert a protocol binding string to a Binding.
fn protocol_binding_to_binding(protocol_binding: &str) -> Option<Binding> {
    protocol_binding
        .parse::<ProtocolBinding>()
        .ok()
        .and_then(|pb| pb.into())
}

/// Extract all available interfaces from an agent card.
///
/// Uses `supported_interfaces` which contains url and protocol_binding pairs.
/// Falls back to deprecated fields (`url`, `preferred_transport`, `additional_interfaces`)
/// for backwards compatibility.
#[allow(deprecated)]
pub fn extract_interfaces(card: &AgentCard) -> Vec<(String, Binding)> {
    let mut interfaces = Vec::new();

    // Prefer supported_interfaces if available
    if !card.supported_interfaces.is_empty() {
        for iface in &card.supported_interfaces {
            if let Some(binding) = protocol_binding_to_binding(&iface.protocol_binding) {
                interfaces.push((iface.url.clone(), binding));
            }
        }
        return interfaces;
    }

    // Fallback to deprecated fields for backwards compatibility
    if let Some(url) = &card.url {
        let main_binding = card
            .preferred_transport
            .as_ref()
            .and_then(|t| protocol_binding_to_binding(t))
            .unwrap_or(Binding::JsonRpc);
        interfaces.push((url.clone(), main_binding));
    }

    // Additional interfaces (deprecated)
    for iface in &card.additional_interfaces {
        if let Some(binding) = protocol_binding_to_binding(&iface.protocol_binding) {
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
        // Construct AgentCard via JSON using new supported_interfaces format
        let json = r#"{
            "name": "test",
            "description": "test",
            "version": "1.0",
            "skills": [],
            "capabilities": {
                "extensions": []
            },
            "supportedInterfaces": [
                {
                    "url": "https://example.com/",
                    "protocolBinding": "JSONRPC",
                    "tenant": ""
                },
                {
                    "url": "https://example.com/v1",
                    "protocolBinding": "HTTP+JSON",
                    "tenant": ""
                }
            ],
            "additionalInterfaces": [],
            "securitySchemes": {},
            "security": [],
            "defaultInputModes": [],
            "defaultOutputModes": [],
            "signatures": []
        }"#;
        let card: AgentCard = serde_json::from_str(json).unwrap();

        let interfaces = extract_interfaces(&card);
        assert_eq!(interfaces.len(), 2);
        assert!(
            interfaces
                .iter()
                .any(|(url, b)| url == "https://example.com/" && *b == Binding::JsonRpc)
        );
        assert!(
            interfaces
                .iter()
                .any(|(url, b)| url == "https://example.com/v1" && *b == Binding::Rest)
        );
    }
}
