use swallowtail_core::ResourceAccess;
use swallowtail_runtime::{RequestId, SessionOptions, WorkingResourceRef};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for preparing one Gemini CLI ACP session.
pub struct GeminiSessionProfileInput {
    request_id: RequestId,
    working_resource: WorkingResourceRef,
    options: SessionOptions,
    resource_access: ResourceAccess,
}

impl GeminiSessionProfileInput {
    /// Creates a read-only session profile.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        options: SessionOptions,
    ) -> Self {
        Self {
            request_id,
            working_resource,
            options,
            resource_access: ResourceAccess::Read,
        }
    }

    /// Creates a session profile with bounded read-write workspace access.
    #[must_use]
    pub const fn bounded_write(
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        options: SessionOptions,
    ) -> Self {
        Self {
            request_id,
            working_resource,
            options,
            resource_access: ResourceAccess::ReadWrite,
        }
    }

    /// Returns the requested working-resource access level.
    #[must_use]
    pub const fn resource_access(&self) -> ResourceAccess {
        self.resource_access
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        WorkingResourceRef,
        SessionOptions,
        ResourceAccess,
    ) {
        (
            self.request_id,
            self.working_resource,
            self.options,
            self.resource_access,
        )
    }
}
