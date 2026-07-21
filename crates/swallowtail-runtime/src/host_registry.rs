use crate::{
    AttachmentService, BlockingWorkService, CredentialService, DiagnosticObserver,
    ModelArtifactService, NetworkPolicyService, ProcessService, RuntimeFailure, SchemaService,
    ScopedTaskService, ServingEndpointService, TimeService, WorkingResourceIoService,
    WorkingResourceService,
};
use std::collections::BTreeSet;
use std::sync::Arc;
use swallowtail_core::{ExecutionHostId, HostServiceKind, SafeDiagnostic};

#[derive(Clone)]
pub struct HostServices {
    execution_host_id: ExecutionHostId,
    task: Option<Arc<dyn ScopedTaskService>>,
    blocking_work: Option<Arc<dyn BlockingWorkService>>,
    time: Option<Arc<dyn TimeService>>,
    process: Option<Arc<dyn ProcessService>>,
    network: Option<Arc<dyn NetworkPolicyService>>,
    credential: Option<Arc<dyn CredentialService>>,
    working_resource: Option<Arc<dyn WorkingResourceService>>,
    working_resource_io: Option<Arc<dyn WorkingResourceIoService>>,
    attachment: Option<Arc<dyn AttachmentService>>,
    model_artifact: Option<Arc<dyn ModelArtifactService>>,
    serving_endpoint: Option<Arc<dyn ServingEndpointService>>,
    schema: Option<Arc<dyn SchemaService>>,
    diagnostic_observer: Option<Arc<dyn DiagnosticObserver>>,
}

impl HostServices {
    #[must_use]
    pub fn new(execution_host_id: ExecutionHostId) -> Self {
        Self {
            execution_host_id,
            task: None,
            blocking_work: None,
            time: None,
            process: None,
            network: None,
            credential: None,
            working_resource: None,
            working_resource_io: None,
            attachment: None,
            model_artifact: None,
            serving_endpoint: None,
            schema: None,
            diagnostic_observer: None,
        }
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    pub fn require_execution_host(&self, expected: &ExecutionHostId) -> Result<(), RuntimeFailure> {
        if &self.execution_host_id == expected {
            Ok(())
        } else {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "swallowtail.execution_host_mismatch",
                "Runtime services belong to a different execution host",
            )))
        }
    }

    #[must_use]
    pub fn with_task(mut self, service: Arc<dyn ScopedTaskService>) -> Self {
        self.task = Some(service);
        self
    }

    #[must_use]
    pub fn with_blocking_work(mut self, service: Arc<dyn BlockingWorkService>) -> Self {
        self.blocking_work = Some(service);
        self
    }

    #[must_use]
    pub fn with_time(mut self, service: Arc<dyn TimeService>) -> Self {
        self.time = Some(service);
        self
    }

    #[must_use]
    pub fn with_process(mut self, service: Arc<dyn ProcessService>) -> Self {
        self.process = Some(service);
        self
    }

    #[must_use]
    pub fn with_network(mut self, service: Arc<dyn NetworkPolicyService>) -> Self {
        self.network = Some(service);
        self
    }

    #[must_use]
    pub fn with_credential(mut self, service: Arc<dyn CredentialService>) -> Self {
        self.credential = Some(service);
        self
    }

    #[must_use]
    pub fn with_working_resource(mut self, service: Arc<dyn WorkingResourceService>) -> Self {
        self.working_resource = Some(service);
        self
    }

    #[must_use]
    pub fn with_working_resource_io(mut self, service: Arc<dyn WorkingResourceIoService>) -> Self {
        self.working_resource_io = Some(service);
        self
    }

    #[must_use]
    pub fn with_attachment(mut self, service: Arc<dyn AttachmentService>) -> Self {
        self.attachment = Some(service);
        self
    }

    #[must_use]
    pub fn with_model_artifact(mut self, service: Arc<dyn ModelArtifactService>) -> Self {
        self.model_artifact = Some(service);
        self
    }

    #[must_use]
    pub fn with_serving_endpoint(mut self, service: Arc<dyn ServingEndpointService>) -> Self {
        self.serving_endpoint = Some(service);
        self
    }

    #[must_use]
    pub fn with_schema(mut self, service: Arc<dyn SchemaService>) -> Self {
        self.schema = Some(service);
        self
    }

    #[must_use]
    pub fn with_diagnostic_observer(mut self, service: Arc<dyn DiagnosticObserver>) -> Self {
        self.diagnostic_observer = Some(service);
        self
    }

    #[must_use]
    pub fn task(&self) -> Option<&Arc<dyn ScopedTaskService>> {
        self.task.as_ref()
    }

    #[must_use]
    pub fn blocking_work(&self) -> Option<&Arc<dyn BlockingWorkService>> {
        self.blocking_work.as_ref()
    }

    #[must_use]
    pub fn time(&self) -> Option<&Arc<dyn TimeService>> {
        self.time.as_ref()
    }

    #[must_use]
    pub fn process(&self) -> Option<&Arc<dyn ProcessService>> {
        self.process.as_ref()
    }

    #[must_use]
    pub fn network(&self) -> Option<&Arc<dyn NetworkPolicyService>> {
        self.network.as_ref()
    }

    #[must_use]
    pub fn credential(&self) -> Option<&Arc<dyn CredentialService>> {
        self.credential.as_ref()
    }

    #[must_use]
    pub fn working_resource(&self) -> Option<&Arc<dyn WorkingResourceService>> {
        self.working_resource.as_ref()
    }

    #[must_use]
    pub fn working_resource_io(&self) -> Option<&Arc<dyn WorkingResourceIoService>> {
        self.working_resource_io.as_ref()
    }

    #[must_use]
    pub fn attachment(&self) -> Option<&Arc<dyn AttachmentService>> {
        self.attachment.as_ref()
    }

    #[must_use]
    pub fn model_artifact(&self) -> Option<&Arc<dyn ModelArtifactService>> {
        self.model_artifact.as_ref()
    }

    #[must_use]
    pub fn serving_endpoint(&self) -> Option<&Arc<dyn ServingEndpointService>> {
        self.serving_endpoint.as_ref()
    }

    #[must_use]
    pub fn schema(&self) -> Option<&Arc<dyn SchemaService>> {
        self.schema.as_ref()
    }

    #[must_use]
    pub fn diagnostic_observer(&self) -> Option<&Arc<dyn DiagnosticObserver>> {
        self.diagnostic_observer.as_ref()
    }

    #[must_use]
    pub fn available_kinds(&self) -> BTreeSet<HostServiceKind> {
        let mut kinds = BTreeSet::new();
        if self.task.is_some() {
            kinds.insert(HostServiceKind::Task);
        }
        if self.blocking_work.is_some() {
            kinds.insert(HostServiceKind::BlockingWork);
        }
        if self.time.is_some() {
            kinds.insert(HostServiceKind::Time);
        }
        if self.process.is_some() {
            kinds.insert(HostServiceKind::Process);
        }
        if self.network.is_some() {
            kinds.insert(HostServiceKind::Network);
        }
        if self.credential.is_some() {
            kinds.insert(HostServiceKind::Credential);
        }
        if self.working_resource.is_some() {
            kinds.insert(HostServiceKind::WorkingResource);
        }
        if self.working_resource_io.is_some() {
            kinds.insert(HostServiceKind::WorkingResourceIo);
        }
        if self.attachment.is_some() {
            kinds.insert(HostServiceKind::Attachment);
        }
        if self.model_artifact.is_some() {
            kinds.insert(HostServiceKind::ModelArtifact);
        }
        if self.serving_endpoint.is_some() {
            kinds.insert(HostServiceKind::ServingEndpoint);
        }
        if self.schema.is_some() {
            kinds.insert(HostServiceKind::Schema);
        }
        if self.diagnostic_observer.is_some() {
            kinds.insert(HostServiceKind::DiagnosticObserver);
        }
        kinds
    }
}

#[cfg(test)]
mod tests {
    use super::HostServices;
    use swallowtail_core::ExecutionHostId;

    #[test]
    fn service_registry_rejects_a_different_execution_host() {
        let local = ExecutionHostId::new("host.local").expect("host id is valid");
        let remote = ExecutionHostId::new("host.remote").expect("host id is valid");
        let services = HostServices::new(local.clone());

        services
            .require_execution_host(&local)
            .expect("matching host is accepted");
        let failure = services
            .require_execution_host(&remote)
            .expect_err("different host is rejected");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.execution_host_mismatch"
        );
        assert!(!format!("{failure}").contains(remote.as_str()));
    }
}
