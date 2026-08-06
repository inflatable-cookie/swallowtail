use crate::diagnostic::{ValueRequired, required_text};

/// Stable adapter identity independent of its display name or version.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdapterId(String);

impl AdapterId {
    /// Creates an adapter identity after rejecting blank text.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("adapter id", value).map(Self)
    }

    #[must_use]
    /// Returns stable adapter identity text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Provider-reported adapter implementation version.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdapterVersion(String);

impl AdapterVersion {
    /// Creates an adapter version after rejecting blank text.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("adapter version", value).map(Self)
    }

    #[must_use]
    /// Returns provider-reported adapter version text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Stable adapter identity coupled with its implementation version.
pub struct AdapterIdentity {
    id: AdapterId,
    version: AdapterVersion,
}

impl AdapterIdentity {
    #[must_use]
    /// Creates an exact adapter identity and version pair.
    pub const fn new(id: AdapterId, version: AdapterVersion) -> Self {
        Self { id, version }
    }

    #[must_use]
    /// Returns stable adapter identity.
    pub const fn id(&self) -> &AdapterId {
        &self.id
    }

    #[must_use]
    /// Returns adapter implementation version.
    pub const fn version(&self) -> &AdapterVersion {
        &self.version
    }
}

#[cfg(test)]
mod tests {
    use super::AdapterId;

    #[test]
    fn adapter_id_rejects_blank_text() {
        let error = AdapterId::new("  ").expect_err("blank identity must fail");
        assert_eq!(error.field(), "adapter id");
        assert_eq!(error.diagnostic().code(), "swallowtail.value_required");
    }
}
