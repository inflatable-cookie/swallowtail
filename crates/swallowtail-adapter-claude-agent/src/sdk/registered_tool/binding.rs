//! Route qualification of one registered selection for `claude-agent.sdk`.
//!
//! Qualification happens at preparation, before any process, connection, or
//! provider work. It owns no runtime resource and opens no lease. The
//! mediated-stdio courier is bound into open through the Card 116 host
//! composition, not through card 084's consumer-declared server type.

use super::carrier::ClaudeAgentSdkRegisteredToolCarrier;
use crate::sdk::failure::failure;
use swallowtail_host_local::LocalHostServices;
use swallowtail_runtime::{
    RegisteredToolAttachment, RegisteredToolPreparation, RegisteredToolSelection,
    RegisteredToolTransport, RuntimeFailure,
};

/// One registered selection qualified for the Claude Agent SDK mediated-stdio
/// courier seam.
///
/// Absence of a host composition is allowed at qualify so the Copy profile can
/// return this binding the same way it returns an MCP binding. Open refuses a
/// mediated-stdio binding that never received [`Self::with_host`].
#[derive(Clone)]
pub struct ClaudeAgentSdkRegisteredToolBinding {
    preparation: RegisteredToolPreparation,
    carrier: ClaudeAgentSdkRegisteredToolCarrier,
    host: Option<LocalHostServices>,
}

impl ClaudeAgentSdkRegisteredToolBinding {
    /// Qualifies one registered selection for the mediated-stdio courier path.
    ///
    /// The selection must carry [`RegisteredToolAttachment::MediatedStdioProxy`]
    /// over [`RegisteredToolTransport::PrivateLoopbackHttp`] with a resolved
    /// proxy recipe, and the carrier must be able to spell every selected MCP
    /// tool. Host-mediated callback selections stay on the Card 116 mediator;
    /// they are not this route-binding seam.
    pub fn qualify(preparation: RegisteredToolPreparation) -> Result<Self, RuntimeFailure> {
        let selection = preparation.selection();
        if selection.attachment() != RegisteredToolAttachment::MediatedStdioProxy
            || selection.transport() != RegisteredToolTransport::PrivateLoopbackHttp
            || selection.proxy_recipe().is_none()
        {
            return Err(failure(
                "swallowtail.claude-agent.sdk.registered_tool.transport_unsupported",
                "Claude Agent SDK registered-tool route binding admits only the mediated stdio proxy over private loopback HTTP",
            ));
        }
        let carrier = ClaudeAgentSdkRegisteredToolCarrier::new(selection)
            .map_err(|error| RuntimeFailure::new(error.diagnostic().safe().clone()))?;
        Ok(Self {
            preparation,
            carrier,
            host: None,
        })
    }

    /// Binds the host composition that mints the one-shot courier rendezvous.
    ///
    /// Open uses this composition for `registered_tool_proxy_launch` only. The
    /// courier process itself starts through the session's `HostServices`
    /// process port, the same launcher the sidecar uses.
    #[must_use]
    pub fn with_host(mut self, host: LocalHostServices) -> Self {
        self.host = Some(host);
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
    pub const fn carrier(&self) -> &ClaudeAgentSdkRegisteredToolCarrier {
        &self.carrier
    }

    /// Returns the host composition bound for rendezvous materialization.
    #[must_use]
    pub const fn host(&self) -> Option<&LocalHostServices> {
        self.host.as_ref()
    }
}
