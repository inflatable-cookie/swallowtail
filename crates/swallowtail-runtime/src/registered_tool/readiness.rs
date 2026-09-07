//! Typed registered-profile preflight over the optional host port.
//!
//! This preflight is deliberately separate from `HostServiceKind`. Contract 063
//! keeps that enum exhaustive and unchanged in this slice, so registered-tool
//! port availability and selected topology are reported through this typed
//! readiness record instead of a new service-kind variant.
//!
//! Readiness is not advisory. [`RegisteredToolReadiness::require_ready`] is the
//! only source of a [`RegisteredToolTopologyProof`], and the kernel refuses to
//! mint a binding or open a lease without one that matches the exact selection.
//! Every mounted open path therefore passes this gate.

use super::attachment::RegisteredToolAttachment;
use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::identity::{
    RegisteredServerId, RegisteredServerRevision, RegisteredToolId, RegisteredToolProtocolVersion,
    RegisteredToolTransport,
};
use super::selection::RegisteredToolSelection;
use crate::HostServices;
use std::collections::BTreeSet;
use swallowtail_core::{ExecutionHostId, HostServiceKind};

/// Carriers this slice qualifies for the registered-tool profile.
///
/// Host-mediated callback dispatch binds no listener. The private loopback HTTP
/// and SSE carriers remain withheld unless a separately qualified attachment
/// selects the mediated-stdio HTTP wire.
pub const REGISTERED_TOOL_QUALIFIED_TRANSPORTS: &[RegisteredToolTransport] =
    &[RegisteredToolTransport::HostMediatedCallback];

/// The one explicitly named protocol version provider-free conformance uses.
///
/// It is not a production route version and enables no provider route.
pub const REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION: &str =
    "swallowtail.registered-tool.conformance-2026-09-07";

/// Availability of the optional registered-tool bridge port.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolPortAvailability {
    /// The optional port is registered on the supplied host registry.
    Registered,
    /// The optional port is absent, so the profile is not available.
    Absent,
}

impl RegisteredToolPortAvailability {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Registered => "registered",
            Self::Absent => "absent",
        }
    }
}

/// Exact mounted topology a registered-tool selection is measured against.
///
/// A mounted host port captures this once, after its registry is assembled, so
/// its low-level `open` can apply the same typed gate as `prepare`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolMountedTopology {
    execution_host_id: ExecutionHostId,
    available_services: BTreeSet<HostServiceKind>,
    port_registered: bool,
}

impl RegisteredToolMountedTopology {
    /// Captures the exact topology one assembled host registry publishes.
    #[must_use]
    pub fn from_hosts(hosts: &HostServices) -> Self {
        Self {
            execution_host_id: hosts.execution_host_id().clone(),
            available_services: hosts.available_kinds(),
            port_registered: hosts.registered_tool_bridge().is_some(),
        }
    }

    /// Returns the execution host that owns every registered service.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    /// Returns every host-service kind the registry exposes.
    #[must_use]
    pub const fn available_services(&self) -> &BTreeSet<HostServiceKind> {
        &self.available_services
    }

    /// Reports whether the optional registered-tool port is registered.
    #[must_use]
    pub const fn port_registered(&self) -> bool {
        self.port_registered
    }
}

/// Proof that one exact selection passed the typed readiness gate.
///
/// There is no public constructor and no way to build one from provider input.
/// The kernel checks it against the exact open request, so a proof taken from
/// another selection, server revision, carrier, or host cannot be replayed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolTopologyProof {
    execution_host_id: ExecutionHostId,
    server_id: RegisteredServerId,
    server_revision: RegisteredServerRevision,
    transport: RegisteredToolTransport,
    protocol_version: RegisteredToolProtocolVersion,
    attachment: RegisteredToolAttachment,
    selected: Vec<RegisteredToolId>,
}

impl RegisteredToolTopologyProof {
    fn for_selection(selection: &RegisteredToolSelection) -> Self {
        let snapshot = selection.snapshot();
        Self {
            execution_host_id: snapshot.execution_host_id().clone(),
            server_id: snapshot.server_id().clone(),
            server_revision: snapshot.revision().clone(),
            transport: selection.transport(),
            protocol_version: selection.protocol_version().clone(),
            attachment: selection.attachment(),
            selected: selection.selected().to_vec(),
        }
    }

    /// Reports whether this proof was issued for exactly this selection.
    #[must_use]
    pub fn matches(&self, selection: &RegisteredToolSelection) -> bool {
        self == &Self::for_selection(selection)
    }
}

/// Typed readiness of one registered-tool selection against a host topology.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolReadiness {
    port: RegisteredToolPortAvailability,
    execution_host_matches: bool,
    missing_services: BTreeSet<HostServiceKind>,
    transport_qualified: bool,
    protocol_qualified: bool,
    attachment_qualified: bool,
    attachment_failure: Option<RegisteredToolFailureKind>,
    proof: RegisteredToolTopologyProof,
}

impl RegisteredToolReadiness {
    /// Evaluates port availability and selected topology before provider work.
    #[must_use]
    pub fn evaluate(hosts: &HostServices, selection: &RegisteredToolSelection) -> Self {
        Self::for_topology(&RegisteredToolMountedTopology::from_hosts(hosts), selection)
    }

    /// Evaluates one selection against an already captured mounted topology.
    #[must_use]
    pub fn for_topology(
        topology: &RegisteredToolMountedTopology,
        selection: &RegisteredToolSelection,
    ) -> Self {
        let snapshot = selection.snapshot();
        let missing_services = snapshot
            .required_services()
            .iter()
            .copied()
            .filter(|kind| !topology.available_services.contains(kind))
            .collect();
        let attachment_failure = selection
            .validate_attachment()
            .err()
            .map(|failure| failure.kind());
        Self {
            port: if topology.port_registered {
                RegisteredToolPortAvailability::Registered
            } else {
                RegisteredToolPortAvailability::Absent
            },
            execution_host_matches: &topology.execution_host_id == snapshot.execution_host_id(),
            missing_services,
            transport_qualified: transport_is_qualified(selection.transport())
                || (selection.attachment() == RegisteredToolAttachment::MediatedStdioProxy
                    && selection.transport() == RegisteredToolTransport::PrivateLoopbackHttp),
            protocol_qualified: protocol_is_qualified(selection.protocol_version()),
            attachment_qualified: attachment_failure.is_none(),
            attachment_failure,
            proof: RegisteredToolTopologyProof::for_selection(selection),
        }
    }

    /// Returns the optional-port availability.
    #[must_use]
    pub const fn port(&self) -> RegisteredToolPortAvailability {
        self.port
    }

    /// Reports whether the registry belongs to the snapshot's execution host.
    #[must_use]
    pub const fn execution_host_matches(&self) -> bool {
        self.execution_host_matches
    }

    /// Returns every required service kind missing from the registry.
    #[must_use]
    pub const fn missing_services(&self) -> &BTreeSet<HostServiceKind> {
        &self.missing_services
    }

    /// Reports whether the selected carrier is qualified in this slice.
    #[must_use]
    pub const fn transport_qualified(&self) -> bool {
        self.transport_qualified
    }

    /// Reports whether the negotiated protocol version is qualified.
    #[must_use]
    pub const fn protocol_qualified(&self) -> bool {
        self.protocol_qualified
    }

    /// Reports whether the attachment matches the carrier and recipe policy.
    #[must_use]
    pub const fn attachment_qualified(&self) -> bool {
        self.attachment_qualified
    }

    /// Reports whether every readiness dimension passed.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        matches!(self.port, RegisteredToolPortAvailability::Registered)
            && self.execution_host_matches
            && self.missing_services.is_empty()
            && self.transport_qualified
            && self.protocol_qualified
            && self.attachment_qualified
    }

    /// Reports whether this record was evaluated for exactly this selection.
    ///
    /// Readiness is evidence about one exact selection and topology. A consumer
    /// or projection that holds a readiness record from another selection must
    /// fail closed rather than compose it, so this check is available whether
    /// or not the record is ready. It exposes no proof and mints none.
    #[must_use]
    pub fn was_evaluated_for(&self, selection: &RegisteredToolSelection) -> bool {
        self.proof.matches(selection)
    }

    /// Returns the topology proof, or the first exact readiness failure.
    ///
    /// This is the only way to obtain a [`RegisteredToolTopologyProof`], and the
    /// kernel refuses to open a lease without one.
    pub fn require_ready(&self) -> Result<RegisteredToolTopologyProof, RegisteredToolFailure> {
        if !matches!(self.port, RegisteredToolPortAvailability::Registered) {
            return Err(reject(RegisteredToolFailureKind::MissingHostService));
        }
        if !self.execution_host_matches {
            return Err(reject(RegisteredToolFailureKind::UnsupportedRegistration));
        }
        if !self.missing_services.is_empty() {
            return Err(reject(RegisteredToolFailureKind::MissingHostService));
        }
        if !self.transport_qualified {
            return Err(reject(RegisteredToolFailureKind::UnsupportedTransport));
        }
        if !self.protocol_qualified {
            return Err(reject(
                RegisteredToolFailureKind::UnsupportedProtocolVersion,
            ));
        }
        if let Some(kind) = self.attachment_failure {
            return Err(reject(kind));
        }
        Ok(self.proof.clone())
    }
}

fn transport_is_qualified(transport: RegisteredToolTransport) -> bool {
    REGISTERED_TOOL_QUALIFIED_TRANSPORTS.contains(&transport)
}

fn protocol_is_qualified(version: &RegisteredToolProtocolVersion) -> bool {
    version.as_str() == REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION
}
