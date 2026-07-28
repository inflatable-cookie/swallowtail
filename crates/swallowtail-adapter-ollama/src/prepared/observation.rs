use std::collections::BTreeSet;
use swallowtail_core::{
    AttachedModelObservation, InterfaceCompatibilityAssessment, InterfaceVersionBinding,
};

#[derive(Clone, Debug, Eq, PartialEq)]
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

    #[must_use]
    pub const fn runtime_version(&self) -> &InterfaceVersionBinding {
        &self.runtime_version
    }

    #[must_use]
    pub const fn compatibility(&self) -> &InterfaceCompatibilityAssessment {
        &self.compatibility
    }

    pub fn installed(&self) -> impl ExactSizeIterator<Item = &AttachedModelObservation> {
        self.installed.iter()
    }

    pub fn running(&self) -> impl ExactSizeIterator<Item = &AttachedModelObservation> {
        self.running.iter()
    }

    #[must_use]
    pub const fn selected_detail(&self) -> &AttachedModelObservation {
        &self.selected_detail
    }

    pub fn selected_capabilities(
        &self,
    ) -> impl ExactSizeIterator<Item = crate::OllamaModelCapability> + '_ {
        self.selected_capabilities.iter().copied()
    }

    #[must_use]
    pub fn selected_model_supports(&self, capability: crate::OllamaModelCapability) -> bool {
        self.selected_capabilities.contains(&capability)
    }
}
