use crate::diagnostic::SafeDiagnostic;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

/// A provider feature which a host must check before use.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Capability {
    ModelCatalog,
    InteractiveSession,
    StructuredRun,
    StreamingEvents,
    ToolCalls,
    Interruption,
    Resume,
    StructuredOutput,
    Attachments,
    ReasoningSelection,
    WorkingResource,
    ProviderExternalNetwork,
    ExternalSearch,
    UsageReporting,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CapabilityManifest {
    supported: BTreeSet<Capability>,
}

impl CapabilityManifest {
    #[must_use]
    pub fn new(supported: impl IntoIterator<Item = Capability>) -> Self {
        Self {
            supported: supported.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn supports(&self, capability: Capability) -> bool {
        self.supported.contains(&capability)
    }

    pub fn require(&self, capability: Capability) -> Result<(), UnsupportedCapability> {
        if self.supports(capability) {
            Ok(())
        } else {
            Err(UnsupportedCapability::new(capability))
        }
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = Capability> + '_ {
        self.supported.iter().copied()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedCapability {
    capability: Capability,
    diagnostic: SafeDiagnostic,
}

impl UnsupportedCapability {
    fn new(capability: Capability) -> Self {
        Self {
            capability,
            diagnostic: SafeDiagnostic::new(
                "swallowtail.unsupported_capability",
                format!("Adapter does not support {capability:?}"),
            ),
        }
    }

    #[must_use]
    pub const fn capability(&self) -> Capability {
        self.capability
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for UnsupportedCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for UnsupportedCapability {}

#[cfg(test)]
mod tests {
    use super::{Capability, CapabilityManifest};

    #[test]
    fn unsupported_capability_fails_before_execution() {
        let manifest = CapabilityManifest::new([Capability::StructuredRun]);

        manifest
            .require(Capability::StructuredRun)
            .expect("declared capability must pass");
        let error = manifest
            .require(Capability::Resume)
            .expect_err("undeclared capability must fail");

        assert_eq!(error.capability(), Capability::Resume);
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.unsupported_capability"
        );
    }
}
