//! Route qualification of one registered selection for `grok-build.acp`.
//!
//! Qualification happens at preparation, before any process, connection, or
//! provider work. It owns no runtime resource and opens no lease. The host
//! composition and the registered-tool open deadline are bound separately so
//! a consumer can qualify a selection without holding either yet; open refuses
//! a binding that is still missing one.

use super::carrier::GrokRegisteredToolCarrier;
use crate::failure::failure;
use swallowtail_host_local::LocalHostServices;
use swallowtail_runtime::{
    Deadline, PreparationFailure, RegisteredToolPreparation, RegisteredToolSelection,
    RuntimeFailure,
};

/// One registered selection qualified for the Grok ACP courier seam.
#[derive(Clone)]
pub struct GrokRegisteredToolBinding {
    preparation: RegisteredToolPreparation,
    carrier: GrokRegisteredToolCarrier,
    host: Option<LocalHostServices>,
    deadline: Option<Deadline>,
}

impl GrokRegisteredToolBinding {
    /// Qualifies one registered selection for the mediated-stdio courier path.
    ///
    /// The selection must carry the Contract 063 mediated stdio proxy over
    /// private loopback HTTP with a resolved proxy recipe, and the carrier must
    /// be able to spell every selected MCP tool.
    pub fn qualify(preparation: RegisteredToolPreparation) -> Result<Self, PreparationFailure> {
        let carrier = GrokRegisteredToolCarrier::new(preparation.selection())?;
        Ok(Self {
            preparation,
            carrier,
            host: None,
            deadline: None,
        })
    }

    /// Binds the host composition that resolves the approved courier path and
    /// environment and mints the one-shot rendezvous.
    ///
    /// Open uses this composition for resolution and `registered_tool_proxy_launch`
    /// only. The courier process itself is spawned by Grok from the declared
    /// ACP `mcpServers` entry.
    #[must_use]
    pub fn with_host(mut self, host: LocalHostServices) -> Self {
        self.host = Some(host);
        self
    }

    /// Binds the deadline the registered-tool lease is opened under.
    ///
    /// This route rejects a session deadline on its open request, so the
    /// registered-tool deadline is stated here instead of being inherited. It
    /// bounds the lease, the courier ready barrier, and every issued call.
    #[must_use]
    pub const fn with_open_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Returns the immutable opt-in preparation this binding qualified.
    #[must_use]
    pub const fn preparation(&self) -> &RegisteredToolPreparation {
        &self.preparation
    }

    /// Returns the immutable selection bound at preparation.
    #[must_use]
    pub const fn selection(&self) -> &RegisteredToolSelection {
        self.preparation.selection()
    }

    /// Returns the Swallowtail-owned carrier derived from this selection.
    #[must_use]
    pub const fn carrier(&self) -> &GrokRegisteredToolCarrier {
        &self.carrier
    }

    /// Returns the host composition bound for path resolution and rendezvous.
    #[must_use]
    pub const fn host(&self) -> Option<&LocalHostServices> {
        self.host.as_ref()
    }

    /// Returns the bound registered-tool open deadline.
    #[must_use]
    pub const fn open_deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    pub(crate) fn require_host(&self) -> Result<&LocalHostServices, RuntimeFailure> {
        self.host.as_ref().ok_or_else(|| {
            failure(
                "swallowtail.grok.acp.registered_tool.host_missing",
                "Grok Build ACP registered-tool open requires the local host composition that mints the courier rendezvous",
            )
        })
    }

    pub(crate) fn require_deadline(&self) -> Result<Deadline, RuntimeFailure> {
        self.deadline.ok_or_else(|| {
            failure(
                "swallowtail.grok.acp.registered_tool.deadline_missing",
                "Grok Build ACP registered-tool open requires an explicit registered-tool deadline",
            )
        })
    }
}
