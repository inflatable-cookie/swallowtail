use crate::containment::ProcessContainmentBackend;
use crate::credential::LocalCredentialLeaseState;
use crate::executable_launch::LocalExecutableLaunch;
use crate::hosted::{LocalCredentialApproval, LocalEndpointApproval};
use crate::limits::{LocalMaterializationLimits, LocalProcessLimits};
use crate::materialization::LocalMaterializationState;
use crate::model_artifact::{LocalModelArtifactApproval, LocalModelArtifactLeaseState};
use crate::serving_endpoint::LocalServingEndpointState;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use swallowtail_core::{
    DEFAULT_MAX_WATCHERS_PER_TURN, ExecutionHostId, ModelArtifactRef, WatcherOperationData,
};
use swallowtail_runtime::{
    AttachmentRef, CredentialRef, EndpointRef, EnvironmentRef, ExecutableRef, ProcessRequest,
    SchemaRef, WorkingResourceRef,
};

type ProcessContainmentFactory =
    Box<dyn FnOnce(Arc<LocalProcessHost>) -> Arc<dyn ProcessContainmentBackend> + Send>;

type EnvironmentValues = Vec<(OsString, OsString)>;

#[derive(Default)]
pub(crate) struct LocalApprovals {
    pub(crate) executables: HashMap<ExecutableRef, LocalExecutableLaunch>,
    pub(crate) environments: HashMap<EnvironmentRef, EnvironmentValues>,
    pub(crate) working_resources: HashMap<WorkingResourceRef, PathBuf>,
    pub(crate) attachments: HashMap<AttachmentRef, PathBuf>,
    pub(crate) schemas: HashMap<SchemaRef, PathBuf>,
    pub(crate) endpoints: HashMap<EndpointRef, LocalEndpointApproval>,
    pub(crate) credentials: HashMap<CredentialRef, LocalCredentialApproval>,
    pub(crate) model_artifacts: HashMap<ModelArtifactRef, LocalModelArtifactApproval>,
    pub(crate) watcher_operations: HashMap<WatcherOperationData, ProcessRequest>,
}

/// Builder for an allowlisted local process and materialization host.
pub struct LocalProcessHostBuilder {
    limits: LocalProcessLimits,
    materialization_limits: LocalMaterializationLimits,
    pub(crate) temporary_root: PathBuf,
    pub(crate) execution_host_id: Option<ExecutionHostId>,
    pub(crate) approvals: LocalApprovals,
    pub(crate) watcher_capacity: usize,
    pub(crate) process_containment: Option<Arc<dyn ProcessContainmentBackend>>,
    pub(crate) process_containment_factory: Option<ProcessContainmentFactory>,
}

impl LocalProcessHostBuilder {
    /// Approves one exact executable path behind an opaque reference.
    #[must_use]
    pub fn approve_executable(
        mut self,
        reference: ExecutableRef,
        path: impl Into<PathBuf>,
    ) -> Self {
        self.approvals
            .executables
            .insert(reference, LocalExecutableLaunch::new(path));
        self
    }

    /// Approves one exact native or interpreted launch behind an opaque
    /// executable reference.
    #[must_use]
    pub fn approve_executable_launch(
        mut self,
        reference: ExecutableRef,
        launch: LocalExecutableLaunch,
    ) -> Self {
        self.approvals.executables.insert(reference, launch);
        self
    }

    /// Approves an exact environment binding behind an opaque reference.
    #[must_use]
    pub fn approve_environment(
        mut self,
        reference: EnvironmentRef,
        values: impl IntoIterator<Item = (OsString, OsString)>,
    ) -> Self {
        self.approvals
            .environments
            .insert(reference, values.into_iter().collect());
        self
    }

    /// Approves an existing working-resource path behind an opaque reference.
    #[must_use]
    pub fn approve_working_resource(
        mut self,
        reference: WorkingResourceRef,
        path: impl Into<PathBuf>,
    ) -> Self {
        self.approvals
            .working_resources
            .insert(reference, path.into());
        self
    }

    /// Approves an attachment path behind an opaque reference.
    #[must_use]
    pub fn approve_attachment(
        mut self,
        reference: AttachmentRef,
        path: impl Into<PathBuf>,
    ) -> Self {
        self.approvals.attachments.insert(reference, path.into());
        self
    }

    /// Approves a structured-output schema path behind an opaque reference.
    #[must_use]
    pub fn approve_schema(mut self, reference: SchemaRef, path: impl Into<PathBuf>) -> Self {
        self.approvals.schemas.insert(reference, path.into());
        self
    }

    /// Approves one exact watcher operation-data value and its host-owned
    /// process recipe.
    ///
    /// Callers can submit only the operation-data value to the watcher
    /// service. The process request remains private to this host policy.
    #[must_use]
    pub fn approve_watcher_operation(
        mut self,
        operation_data: WatcherOperationData,
        request: ProcessRequest,
    ) -> Self {
        self.approvals
            .watcher_operations
            .insert(operation_data, request);
        self
    }

    /// Replaces the positive per-turn watcher capacity.
    #[must_use]
    pub fn with_watcher_capacity(mut self, capacity: usize) -> Self {
        self.watcher_capacity = capacity;
        self
    }

    /// Supplies the exact Contract 059 containment backend for process-backed
    /// watcher starts.
    ///
    /// Default local composition omits this backend. Process groups and the
    /// ordinary process service do not satisfy the gate.
    #[must_use]
    pub fn with_process_containment(mut self, backend: Arc<dyn ProcessContainmentBackend>) -> Self {
        self.process_containment = Some(backend);
        self.process_containment_factory = None;
        self
    }

    /// Builds the host first, then installs a containment backend constructed
    /// from that host.
    ///
    /// Use this when the backend needs the completed local process host. The
    /// factory must supply an exact Contract 059 backend; ordinary process
    /// handles do not qualify merely by being wrapped.
    #[must_use]
    pub fn with_process_containment_factory<F>(mut self, factory: F) -> Self
    where
        F: FnOnce(Arc<LocalProcessHost>) -> Arc<dyn ProcessContainmentBackend> + Send + 'static,
    {
        self.process_containment = None;
        self.process_containment_factory = Some(Box::new(factory));
        self
    }

    /// Replaces the default attachment and schema materialization limits.
    #[must_use]
    pub fn with_materialization_limits(mut self, limits: LocalMaterializationLimits) -> Self {
        self.materialization_limits = limits;
        self
    }

    /// Builds the local host without composing a complete service registry.
    #[must_use]
    pub fn build(self) -> LocalProcessHost {
        let factory = self.process_containment_factory;
        let mut host = LocalProcessHost {
            limits: self.limits,
            materialization_limits: self.materialization_limits,
            approvals: Arc::new(self.approvals),
            materialization: Arc::new(LocalMaterializationState::new(self.temporary_root)),
            credential_leases: Arc::new(LocalCredentialLeaseState::default()),
            model_artifact_leases: Arc::new(LocalModelArtifactLeaseState::default()),
            serving_endpoints: Arc::new(LocalServingEndpointState::default()),
            execution_host_id: self.execution_host_id,
            watcher_capacity: self.watcher_capacity,
            process_containment: self.process_containment,
            monotonic_origin: Instant::now(),
        };
        if let Some(factory) = factory {
            let shared = Arc::new(host.clone());
            host.process_containment = Some(factory(shared));
        }
        host
    }
}

/// Local implementation of process, materialization, credential, and network services.
#[derive(Clone)]
pub struct LocalProcessHost {
    pub(crate) limits: LocalProcessLimits,
    pub(crate) materialization_limits: LocalMaterializationLimits,
    pub(crate) approvals: Arc<LocalApprovals>,
    pub(crate) materialization: Arc<LocalMaterializationState>,
    pub(crate) credential_leases: Arc<LocalCredentialLeaseState>,
    pub(crate) model_artifact_leases: Arc<LocalModelArtifactLeaseState>,
    pub(crate) serving_endpoints: Arc<LocalServingEndpointState>,
    pub(crate) execution_host_id: Option<ExecutionHostId>,
    pub(crate) watcher_capacity: usize,
    pub(crate) process_containment: Option<Arc<dyn ProcessContainmentBackend>>,
    pub(crate) monotonic_origin: Instant,
}

impl LocalProcessHost {
    /// Starts a builder with explicit process limits and conservative defaults.
    #[must_use]
    pub fn builder(limits: LocalProcessLimits) -> LocalProcessHostBuilder {
        LocalProcessHostBuilder {
            limits,
            materialization_limits: LocalMaterializationLimits::default(),
            temporary_root: std::env::temp_dir(),
            execution_host_id: None,
            approvals: LocalApprovals::default(),
            watcher_capacity: DEFAULT_MAX_WATCHERS_PER_TURN,
            process_containment: None,
            process_containment_factory: None,
        }
    }

    /// Returns the injected process-containment backend when one was supplied.
    #[must_use]
    pub fn process_containment(&self) -> Option<&Arc<dyn ProcessContainmentBackend>> {
        self.process_containment.as_ref()
    }
}
