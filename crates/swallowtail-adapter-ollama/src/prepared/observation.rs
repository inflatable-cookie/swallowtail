use std::collections::BTreeSet;
use swallowtail_core::{
    AttachedModelObservation, InterfaceCompatibilityAssessment, InterfaceVersionBinding,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Qualified runtime plus installed, running, and selected-model evidence.
pub struct OllamaPreparedRuntimeObservation {
    runtime_version: InterfaceVersionBinding,
    compatibility: InterfaceCompatibilityAssessment,
    installed: Vec<AttachedModelObservation>,
    running: Vec<AttachedModelObservation>,
    selected_detail: AttachedModelObservation,
    selected_capabilities: BTreeSet<crate::OllamaModelCapability>,
}

impl OllamaPreparedRuntimeObservation {
    pub(super) fn new(
        runtime_version: InterfaceVersionBinding,
        installed: Vec<AttachedModelObservation>,
        running: Vec<AttachedModelObservation>,
        selected_detail: crate::protocol::SelectedModelDetail,
    ) -> Self {
        let compatibility = crate::ollama_runtime_claim().assess(runtime_version.version());
        let (selected_detail, selected_capabilities) = selected_detail.into_parts();
        Self {
            runtime_version,
            compatibility,
            installed,
            running,
            selected_detail,
            selected_capabilities,
        }
    }

    /// Returns the exact observed runtime-version binding.
    #[must_use]
    pub const fn runtime_version(&self) -> &InterfaceVersionBinding {
        &self.runtime_version
    }

    /// Returns the compatibility assessment for the observed runtime.
    #[must_use]
    pub const fn compatibility(&self) -> &InterfaceCompatibilityAssessment {
        &self.compatibility
    }

    /// Iterates the bounded installed-model observations.
    pub fn installed(&self) -> impl ExactSizeIterator<Item = &AttachedModelObservation> {
        self.installed.iter()
    }

    /// Iterates the bounded running-model observations.
    pub fn running(&self) -> impl ExactSizeIterator<Item = &AttachedModelObservation> {
        self.running.iter()
    }

    /// Returns detail for the exact selected tag and manifest digest.
    #[must_use]
    pub const fn selected_detail(&self) -> &AttachedModelObservation {
        &self.selected_detail
    }

    /// Iterates capabilities admitted for the selected model.
    pub fn selected_capabilities(
        &self,
    ) -> impl ExactSizeIterator<Item = crate::OllamaModelCapability> + '_ {
        self.selected_capabilities.iter().copied()
    }

    /// Reports whether the selected model exposes one admitted capability.
    #[must_use]
    pub fn selected_model_supports(&self, capability: crate::OllamaModelCapability) -> bool {
        self.selected_capabilities.contains(&capability)
    }
}
