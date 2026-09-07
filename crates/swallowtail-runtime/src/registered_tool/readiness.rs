//! Typed registered-profile preflight over the optional host port.
//!
//! This preflight is deliberately separate from `HostServiceKind`. Contract 063
//! keeps that enum exhaustive and unchanged in this slice, so registered-tool
//! port availability and selected topology are reported through this typed
//! readiness record instead of a new service-kind variant.

use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::identity::{RegisteredToolProtocolVersion, RegisteredToolTransport};
use super::selection::RegisteredToolSelection;
use crate::HostServices;
use std::collections::BTreeSet;
use swallowtail_core::{ExecutionHostId, HostServiceKind};

/// Carriers this slice qualifies for the registered-tool profile.
///
/// Host-mediated callback dispatch binds no listener. The private loopback HTTP
/// and SSE carriers stay withheld until a route corpus qualifies them.
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

/// Typed readiness of one registered-tool selection against a host registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolReadiness {
    port: RegisteredToolPortAvailability,
    execution_host_matches: bool,
    missing_services: BTreeSet<HostServiceKind>,
    transport_qualified: bool,
    protocol_qualified: bool,
}

impl RegisteredToolReadiness {
    /// Evaluates port availability and selected topology before provider work.
    #[must_use]
    pub fn evaluate(hosts: &HostServices, selection: &RegisteredToolSelection) -> Self {
        let snapshot = selection.snapshot();
        let available = hosts.available_kinds();
        let missing_services = snapshot
            .required_services()
            .iter()
            .copied()
            .filter(|kind| !available.contains(kind))
            .collect();
        Self {
            port: if hosts.registered_tool_bridge().is_some() {
                RegisteredToolPortAvailability::Registered
            } else {
                RegisteredToolPortAvailability::Absent
            },
            execution_host_matches: host_matches(
                hosts.execution_host_id(),
                snapshot.execution_host_id(),
            ),
            missing_services,
            transport_qualified: transport_is_qualified(selection.transport()),
            protocol_qualified: protocol_is_qualified(selection.protocol_version()),
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

    /// Reports whether every readiness dimension passed.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        matches!(self.port, RegisteredToolPortAvailability::Registered)
            && self.execution_host_matches
            && self.missing_services.is_empty()
            && self.transport_qualified
            && self.protocol_qualified
    }

    /// Returns the first exact readiness failure, if any.
    pub fn require_ready(&self) -> Result<(), RegisteredToolFailure> {
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
        Ok(())
    }
}

fn host_matches(registry: &ExecutionHostId, snapshot: &ExecutionHostId) -> bool {
    registry == snapshot
}

fn transport_is_qualified(transport: RegisteredToolTransport) -> bool {
    REGISTERED_TOOL_QUALIFIED_TRANSPORTS.contains(&transport)
}

fn protocol_is_qualified(version: &RegisteredToolProtocolVersion) -> bool {
    version.as_str() == REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION
}
