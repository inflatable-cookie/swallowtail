#![deny(missing_docs)]

use crate::debug_observation::DebugObservation;
use crate::{
    AttachmentDescriptor, AttachmentRef, BoxFuture, CleanupOutcome, CredentialLease, CredentialRef,
    Deadline, DeadlineObservation, LeaseCleanupAuthority, MaterializedFileRef,
    MaterializedResourceRef, MonotonicInstant, ProcessExit, ProcessInputChunk, ProcessOutputChunk,
    ProcessRequest, ResourceAccess, ResourceRepresentation, RuntimeFailure, SchemaDocument,
    ScopeId, WorkingResourceRef,
};
use std::any::Any;
use std::task::Waker;
use swallowtail_core::{
    CatalogTimestamp, Diagnostic, EndpointAudience, ExecutionHostId, SafeDiagnostic,
};

/// Outcome of transferring an unfinished joined task back to its owning host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskRelinquishOutcome {
    /// The owning host accepted the task and will reap it after it finishes.
    ///
    /// This is ownership-transfer evidence only. It does not mean the task
    /// finished, joined, or completed cleanup successfully.
    AcceptedForReap,
}

/// Opaque host-issued authority for starting one task that may later be reaped.
///
/// The grant is bound to the exact execution host and operation scope selected
/// when [`ScopedTaskService::reserve_reap`] issued it. Dropping an unused grant
/// releases its reserved capacity without starting work.
pub trait TaskReapReservation: Send + std::fmt::Debug {
    /// Converts the opaque grant for validation by its issuing host.
    #[doc(hidden)]
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send>;
}

/// Join handle for one task created inside a runtime operation scope.
pub trait JoinedTask: Send {
    /// Waits for the task to finish and reports runtime-safe failure evidence.
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>>;

    /// Reports whether [`Self::join`] can be called without blocking.
    ///
    /// Hosts that cannot make this observation return `false`. Runtime code
    /// uses this optional capability only when it must keep future polling
    /// non-blocking.
    fn is_finished(&self) -> bool {
        false
    }

    /// Registers a waker for the transition observed by [`Self::is_finished`].
    ///
    /// Hosts that cannot provide a notification leave the default no-op in
    /// place; callers must then use their host-specific polling policy.
    fn register_waker(&self, _waker: &Waker) {}

    /// Transfers this task's worker to its exact owning host for later reaping.
    ///
    /// This is the handle-side half of [`ScopedTaskService::relinquish`].
    /// Runtime and adapter code must use the service operation so the selected
    /// execution host remains the authority. The default fails closed without
    /// changing task ownership.
    #[doc(hidden)]
    fn relinquish_to_host(
        &mut self,
        _execution_host_id: &ExecutionHostId,
        _scope: &ScopeId,
    ) -> Result<(), RuntimeFailure> {
        Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.task.relinquish_unsupported",
            "Runtime host cannot accept this task for autonomous reaping",
        )))
    }
}

/// Host boundary for spawning tasks that must be joined before scope cleanup.
pub trait ScopedTaskService: Send + Sync {
    /// Spawns `task` under `scope` and returns its required join handle.
    fn spawn(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure>;

    /// Reserves authority to start and later relinquish one task under `scope`.
    ///
    /// Unsupported, closing, or capacity-exhausted hosts fail here, before the
    /// operation acquires credentials, resources, processes, tasks, or provider
    /// work. The returned grant is ownership, not a boolean support probe.
    fn reserve_reap(
        &self,
        _scope: ScopeId,
    ) -> Result<Box<dyn TaskReapReservation>, RuntimeFailure> {
        Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.task.reap_reservation_unsupported",
            "Runtime host cannot reserve autonomous task reaping",
        )))
    }

    /// Starts `task` only after binding its host-issued reap reservation.
    ///
    /// The reservation supplies the exact operation scope. A foreign, released,
    /// or otherwise invalid grant fails closed before `task` is polled. Once
    /// bound, cancelling a later join future must not discard the live task or
    /// release its host shutdown barrier.
    fn spawn_reapable(
        &self,
        _reservation: Box<dyn TaskReapReservation>,
        _task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.task.reap_reservation_unsupported",
            "Runtime host cannot start a reservation-backed task",
        )))
    }

    /// Takes back an unfinished task for autonomous host-side reaping.
    ///
    /// The task must have consumed this host's exact-scope reservation through
    /// [`Self::spawn_reapable`] before its work began.
    ///
    /// On success, the task slot is cleared and the result says only that the
    /// host accepted ownership. It is not join or cleanup-completion evidence.
    /// On failure, the slot retains the task and ordinary join/drop ownership.
    fn relinquish(
        &self,
        _scope: &ScopeId,
        _task: &mut Option<Box<dyn JoinedTask>>,
    ) -> Result<TaskRelinquishOutcome, RuntimeFailure> {
        Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.task.relinquish_unsupported",
            "Runtime host cannot accept this task for autonomous reaping",
        )))
    }
}

/// Blocking unit of host work with a redacted runtime failure boundary.
pub type BlockingJob = Box<dyn FnOnce() -> Result<(), RuntimeFailure> + Send + 'static>;

/// Host boundary for moving blocking work away from runtime task execution.
pub trait BlockingWorkService: Send + Sync {
    /// Runs one blocking job within the supplied operation scope.
    fn run(
        &self,
        scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>>;
}

/// Host-provided monotonic and observation clocks.
pub trait TimeService: Send + Sync {
    /// Reads the host's monotonic clock.
    fn now(&self) -> MonotonicInstant;
    /// Waits until an absolute monotonic deadline is observed.
    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation>;

    /// Reads a wall-clock timestamp suitable for catalogue evidence.
    ///
    /// Hosts without an approved observation clock return a safe failure.
    fn catalog_now(&self) -> Result<CatalogTimestamp, RuntimeFailure> {
        Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.catalog_clock_unavailable",
            "Runtime host does not expose an observation clock",
        )))
    }
}

/// Host-owned process started for one admitted runtime operation.
pub trait ProcessHandle: Send + Sync {
    /// Writes one chunk to the process standard input.
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
    /// Closes the process standard input stream.
    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
    /// Reads the next ordered stdout or stderr chunk, or `None` at end of stream.
    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>>;
    /// Requests graceful process termination.
    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
    /// Requests forced process termination when graceful shutdown is insufficient.
    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
    /// Waits for the process exit result.
    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>>;
}

/// Host boundary for starting an explicitly authorized local process.
pub trait ProcessService: Send + Sync {
    /// Starts `request` within `scope` without ambient launch-policy inference.
    fn start(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>>;
}

/// Host boundary for acquiring and releasing credential leases.
pub trait CredentialService: Send + Sync {
    /// Acquires an opaque credential for the exact reference and audience.
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>>;

    /// Releases a previously acquired credential lease.
    fn release(&self, lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome>;
}

/// Host-authorized view of one working resource.
///
/// The lease records access, representation, optional filesystem
/// materialization, and which owner may clean that materialization up.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceLease {
    scope: ScopeId,
    reference: WorkingResourceRef,
    access: ResourceAccess,
    representation: ResourceRepresentation,
    filesystem: Option<MaterializedResourceRef>,
    cleanup_authority: LeaseCleanupAuthority,
}

impl ResourceLease {
    /// Creates a lease whose underlying resource remains consumer-owned.
    #[must_use]
    pub const fn consumer_owned(
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> Self {
        Self {
            scope,
            reference,
            access,
            representation,
            filesystem: None,
            cleanup_authority: LeaseCleanupAuthority::Consumer,
        }
    }

    /// Creates a lease whose materialization belongs to the operation scope.
    #[must_use]
    pub const fn operation_scoped(
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> Self {
        Self {
            scope,
            reference,
            access,
            representation,
            filesystem: None,
            cleanup_authority: LeaseCleanupAuthority::OperationScope,
        }
    }

    /// Attaches a host-authorized filesystem materialization.
    #[must_use]
    pub fn with_filesystem(mut self, filesystem: MaterializedResourceRef) -> Self {
        self.filesystem = Some(filesystem);
        self
    }

    /// Returns the opaque working-resource reference.
    #[must_use]
    pub const fn reference(&self) -> &WorkingResourceRef {
        &self.reference
    }

    /// Returns the runtime scope that owns this lease.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Returns the admitted resource access.
    #[must_use]
    pub const fn access(&self) -> ResourceAccess {
        self.access
    }

    /// Returns the representation granted to the driver.
    #[must_use]
    pub const fn representation(&self) -> ResourceRepresentation {
        self.representation
    }

    /// Returns the filesystem materialization when one was granted.
    #[must_use]
    pub const fn filesystem(&self) -> Option<&MaterializedResourceRef> {
        self.filesystem.as_ref()
    }

    /// Returns which boundary owns cleanup of the materialization.
    #[must_use]
    pub const fn cleanup_authority(&self) -> LeaseCleanupAuthority {
        self.cleanup_authority
    }
}

/// Host boundary for resolving and materializing working resources.
pub trait WorkingResourceService: Send + Sync {
    /// Resolves one opaque resource reference under exact access and representation.
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>>;

    /// Creates one temporary working resource within an operation scope.
    fn create_temporary(
        &self,
        scope: ScopeId,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>>;

    /// Releases a working-resource lease according to its cleanup authority.
    fn release(&self, lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome>;
}

/// Operation-scoped file materialization of one attachment.
#[derive(Debug, Eq, PartialEq)]
pub struct AttachmentFileLease {
    scope: ScopeId,
    reference: AttachmentRef,
    file: MaterializedFileRef,
    cleanup_authority: LeaseCleanupAuthority,
}

impl AttachmentFileLease {
    /// Creates a file lease whose materialization belongs to the operation scope.
    #[must_use]
    pub const fn operation_scoped(
        scope: ScopeId,
        reference: AttachmentRef,
        file: MaterializedFileRef,
    ) -> Self {
        Self {
            scope,
            reference,
            file,
            cleanup_authority: LeaseCleanupAuthority::OperationScope,
        }
    }

    /// Returns the opaque attachment reference.
    #[must_use]
    pub const fn reference(&self) -> &AttachmentRef {
        &self.reference
    }

    /// Returns the runtime scope that owns this lease.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Passes the host-authorized materialization to a driver.
    #[must_use]
    pub const fn file(&self) -> &MaterializedFileRef {
        &self.file
    }

    /// Returns which boundary owns cleanup of the materialized file.
    #[must_use]
    pub const fn cleanup_authority(&self) -> LeaseCleanupAuthority {
        self.cleanup_authority
    }
}

/// Host boundary for materializing attachments as files.
pub trait AttachmentService: Send + Sync {
    /// Materializes one admitted attachment within the operation scope.
    fn materialize_file(
        &self,
        scope: ScopeId,
        descriptor: AttachmentDescriptor,
    ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>>;

    /// Releases an attachment file lease according to its cleanup authority.
    fn release_file(&self, lease: AttachmentFileLease) -> BoxFuture<'static, CleanupOutcome>;
}

/// Operation-scoped file materialization of one structured-output schema.
#[derive(Debug, Eq, PartialEq)]
pub struct SchemaFileLease {
    scope: ScopeId,
    file: MaterializedFileRef,
    cleanup_authority: LeaseCleanupAuthority,
}

impl SchemaFileLease {
    /// Creates a schema lease whose file belongs to the operation scope.
    #[must_use]
    pub const fn operation_scoped(scope: ScopeId, file: MaterializedFileRef) -> Self {
        Self {
            scope,
            file,
            cleanup_authority: LeaseCleanupAuthority::OperationScope,
        }
    }

    /// Passes the host-authorized materialization to a driver.
    #[must_use]
    pub const fn file(&self) -> &MaterializedFileRef {
        &self.file
    }

    /// Returns the runtime scope that owns this lease.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Returns which boundary owns cleanup of the materialized file.
    #[must_use]
    pub const fn cleanup_authority(&self) -> LeaseCleanupAuthority {
        self.cleanup_authority
    }
}

/// Host boundary for materializing structured-output schemas as files.
pub trait SchemaService: Send + Sync {
    /// Materializes one schema document within the operation scope.
    fn materialize_file(
        &self,
        scope: ScopeId,
        document: SchemaDocument,
    ) -> BoxFuture<'static, Result<SchemaFileLease, RuntimeFailure>>;

    /// Releases a schema file lease according to its cleanup authority.
    fn release_file(&self, lease: SchemaFileLease) -> BoxFuture<'static, CleanupOutcome>;
}

/// Optional sink for redacted runtime diagnostics and debug observations.
pub trait DiagnosticObserver: Send + Sync {
    /// Observes one diagnostic without gaining control of the operation.
    fn observe(&self, diagnostic: &Diagnostic);

    /// Observes one structured debug record without gaining control of the operation.
    ///
    /// The default implementation ignores the observation so existing hosts keep
    /// compiling. Hosts that want restricted debug context override this method.
    fn observe_debug(&self, observation: &DebugObservation) {
        let _ = observation;
    }
}
