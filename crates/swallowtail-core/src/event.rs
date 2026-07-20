use crate::diagnostic::{SafeDiagnostic, ValueRequired, required_text};
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventKind {
    Started,
    Progress,
    OutputAvailable,
    ToolCallRequested,
    Completed,
    Failed,
    Interrupted,
}

/// Provider-owned namespace for an extension payload.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExtensionNamespace(String);

impl ExtensionNamespace {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("extension namespace", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Opaque provider data which is excluded from default formatting.
#[derive(Clone, Eq, PartialEq)]
pub struct ProviderExtension {
    namespace: ExtensionNamespace,
    payload: Vec<u8>,
}

impl ProviderExtension {
    #[must_use]
    pub const fn new(namespace: ExtensionNamespace, payload: Vec<u8>) -> Self {
        Self { namespace, payload }
    }

    #[must_use]
    pub const fn namespace(&self) -> &ExtensionNamespace {
        &self.namespace
    }

    /// Passes uninterpreted bytes back to provider-aware code.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl fmt::Debug for ProviderExtension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderExtension")
            .field("namespace", &self.namespace)
            .field(
                "payload",
                &format_args!("<opaque:{} bytes>", self.payload.len()),
            )
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExtensionPolicy {
    Preserve,
    Reject,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventEnvelope {
    sequence: u64,
    kind: EventKind,
    extension: Option<ProviderExtension>,
}

impl EventEnvelope {
    #[must_use]
    pub const fn common(sequence: u64, kind: EventKind) -> Self {
        Self {
            sequence,
            kind,
            extension: None,
        }
    }

    #[must_use]
    pub const fn with_extension(
        sequence: u64,
        kind: EventKind,
        extension: ProviderExtension,
    ) -> Self {
        Self {
            sequence,
            kind,
            extension: Some(extension),
        }
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn kind(&self) -> EventKind {
        self.kind
    }

    #[must_use]
    pub const fn extension(&self) -> Option<&ProviderExtension> {
        self.extension.as_ref()
    }

    pub fn apply_extension_policy(
        self,
        policy: ExtensionPolicy,
    ) -> Result<Self, ExtensionRejected> {
        match (policy, self.extension.as_ref()) {
            (ExtensionPolicy::Reject, Some(extension)) => {
                Err(ExtensionRejected::new(extension.namespace.clone()))
            }
            _ => Ok(self),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtensionRejected {
    namespace: ExtensionNamespace,
    diagnostic: SafeDiagnostic,
}

impl ExtensionRejected {
    fn new(namespace: ExtensionNamespace) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.provider_extension_rejected",
                format!("Provider extension '{}' was rejected", namespace.as_str()),
            ),
            namespace,
        }
    }

    #[must_use]
    pub const fn namespace(&self) -> &ExtensionNamespace {
        &self.namespace
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for ExtensionRejected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for ExtensionRejected {}

#[cfg(test)]
mod tests {
    use super::{EventEnvelope, EventKind, ExtensionNamespace, ExtensionPolicy, ProviderExtension};

    #[test]
    fn extension_payload_is_isolated_and_redacted() {
        let extension = ProviderExtension::new(
            ExtensionNamespace::new("example.provider").expect("namespace is valid"),
            b"raw secret payload".to_vec(),
        );
        let event = EventEnvelope::with_extension(4, EventKind::Progress, extension);

        let rendered = format!("{event:?}");
        assert!(rendered.contains("example.provider"));
        assert!(!rendered.contains("raw secret payload"));
        assert_eq!(
            event.extension().expect("extension is present").payload(),
            b"raw secret payload"
        );
    }

    #[test]
    fn extension_policy_preserves_or_rejects_deliberately() {
        let extension = ProviderExtension::new(
            ExtensionNamespace::new("example.provider").expect("namespace is valid"),
            vec![1, 2, 3],
        );
        let event = EventEnvelope::with_extension(5, EventKind::Progress, extension);

        event
            .clone()
            .apply_extension_policy(ExtensionPolicy::Preserve)
            .expect("preserve policy must retain the event");
        let error = event
            .apply_extension_policy(ExtensionPolicy::Reject)
            .expect_err("reject policy must fail explicitly");

        assert_eq!(error.namespace().as_str(), "example.provider");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.provider_extension_rejected"
        );
    }
}
