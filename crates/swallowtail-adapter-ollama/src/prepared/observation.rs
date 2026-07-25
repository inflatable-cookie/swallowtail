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
}

impl OllamaPreparedRuntimeObservation {
    pub(super) fn new(
        runtime_version: InterfaceVersionBinding,
        installed: Vec<AttachedModelObservation>,
        running: Vec<AttachedModelObservation>,
        selected_detail: AttachedModelObservation,
    ) -> Self {
        let compatibility = crate::ollama_runtime_claim().assess(runtime_version.version());
        Self {
            runtime_version,
            compatibility,
            installed,
            running,
            selected_detail,
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
}
