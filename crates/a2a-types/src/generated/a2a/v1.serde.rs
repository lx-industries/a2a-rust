impl serde::Serialize for ApiKeySecurityScheme {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.APIKeySecurityScheme", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiKeySecurityScheme {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "location",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Location,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "description" => Ok(GeneratedField::Description),
                            "location" => Ok(GeneratedField::Location),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiKeySecurityScheme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.APIKeySecurityScheme")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApiKeySecurityScheme, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut location__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ApiKeySecurityScheme {
                    description: description__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.APIKeySecurityScheme", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentCapabilities {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.streaming.is_some() {
            len += 1;
        }
        if self.push_notifications.is_some() {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        if self.state_transition_history.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.AgentCapabilities", len)?;
        if let Some(v) = self.streaming.as_ref() {
            struct_ser.serialize_field("streaming", v)?;
        }
        if let Some(v) = self.push_notifications.as_ref() {
            struct_ser.serialize_field("pushNotifications", v)?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        if let Some(v) = self.state_transition_history.as_ref() {
            struct_ser.serialize_field("stateTransitionHistory", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentCapabilities {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "streaming",
            "push_notifications",
            "pushNotifications",
            "extensions",
            "state_transition_history",
            "stateTransitionHistory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Streaming,
            PushNotifications,
            Extensions,
            StateTransitionHistory,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "streaming" => Ok(GeneratedField::Streaming),
                            "pushNotifications" | "push_notifications" => Ok(GeneratedField::PushNotifications),
                            "extensions" => Ok(GeneratedField::Extensions),
                            "stateTransitionHistory" | "state_transition_history" => Ok(GeneratedField::StateTransitionHistory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentCapabilities;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.AgentCapabilities")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentCapabilities, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut streaming__ = None;
                let mut push_notifications__ = None;
                let mut extensions__ = None;
                let mut state_transition_history__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Streaming => {
                            if streaming__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streaming"));
                            }
                            streaming__ = map_.next_value()?;
                        }
                        GeneratedField::PushNotifications => {
                            if push_notifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pushNotifications"));
                            }
                            push_notifications__ = map_.next_value()?;
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StateTransitionHistory => {
                            if state_transition_history__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateTransitionHistory"));
                            }
                            state_transition_history__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AgentCapabilities {
                    streaming: streaming__,
                    push_notifications: push_notifications__,
                    extensions: extensions__.unwrap_or_default(),
                    state_transition_history: state_transition_history__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.AgentCapabilities", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentCard {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protocol_version.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.supported_interfaces.is_empty() {
            len += 1;
        }
        if self.url.is_some() {
            len += 1;
        }
        if self.preferred_transport.is_some() {
            len += 1;
        }
        if !self.additional_interfaces.is_empty() {
            len += 1;
        }
        if self.provider.is_some() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if self.documentation_url.is_some() {
            len += 1;
        }
        if self.capabilities.is_some() {
            len += 1;
        }
        if !self.security_schemes.is_empty() {
            len += 1;
        }
        if !self.security.is_empty() {
            len += 1;
        }
        if !self.default_input_modes.is_empty() {
            len += 1;
        }
        if !self.default_output_modes.is_empty() {
            len += 1;
        }
        if !self.skills.is_empty() {
            len += 1;
        }
        if self.supports_extended_agent_card.is_some() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        if self.icon_url.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.AgentCard", len)?;
        if let Some(v) = self.protocol_version.as_ref() {
            struct_ser.serialize_field("protocolVersion", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.supported_interfaces.is_empty() {
            struct_ser.serialize_field("supportedInterfaces", &self.supported_interfaces)?;
        }
        if let Some(v) = self.url.as_ref() {
            struct_ser.serialize_field("url", v)?;
        }
        if let Some(v) = self.preferred_transport.as_ref() {
            struct_ser.serialize_field("preferredTransport", v)?;
        }
        if !self.additional_interfaces.is_empty() {
            struct_ser.serialize_field("additionalInterfaces", &self.additional_interfaces)?;
        }
        if let Some(v) = self.provider.as_ref() {
            struct_ser.serialize_field("provider", v)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.documentation_url.as_ref() {
            struct_ser.serialize_field("documentationUrl", v)?;
        }
        if let Some(v) = self.capabilities.as_ref() {
            struct_ser.serialize_field("capabilities", v)?;
        }
        if !self.security_schemes.is_empty() {
            struct_ser.serialize_field("securitySchemes", &self.security_schemes)?;
        }
        if !self.security.is_empty() {
            struct_ser.serialize_field("security", &self.security)?;
        }
        if !self.default_input_modes.is_empty() {
            struct_ser.serialize_field("defaultInputModes", &self.default_input_modes)?;
        }
        if !self.default_output_modes.is_empty() {
            struct_ser.serialize_field("defaultOutputModes", &self.default_output_modes)?;
        }
        if !self.skills.is_empty() {
            struct_ser.serialize_field("skills", &self.skills)?;
        }
        if let Some(v) = self.supports_extended_agent_card.as_ref() {
            struct_ser.serialize_field("supportsExtendedAgentCard", v)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        if let Some(v) = self.icon_url.as_ref() {
            struct_ser.serialize_field("iconUrl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentCard {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocol_version",
            "protocolVersion",
            "name",
            "description",
            "supported_interfaces",
            "supportedInterfaces",
            "url",
            "preferred_transport",
            "preferredTransport",
            "additional_interfaces",
            "additionalInterfaces",
            "provider",
            "version",
            "documentation_url",
            "documentationUrl",
            "capabilities",
            "security_schemes",
            "securitySchemes",
            "security",
            "default_input_modes",
            "defaultInputModes",
            "default_output_modes",
            "defaultOutputModes",
            "skills",
            "supports_extended_agent_card",
            "supportsExtendedAgentCard",
            "signatures",
            "icon_url",
            "iconUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtocolVersion,
            Name,
            Description,
            SupportedInterfaces,
            Url,
            PreferredTransport,
            AdditionalInterfaces,
            Provider,
            Version,
            DocumentationUrl,
            Capabilities,
            SecuritySchemes,
            Security,
            DefaultInputModes,
            DefaultOutputModes,
            Skills,
            SupportsExtendedAgentCard,
            Signatures,
            IconUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "protocolVersion" | "protocol_version" => Ok(GeneratedField::ProtocolVersion),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "supportedInterfaces" | "supported_interfaces" => Ok(GeneratedField::SupportedInterfaces),
                            "url" => Ok(GeneratedField::Url),
                            "preferredTransport" | "preferred_transport" => Ok(GeneratedField::PreferredTransport),
                            "additionalInterfaces" | "additional_interfaces" => Ok(GeneratedField::AdditionalInterfaces),
                            "provider" => Ok(GeneratedField::Provider),
                            "version" => Ok(GeneratedField::Version),
                            "documentationUrl" | "documentation_url" => Ok(GeneratedField::DocumentationUrl),
                            "capabilities" => Ok(GeneratedField::Capabilities),
                            "securitySchemes" | "security_schemes" => Ok(GeneratedField::SecuritySchemes),
                            "security" => Ok(GeneratedField::Security),
                            "defaultInputModes" | "default_input_modes" => Ok(GeneratedField::DefaultInputModes),
                            "defaultOutputModes" | "default_output_modes" => Ok(GeneratedField::DefaultOutputModes),
                            "skills" => Ok(GeneratedField::Skills),
                            "supportsExtendedAgentCard" | "supports_extended_agent_card" => Ok(GeneratedField::SupportsExtendedAgentCard),
                            "signatures" => Ok(GeneratedField::Signatures),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentCard;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.AgentCard")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentCard, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol_version__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut supported_interfaces__ = None;
                let mut url__ = None;
                let mut preferred_transport__ = None;
                let mut additional_interfaces__ = None;
                let mut provider__ = None;
                let mut version__ = None;
                let mut documentation_url__ = None;
                let mut capabilities__ = None;
                let mut security_schemes__ = None;
                let mut security__ = None;
                let mut default_input_modes__ = None;
                let mut default_output_modes__ = None;
                let mut skills__ = None;
                let mut supports_extended_agent_card__ = None;
                let mut signatures__ = None;
                let mut icon_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProtocolVersion => {
                            if protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolVersion"));
                            }
                            protocol_version__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SupportedInterfaces => {
                            if supported_interfaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportedInterfaces"));
                            }
                            supported_interfaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = map_.next_value()?;
                        }
                        GeneratedField::PreferredTransport => {
                            if preferred_transport__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredTransport"));
                            }
                            preferred_transport__ = map_.next_value()?;
                        }
                        GeneratedField::AdditionalInterfaces => {
                            if additional_interfaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalInterfaces"));
                            }
                            additional_interfaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = map_.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DocumentationUrl => {
                            if documentation_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentationUrl"));
                            }
                            documentation_url__ = map_.next_value()?;
                        }
                        GeneratedField::Capabilities => {
                            if capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capabilities"));
                            }
                            capabilities__ = map_.next_value()?;
                        }
                        GeneratedField::SecuritySchemes => {
                            if security_schemes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("securitySchemes"));
                            }
                            security_schemes__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Security => {
                            if security__.is_some() {
                                return Err(serde::de::Error::duplicate_field("security"));
                            }
                            security__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultInputModes => {
                            if default_input_modes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultInputModes"));
                            }
                            default_input_modes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultOutputModes => {
                            if default_output_modes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultOutputModes"));
                            }
                            default_output_modes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Skills => {
                            if skills__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skills"));
                            }
                            skills__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SupportsExtendedAgentCard => {
                            if supports_extended_agent_card__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportsExtendedAgentCard"));
                            }
                            supports_extended_agent_card__ = map_.next_value()?;
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AgentCard {
                    protocol_version: protocol_version__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    supported_interfaces: supported_interfaces__.unwrap_or_default(),
                    url: url__,
                    preferred_transport: preferred_transport__,
                    additional_interfaces: additional_interfaces__.unwrap_or_default(),
                    provider: provider__,
                    version: version__.unwrap_or_default(),
                    documentation_url: documentation_url__,
                    capabilities: capabilities__,
                    security_schemes: security_schemes__.unwrap_or_default(),
                    security: security__.unwrap_or_default(),
                    default_input_modes: default_input_modes__.unwrap_or_default(),
                    default_output_modes: default_output_modes__.unwrap_or_default(),
                    skills: skills__.unwrap_or_default(),
                    supports_extended_agent_card: supports_extended_agent_card__,
                    signatures: signatures__.unwrap_or_default(),
                    icon_url: icon_url__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.AgentCard", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentCardSignature {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.protected.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        if self.header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.AgentCardSignature", len)?;
        if !self.protected.is_empty() {
            struct_ser.serialize_field("protected", &self.protected)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentCardSignature {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protected",
            "signature",
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Protected,
            Signature,
            Header,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "protected" => Ok(GeneratedField::Protected),
                            "signature" => Ok(GeneratedField::Signature),
                            "header" => Ok(GeneratedField::Header),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentCardSignature;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.AgentCardSignature")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentCardSignature, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protected__ = None;
                let mut signature__ = None;
                let mut header__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Protected => {
                            if protected__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protected"));
                            }
                            protected__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AgentCardSignature {
                    protected: protected__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.AgentCardSignature", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uri.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.required {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.AgentExtension", len)?;
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.required {
            struct_ser.serialize_field("required", &self.required)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uri",
            "description",
            "required",
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uri,
            Description,
            Required,
            Params,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uri" => Ok(GeneratedField::Uri),
                            "description" => Ok(GeneratedField::Description),
                            "required" => Ok(GeneratedField::Required),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.AgentExtension")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uri__ = None;
                let mut description__ = None;
                let mut required__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AgentExtension {
                    uri: uri__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    required: required__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.AgentExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentInterface {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.protocol_binding.is_empty() {
            len += 1;
        }
        if !self.tenant.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.AgentInterface", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.protocol_binding.is_empty() {
            struct_ser.serialize_field("protocolBinding", &self.protocol_binding)?;
        }
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentInterface {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "protocol_binding",
            "protocolBinding",
            "tenant",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            ProtocolBinding,
            Tenant,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "url" => Ok(GeneratedField::Url),
                            "protocolBinding" | "protocol_binding" => Ok(GeneratedField::ProtocolBinding),
                            "tenant" => Ok(GeneratedField::Tenant),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentInterface;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.AgentInterface")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentInterface, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut protocol_binding__ = None;
                let mut tenant__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProtocolBinding => {
                            if protocol_binding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolBinding"));
                            }
                            protocol_binding__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AgentInterface {
                    url: url__.unwrap_or_default(),
                    protocol_binding: protocol_binding__.unwrap_or_default(),
                    tenant: tenant__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.AgentInterface", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.organization.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.AgentProvider", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.organization.is_empty() {
            struct_ser.serialize_field("organization", &self.organization)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "organization",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            Organization,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "url" => Ok(GeneratedField::Url),
                            "organization" => Ok(GeneratedField::Organization),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.AgentProvider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut organization__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Organization => {
                            if organization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            organization__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AgentProvider {
                    url: url__.unwrap_or_default(),
                    organization: organization__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.AgentProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AgentSkill {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if !self.examples.is_empty() {
            len += 1;
        }
        if !self.input_modes.is_empty() {
            len += 1;
        }
        if !self.output_modes.is_empty() {
            len += 1;
        }
        if !self.security.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.AgentSkill", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if !self.examples.is_empty() {
            struct_ser.serialize_field("examples", &self.examples)?;
        }
        if !self.input_modes.is_empty() {
            struct_ser.serialize_field("inputModes", &self.input_modes)?;
        }
        if !self.output_modes.is_empty() {
            struct_ser.serialize_field("outputModes", &self.output_modes)?;
        }
        if !self.security.is_empty() {
            struct_ser.serialize_field("security", &self.security)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AgentSkill {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "description",
            "tags",
            "examples",
            "input_modes",
            "inputModes",
            "output_modes",
            "outputModes",
            "security",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Description,
            Tags,
            Examples,
            InputModes,
            OutputModes,
            Security,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "tags" => Ok(GeneratedField::Tags),
                            "examples" => Ok(GeneratedField::Examples),
                            "inputModes" | "input_modes" => Ok(GeneratedField::InputModes),
                            "outputModes" | "output_modes" => Ok(GeneratedField::OutputModes),
                            "security" => Ok(GeneratedField::Security),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentSkill;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.AgentSkill")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AgentSkill, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut tags__ = None;
                let mut examples__ = None;
                let mut input_modes__ = None;
                let mut output_modes__ = None;
                let mut security__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Examples => {
                            if examples__.is_some() {
                                return Err(serde::de::Error::duplicate_field("examples"));
                            }
                            examples__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InputModes => {
                            if input_modes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputModes"));
                            }
                            input_modes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputModes => {
                            if output_modes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputModes"));
                            }
                            output_modes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Security => {
                            if security__.is_some() {
                                return Err(serde::de::Error::duplicate_field("security"));
                            }
                            security__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AgentSkill {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                    examples: examples__.unwrap_or_default(),
                    input_modes: input_modes__.unwrap_or_default(),
                    output_modes: output_modes__.unwrap_or_default(),
                    security: security__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.AgentSkill", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Artifact {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifact_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.parts.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.Artifact", len)?;
        if !self.artifact_id.is_empty() {
            struct_ser.serialize_field("artifactId", &self.artifact_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.parts.is_empty() {
            struct_ser.serialize_field("parts", &self.parts)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Artifact {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_id",
            "artifactId",
            "name",
            "description",
            "parts",
            "metadata",
            "extensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactId,
            Name,
            Description,
            Parts,
            Metadata,
            Extensions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "parts" => Ok(GeneratedField::Parts),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "extensions" => Ok(GeneratedField::Extensions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Artifact;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.Artifact")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Artifact, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut parts__ = None;
                let mut metadata__ = None;
                let mut extensions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parts => {
                            if parts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parts"));
                            }
                            parts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Artifact {
                    artifact_id: artifact_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    parts: parts__.unwrap_or_default(),
                    metadata: metadata__,
                    extensions: extensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.Artifact", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticationInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.schemes.is_empty() {
            len += 1;
        }
        if !self.credentials.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.AuthenticationInfo", len)?;
        if !self.schemes.is_empty() {
            struct_ser.serialize_field("schemes", &self.schemes)?;
        }
        if !self.credentials.is_empty() {
            struct_ser.serialize_field("credentials", &self.credentials)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schemes",
            "credentials",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schemes,
            Credentials,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "schemes" => Ok(GeneratedField::Schemes),
                            "credentials" => Ok(GeneratedField::Credentials),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.AuthenticationInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthenticationInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schemes__ = None;
                let mut credentials__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Schemes => {
                            if schemes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemes"));
                            }
                            schemes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credentials => {
                            if credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentials"));
                            }
                            credentials__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthenticationInfo {
                    schemes: schemes__.unwrap_or_default(),
                    credentials: credentials__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.AuthenticationInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthorizationCodeOAuthFlow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authorization_url.is_empty() {
            len += 1;
        }
        if !self.token_url.is_empty() {
            len += 1;
        }
        if !self.refresh_url.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.AuthorizationCodeOAuthFlow", len)?;
        if !self.authorization_url.is_empty() {
            struct_ser.serialize_field("authorizationUrl", &self.authorization_url)?;
        }
        if !self.token_url.is_empty() {
            struct_ser.serialize_field("tokenUrl", &self.token_url)?;
        }
        if !self.refresh_url.is_empty() {
            struct_ser.serialize_field("refreshUrl", &self.refresh_url)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationCodeOAuthFlow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization_url",
            "authorizationUrl",
            "token_url",
            "tokenUrl",
            "refresh_url",
            "refreshUrl",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthorizationUrl,
            TokenUrl,
            RefreshUrl,
            Scopes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authorizationUrl" | "authorization_url" => Ok(GeneratedField::AuthorizationUrl),
                            "tokenUrl" | "token_url" => Ok(GeneratedField::TokenUrl),
                            "refreshUrl" | "refresh_url" => Ok(GeneratedField::RefreshUrl),
                            "scopes" => Ok(GeneratedField::Scopes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizationCodeOAuthFlow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.AuthorizationCodeOAuthFlow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthorizationCodeOAuthFlow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authorization_url__ = None;
                let mut token_url__ = None;
                let mut refresh_url__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthorizationUrl => {
                            if authorization_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationUrl"));
                            }
                            authorization_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenUrl => {
                            if token_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenUrl"));
                            }
                            token_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshUrl => {
                            if refresh_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshUrl"));
                            }
                            refresh_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(AuthorizationCodeOAuthFlow {
                    authorization_url: authorization_url__.unwrap_or_default(),
                    token_url: token_url__.unwrap_or_default(),
                    refresh_url: refresh_url__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.AuthorizationCodeOAuthFlow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelTaskRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.CancelTaskRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelTaskRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelTaskRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.CancelTaskRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelTaskRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CancelTaskRequest {
                    tenant: tenant__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.CancelTaskRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientCredentialsOAuthFlow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token_url.is_empty() {
            len += 1;
        }
        if !self.refresh_url.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.ClientCredentialsOAuthFlow", len)?;
        if !self.token_url.is_empty() {
            struct_ser.serialize_field("tokenUrl", &self.token_url)?;
        }
        if !self.refresh_url.is_empty() {
            struct_ser.serialize_field("refreshUrl", &self.refresh_url)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientCredentialsOAuthFlow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token_url",
            "tokenUrl",
            "refresh_url",
            "refreshUrl",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TokenUrl,
            RefreshUrl,
            Scopes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tokenUrl" | "token_url" => Ok(GeneratedField::TokenUrl),
                            "refreshUrl" | "refresh_url" => Ok(GeneratedField::RefreshUrl),
                            "scopes" => Ok(GeneratedField::Scopes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientCredentialsOAuthFlow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.ClientCredentialsOAuthFlow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClientCredentialsOAuthFlow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token_url__ = None;
                let mut refresh_url__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TokenUrl => {
                            if token_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenUrl"));
                            }
                            token_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshUrl => {
                            if refresh_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshUrl"));
                            }
                            refresh_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ClientCredentialsOAuthFlow {
                    token_url: token_url__.unwrap_or_default(),
                    refresh_url: refresh_url__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.ClientCredentialsOAuthFlow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataPart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.DataPart", len)?;
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataPart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataPart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.DataPart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataPart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DataPart {
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.DataPart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTaskPushNotificationConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.DeleteTaskPushNotificationConfigRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTaskPushNotificationConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteTaskPushNotificationConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.DeleteTaskPushNotificationConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTaskPushNotificationConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteTaskPushNotificationConfigRequest {
                    tenant: tenant__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.DeleteTaskPushNotificationConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilePart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.media_type.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.file.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.FilePart", len)?;
        if !self.media_type.is_empty() {
            struct_ser.serialize_field("mediaType", &self.media_type)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.file.as_ref() {
            match v {
                file_part::File::FileWithUri(v) => {
                    struct_ser.serialize_field("fileWithUri", v)?;
                }
                file_part::File::FileWithBytes(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("fileWithBytes", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilePart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "media_type",
            "mediaType",
            "name",
            "file_with_uri",
            "fileWithUri",
            "file_with_bytes",
            "fileWithBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MediaType,
            Name,
            FileWithUri,
            FileWithBytes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "mediaType" | "media_type" => Ok(GeneratedField::MediaType),
                            "name" => Ok(GeneratedField::Name),
                            "fileWithUri" | "file_with_uri" => Ok(GeneratedField::FileWithUri),
                            "fileWithBytes" | "file_with_bytes" => Ok(GeneratedField::FileWithBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilePart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.FilePart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilePart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut media_type__ = None;
                let mut name__ = None;
                let mut file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MediaType => {
                            if media_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mediaType"));
                            }
                            media_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileWithUri => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileWithUri"));
                            }
                            file__ = map_.next_value::<::std::option::Option<_>>()?.map(file_part::File::FileWithUri);
                        }
                        GeneratedField::FileWithBytes => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileWithBytes"));
                            }
                            file__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| file_part::File::FileWithBytes(x.0));
                        }
                    }
                }
                Ok(FilePart {
                    media_type: media_type__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    file: file__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.FilePart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetExtendedAgentCardRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.GetExtendedAgentCardRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetExtendedAgentCardRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetExtendedAgentCardRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.GetExtendedAgentCardRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetExtendedAgentCardRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetExtendedAgentCardRequest {
                    tenant: tenant__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.GetExtendedAgentCardRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTaskPushNotificationConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.GetTaskPushNotificationConfigRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTaskPushNotificationConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTaskPushNotificationConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.GetTaskPushNotificationConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTaskPushNotificationConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetTaskPushNotificationConfigRequest {
                    tenant: tenant__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.GetTaskPushNotificationConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTaskRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.history_length.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.GetTaskRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.history_length.as_ref() {
            struct_ser.serialize_field("historyLength", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTaskRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
            "name",
            "history_length",
            "historyLength",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
            Name,
            HistoryLength,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            "name" => Ok(GeneratedField::Name),
                            "historyLength" | "history_length" => Ok(GeneratedField::HistoryLength),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTaskRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.GetTaskRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTaskRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                let mut name__ = None;
                let mut history_length__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HistoryLength => {
                            if history_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyLength"));
                            }
                            history_length__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(GetTaskRequest {
                    tenant: tenant__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    history_length: history_length__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.GetTaskRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpAuthSecurityScheme {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.scheme.is_empty() {
            len += 1;
        }
        if !self.bearer_format.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.HTTPAuthSecurityScheme", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.scheme.is_empty() {
            struct_ser.serialize_field("scheme", &self.scheme)?;
        }
        if !self.bearer_format.is_empty() {
            struct_ser.serialize_field("bearerFormat", &self.bearer_format)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpAuthSecurityScheme {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "scheme",
            "bearer_format",
            "bearerFormat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Scheme,
            BearerFormat,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "description" => Ok(GeneratedField::Description),
                            "scheme" => Ok(GeneratedField::Scheme),
                            "bearerFormat" | "bearer_format" => Ok(GeneratedField::BearerFormat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpAuthSecurityScheme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.HTTPAuthSecurityScheme")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpAuthSecurityScheme, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut scheme__ = None;
                let mut bearer_format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheme"));
                            }
                            scheme__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BearerFormat => {
                            if bearer_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bearerFormat"));
                            }
                            bearer_format__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpAuthSecurityScheme {
                    description: description__.unwrap_or_default(),
                    scheme: scheme__.unwrap_or_default(),
                    bearer_format: bearer_format__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.HTTPAuthSecurityScheme", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImplicitOAuthFlow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authorization_url.is_empty() {
            len += 1;
        }
        if !self.refresh_url.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.ImplicitOAuthFlow", len)?;
        if !self.authorization_url.is_empty() {
            struct_ser.serialize_field("authorizationUrl", &self.authorization_url)?;
        }
        if !self.refresh_url.is_empty() {
            struct_ser.serialize_field("refreshUrl", &self.refresh_url)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImplicitOAuthFlow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization_url",
            "authorizationUrl",
            "refresh_url",
            "refreshUrl",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthorizationUrl,
            RefreshUrl,
            Scopes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authorizationUrl" | "authorization_url" => Ok(GeneratedField::AuthorizationUrl),
                            "refreshUrl" | "refresh_url" => Ok(GeneratedField::RefreshUrl),
                            "scopes" => Ok(GeneratedField::Scopes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImplicitOAuthFlow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.ImplicitOAuthFlow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImplicitOAuthFlow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authorization_url__ = None;
                let mut refresh_url__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthorizationUrl => {
                            if authorization_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationUrl"));
                            }
                            authorization_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshUrl => {
                            if refresh_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshUrl"));
                            }
                            refresh_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ImplicitOAuthFlow {
                    authorization_url: authorization_url__.unwrap_or_default(),
                    refresh_url: refresh_url__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.ImplicitOAuthFlow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTaskPushNotificationConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.ListTaskPushNotificationConfigRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTaskPushNotificationConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
            "parent",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
            Parent,
            PageSize,
            PageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            "parent" => Ok(GeneratedField::Parent),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTaskPushNotificationConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.ListTaskPushNotificationConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTaskPushNotificationConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                let mut parent__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListTaskPushNotificationConfigRequest {
                    tenant: tenant__.unwrap_or_default(),
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.ListTaskPushNotificationConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTaskPushNotificationConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.configs.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.ListTaskPushNotificationConfigResponse", len)?;
        if !self.configs.is_empty() {
            struct_ser.serialize_field("configs", &self.configs)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTaskPushNotificationConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "configs",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Configs,
            NextPageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "configs" => Ok(GeneratedField::Configs),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTaskPushNotificationConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.ListTaskPushNotificationConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTaskPushNotificationConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut configs__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Configs => {
                            if configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configs"));
                            }
                            configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListTaskPushNotificationConfigResponse {
                    configs: configs__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.ListTaskPushNotificationConfigResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTasksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        if !self.context_id.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.page_size.is_some() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.history_length.is_some() {
            len += 1;
        }
        if self.last_updated_after != 0 {
            len += 1;
        }
        if self.include_artifacts.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.ListTasksRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        if !self.context_id.is_empty() {
            struct_ser.serialize_field("contextId", &self.context_id)?;
        }
        if self.status != 0 {
            let v = TaskState::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.page_size.as_ref() {
            struct_ser.serialize_field("pageSize", v)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if let Some(v) = self.history_length.as_ref() {
            struct_ser.serialize_field("historyLength", v)?;
        }
        if self.last_updated_after != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lastUpdatedAfter", ToString::to_string(&self.last_updated_after).as_str())?;
        }
        if let Some(v) = self.include_artifacts.as_ref() {
            struct_ser.serialize_field("includeArtifacts", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTasksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
            "context_id",
            "contextId",
            "status",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "history_length",
            "historyLength",
            "last_updated_after",
            "lastUpdatedAfter",
            "include_artifacts",
            "includeArtifacts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
            ContextId,
            Status,
            PageSize,
            PageToken,
            HistoryLength,
            LastUpdatedAfter,
            IncludeArtifacts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            "contextId" | "context_id" => Ok(GeneratedField::ContextId),
                            "status" => Ok(GeneratedField::Status),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "historyLength" | "history_length" => Ok(GeneratedField::HistoryLength),
                            "lastUpdatedAfter" | "last_updated_after" => Ok(GeneratedField::LastUpdatedAfter),
                            "includeArtifacts" | "include_artifacts" => Ok(GeneratedField::IncludeArtifacts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTasksRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.ListTasksRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTasksRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                let mut context_id__ = None;
                let mut status__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut history_length__ = None;
                let mut last_updated_after__ = None;
                let mut include_artifacts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextId => {
                            if context_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextId"));
                            }
                            context_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<TaskState>()? as i32);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HistoryLength => {
                            if history_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyLength"));
                            }
                            history_length__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LastUpdatedAfter => {
                            if last_updated_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdatedAfter"));
                            }
                            last_updated_after__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IncludeArtifacts => {
                            if include_artifacts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArtifacts"));
                            }
                            include_artifacts__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListTasksRequest {
                    tenant: tenant__.unwrap_or_default(),
                    context_id: context_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    page_size: page_size__,
                    page_token: page_token__.unwrap_or_default(),
                    history_length: history_length__,
                    last_updated_after: last_updated_after__.unwrap_or_default(),
                    include_artifacts: include_artifacts__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.ListTasksRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTasksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tasks.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if self.total_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.ListTasksResponse", len)?;
        if !self.tasks.is_empty() {
            struct_ser.serialize_field("tasks", &self.tasks)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if self.total_size != 0 {
            struct_ser.serialize_field("totalSize", &self.total_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTasksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tasks",
            "next_page_token",
            "nextPageToken",
            "page_size",
            "pageSize",
            "total_size",
            "totalSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tasks,
            NextPageToken,
            PageSize,
            TotalSize,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tasks" => Ok(GeneratedField::Tasks),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "totalSize" | "total_size" => Ok(GeneratedField::TotalSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTasksResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.ListTasksResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTasksResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tasks__ = None;
                let mut next_page_token__ = None;
                let mut page_size__ = None;
                let mut total_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tasks => {
                            if tasks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tasks"));
                            }
                            tasks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalSize => {
                            if total_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSize"));
                            }
                            total_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListTasksResponse {
                    tasks: tasks__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    total_size: total_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.ListTasksResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Message {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_id.is_empty() {
            len += 1;
        }
        if !self.context_id.is_empty() {
            len += 1;
        }
        if !self.task_id.is_empty() {
            len += 1;
        }
        if self.role != 0 {
            len += 1;
        }
        if !self.parts.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        if !self.reference_task_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.Message", len)?;
        if !self.message_id.is_empty() {
            struct_ser.serialize_field("messageId", &self.message_id)?;
        }
        if !self.context_id.is_empty() {
            struct_ser.serialize_field("contextId", &self.context_id)?;
        }
        if !self.task_id.is_empty() {
            struct_ser.serialize_field("taskId", &self.task_id)?;
        }
        if self.role != 0 {
            let v = Role::try_from(self.role)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.role)))?;
            struct_ser.serialize_field("role", &v)?;
        }
        if !self.parts.is_empty() {
            struct_ser.serialize_field("parts", &self.parts)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        if !self.reference_task_ids.is_empty() {
            struct_ser.serialize_field("referenceTaskIds", &self.reference_task_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Message {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_id",
            "messageId",
            "context_id",
            "contextId",
            "task_id",
            "taskId",
            "role",
            "parts",
            "metadata",
            "extensions",
            "reference_task_ids",
            "referenceTaskIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessageId,
            ContextId,
            TaskId,
            Role,
            Parts,
            Metadata,
            Extensions,
            ReferenceTaskIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "messageId" | "message_id" => Ok(GeneratedField::MessageId),
                            "contextId" | "context_id" => Ok(GeneratedField::ContextId),
                            "taskId" | "task_id" => Ok(GeneratedField::TaskId),
                            "role" => Ok(GeneratedField::Role),
                            "parts" => Ok(GeneratedField::Parts),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "extensions" => Ok(GeneratedField::Extensions),
                            "referenceTaskIds" | "reference_task_ids" => Ok(GeneratedField::ReferenceTaskIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.Message")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Message, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_id__ = None;
                let mut context_id__ = None;
                let mut task_id__ = None;
                let mut role__ = None;
                let mut parts__ = None;
                let mut metadata__ = None;
                let mut extensions__ = None;
                let mut reference_task_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessageId => {
                            if message_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageId"));
                            }
                            message_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextId => {
                            if context_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextId"));
                            }
                            context_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TaskId => {
                            if task_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskId"));
                            }
                            task_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value::<Role>()? as i32);
                        }
                        GeneratedField::Parts => {
                            if parts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parts"));
                            }
                            parts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReferenceTaskIds => {
                            if reference_task_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceTaskIds"));
                            }
                            reference_task_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Message {
                    message_id: message_id__.unwrap_or_default(),
                    context_id: context_id__.unwrap_or_default(),
                    task_id: task_id__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    parts: parts__.unwrap_or_default(),
                    metadata: metadata__,
                    extensions: extensions__.unwrap_or_default(),
                    reference_task_ids: reference_task_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.Message", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MutualTlsSecurityScheme {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.MutualTlsSecurityScheme", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MutualTlsSecurityScheme {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MutualTlsSecurityScheme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.MutualTlsSecurityScheme")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MutualTlsSecurityScheme, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MutualTlsSecurityScheme {
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.MutualTlsSecurityScheme", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OAuth2SecurityScheme {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        if self.flows.is_some() {
            len += 1;
        }
        if !self.oauth2_metadata_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.OAuth2SecurityScheme", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.flows.as_ref() {
            struct_ser.serialize_field("flows", v)?;
        }
        if !self.oauth2_metadata_url.is_empty() {
            struct_ser.serialize_field("oauth2MetadataUrl", &self.oauth2_metadata_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OAuth2SecurityScheme {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "flows",
            "oauth2_metadata_url",
            "oauth2MetadataUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Flows,
            Oauth2MetadataUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "description" => Ok(GeneratedField::Description),
                            "flows" => Ok(GeneratedField::Flows),
                            "oauth2MetadataUrl" | "oauth2_metadata_url" => Ok(GeneratedField::Oauth2MetadataUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OAuth2SecurityScheme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.OAuth2SecurityScheme")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OAuth2SecurityScheme, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut flows__ = None;
                let mut oauth2_metadata_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Flows => {
                            if flows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flows"));
                            }
                            flows__ = map_.next_value()?;
                        }
                        GeneratedField::Oauth2MetadataUrl => {
                            if oauth2_metadata_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oauth2MetadataUrl"));
                            }
                            oauth2_metadata_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OAuth2SecurityScheme {
                    description: description__.unwrap_or_default(),
                    flows: flows__,
                    oauth2_metadata_url: oauth2_metadata_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.OAuth2SecurityScheme", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OAuthFlows {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.flow.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.OAuthFlows", len)?;
        if let Some(v) = self.flow.as_ref() {
            match v {
                o_auth_flows::Flow::AuthorizationCode(v) => {
                    struct_ser.serialize_field("authorizationCode", v)?;
                }
                o_auth_flows::Flow::ClientCredentials(v) => {
                    struct_ser.serialize_field("clientCredentials", v)?;
                }
                o_auth_flows::Flow::Implicit(v) => {
                    struct_ser.serialize_field("implicit", v)?;
                }
                o_auth_flows::Flow::Password(v) => {
                    struct_ser.serialize_field("password", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OAuthFlows {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization_code",
            "authorizationCode",
            "client_credentials",
            "clientCredentials",
            "implicit",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthorizationCode,
            ClientCredentials,
            Implicit,
            Password,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authorizationCode" | "authorization_code" => Ok(GeneratedField::AuthorizationCode),
                            "clientCredentials" | "client_credentials" => Ok(GeneratedField::ClientCredentials),
                            "implicit" => Ok(GeneratedField::Implicit),
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OAuthFlows;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.OAuthFlows")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OAuthFlows, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut flow__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthorizationCode => {
                            if flow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationCode"));
                            }
                            flow__ = map_.next_value::<::std::option::Option<_>>()?.map(o_auth_flows::Flow::AuthorizationCode)
;
                        }
                        GeneratedField::ClientCredentials => {
                            if flow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientCredentials"));
                            }
                            flow__ = map_.next_value::<::std::option::Option<_>>()?.map(o_auth_flows::Flow::ClientCredentials)
;
                        }
                        GeneratedField::Implicit => {
                            if flow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicit"));
                            }
                            flow__ = map_.next_value::<::std::option::Option<_>>()?.map(o_auth_flows::Flow::Implicit)
;
                        }
                        GeneratedField::Password => {
                            if flow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            flow__ = map_.next_value::<::std::option::Option<_>>()?.map(o_auth_flows::Flow::Password)
;
                        }
                    }
                }
                Ok(OAuthFlows {
                    flow: flow__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.OAuthFlows", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OpenIdConnectSecurityScheme {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.open_id_connect_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.OpenIdConnectSecurityScheme", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.open_id_connect_url.is_empty() {
            struct_ser.serialize_field("openIdConnectUrl", &self.open_id_connect_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OpenIdConnectSecurityScheme {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "open_id_connect_url",
            "openIdConnectUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            OpenIdConnectUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "description" => Ok(GeneratedField::Description),
                            "openIdConnectUrl" | "open_id_connect_url" => Ok(GeneratedField::OpenIdConnectUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OpenIdConnectSecurityScheme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.OpenIdConnectSecurityScheme")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OpenIdConnectSecurityScheme, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut open_id_connect_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OpenIdConnectUrl => {
                            if open_id_connect_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openIdConnectUrl"));
                            }
                            open_id_connect_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OpenIdConnectSecurityScheme {
                    description: description__.unwrap_or_default(),
                    open_id_connect_url: open_id_connect_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.OpenIdConnectSecurityScheme", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Part {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.part.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.Part", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.part.as_ref() {
            match v {
                part::Part::Text(v) => {
                    struct_ser.serialize_field("text", v)?;
                }
                part::Part::File(v) => {
                    struct_ser.serialize_field("file", v)?;
                }
                part::Part::Data(v) => {
                    struct_ser.serialize_field("data", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Part {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "text",
            "file",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Text,
            File,
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "metadata" => Ok(GeneratedField::Metadata),
                            "text" => Ok(GeneratedField::Text),
                            "file" => Ok(GeneratedField::File),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Part;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.Part")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Part, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut part__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Text => {
                            if part__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            part__ = map_.next_value::<::std::option::Option<_>>()?.map(part::Part::Text);
                        }
                        GeneratedField::File => {
                            if part__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            part__ = map_.next_value::<::std::option::Option<_>>()?.map(part::Part::File)
;
                        }
                        GeneratedField::Data => {
                            if part__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            part__ = map_.next_value::<::std::option::Option<_>>()?.map(part::Part::Data)
;
                        }
                    }
                }
                Ok(Part {
                    metadata: metadata__,
                    part: part__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.Part", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordOAuthFlow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token_url.is_empty() {
            len += 1;
        }
        if !self.refresh_url.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.PasswordOAuthFlow", len)?;
        if !self.token_url.is_empty() {
            struct_ser.serialize_field("tokenUrl", &self.token_url)?;
        }
        if !self.refresh_url.is_empty() {
            struct_ser.serialize_field("refreshUrl", &self.refresh_url)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordOAuthFlow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token_url",
            "tokenUrl",
            "refresh_url",
            "refreshUrl",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TokenUrl,
            RefreshUrl,
            Scopes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tokenUrl" | "token_url" => Ok(GeneratedField::TokenUrl),
                            "refreshUrl" | "refresh_url" => Ok(GeneratedField::RefreshUrl),
                            "scopes" => Ok(GeneratedField::Scopes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordOAuthFlow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.PasswordOAuthFlow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordOAuthFlow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token_url__ = None;
                let mut refresh_url__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TokenUrl => {
                            if token_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenUrl"));
                            }
                            token_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshUrl => {
                            if refresh_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshUrl"));
                            }
                            refresh_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(PasswordOAuthFlow {
                    token_url: token_url__.unwrap_or_default(),
                    refresh_url: refresh_url__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.PasswordOAuthFlow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PushNotificationConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.token.is_empty() {
            len += 1;
        }
        if self.authentication.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.PushNotificationConfig", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        if let Some(v) = self.authentication.as_ref() {
            struct_ser.serialize_field("authentication", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PushNotificationConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "url",
            "token",
            "authentication",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Url,
            Token,
            Authentication,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "url" => Ok(GeneratedField::Url),
                            "token" => Ok(GeneratedField::Token),
                            "authentication" => Ok(GeneratedField::Authentication),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PushNotificationConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.PushNotificationConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PushNotificationConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut url__ = None;
                let mut token__ = None;
                let mut authentication__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Authentication => {
                            if authentication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authentication"));
                            }
                            authentication__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PushNotificationConfig {
                    id: id__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                    token: token__.unwrap_or_default(),
                    authentication: authentication__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.PushNotificationConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Role {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ROLE_UNSPECIFIED",
            Self::User => "ROLE_USER",
            Self::Agent => "ROLE_AGENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Role {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ROLE_UNSPECIFIED",
            "ROLE_USER",
            "ROLE_AGENT",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = Role;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ROLE_UNSPECIFIED" => Ok(Role::Unspecified),
                    "ROLE_USER" => Ok(Role::User),
                    "ROLE_AGENT" => Ok(Role::Agent),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Security {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.schemes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.Security", len)?;
        if !self.schemes.is_empty() {
            struct_ser.serialize_field("schemes", &self.schemes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Security {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schemes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schemes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "schemes" => Ok(GeneratedField::Schemes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Security;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.Security")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Security, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schemes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Schemes => {
                            if schemes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemes"));
                            }
                            schemes__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Security {
                    schemes: schemes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.Security", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SecurityScheme {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.scheme.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.SecurityScheme", len)?;
        if let Some(v) = self.scheme.as_ref() {
            match v {
                security_scheme::Scheme::ApiKeySecurityScheme(v) => {
                    struct_ser.serialize_field("apiKeySecurityScheme", v)?;
                }
                security_scheme::Scheme::HttpAuthSecurityScheme(v) => {
                    struct_ser.serialize_field("httpAuthSecurityScheme", v)?;
                }
                security_scheme::Scheme::Oauth2SecurityScheme(v) => {
                    struct_ser.serialize_field("oauth2SecurityScheme", v)?;
                }
                security_scheme::Scheme::OpenIdConnectSecurityScheme(v) => {
                    struct_ser.serialize_field("openIdConnectSecurityScheme", v)?;
                }
                security_scheme::Scheme::MtlsSecurityScheme(v) => {
                    struct_ser.serialize_field("mtlsSecurityScheme", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecurityScheme {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "api_key_security_scheme",
            "apiKeySecurityScheme",
            "http_auth_security_scheme",
            "httpAuthSecurityScheme",
            "oauth2_security_scheme",
            "oauth2SecurityScheme",
            "open_id_connect_security_scheme",
            "openIdConnectSecurityScheme",
            "mtls_security_scheme",
            "mtlsSecurityScheme",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ApiKeySecurityScheme,
            HttpAuthSecurityScheme,
            Oauth2SecurityScheme,
            OpenIdConnectSecurityScheme,
            MtlsSecurityScheme,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "apiKeySecurityScheme" | "api_key_security_scheme" => Ok(GeneratedField::ApiKeySecurityScheme),
                            "httpAuthSecurityScheme" | "http_auth_security_scheme" => Ok(GeneratedField::HttpAuthSecurityScheme),
                            "oauth2SecurityScheme" | "oauth2_security_scheme" => Ok(GeneratedField::Oauth2SecurityScheme),
                            "openIdConnectSecurityScheme" | "open_id_connect_security_scheme" => Ok(GeneratedField::OpenIdConnectSecurityScheme),
                            "mtlsSecurityScheme" | "mtls_security_scheme" => Ok(GeneratedField::MtlsSecurityScheme),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecurityScheme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.SecurityScheme")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SecurityScheme, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scheme__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ApiKeySecurityScheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiKeySecurityScheme"));
                            }
                            scheme__ = map_.next_value::<::std::option::Option<_>>()?.map(security_scheme::Scheme::ApiKeySecurityScheme)
;
                        }
                        GeneratedField::HttpAuthSecurityScheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpAuthSecurityScheme"));
                            }
                            scheme__ = map_.next_value::<::std::option::Option<_>>()?.map(security_scheme::Scheme::HttpAuthSecurityScheme)
;
                        }
                        GeneratedField::Oauth2SecurityScheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oauth2SecurityScheme"));
                            }
                            scheme__ = map_.next_value::<::std::option::Option<_>>()?.map(security_scheme::Scheme::Oauth2SecurityScheme)
;
                        }
                        GeneratedField::OpenIdConnectSecurityScheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openIdConnectSecurityScheme"));
                            }
                            scheme__ = map_.next_value::<::std::option::Option<_>>()?.map(security_scheme::Scheme::OpenIdConnectSecurityScheme)
;
                        }
                        GeneratedField::MtlsSecurityScheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mtlsSecurityScheme"));
                            }
                            scheme__ = map_.next_value::<::std::option::Option<_>>()?.map(security_scheme::Scheme::MtlsSecurityScheme)
;
                        }
                    }
                }
                Ok(SecurityScheme {
                    scheme: scheme__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.SecurityScheme", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendMessageConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.accepted_output_modes.is_empty() {
            len += 1;
        }
        if self.push_notification_config.is_some() {
            len += 1;
        }
        if self.history_length.is_some() {
            len += 1;
        }
        if self.blocking {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.SendMessageConfiguration", len)?;
        if !self.accepted_output_modes.is_empty() {
            struct_ser.serialize_field("acceptedOutputModes", &self.accepted_output_modes)?;
        }
        if let Some(v) = self.push_notification_config.as_ref() {
            struct_ser.serialize_field("pushNotificationConfig", v)?;
        }
        if let Some(v) = self.history_length.as_ref() {
            struct_ser.serialize_field("historyLength", v)?;
        }
        if self.blocking {
            struct_ser.serialize_field("blocking", &self.blocking)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendMessageConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "accepted_output_modes",
            "acceptedOutputModes",
            "push_notification_config",
            "pushNotificationConfig",
            "history_length",
            "historyLength",
            "blocking",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AcceptedOutputModes,
            PushNotificationConfig,
            HistoryLength,
            Blocking,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "acceptedOutputModes" | "accepted_output_modes" => Ok(GeneratedField::AcceptedOutputModes),
                            "pushNotificationConfig" | "push_notification_config" => Ok(GeneratedField::PushNotificationConfig),
                            "historyLength" | "history_length" => Ok(GeneratedField::HistoryLength),
                            "blocking" => Ok(GeneratedField::Blocking),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendMessageConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.SendMessageConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SendMessageConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut accepted_output_modes__ = None;
                let mut push_notification_config__ = None;
                let mut history_length__ = None;
                let mut blocking__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AcceptedOutputModes => {
                            if accepted_output_modes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("acceptedOutputModes"));
                            }
                            accepted_output_modes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PushNotificationConfig => {
                            if push_notification_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pushNotificationConfig"));
                            }
                            push_notification_config__ = map_.next_value()?;
                        }
                        GeneratedField::HistoryLength => {
                            if history_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyLength"));
                            }
                            history_length__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Blocking => {
                            if blocking__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocking"));
                            }
                            blocking__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SendMessageConfiguration {
                    accepted_output_modes: accepted_output_modes__.unwrap_or_default(),
                    push_notification_config: push_notification_config__,
                    history_length: history_length__,
                    blocking: blocking__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.SendMessageConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.SendMessageRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
            "request",
            "message",
            "configuration",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
            Request,
            Configuration,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            "message" | "request" => Ok(GeneratedField::Request),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.SendMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SendMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                let mut request__ = None;
                let mut configuration__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            request__ = map_.next_value()?;
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SendMessageRequest {
                    tenant: tenant__.unwrap_or_default(),
                    request: request__,
                    configuration: configuration__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.SendMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.SendMessageResponse", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                send_message_response::Payload::Task(v) => {
                    struct_ser.serialize_field("task", v)?;
                }
                send_message_response::Payload::Msg(v) => {
                    struct_ser.serialize_field("message", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task",
            "msg",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Task,
            Msg,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "task" => Ok(GeneratedField::Task),
                            "message" | "msg" => Ok(GeneratedField::Msg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SendMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.SendMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SendMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Task => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(send_message_response::Payload::Task)
;
                        }
                        GeneratedField::Msg => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(send_message_response::Payload::Msg)
;
                        }
                    }
                }
                Ok(SendMessageResponse {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.SendMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetTaskPushNotificationConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        if !self.parent.is_empty() {
            len += 1;
        }
        if !self.config_id.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.SetTaskPushNotificationConfigRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if !self.config_id.is_empty() {
            struct_ser.serialize_field("configId", &self.config_id)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetTaskPushNotificationConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
            "parent",
            "config_id",
            "configId",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
            Parent,
            ConfigId,
            Config,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            "parent" => Ok(GeneratedField::Parent),
                            "configId" | "config_id" => Ok(GeneratedField::ConfigId),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetTaskPushNotificationConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.SetTaskPushNotificationConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetTaskPushNotificationConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                let mut parent__ = None;
                let mut config_id__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigId => {
                            if config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configId"));
                            }
                            config_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetTaskPushNotificationConfigRequest {
                    tenant: tenant__.unwrap_or_default(),
                    parent: parent__.unwrap_or_default(),
                    config_id: config_id__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.SetTaskPushNotificationConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.StreamResponse", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                stream_response::Payload::Task(v) => {
                    struct_ser.serialize_field("task", v)?;
                }
                stream_response::Payload::Msg(v) => {
                    struct_ser.serialize_field("message", v)?;
                }
                stream_response::Payload::StatusUpdate(v) => {
                    struct_ser.serialize_field("statusUpdate", v)?;
                }
                stream_response::Payload::ArtifactUpdate(v) => {
                    struct_ser.serialize_field("artifactUpdate", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task",
            "msg",
            "message",
            "status_update",
            "statusUpdate",
            "artifact_update",
            "artifactUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Task,
            Msg,
            StatusUpdate,
            ArtifactUpdate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "task" => Ok(GeneratedField::Task),
                            "message" | "msg" => Ok(GeneratedField::Msg),
                            "statusUpdate" | "status_update" => Ok(GeneratedField::StatusUpdate),
                            "artifactUpdate" | "artifact_update" => Ok(GeneratedField::ArtifactUpdate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.StreamResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Task => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("task"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_response::Payload::Task)
;
                        }
                        GeneratedField::Msg => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_response::Payload::Msg)
;
                        }
                        GeneratedField::StatusUpdate => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusUpdate"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_response::Payload::StatusUpdate)
;
                        }
                        GeneratedField::ArtifactUpdate => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactUpdate"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(stream_response::Payload::ArtifactUpdate)
;
                        }
                    }
                }
                Ok(StreamResponse {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.StreamResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StringList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.list.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.StringList", len)?;
        if !self.list.is_empty() {
            struct_ser.serialize_field("list", &self.list)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "list",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            List,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "list" => Ok(GeneratedField::List),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StringList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.StringList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StringList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut list__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::List => {
                            if list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("list"));
                            }
                            list__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StringList {
                    list: list__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.StringList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscribeToTaskRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tenant.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.SubscribeToTaskRequest", len)?;
        if !self.tenant.is_empty() {
            struct_ser.serialize_field("tenant", &self.tenant)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscribeToTaskRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tenant,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tenant" => Ok(GeneratedField::Tenant),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubscribeToTaskRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.SubscribeToTaskRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscribeToTaskRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tenant__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubscribeToTaskRequest {
                    tenant: tenant__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.SubscribeToTaskRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Task {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.context_id.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if !self.artifacts.is_empty() {
            len += 1;
        }
        if !self.history.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.Task", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.context_id.is_empty() {
            struct_ser.serialize_field("contextId", &self.context_id)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if !self.artifacts.is_empty() {
            struct_ser.serialize_field("artifacts", &self.artifacts)?;
        }
        if !self.history.is_empty() {
            struct_ser.serialize_field("history", &self.history)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Task {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "context_id",
            "contextId",
            "status",
            "artifacts",
            "history",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ContextId,
            Status,
            Artifacts,
            History,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "contextId" | "context_id" => Ok(GeneratedField::ContextId),
                            "status" => Ok(GeneratedField::Status),
                            "artifacts" => Ok(GeneratedField::Artifacts),
                            "history" => Ok(GeneratedField::History),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Task;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.Task")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Task, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut context_id__ = None;
                let mut status__ = None;
                let mut artifacts__ = None;
                let mut history__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextId => {
                            if context_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextId"));
                            }
                            context_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::Artifacts => {
                            if artifacts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifacts"));
                            }
                            artifacts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::History => {
                            if history__.is_some() {
                                return Err(serde::de::Error::duplicate_field("history"));
                            }
                            history__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Task {
                    id: id__.unwrap_or_default(),
                    context_id: context_id__.unwrap_or_default(),
                    status: status__,
                    artifacts: artifacts__.unwrap_or_default(),
                    history: history__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.Task", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskArtifactUpdateEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.task_id.is_empty() {
            len += 1;
        }
        if !self.context_id.is_empty() {
            len += 1;
        }
        if self.artifact.is_some() {
            len += 1;
        }
        if self.append {
            len += 1;
        }
        if self.last_chunk {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.TaskArtifactUpdateEvent", len)?;
        if !self.task_id.is_empty() {
            struct_ser.serialize_field("taskId", &self.task_id)?;
        }
        if !self.context_id.is_empty() {
            struct_ser.serialize_field("contextId", &self.context_id)?;
        }
        if let Some(v) = self.artifact.as_ref() {
            struct_ser.serialize_field("artifact", v)?;
        }
        if self.append {
            struct_ser.serialize_field("append", &self.append)?;
        }
        if self.last_chunk {
            struct_ser.serialize_field("lastChunk", &self.last_chunk)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskArtifactUpdateEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task_id",
            "taskId",
            "context_id",
            "contextId",
            "artifact",
            "append",
            "last_chunk",
            "lastChunk",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaskId,
            ContextId,
            Artifact,
            Append,
            LastChunk,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "taskId" | "task_id" => Ok(GeneratedField::TaskId),
                            "contextId" | "context_id" => Ok(GeneratedField::ContextId),
                            "artifact" => Ok(GeneratedField::Artifact),
                            "append" => Ok(GeneratedField::Append),
                            "lastChunk" | "last_chunk" => Ok(GeneratedField::LastChunk),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskArtifactUpdateEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.TaskArtifactUpdateEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskArtifactUpdateEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task_id__ = None;
                let mut context_id__ = None;
                let mut artifact__ = None;
                let mut append__ = None;
                let mut last_chunk__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaskId => {
                            if task_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskId"));
                            }
                            task_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextId => {
                            if context_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextId"));
                            }
                            context_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Artifact => {
                            if artifact__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifact"));
                            }
                            artifact__ = map_.next_value()?;
                        }
                        GeneratedField::Append => {
                            if append__.is_some() {
                                return Err(serde::de::Error::duplicate_field("append"));
                            }
                            append__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastChunk => {
                            if last_chunk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastChunk"));
                            }
                            last_chunk__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TaskArtifactUpdateEvent {
                    task_id: task_id__.unwrap_or_default(),
                    context_id: context_id__.unwrap_or_default(),
                    artifact: artifact__,
                    append: append__.unwrap_or_default(),
                    last_chunk: last_chunk__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.TaskArtifactUpdateEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskPushNotificationConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.push_notification_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.TaskPushNotificationConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.push_notification_config.as_ref() {
            struct_ser.serialize_field("pushNotificationConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskPushNotificationConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "push_notification_config",
            "pushNotificationConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            PushNotificationConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "pushNotificationConfig" | "push_notification_config" => Ok(GeneratedField::PushNotificationConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskPushNotificationConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.TaskPushNotificationConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskPushNotificationConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut push_notification_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PushNotificationConfig => {
                            if push_notification_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pushNotificationConfig"));
                            }
                            push_notification_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TaskPushNotificationConfig {
                    name: name__.unwrap_or_default(),
                    push_notification_config: push_notification_config__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.TaskPushNotificationConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TASK_STATE_UNSPECIFIED",
            Self::Submitted => "TASK_STATE_SUBMITTED",
            Self::Working => "TASK_STATE_WORKING",
            Self::Completed => "TASK_STATE_COMPLETED",
            Self::Failed => "TASK_STATE_FAILED",
            Self::Cancelled => "TASK_STATE_CANCELLED",
            Self::InputRequired => "TASK_STATE_INPUT_REQUIRED",
            Self::Rejected => "TASK_STATE_REJECTED",
            Self::AuthRequired => "TASK_STATE_AUTH_REQUIRED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TaskState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TASK_STATE_UNSPECIFIED",
            "TASK_STATE_SUBMITTED",
            "TASK_STATE_WORKING",
            "TASK_STATE_COMPLETED",
            "TASK_STATE_FAILED",
            "TASK_STATE_CANCELLED",
            "TASK_STATE_INPUT_REQUIRED",
            "TASK_STATE_REJECTED",
            "TASK_STATE_AUTH_REQUIRED",
        ];

        struct GeneratedVisitor;

        impl serde::de::Visitor<'_> for GeneratedVisitor {
            type Value = TaskState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TASK_STATE_UNSPECIFIED" => Ok(TaskState::Unspecified),
                    "TASK_STATE_SUBMITTED" => Ok(TaskState::Submitted),
                    "TASK_STATE_WORKING" => Ok(TaskState::Working),
                    "TASK_STATE_COMPLETED" => Ok(TaskState::Completed),
                    "TASK_STATE_FAILED" => Ok(TaskState::Failed),
                    "TASK_STATE_CANCELLED" => Ok(TaskState::Cancelled),
                    "TASK_STATE_INPUT_REQUIRED" => Ok(TaskState::InputRequired),
                    "TASK_STATE_REJECTED" => Ok(TaskState::Rejected),
                    "TASK_STATE_AUTH_REQUIRED" => Ok(TaskState::AuthRequired),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TaskStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.TaskStatus", len)?;
        if self.state != 0 {
            let v = TaskState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "message",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            Message,
            Timestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "state" => Ok(GeneratedField::State),
                            "message" => Ok(GeneratedField::Message),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.TaskStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut message__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<TaskState>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map_.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TaskStatus {
                    state: state__.unwrap_or_default(),
                    message: message__,
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.TaskStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskStatusUpdateEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.task_id.is_empty() {
            len += 1;
        }
        if !self.context_id.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.r#final {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("a2a.v1.TaskStatusUpdateEvent", len)?;
        if !self.task_id.is_empty() {
            struct_ser.serialize_field("taskId", &self.task_id)?;
        }
        if !self.context_id.is_empty() {
            struct_ser.serialize_field("contextId", &self.context_id)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if self.r#final {
            struct_ser.serialize_field("final", &self.r#final)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskStatusUpdateEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "task_id",
            "taskId",
            "context_id",
            "contextId",
            "status",
            "final",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TaskId,
            ContextId,
            Status,
            Final,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "taskId" | "task_id" => Ok(GeneratedField::TaskId),
                            "contextId" | "context_id" => Ok(GeneratedField::ContextId),
                            "status" => Ok(GeneratedField::Status),
                            "final" => Ok(GeneratedField::Final),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskStatusUpdateEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct a2a.v1.TaskStatusUpdateEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TaskStatusUpdateEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut task_id__ = None;
                let mut context_id__ = None;
                let mut status__ = None;
                let mut r#final__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TaskId => {
                            if task_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskId"));
                            }
                            task_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextId => {
                            if context_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextId"));
                            }
                            context_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::Final => {
                            if r#final__.is_some() {
                                return Err(serde::de::Error::duplicate_field("final"));
                            }
                            r#final__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TaskStatusUpdateEvent {
                    task_id: task_id__.unwrap_or_default(),
                    context_id: context_id__.unwrap_or_default(),
                    status: status__,
                    r#final: r#final__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("a2a.v1.TaskStatusUpdateEvent", FIELDS, GeneratedVisitor)
    }
}
