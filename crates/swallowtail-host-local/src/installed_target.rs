use crate::{LocalExecutableLaunch, LocalProcessHostBuilder};
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
            .insert(reference.clone(), LocalExecutableLaunch::new(path));
        let target = InstalledExecutableTarget::new(reference, version_axis);
        (self, target)
    }

    /// Approves one exact native or interpreted launch and returns its opaque
    /// discovery target.
    #[must_use]
    pub fn approve_installed_executable_launch(
        mut self,
        reference: ExecutableRef,
        version_axis: InterfaceVersionAxis,
        launch: LocalExecutableLaunch,
    ) -> (Self, InstalledExecutableTarget) {
        self.approvals.executables.insert(reference.clone(), launch);
        let target = InstalledExecutableTarget::new(reference, version_axis);
        (self, target)
    }
}
