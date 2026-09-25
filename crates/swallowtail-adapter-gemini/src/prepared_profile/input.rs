use crate::GeminiAcpHttpMcpPlacement;
use swallowtail_core::ResourceAccess;
use swallowtail_runtime::{RequestId, SessionOptions, WorkingResourceRef};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for preparing one Gemini CLI ACP session.
pub struct GeminiSessionProfileInput {
    request_id: RequestId,
    working_resource: WorkingResourceRef,
    options: SessionOptions,
    resource_access: ResourceAccess,
    http_mcp: Option<GeminiAcpHttpMcpPlacement>,
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
            http_mcp: None,
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
            http_mcp: None,
        }
    }

    /// Binds one route-owned consumer-supplied streamable-HTTP MCP declaration
    /// to this session.
    #[must_use]
    pub fn with_http_mcp_placement(mut self, placement: GeminiAcpHttpMcpPlacement) -> Self {
        self.http_mcp = Some(placement);
        self
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
        Option<GeminiAcpHttpMcpPlacement>,
    ) {
        (
            self.request_id,
            self.working_resource,
            self.options,
            self.resource_access,
            self.http_mcp,
        )
    }
}
