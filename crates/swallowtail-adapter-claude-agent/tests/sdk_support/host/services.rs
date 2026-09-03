//! Host-service composition for the provider-free SDK fixture.

use super::super::host::{FixtureReaper, SdkFixtureHost};
use super::task_service::ThreadTaskService;
use std::sync::Arc;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::HostServices;

impl SdkFixtureHost {
    /// Composes the real local host so its own reservation, reapable-task,
    /// and outer reaper lifecycle back this fixture's task seam.
    #[must_use]
    pub fn local_task_composition(
        host: ExecutionHostId,
    ) -> swallowtail_host_local::LocalHostServices {
        swallowtail_host_local::LocalProcessHost::builder(
            swallowtail_host_local::LocalProcessLimits::default(),
        )
        .build_services(host)
    }

    /// Services whose task seam is the real local host's, so every guardian
    /// runs behind a handle that owns its worker thread, joins on drop, and is
    /// reaped by the outer owner rather than by this route.
    pub fn services_with_local_tasks(
        &self,
        host: ExecutionHostId,
        local: &swallowtail_host_local::LocalHostServices,
    ) -> HostServices {
        self.services(host).with_task(
            Arc::clone(local.task_service()) as Arc<dyn swallowtail_runtime::ScopedTaskService>
        )
    }

    /// The fixture's outer reaper owner.
    #[must_use]
    pub fn reaper(&self) -> Arc<FixtureReaper> {
        Arc::clone(
            self.shared
                .reaper
                .lock()
                .expect("SDK fixture reaper lock poisoned")
                .as_ref()
                .expect("SDK fixture services were composed"),
        )
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        let (task_service, relinquished, reaper) = ThreadTaskService::new(host.clone());
        *self
            .shared
            .reaper
            .lock()
            .expect("SDK fixture reaper lock poisoned") = Some(reaper);
        *self
            .shared
            .relinquished
            .lock()
            .expect("SDK fixture relinquish lock poisoned") = Some(relinquished);
        HostServices::new(host)
            .with_task(Arc::new(task_service))
            .with_process(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_time(Arc::new(self.clone()))
    }
}
