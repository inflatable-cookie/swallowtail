use crate::child::LocalProcessHandle;
use crate::credential::LocalCredentialLeaseState;
use crate::executable_launch::{
    LocalExecutableLaunch, MAXIMUM_BOOTSTRAP_ENVIRONMENT_BINDINGS,
    MAXIMUM_BOOTSTRAP_ENVIRONMENT_BYTES,
};
use crate::hosted::{LocalCredentialApproval, LocalEndpointApproval};
use crate::limits::{LocalMaterializationLimits, LocalProcessLimits};
use crate::materialization::LocalMaterializationState;
use crate::model_artifact::{LocalModelArtifactApproval, LocalModelArtifactLeaseState};
use crate::output::failure;
use crate::serving_endpoint::LocalServingEndpointState;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Instant;
use swallowtail_core::{ExecutionHostId, ModelArtifactRef};
use swallowtail_runtime::{
    AttachmentRef, BoxFuture, CredentialRef, EndpointRef, EnvironmentRef, ExecutableRef,
    ProcessHandle, ProcessRequest, ProcessService, RuntimeFailure, SchemaRef, ScopeId,
    WorkingResourceRef,
};

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
}

/// Builder for an allowlisted local process and materialization host.
pub struct LocalProcessHostBuilder {
    limits: LocalProcessLimits,
    materialization_limits: LocalMaterializationLimits,
    pub(crate) temporary_root: PathBuf,
    pub(crate) execution_host_id: Option<ExecutionHostId>,
    pub(crate) approvals: LocalApprovals,
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

    /// Replaces the default attachment and schema materialization limits.
    #[must_use]
    pub fn with_materialization_limits(mut self, limits: LocalMaterializationLimits) -> Self {
        self.materialization_limits = limits;
        self
    }

    /// Builds the local host without composing a complete service registry.
    #[must_use]
    pub fn build(self) -> LocalProcessHost {
        LocalProcessHost {
            limits: self.limits,
            materialization_limits: self.materialization_limits,
            approvals: Arc::new(self.approvals),
            materialization: Arc::new(LocalMaterializationState::new(self.temporary_root)),
            credential_leases: Arc::new(LocalCredentialLeaseState::default()),
            model_artifact_leases: Arc::new(LocalModelArtifactLeaseState::default()),
            serving_endpoints: Arc::new(LocalServingEndpointState::default()),
            execution_host_id: self.execution_host_id,
            monotonic_origin: Instant::now(),
        }
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
        }
    }

    fn start_process(
        &self,
        scope: &ScopeId,
        request: ProcessRequest,
    ) -> Result<Box<dyn ProcessHandle>, RuntimeFailure> {
        self.validate_arguments(&request)?;
        let launch = self
            .approvals
            .executables
            .get(request.executable())
            .ok_or_else(|| {
                failure(
                    "swallowtail.local_process.executable_not_approved",
                    "Local executable reference is not approved",
                )
            })?;
        self.validate_launch(launch, &request)?;
        let mut command = Command::new(launch.program());
        command
            .args(launch.prefix_arguments())
            .args(request.arguments())
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command.envs(launch.bootstrap_environment().iter().cloned());
        self.apply_environment(&mut command, &request)?;
        self.apply_working_resource(&mut command, scope, &request)?;
        let mut child = command.spawn().map_err(|_| {
            failure(
                "swallowtail.local_process.spawn_failed",
                "Local process could not be started",
            )
        })?;
        let stdin = child.stdin.take().ok_or_else(|| {
            failure(
                "swallowtail.local_process.stdin_unavailable",
                "Local process input is unavailable",
            )
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            failure(
                "swallowtail.local_process.stdout_unavailable",
                "Local process output is unavailable",
            )
        })?;
        let stderr = child.stderr.take().ok_or_else(|| {
            failure(
                "swallowtail.local_process.stderr_unavailable",
                "Local process error output is unavailable",
            )
        })?;
        LocalProcessHandle::supervise(
            child,
            stdin,
            stdout,
            stderr,
            self.limits.stdin_bytes(),
            self.limits.stdout_bytes(),
            self.limits.stderr_bytes(),
        )
        .map(|handle| Box::new(handle) as Box<dyn ProcessHandle>)
    }

    fn validate_arguments(&self, request: &ProcessRequest) -> Result<(), RuntimeFailure> {
        let count = request.arguments().len();
        let bytes = request.arguments().map(str::len).sum::<usize>();
        if count > self.limits.arguments() || bytes > self.limits.argument_bytes() {
            Err(failure(
                "swallowtail.local_process.argument_limit_exceeded",
                "Local process arguments exceeded host-approved limits",
            ))
        } else {
            Ok(())
        }
    }

    fn validate_launch(
        &self,
        launch: &LocalExecutableLaunch,
        request: &ProcessRequest,
    ) -> Result<(), RuntimeFailure> {
        let count = launch
            .prefix_arguments()
            .len()
            .saturating_add(request.arguments().len());
        let prefix_bytes = launch
            .prefix_arguments()
            .iter()
            .map(|argument| argument.as_os_str().as_encoded_bytes().len())
            .fold(0usize, usize::saturating_add);
        let argument_bytes = request
            .arguments()
            .map(str::len)
            .fold(prefix_bytes, usize::saturating_add);
        if count > self.limits.arguments() || argument_bytes > self.limits.argument_bytes() {
            return Err(failure(
                "swallowtail.local_process.argument_limit_exceeded",
                "Local process arguments exceeded host-approved limits",
            ));
        }

        let environment = launch.bootstrap_environment();
        let environment_bytes = environment
            .iter()
            .map(|(name, value)| {
                name.as_os_str()
                    .as_encoded_bytes()
                    .len()
                    .saturating_add(value.as_os_str().as_encoded_bytes().len())
            })
            .fold(0usize, usize::saturating_add);
        if environment.len() > MAXIMUM_BOOTSTRAP_ENVIRONMENT_BINDINGS
            || environment_bytes > MAXIMUM_BOOTSTRAP_ENVIRONMENT_BYTES
        {
            return Err(failure(
                "swallowtail.local_process.bootstrap_environment_limit_exceeded",
                "Local process bootstrap environment exceeded host-approved limits",
            ));
        }
        Ok(())
    }

    fn apply_environment(
        &self,
        command: &mut Command,
        request: &ProcessRequest,
    ) -> Result<(), RuntimeFailure> {
        for reference in request.environment() {
            let values = self.approvals.environments.get(reference).ok_or_else(|| {
                failure(
                    "swallowtail.local_process.environment_not_approved",
                    "Local environment reference is not approved",
                )
            })?;
            command.envs(values.iter().cloned());
        }
        Ok(())
    }

    fn apply_working_resource(
        &self,
        command: &mut Command,
        scope: &ScopeId,
        request: &ProcessRequest,
    ) -> Result<(), RuntimeFailure> {
        if let Some(reference) = request.working_resource() {
            let approved = self.approvals.working_resources.get(reference).cloned();
            let path = approved
                .or_else(|| self.materialization.working_resource_path(scope, reference))
                .ok_or_else(|| {
                    failure(
                        "swallowtail.local_process.working_resource_not_approved",
                        "Local working-resource reference is not approved",
                    )
                })?;
            command.current_dir(path);
        }
        Ok(())
    }
}

impl ProcessService for LocalProcessHost {
    fn start(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        let result = self.start_process(&scope, request);
        Box::pin(async move { result })
    }
}
