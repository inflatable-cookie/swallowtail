use crate::child::LocalProcessHandle;
use crate::limits::{LocalMaterializationLimits, LocalProcessLimits};
use crate::materialization::LocalMaterializationState;
use crate::output::failure;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Instant;
use swallowtail_runtime::{
    AttachmentRef, BoxFuture, EnvironmentRef, ExecutableRef, ProcessHandle, ProcessRequest,
    ProcessService, RuntimeFailure, SchemaRef, ScopeId, WorkingResourceRef,
};

type EnvironmentValues = Vec<(OsString, OsString)>;

#[derive(Default)]
pub(crate) struct LocalApprovals {
    pub(crate) executables: HashMap<ExecutableRef, PathBuf>,
    pub(crate) environments: HashMap<EnvironmentRef, EnvironmentValues>,
    pub(crate) working_resources: HashMap<WorkingResourceRef, PathBuf>,
    pub(crate) attachments: HashMap<AttachmentRef, PathBuf>,
    pub(crate) schemas: HashMap<SchemaRef, PathBuf>,
}

pub struct LocalProcessHostBuilder {
    limits: LocalProcessLimits,
    materialization_limits: LocalMaterializationLimits,
    temporary_root: PathBuf,
    approvals: LocalApprovals,
}

impl LocalProcessHostBuilder {
    #[must_use]
    pub fn approve_executable(
        mut self,
        reference: ExecutableRef,
        path: impl Into<PathBuf>,
    ) -> Self {
        self.approvals.executables.insert(reference, path.into());
        self
    }

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

    #[must_use]
    pub fn approve_attachment(
        mut self,
        reference: AttachmentRef,
        path: impl Into<PathBuf>,
    ) -> Self {
        self.approvals.attachments.insert(reference, path.into());
        self
    }

    #[must_use]
    pub fn approve_schema(mut self, reference: SchemaRef, path: impl Into<PathBuf>) -> Self {
        self.approvals.schemas.insert(reference, path.into());
        self
    }

    #[must_use]
    pub fn with_materialization_limits(mut self, limits: LocalMaterializationLimits) -> Self {
        self.materialization_limits = limits;
        self
    }

    #[must_use]
    pub fn with_temporary_root(mut self, path: impl Into<PathBuf>) -> Self {
        self.temporary_root = path.into();
        self
    }

    #[must_use]
    pub fn build(self) -> LocalProcessHost {
        LocalProcessHost {
            limits: self.limits,
            materialization_limits: self.materialization_limits,
            approvals: Arc::new(self.approvals),
            materialization: Arc::new(LocalMaterializationState::new(self.temporary_root)),
            monotonic_origin: Instant::now(),
        }
    }
}

#[derive(Clone)]
pub struct LocalProcessHost {
    pub(crate) limits: LocalProcessLimits,
    pub(crate) materialization_limits: LocalMaterializationLimits,
    pub(crate) approvals: Arc<LocalApprovals>,
    pub(crate) materialization: Arc<LocalMaterializationState>,
    pub(crate) monotonic_origin: Instant,
}

impl LocalProcessHost {
    #[must_use]
    pub fn builder(limits: LocalProcessLimits) -> LocalProcessHostBuilder {
        LocalProcessHostBuilder {
            limits,
            materialization_limits: LocalMaterializationLimits::default(),
            temporary_root: std::env::temp_dir(),
            approvals: LocalApprovals::default(),
        }
    }

    fn start_process(
        &self,
        scope: &ScopeId,
        request: ProcessRequest,
    ) -> Result<Box<dyn ProcessHandle>, RuntimeFailure> {
        self.validate_arguments(&request)?;
        let executable = self
            .approvals
            .executables
            .get(request.executable())
            .ok_or_else(|| {
                failure(
                    "swallowtail.local_process.executable_not_approved",
                    "Local executable reference is not approved",
                )
            })?;
        let mut command = Command::new(executable);
        command
            .args(request.arguments())
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
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
