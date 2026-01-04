// crates/a2a-client/src/binding.rs
//! Protocol binding selection and configuration.

use a2a_types::Binding;

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
}
