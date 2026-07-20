use crate::diagnostic::{ValueRequired, required_text};

/// Stable adapter identity independent of its display name or version.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdapterId(String);

impl AdapterId {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("adapter id", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Provider-reported adapter implementation version.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdapterVersion(String);

impl AdapterVersion {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("adapter version", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterIdentity {
    id: AdapterId,
    version: AdapterVersion,
}

impl AdapterIdentity {
    #[must_use]
    pub const fn new(id: AdapterId, version: AdapterVersion) -> Self {
        Self { id, version }
    }

    #[must_use]
    pub const fn id(&self) -> &AdapterId {
        &self.id
    }

    #[must_use]
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
