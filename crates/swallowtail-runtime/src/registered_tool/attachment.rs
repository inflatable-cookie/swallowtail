//! Additive registered-tool attachment vocabulary.

use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::identity::RegisteredToolTransport;
use crate::host_reference::{EnvironmentRef, ExecutableRef};

/// Fixed wire tag used by the Swallowtail reference mediated-stdio courier.
pub const REGISTERED_TOOL_PROXY_WIRE_TAG: &str = "swallowtail-registered-tool-mcp-v1";

/// Exact provider-facing attachment shape for one registered-tool selection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolAttachment {
    /// Dispatch through the host port without an operation-owned listener.
    HostMediated,
    /// Spawn a Swallowtail MCP stdio courier over the private HTTP carrier.
    MediatedStdioProxy,
}

impl RegisteredToolAttachment {
    /// Returns the stable attachment label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostMediated => "host-mediated",
            Self::MediatedStdioProxy => "mediated-stdio-proxy",
        }
    }

    /// Returns the carrier required by this attachment shape.
    #[must_use]
    pub const fn transport(self) -> RegisteredToolTransport {
        match self {
            Self::HostMediated => RegisteredToolTransport::HostMediatedCallback,
            Self::MediatedStdioProxy => RegisteredToolTransport::PrivateLoopbackHttp,
        }
    }
}

/// Host-approved recipe for the reference courier process.
///
/// The executable and environment remain opaque host references. The wire tag
/// is fixed by Swallowtail; endpoint, bearer, and generation values are never
/// stored in or derived from this recipe.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolProxyRecipe {
    executable: ExecutableRef,
    environment: EnvironmentRef,
}

impl RegisteredToolProxyRecipe {
    /// Creates a recipe for the fixed Swallowtail courier wire.
    pub fn new(
        executable: ExecutableRef,
        environment: EnvironmentRef,
        wire_tag: impl AsRef<str>,
    ) -> Result<Self, RegisteredToolFailure> {
        if wire_tag.as_ref() != REGISTERED_TOOL_PROXY_WIRE_TAG {
            return Err(reject(RegisteredToolFailureKind::IdentityRejected));
        }
        Ok(Self {
            executable,
            environment,
        })
    }

    /// Returns the opaque executable reference.
    #[must_use]
    pub const fn executable(&self) -> &ExecutableRef {
        &self.executable
    }

    /// Returns the opaque allowlisted environment reference.
    #[must_use]
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    /// Returns the fixed courier wire tag.
    #[must_use]
    pub const fn wire_tag(&self) -> &'static str {
        REGISTERED_TOOL_PROXY_WIRE_TAG
    }
}

/// Safe description of an exact registered-tool attachment.
///
/// This descriptor carries no endpoint, bearer, generation, credential, raw
/// executable path, or environment body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolAttachmentDescriptor {
    attachment: RegisteredToolAttachment,
    transport: RegisteredToolTransport,
    wire_tag: Option<&'static str>,
}

impl RegisteredToolAttachmentDescriptor {
    /// Creates a safe description for an attachment and carrier.
    #[must_use]
    pub const fn new(
        attachment: RegisteredToolAttachment,
        transport: RegisteredToolTransport,
    ) -> Self {
        Self {
            attachment,
            transport,
            wire_tag: match attachment {
                RegisteredToolAttachment::HostMediated => None,
                RegisteredToolAttachment::MediatedStdioProxy => {
                    Some(REGISTERED_TOOL_PROXY_WIRE_TAG)
                }
            },
        }
    }

    /// Returns the selected attachment shape.
    #[must_use]
    pub const fn attachment(&self) -> RegisteredToolAttachment {
        self.attachment
    }

    /// Returns the selected carrier.
    #[must_use]
    pub const fn transport(&self) -> RegisteredToolTransport {
        self.transport
    }

    /// Returns the fixed wire tag when the attachment uses the courier.
    #[must_use]
    pub const fn wire_tag(&self) -> Option<&'static str> {
        self.wire_tag
    }
}
