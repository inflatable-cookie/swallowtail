use crate::{
    AttachmentDescriptor, AttachmentRef, BoxFuture, CleanupOutcome, CredentialLease, CredentialRef,
    Deadline, DeadlineObservation, LeaseCleanupAuthority, MaterializedFileRef,
    MaterializedResourceRef, MonotonicInstant, ProcessExit, ProcessInputChunk, ProcessOutputChunk,
    ProcessRequest, ResourceAccess, ResourceRepresentation, RuntimeFailure, SchemaDocument,
    ScopeId, WorkingResourceRef,
};
use swallowtail_core::{Diagnostic, EndpointAudience};

pub trait JoinedTask: Send {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>>;
}

pub trait ScopedTaskService: Send + Sync {
    fn spawn(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure>;
}

pub type BlockingJob = Box<dyn FnOnce() -> Result<(), RuntimeFailure> + Send + 'static>;

pub trait BlockingWorkService: Send + Sync {
    fn run(
        &self,
        scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>>;
}

pub trait TimeService: Send + Sync {
    fn now(&self) -> MonotonicInstant;
    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation>;
}

pub trait ProcessHandle: Send + Sync {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>>;
    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>>;
}

pub trait ProcessService: Send + Sync {
    fn start(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>>;
}

pub trait CredentialService: Send + Sync {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>>;

    fn release(&self, lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome>;
}

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

    #[must_use]
    pub fn with_filesystem(mut self, filesystem: MaterializedResourceRef) -> Self {
        self.filesystem = Some(filesystem);
        self
    }

    #[must_use]
    pub const fn reference(&self) -> &WorkingResourceRef {
        &self.reference
    }

    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    #[must_use]
    pub const fn access(&self) -> ResourceAccess {
        self.access
    }

    #[must_use]
    pub const fn representation(&self) -> ResourceRepresentation {
        self.representation
    }

    #[must_use]
    pub const fn filesystem(&self) -> Option<&MaterializedResourceRef> {
        self.filesystem.as_ref()
    }

    #[must_use]
    pub const fn cleanup_authority(&self) -> LeaseCleanupAuthority {
        self.cleanup_authority
    }
}

pub trait WorkingResourceService: Send + Sync {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>>;

    fn create_temporary(
        &self,
        scope: ScopeId,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>>;

    fn release(&self, lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct AttachmentFileLease {
    scope: ScopeId,
    reference: AttachmentRef,
    file: MaterializedFileRef,
    cleanup_authority: LeaseCleanupAuthority,
}

impl AttachmentFileLease {
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

    #[must_use]
    pub const fn reference(&self) -> &AttachmentRef {
        &self.reference
    }

    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Passes the host-authorized materialization to a driver.
    #[must_use]
    pub const fn file(&self) -> &MaterializedFileRef {
        &self.file
    }

    #[must_use]
    pub const fn cleanup_authority(&self) -> LeaseCleanupAuthority {
        self.cleanup_authority
    }
}

pub trait AttachmentService: Send + Sync {
    fn materialize_file(
        &self,
        scope: ScopeId,
        descriptor: AttachmentDescriptor,
    ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>>;

    fn release_file(&self, lease: AttachmentFileLease) -> BoxFuture<'static, CleanupOutcome>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct SchemaFileLease {
    scope: ScopeId,
    file: MaterializedFileRef,
    cleanup_authority: LeaseCleanupAuthority,
}

impl SchemaFileLease {
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

    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    #[must_use]
    pub const fn cleanup_authority(&self) -> LeaseCleanupAuthority {
        self.cleanup_authority
    }
}

pub trait SchemaService: Send + Sync {
    fn materialize_file(
        &self,
        scope: ScopeId,
        document: SchemaDocument,
    ) -> BoxFuture<'static, Result<SchemaFileLease, RuntimeFailure>>;

    fn release_file(&self, lease: SchemaFileLease) -> BoxFuture<'static, CleanupOutcome>;
}

pub trait DiagnosticObserver: Send + Sync {
    fn observe(&self, diagnostic: &Diagnostic);
}
