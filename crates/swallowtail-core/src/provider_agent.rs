use crate::diagnostic::{ValueRequired, required_text};
use std::fmt;

/// Opaque provider-owned agent-definition identity.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderAgentId(String);

impl ProviderAgentId {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("provider agent id", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ProviderAgentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProviderAgentId(<redacted>)")
    }
}

/// Opaque provider-owned agent-definition version.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderAgentVersion(String);

impl ProviderAgentVersion {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("provider agent version", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ProviderAgentVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProviderAgentVersion(<redacted>)")
    }
}

/// Exact operator-owned provider agent definition selected by an instance.
#[derive(Clone, Eq, PartialEq)]
pub struct ProviderAgentBinding {
    id: ProviderAgentId,
    version: ProviderAgentVersion,
}

impl ProviderAgentBinding {
    #[must_use]
    pub const fn new(id: ProviderAgentId, version: ProviderAgentVersion) -> Self {
        Self { id, version }
    }

    #[must_use]
    pub const fn id(&self) -> &ProviderAgentId {
        &self.id
    }

    #[must_use]
    pub const fn version(&self) -> &ProviderAgentVersion {
        &self.version
    }
}

impl fmt::Debug for ProviderAgentBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderAgentBinding")
            .field("id", &self.id)
            .field("version", &self.version)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{ProviderAgentBinding, ProviderAgentId, ProviderAgentVersion};

    #[test]
    fn binding_is_typed_and_redacted() {
        let binding = ProviderAgentBinding::new(
            ProviderAgentId::new("agent-private").expect("agent id is valid"),
            ProviderAgentVersion::new("7-private").expect("agent version is valid"),
        );

        assert_eq!(binding.id().as_str(), "agent-private");
        assert_eq!(binding.version().as_str(), "7-private");
        let rendered = format!("{binding:?}");
        assert!(!rendered.contains("agent-private"));
        assert!(!rendered.contains("7-private"));
    }
}
