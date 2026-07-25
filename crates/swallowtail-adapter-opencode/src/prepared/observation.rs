use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersionBinding};

#[derive(Clone, Debug, Eq, PartialEq)]
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

    #[must_use]
    pub const fn binding(&self) -> &InterfaceVersionBinding {
        &self.binding
    }

    #[must_use]
    pub const fn compatibility(&self) -> &InterfaceCompatibilityAssessment {
        &self.compatibility
    }
}
