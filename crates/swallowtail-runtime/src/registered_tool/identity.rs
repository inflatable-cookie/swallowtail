//! Bounded registered-tool identities, kinds, transports, and generations.

use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::limits::MAX_REGISTERED_TOOL_IDENTITY_BYTES;
use std::fmt;
use std::num::NonZeroU64;

pub(super) fn admit_identity(value: String) -> Result<String, RegisteredToolFailure> {
    if value.trim().is_empty()
        || value
            .chars()
            .any(|character| character.is_control() || character == '\u{7f}')
    {
        return Err(reject(RegisteredToolFailureKind::IdentityRejected));
    }
    if value.len() > MAX_REGISTERED_TOOL_IDENTITY_BYTES {
        return Err(reject(RegisteredToolFailureKind::LimitExceeded));
    }
    Ok(value)
}

macro_rules! bounded_identity {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Admits bounded, non-blank, control-free identity text.
            pub fn new(value: impl Into<String>) -> Result<Self, RegisteredToolFailure> {
                admit_identity(value.into()).map(Self)
            }

            /// Returns the exact admitted identity text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

bounded_identity!(
    RegisteredServerId,
    "Stable consumer-supplied server identity."
);
bounded_identity!(
    RegisteredServerRevision,
    "Stable revision of one registration snapshot."
);
bounded_identity!(
    RegisteredToolNamespace,
    "Producer namespace supplied by the consumer for one tool family."
);
bounded_identity!(
    RegisteredToolLocalName,
    "Local tool name inside one producer namespace."
);
bounded_identity!(
    RegisteredToolCallId,
    "Unique identity of one issued registered-tool call."
);
bounded_identity!(
    RegisteredToolProtocolVersion,
    "Exact registered-tool protocol version admitted by a qualified route."
);
bounded_identity!(
    RegisteredToolReasonCode,
    "Bounded safe reason code for a revocation or denial."
);

/// Stable namespaced identity of exactly one registered tool.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegisteredToolId {
    namespace: RegisteredToolNamespace,
    local_name: RegisteredToolLocalName,
}

impl RegisteredToolId {
    /// Binds one producer namespace to one local tool name.
    #[must_use]
    pub const fn new(
        namespace: RegisteredToolNamespace,
        local_name: RegisteredToolLocalName,
    ) -> Self {
        Self {
            namespace,
            local_name,
        }
    }

    /// Returns the producer namespace.
    #[must_use]
    pub const fn namespace(&self) -> &RegisteredToolNamespace {
        &self.namespace
    }

    /// Returns the local tool name.
    #[must_use]
    pub const fn local_name(&self) -> &RegisteredToolLocalName {
        &self.local_name
    }
}

impl fmt::Display for RegisteredToolId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.namespace, self.local_name)
    }
}

/// Executor that owns one registered tool identity.
///
/// One namespaced identity binds exactly one kind inside one snapshot.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolExecutionKind {
    /// Declared through the provider wire and dispatched to the linked host.
    NativeClient,
    /// Carried through a private MCP attachment or host-mediated MCP call.
    Mcp,
    /// Transported through an exact consumer callback with no MCP identity.
    App,
    /// Owned by the provider or harness and only observed.
    ProviderOwned,
}

impl RegisteredToolExecutionKind {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeClient => "native-client",
            Self::Mcp => "mcp",
            Self::App => "app",
            Self::ProviderOwned => "provider-owned",
        }
    }

    /// Reports whether Swallowtail may dispatch this kind to a linked host.
    #[must_use]
    pub const fn is_host_dispatchable(self) -> bool {
        matches!(self, Self::NativeClient | Self::Mcp | Self::App)
    }
}

impl fmt::Display for RegisteredToolExecutionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Carrier selected for one registered-tool profile.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolTransport {
    /// Host-mediated native callback dispatch that binds no listener.
    HostMediatedCallback,
    /// Private loopback HTTP carrying the registered-tool MCP profile.
    PrivateLoopbackHttp,
    /// Bounded server-sent events over the private loopback carrier.
    PrivateLoopbackSse,
}

impl RegisteredToolTransport {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostMediatedCallback => "host-mediated-callback",
            Self::PrivateLoopbackHttp => "private-loopback-http",
            Self::PrivateLoopbackSse => "private-loopback-sse",
        }
    }

    /// Reports whether this carrier binds an operation-owned listener.
    #[must_use]
    pub const fn binds_listener(self) -> bool {
        matches!(self, Self::PrivateLoopbackHttp | Self::PrivateLoopbackSse)
    }
}

impl fmt::Display for RegisteredToolTransport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

macro_rules! positive_generation {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(NonZeroU64);

        impl $name {
            /// Creates generation `1`.
            #[must_use]
            pub const fn initial() -> Self {
                Self(NonZeroU64::MIN)
            }

            /// Creates a generation from an exact positive counter.
            #[must_use]
            pub const fn new(value: u64) -> Option<Self> {
                match NonZeroU64::new(value) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Returns the raw positive counter.
            #[must_use]
            pub const fn get(self) -> u64 {
                self.0.get()
            }

            /// Returns the next generation, saturating at the maximum.
            #[must_use]
            pub const fn next(self) -> Self {
                match self.0.checked_add(1) {
                    Some(value) => Self(value),
                    None => self,
                }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&self.get())
                    .finish()
            }
        }
    };
}

positive_generation!(
    RegisteredToolLeaseGeneration,
    "Monotonic generation assigned to one opened registered-tool lease."
);
positive_generation!(
    RegisteredToolTransportGeneration,
    "Monotonic generation of one transport attachment inside a lease."
);
positive_generation!(
    ConsumerWorkspaceGeneration,
    "Consumer-issued generation of the admitted workspace."
);
positive_generation!(
    ConsumerTaskGeneration,
    "Consumer-issued generation of the admitted task."
);
