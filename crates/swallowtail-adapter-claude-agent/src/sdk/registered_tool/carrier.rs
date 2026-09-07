//! The Swallowtail-owned registered-tool MCP carrier for this exact route.
//!
//! This carrier is not a consumer-declared server. [`ClaudeAgentSdkMcpServer`]
//! stays exactly what card 084 merged: a consumer-declared stdio process whose
//! command, arguments, and env allowlist the consumer supplies, admitted only
//! through the existing profile binding. The registered-tool carrier is owned
//! by Swallowtail, derived only from an immutable Contract 063 selection, and
//! reserved under its own server name so neither path can present the other's
//! identity.
//!
//! The carrier projects namespaced registered tool identities onto the exact
//! provider spelling `mcp__<server>__<tool>` and parses that spelling back to
//! exactly one identity. It owns no process, opens no lease, and dispatches
//! nothing: every admitted call is issued through the Contract 063 kernel by
//! [`super::mediation`].
//!
//! [`ClaudeAgentSdkMcpServer`]: crate::sdk::ClaudeAgentSdkMcpServer

use super::super::mcp::mcp_tool_name;
use super::super::prepared::preparation_failure;
use swallowtail_runtime::{
    PreparationFailure, PreparationStage, RegisteredToolAttachment, RegisteredToolExecutionKind,
    RegisteredToolId, RegisteredToolSelection, RegisteredToolTransport,
};

/// Reserved MCP server name of the Swallowtail-owned registered-tool carrier.
///
/// A consumer-declared server may not take this name, exactly as it may not
/// take the reserved Contract 060 watcher name.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER: &str = "swallowtail-registered-tools";

/// Exact route-local mediation kind Contract 061 publishes for this carrier.
///
/// It is deliberately not `common-dispatch`: the SDK exposes no host-dispatch
/// callback, so the registered call is mediated through this route's own MCP
/// carrier and never through a portable dispatch surface.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_MEDIATION: &str = "route-local-stdio-mcp-mediation";

/// Separator joining a producer namespace to a local tool name on the wire.
const NAMESPACE_SEPARATOR: char = '_';
/// Maximum bytes of one namespace or local name this carrier admits.
const MAXIMUM_IDENTITY_BYTES: usize = 64;
/// Maximum registered tools this carrier presents in one selection.
const MAXIMUM_CARRIER_TOOLS: usize = 32;

/// One selected registered tool and the exact provider name it is presented as.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkRegisteredTool {
    id: RegisteredToolId,
    provider_tool_name: String,
    local_name: String,
}

impl ClaudeAgentSdkRegisteredTool {
    /// Returns the namespaced Contract 063 identity.
    #[must_use]
    pub const fn id(&self) -> &RegisteredToolId {
        &self.id
    }

    /// Returns the exact `mcp__<server>__<tool>` name the provider sees.
    #[must_use]
    pub fn provider_tool_name(&self) -> &str {
        &self.provider_tool_name
    }

    /// Returns the carrier-local MCP tool name, without the server prefix.
    #[must_use]
    pub fn carrier_tool_name(&self) -> &str {
        &self.local_name
    }
}

/// Swallowtail-owned registered-tool MCP carrier for one exact selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentSdkRegisteredToolCarrier {
    selection: RegisteredToolSelection,
    tools: Vec<ClaudeAgentSdkRegisteredTool>,
}

impl ClaudeAgentSdkRegisteredToolCarrier {
    /// Derives the carrier from one immutable Contract 063 selection.
    ///
    /// The selection must use the listener-free host-mediated carrier or the
    /// Contract 063 mediated-stdio proxy over private loopback HTTP, and may
    /// select MCP-kind tools only: this route presents registered tools to the
    /// provider as MCP tools, so admitting another kind here would let one
    /// namespaced identity reach the provider under a kind its snapshot never
    /// declared. Every namespace and local name must also survive the exact
    /// provider spelling without ambiguity.
    pub fn new(selection: &RegisteredToolSelection) -> Result<Self, PreparationFailure> {
        if !admits_carrier(selection) {
            return Err(carrier_failure(
                "swallowtail.claude-agent.sdk.registered_tool.transport_unsupported",
                "Claude Agent SDK registered-tool mediation admits the listener-free host-mediated carrier or the mediated stdio proxy",
            ));
        }
        if selection.selected().len() > MAXIMUM_CARRIER_TOOLS {
            return Err(carrier_failure(
                "swallowtail.claude-agent.sdk.registered_tool.selection_rejected",
                "Claude Agent SDK registered-tool mediation rejects a selection outside its carrier bound",
            ));
        }
        let snapshot = selection.snapshot();
        let mut tools = Vec::with_capacity(selection.selected().len());
        for id in selection.selected() {
            let declaration = snapshot.declaration(id).ok_or_else(|| {
                carrier_failure(
                    "swallowtail.claude-agent.sdk.registered_tool.tool_unknown",
                    "Claude Agent SDK registered-tool mediation rejects a tool absent from its snapshot",
                )
            })?;
            if declaration.kind() != RegisteredToolExecutionKind::Mcp {
                return Err(carrier_failure(
                    "swallowtail.claude-agent.sdk.registered_tool.kind_unsupported",
                    "Claude Agent SDK registered-tool mediation presents MCP-kind tools only",
                ));
            }
            let local_name = carrier_tool_name(id)?;
            tools.push(ClaudeAgentSdkRegisteredTool {
                id: id.clone(),
                provider_tool_name: mcp_tool_name(
                    CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER,
                    &local_name,
                ),
                local_name,
            });
        }
        if tools.is_empty() {
            return Err(carrier_failure(
                "swallowtail.claude-agent.sdk.registered_tool.selection_rejected",
                "Claude Agent SDK registered-tool mediation requires at least one selected tool",
            ));
        }
        Ok(Self {
            selection: selection.clone(),
            tools,
        })
    }

    /// Returns the reserved Swallowtail-owned MCP server name.
    #[must_use]
    pub const fn server_name(&self) -> &'static str {
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER
    }

    /// Returns the exact route-local mediation kind this carrier publishes.
    #[must_use]
    pub const fn mediation_kind(&self) -> &'static str {
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_MEDIATION
    }

    /// Returns the immutable selection this carrier was derived from.
    #[must_use]
    pub const fn selection(&self) -> &RegisteredToolSelection {
        &self.selection
    }

    /// Returns every presented tool in selection order.
    #[must_use]
    pub fn tools(&self) -> &[ClaudeAgentSdkRegisteredTool] {
        &self.tools
    }

    /// Returns the exact provider tool names this carrier admits.
    #[must_use]
    pub fn admitted_tool_names(&self) -> Vec<String> {
        self.tools
            .iter()
            .map(|tool| tool.provider_tool_name.clone())
            .collect()
    }

    /// Resolves one carrier-local MCP tool name to its namespaced identity.
    #[must_use]
    pub fn resolve_carrier_tool(&self, name: &str) -> Option<&ClaudeAgentSdkRegisteredTool> {
        self.tools.iter().find(|tool| tool.local_name == name)
    }

    /// Resolves one exact `mcp__<server>__<tool>` name to its identity.
    #[must_use]
    pub fn resolve_provider_tool(&self, name: &str) -> Option<&ClaudeAgentSdkRegisteredTool> {
        self.tools
            .iter()
            .find(|tool| tool.provider_tool_name == name)
    }
}

/// Projects one namespaced identity onto its unambiguous carrier-local name.
///
/// Namespace and local name are restricted to `[A-Za-z0-9][A-Za-z0-9-]*` so the
/// single `_` separator can never appear inside either part. The projection is
/// therefore reversible, and a namespaced identity carrying `_`, `.`, or any
/// other character fails closed before provider work rather than colliding with
/// a different identity on the wire.
fn carrier_tool_name(id: &RegisteredToolId) -> Result<String, PreparationFailure> {
    let namespace = id.namespace().as_str();
    let local_name = id.local_name().as_str();
    for part in [namespace, local_name] {
        if !is_carrier_identifier(part) {
            return Err(carrier_failure(
                "swallowtail.claude-agent.sdk.registered_tool.identity_unprojectable",
                "Claude Agent SDK registered-tool mediation rejects an identity it cannot spell unambiguously",
            ));
        }
    }
    Ok(format!("{namespace}{NAMESPACE_SEPARATOR}{local_name}"))
}

fn admits_carrier(selection: &RegisteredToolSelection) -> bool {
    match (
        selection.attachment(),
        selection.transport(),
        selection.proxy_recipe().is_some(),
    ) {
        (
            RegisteredToolAttachment::HostMediated,
            RegisteredToolTransport::HostMediatedCallback,
            false,
        ) => true,
        (
            RegisteredToolAttachment::MediatedStdioProxy,
            RegisteredToolTransport::PrivateLoopbackHttp,
            true,
        ) => true,
        (
            RegisteredToolAttachment::HostMediated | RegisteredToolAttachment::MediatedStdioProxy,
            _,
            _,
        ) => false,
    }
}

fn is_carrier_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    value.len() <= MAXIMUM_IDENTITY_BYTES
        && first.is_ascii_alphanumeric()
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '-')
}

fn carrier_failure(code: &'static str, message: &'static str) -> PreparationFailure {
    preparation_failure(PreparationStage::Preflight, code, message)
}
