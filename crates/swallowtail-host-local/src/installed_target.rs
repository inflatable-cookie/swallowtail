use crate::LocalProcessHostBuilder;
use std::path::PathBuf;
use swallowtail_core::InterfaceVersionAxis;
use swallowtail_runtime::{ExecutableRef, InstalledExecutableTarget};

impl LocalProcessHostBuilder {
    /// Approves one exact executable and returns its opaque discovery target.
    #[must_use]
    pub fn approve_installed_executable(
        mut self,
        reference: ExecutableRef,
        version_axis: InterfaceVersionAxis,
        path: impl Into<PathBuf>,
    ) -> (Self, InstalledExecutableTarget) {
        self.approvals
            .executables
            .insert(reference.clone(), path.into());
        let target = InstalledExecutableTarget::new(reference, version_axis);
        (self, target)
    }
}
