//! The sole immutable Swallowtail registration snapshot.

use super::declaration::RegisteredToolDeclaration;
use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::identity::{
    RegisteredServerId, RegisteredServerRevision, RegisteredToolId, RegisteredToolProtocolVersion,
    RegisteredToolTransport, admit_identity,
};
use super::limits::{
    MAX_REGISTERED_TOOL_AGGREGATE_SCHEMA_BYTES, MAX_REGISTERED_TOOL_CREDENTIAL_REFERENCES,
    MAX_REGISTERED_TOOL_DECLARATIONS, MAX_REGISTERED_TOOL_PROTOCOL_VERSIONS,
    MAX_REGISTERED_TOOL_RECIPE_REFERENCES, MAX_REGISTERED_TOOL_REQUIRED_SERVICES,
    MAX_REGISTERED_TOOL_TRANSPORTS, RegisteredToolBounds,
};
use crate::MonotonicInstant;
use crate::host_reference::{EnvironmentRef, ExecutableRef};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use swallowtail_core::{CredentialRef, ExecutionHostId, HostServiceKind};

/// Bounded identity of the consumer record this snapshot was derived from.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegisteredToolSourceId(String);

impl RegisteredToolSourceId {
    /// Admits bounded, non-blank, control-free source identity text.
    pub fn new(value: impl Into<String>) -> Result<Self, RegisteredToolFailure> {
        admit_identity(value.into()).map(Self)
    }

    /// Returns the exact admitted source identity text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RegisteredToolSourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Source identity and freshness suitable for Contract 061 projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolSource {
    id: RegisteredToolSourceId,
    observed_at: MonotonicInstant,
}

impl RegisteredToolSource {
    /// Binds one bounded source identity to the instant it was observed.
    #[must_use]
    pub const fn new(id: RegisteredToolSourceId, observed_at: MonotonicInstant) -> Self {
        Self { id, observed_at }
    }

    /// Returns the bounded source identity.
    #[must_use]
    pub const fn id(&self) -> &RegisteredToolSourceId {
        &self.id
    }

    /// Returns the host monotonic instant at which the source was observed.
    #[must_use]
    pub const fn observed_at(&self) -> MonotonicInstant {
        self.observed_at
    }
}

/// Exact transports and qualified protocol versions one snapshot supports.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolTransportSupport {
    transport: RegisteredToolTransport,
    protocol_versions: BTreeSet<RegisteredToolProtocolVersion>,
}

impl RegisteredToolTransportSupport {
    /// Binds one carrier to a nonempty explicit protocol-version subset.
    pub fn new(
        transport: RegisteredToolTransport,
        protocol_versions: impl IntoIterator<Item = RegisteredToolProtocolVersion>,
    ) -> Result<Self, RegisteredToolFailure> {
        let protocol_versions: BTreeSet<_> = protocol_versions.into_iter().collect();
        if protocol_versions.is_empty() {
            return Err(reject(
                RegisteredToolFailureKind::UnsupportedProtocolVersion,
            ));
        }
        if protocol_versions.len() > MAX_REGISTERED_TOOL_PROTOCOL_VERSIONS {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(Self {
            transport,
            protocol_versions,
        })
    }

    /// Returns the declared carrier.
    #[must_use]
    pub const fn transport(&self) -> RegisteredToolTransport {
        self.transport
    }

    /// Returns the exact declared protocol versions.
    #[must_use]
    pub const fn protocol_versions(&self) -> &BTreeSet<RegisteredToolProtocolVersion> {
        &self.protocol_versions
    }
}

/// Immutable registration snapshot. It holds descriptions and owns no resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolSnapshot {
    server_id: RegisteredServerId,
    revision: RegisteredServerRevision,
    execution_host_id: ExecutionHostId,
    declarations: Vec<RegisteredToolDeclaration>,
    transports: Vec<RegisteredToolTransportSupport>,
    required_services: BTreeSet<HostServiceKind>,
    credential_references: Vec<CredentialRef>,
    executable_recipes: Vec<ExecutableRef>,
    environment_recipes: Vec<EnvironmentRef>,
    bounds: RegisteredToolBounds,
    source: RegisteredToolSource,
}

/// Builder inputs for one immutable registration snapshot.
#[derive(Clone, Debug)]
pub struct RegisteredToolSnapshotInput {
    /// Stable server identity supplied by the consumer.
    pub server_id: RegisteredServerId,
    /// Stable revision of this snapshot.
    pub revision: RegisteredServerRevision,
    /// Execution host that must own every required service.
    pub execution_host_id: ExecutionHostId,
    /// Consumer-declared tools, each binding one namespaced identity.
    pub declarations: Vec<RegisteredToolDeclaration>,
    /// Supported carriers with their exact protocol-version subsets.
    pub transports: Vec<RegisteredToolTransportSupport>,
    /// Host-service kinds this registration requires before prepare.
    pub required_services: BTreeSet<HostServiceKind>,
    /// Credential references, never credential material.
    pub credential_references: Vec<CredentialRef>,
    /// Opaque process recipe references, never raw executable paths.
    pub executable_recipes: Vec<ExecutableRef>,
    /// Opaque environment recipe references, never a complete environment.
    pub environment_recipes: Vec<EnvironmentRef>,
    /// Positive server-level bounds.
    pub bounds: RegisteredToolBounds,
    /// Source identity and freshness for projection.
    pub source: RegisteredToolSource,
}

impl RegisteredToolSnapshot {
    /// Validates identity uniqueness, kind uniqueness, and every positive bound.
    pub fn new(input: RegisteredToolSnapshotInput) -> Result<Self, RegisteredToolFailure> {
        let RegisteredToolSnapshotInput {
            server_id,
            revision,
            execution_host_id,
            declarations,
            transports,
            required_services,
            credential_references,
            executable_recipes,
            environment_recipes,
            bounds,
            source,
        } = input;
        if declarations.is_empty() || transports.is_empty() {
            return Err(reject(RegisteredToolFailureKind::UnsupportedRegistration));
        }
        if declarations.len() > MAX_REGISTERED_TOOL_DECLARATIONS
            || transports.len() > MAX_REGISTERED_TOOL_TRANSPORTS
            || required_services.len() > MAX_REGISTERED_TOOL_REQUIRED_SERVICES
            || credential_references.len() > MAX_REGISTERED_TOOL_CREDENTIAL_REFERENCES
            || executable_recipes.len() > MAX_REGISTERED_TOOL_RECIPE_REFERENCES
            || environment_recipes.len() > MAX_REGISTERED_TOOL_RECIPE_REFERENCES
        {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        let mut seen: BTreeMap<&RegisteredToolId, ()> = BTreeMap::new();
        let mut aggregate_schema_bytes = 0_usize;
        for declaration in &declarations {
            if seen.insert(declaration.id(), ()).is_some() {
                return Err(reject(RegisteredToolFailureKind::IdentityRejected));
            }
            aggregate_schema_bytes =
                aggregate_schema_bytes.saturating_add(declaration.schema_byte_len());
        }
        if aggregate_schema_bytes > MAX_REGISTERED_TOOL_AGGREGATE_SCHEMA_BYTES {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        let mut seen_transports: BTreeSet<RegisteredToolTransport> = BTreeSet::new();
        for support in &transports {
            if !seen_transports.insert(support.transport()) {
                return Err(reject(RegisteredToolFailureKind::UnsupportedTransport));
            }
        }
        Ok(Self {
            server_id,
            revision,
            execution_host_id,
            declarations,
            transports,
            required_services,
            credential_references,
            executable_recipes,
            environment_recipes,
            bounds,
            source,
        })
    }

    /// Returns the stable server identity.
    #[must_use]
    pub const fn server_id(&self) -> &RegisteredServerId {
        &self.server_id
    }

    /// Returns the exact snapshot revision.
    #[must_use]
    pub const fn revision(&self) -> &RegisteredServerRevision {
        &self.revision
    }

    /// Returns the execution host that must own every required service.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    /// Returns every declared tool.
    #[must_use]
    pub fn declarations(&self) -> &[RegisteredToolDeclaration] {
        &self.declarations
    }

    /// Returns the declaration for one namespaced identity.
    #[must_use]
    pub fn declaration(&self, id: &RegisteredToolId) -> Option<&RegisteredToolDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.id() == id)
    }

    /// Returns the declared carriers and protocol subsets.
    #[must_use]
    pub fn transports(&self) -> &[RegisteredToolTransportSupport] {
        &self.transports
    }

    /// Returns the declared support for one exact carrier.
    #[must_use]
    pub fn transport_support(
        &self,
        transport: RegisteredToolTransport,
    ) -> Option<&RegisteredToolTransportSupport> {
        self.transports
            .iter()
            .find(|support| support.transport() == transport)
    }

    /// Returns the host-service kinds required before prepare.
    #[must_use]
    pub const fn required_services(&self) -> &BTreeSet<HostServiceKind> {
        &self.required_services
    }

    /// Returns declared credential references without credential material.
    #[must_use]
    pub fn credential_references(&self) -> &[CredentialRef] {
        &self.credential_references
    }

    /// Returns opaque process recipe references.
    #[must_use]
    pub fn executable_recipes(&self) -> &[ExecutableRef] {
        &self.executable_recipes
    }

    /// Returns opaque environment recipe references.
    #[must_use]
    pub fn environment_recipes(&self) -> &[EnvironmentRef] {
        &self.environment_recipes
    }

    /// Returns the declared server-level bounds.
    #[must_use]
    pub const fn bounds(&self) -> RegisteredToolBounds {
        self.bounds
    }

    /// Returns the source identity and freshness.
    #[must_use]
    pub const fn source(&self) -> &RegisteredToolSource {
        &self.source
    }
}
