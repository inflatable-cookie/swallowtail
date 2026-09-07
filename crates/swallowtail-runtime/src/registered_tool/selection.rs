//! Immutable selection binding one snapshot revision into a prepared plan.

use super::attachment::{
    RegisteredToolAttachment, RegisteredToolAttachmentDescriptor, RegisteredToolProxyRecipe,
};
use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::identity::{
    RegisteredToolExecutionKind, RegisteredToolId, RegisteredToolProtocolVersion,
    RegisteredToolTransport,
};
use super::limits::{MAX_REGISTERED_TOOL_SELECTED_TOOLS, RegisteredToolBounds};
use super::snapshot::RegisteredToolSnapshot;
use std::collections::BTreeSet;
use std::sync::Arc;

/// Exact registration revision, tools, carrier, and bounds bound to a plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolSelection {
    snapshot: Arc<RegisteredToolSnapshot>,
    selected: Vec<RegisteredToolId>,
    transport: RegisteredToolTransport,
    protocol_version: RegisteredToolProtocolVersion,
    effective_bounds: RegisteredToolBounds,
    attachment: RegisteredToolAttachment,
    proxy_recipe: Option<RegisteredToolProxyRecipe>,
}

impl RegisteredToolSelection {
    /// Selects an exact tool subset, carrier, and protocol version.
    ///
    /// Selection fails before prepare when a tool is unknown, duplicated, the
    /// carrier is undeclared, or the protocol version is outside the snapshot's
    /// explicit subset.
    pub fn new(
        snapshot: Arc<RegisteredToolSnapshot>,
        selected: impl IntoIterator<Item = RegisteredToolId>,
        transport: RegisteredToolTransport,
        protocol_version: RegisteredToolProtocolVersion,
    ) -> Result<Self, RegisteredToolFailure> {
        let selected: Vec<_> = selected.into_iter().collect();
        if selected.is_empty() {
            return Err(reject(RegisteredToolFailureKind::UnsupportedTool));
        }
        if selected.len() > MAX_REGISTERED_TOOL_SELECTED_TOOLS {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        let unique: BTreeSet<&RegisteredToolId> = selected.iter().collect();
        if unique.len() != selected.len() {
            return Err(reject(RegisteredToolFailureKind::IdentityRejected));
        }
        let support = snapshot
            .transport_support(transport)
            .ok_or_else(|| reject(RegisteredToolFailureKind::UnsupportedTransport))?;
        if !support.protocol_versions().contains(&protocol_version) {
            return Err(reject(
                RegisteredToolFailureKind::UnsupportedProtocolVersion,
            ));
        }
        let mut effective_bounds = snapshot.bounds();
        for id in &selected {
            let declaration = snapshot
                .declaration(id)
                .ok_or_else(|| reject(RegisteredToolFailureKind::UnsupportedTool))?;
            effective_bounds = effective_bounds.narrowed(declaration.bounds());
        }
        Ok(Self {
            snapshot,
            selected,
            transport,
            protocol_version,
            effective_bounds,
            attachment: RegisteredToolAttachment::HostMediated,
            proxy_recipe: None,
        })
    }

    /// Selects the exact attachment shape without opening a resource.
    #[must_use]
    pub fn with_attachment(mut self, attachment: RegisteredToolAttachment) -> Self {
        self.attachment = attachment;
        self
    }

    /// Binds the host-approved courier recipe and selects mediated stdio.
    #[must_use]
    pub fn with_proxy_recipe(mut self, recipe: RegisteredToolProxyRecipe) -> Self {
        self.attachment = RegisteredToolAttachment::MediatedStdioProxy;
        self.proxy_recipe = Some(recipe);
        self
    }

    /// Returns the exact selected attachment shape.
    #[must_use]
    pub const fn attachment(&self) -> RegisteredToolAttachment {
        self.attachment
    }

    /// Returns the host-approved courier recipe, when selected.
    #[must_use]
    pub const fn proxy_recipe(&self) -> Option<&RegisteredToolProxyRecipe> {
        self.proxy_recipe.as_ref()
    }

    /// Returns a descriptor safe to carry into route projection.
    #[must_use]
    pub const fn attachment_descriptor(&self) -> RegisteredToolAttachmentDescriptor {
        RegisteredToolAttachmentDescriptor::new(self.attachment, self.transport)
    }

    /// Validates attachment/carrier and host-recipe binding before open.
    pub(super) fn validate_attachment(&self) -> Result<(), RegisteredToolFailure> {
        if self.attachment.transport() != self.transport {
            return Err(reject(RegisteredToolFailureKind::UnsupportedTransport));
        }
        match self.attachment {
            RegisteredToolAttachment::HostMediated if self.proxy_recipe.is_none() => Ok(()),
            RegisteredToolAttachment::HostMediated => {
                Err(reject(RegisteredToolFailureKind::ProcessRecipeUnavailable))
            }
            RegisteredToolAttachment::MediatedStdioProxy => {
                let recipe = self
                    .proxy_recipe
                    .as_ref()
                    .ok_or_else(|| reject(RegisteredToolFailureKind::ProcessRecipeUnavailable))?;
                if !self
                    .snapshot
                    .executable_recipes()
                    .iter()
                    .any(|reference| reference == recipe.executable())
                    || !self
                        .snapshot
                        .environment_recipes()
                        .iter()
                        .any(|reference| reference == recipe.environment())
                {
                    return Err(reject(RegisteredToolFailureKind::ProcessRecipeUnavailable));
                }
                // A mixed selection is admitted so the mediated proxy can
                // apply its defense-in-depth MCP-only exposure and dispatch
                // filters. A selection with no MCP declaration remains an
                // unsupported attachment before any host work is opened.
                if self.selected.iter().all(|id| {
                    self.snapshot.declaration(id).is_none_or(|declaration| {
                        declaration.kind() != RegisteredToolExecutionKind::Mcp
                    })
                }) {
                    return Err(reject(RegisteredToolFailureKind::UnsupportedTool));
                }
                Ok(())
            }
        }
    }

    /// Returns the exact snapshot this selection binds.
    #[must_use]
    pub fn snapshot(&self) -> &Arc<RegisteredToolSnapshot> {
        &self.snapshot
    }

    /// Returns the selected namespaced tool identities.
    #[must_use]
    pub fn selected(&self) -> &[RegisteredToolId] {
        &self.selected
    }

    /// Reports whether one namespaced identity is selected.
    #[must_use]
    pub fn contains(&self, id: &RegisteredToolId) -> bool {
        self.selected.iter().any(|selected| selected == id)
    }

    /// Returns the exact selected carrier.
    #[must_use]
    pub const fn transport(&self) -> RegisteredToolTransport {
        self.transport
    }

    /// Returns the exact negotiated protocol version.
    #[must_use]
    pub const fn protocol_version(&self) -> &RegisteredToolProtocolVersion {
        &self.protocol_version
    }

    /// Returns the bounds after narrowing by every selected declaration.
    #[must_use]
    pub const fn effective_bounds(&self) -> RegisteredToolBounds {
        self.effective_bounds
    }
}
