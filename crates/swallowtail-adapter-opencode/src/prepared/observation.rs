use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersionBinding};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact server-version evidence retained by a prepared integration.
pub struct OpenCodePreparedServerObservation {
    binding: InterfaceVersionBinding,
    compatibility: InterfaceCompatibilityAssessment,
}

impl OpenCodePreparedServerObservation {
    pub(super) const fn new(
        binding: InterfaceVersionBinding,
        compatibility: InterfaceCompatibilityAssessment,
    ) -> Self {
        Self {
            binding,
            compatibility,
        }
    }

    /// Returns the observed semantic-version binding.
    #[must_use]
    pub const fn binding(&self) -> &InterfaceVersionBinding {
        &self.binding
    }

    /// Returns the compatibility assessment for the observed server.
    #[must_use]
    pub const fn compatibility(&self) -> &InterfaceCompatibilityAssessment {
        &self.compatibility
    }

    /// Reports whether the version has qualified behavior evidence.
    #[must_use]
    pub const fn is_qualified(&self) -> bool {
        matches!(
            self.compatibility,
            InterfaceCompatibilityAssessment::Qualified(_)
        )
    }
}
