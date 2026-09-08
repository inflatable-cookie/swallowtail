//! The Swallowtail-owned registered-tool MCP carrier for this exact route.
//!
//! The carrier is not a consumer-declared ACP server. It is derived only from
//! an immutable Contract 063 selection and is reserved under the Contract 063
//! courier server name, so a future consumer-declared `mcpServers` entry can
//! never present this identity.
//!
//! The carrier owns no process, opens no lease, and dispatches nothing. It
//! names the exact tools the courier will present and proves that each
//! namespaced identity survives the courier's `namespace/local-name` spelling
//! without ambiguity.

use super::registered_preparation_failure;
use swallowtail_host_local::wire::REGISTERED_TOOL_PROXY_SERVER_NAME;
use swallowtail_runtime::{
    PreparationFailure, RegisteredToolAttachment, RegisteredToolExecutionKind, RegisteredToolId,
    RegisteredToolSelection, RegisteredToolTransport,
};

/// Reserved ACP MCP server name of the Swallowtail-owned courier.
///
/// This is the Contract 063 courier's own `serverInfo` name. Declaring it on
/// the ACP wire under any other name would let the provider connect a server
/// whose identity does not match the one the kernel opened.
pub const GROK_ACP_REGISTERED_TOOL_SERVER: &str = REGISTERED_TOOL_PROXY_SERVER_NAME;

/// Exact route-local mediation kind Contract 061 publishes for this carrier.
///
/// It is deliberately not `common-dispatch`: Grok exposes no host-dispatch
/// callback. The registered call arrives as a client-supplied ACP MCP server
/// connection and is mediated by the Contract 063 courier, never by a portable
/// dispatch surface this route does not have.
pub const GROK_ACP_REGISTERED_TOOL_MEDIATION: &str = "route-local-acp-client-mcp-courier";

/// Separator the courier uses between a namespace and a local tool name.
const COURIER_NAME_SEPARATOR: char = '/';
/// Maximum registered tools this carrier presents in one selection.
const MAXIMUM_CARRIER_TOOLS: usize = 32;

/// One selected registered tool and the exact courier name it is presented as.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokRegisteredTool {
    id: RegisteredToolId,
    courier_tool_name: String,
}

impl GrokRegisteredTool {
    /// Returns the namespaced Contract 063 identity.
    #[must_use]
    pub const fn id(&self) -> &RegisteredToolId {
        &self.id
    }

    /// Returns the exact MCP tool name the courier presents to the provider.
    #[must_use]
    pub fn courier_tool_name(&self) -> &str {
        &self.courier_tool_name
    }
}

/// Swallowtail-owned registered-tool carrier for one immutable selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokRegisteredToolCarrier {
    selection: RegisteredToolSelection,
    tools: Vec<GrokRegisteredTool>,
}

impl GrokRegisteredToolCarrier {
    /// Derives the carrier from one immutable Contract 063 selection.
    ///
    /// The selection must carry the Contract 063 mediated-stdio proxy over
    /// private loopback HTTP with a resolved proxy recipe, and may select
    /// MCP-kind tools only: the provider is told these are MCP tools, so
    /// admitting another kind would let one namespaced identity reach Grok
    /// under a kind its snapshot never declared.
    pub fn new(selection: &RegisteredToolSelection) -> Result<Self, PreparationFailure> {
        if selection.attachment() != RegisteredToolAttachment::MediatedStdioProxy
            || selection.transport() != RegisteredToolTransport::PrivateLoopbackHttp
            || selection.proxy_recipe().is_none()
        {
            return Err(registered_preparation_failure(
                "swallowtail.grok.acp.registered_tool.transport_unsupported",
                "Grok Build ACP carries registered tools over the mediated stdio courier only",
            ));
        }
        if selection.selected().len() > MAXIMUM_CARRIER_TOOLS {
            return Err(registered_preparation_failure(
                "swallowtail.grok.acp.registered_tool.selection_rejected",
                "Grok Build ACP registered selection is outside its carrier bound",
            ));
        }
        let snapshot = selection.snapshot();
        let mut tools: Vec<GrokRegisteredTool> = Vec::with_capacity(selection.selected().len());
        for id in selection.selected() {
            let declaration = snapshot.declaration(id).ok_or_else(|| {
                registered_preparation_failure(
                    "swallowtail.grok.acp.registered_tool.tool_unknown",
                    "Grok Build ACP registered selection names a tool its snapshot does not declare",
                )
            })?;
            if declaration.kind() != RegisteredToolExecutionKind::Mcp {
                return Err(registered_preparation_failure(
                    "swallowtail.grok.acp.registered_tool.kind_unsupported",
                    "Grok Build ACP registered mediation presents MCP-kind tools only",
                ));
            }
            if serde_json::from_str::<serde_json::Value>(
                declaration.input_schema().document().expose_for_execution(),
            )
            .is_err()
            {
                return Err(registered_preparation_failure(
                    "swallowtail.grok.acp.registered_tool.schema_invalid",
                    "Grok Build ACP registered tool schema is not a valid JSON document",
                ));
            }
            let courier_tool_name = courier_tool_name(id)?;
            if tools
                .iter()
                .any(|tool| tool.courier_tool_name == courier_tool_name)
            {
                return Err(registered_preparation_failure(
                    "swallowtail.grok.acp.registered_tool.identity_unprojectable",
                    "Grok Build ACP registered tools collide on one courier tool name",
                ));
            }
            tools.push(GrokRegisteredTool {
                id: id.clone(),
                courier_tool_name,
            });
        }
        if tools.is_empty() {
            return Err(registered_preparation_failure(
                "swallowtail.grok.acp.registered_tool.selection_rejected",
                "Grok Build ACP registered mediation requires at least one selected tool",
            ));
        }
        Ok(Self {
            selection: selection.clone(),
            tools,
        })
    }

    /// Returns the reserved Swallowtail-owned ACP MCP server name.
    #[must_use]
    pub const fn server_name(&self) -> &'static str {
        GROK_ACP_REGISTERED_TOOL_SERVER
    }

    /// Returns the exact route-local mediation kind this carrier publishes.
    #[must_use]
    pub const fn mediation_kind(&self) -> &'static str {
        GROK_ACP_REGISTERED_TOOL_MEDIATION
    }

    /// Returns the immutable selection this carrier was derived from.
    #[must_use]
    pub const fn selection(&self) -> &RegisteredToolSelection {
        &self.selection
    }

    /// Returns every presented tool in selection order.
    #[must_use]
    pub fn tools(&self) -> &[GrokRegisteredTool] {
        &self.tools
    }

    /// Resolves one courier tool name to its namespaced identity.
    #[must_use]
    pub fn resolve_courier_tool(&self, name: &str) -> Option<&GrokRegisteredTool> {
        self.tools
            .iter()
            .find(|tool| tool.courier_tool_name == name)
    }
}

/// Projects one namespaced identity onto its unambiguous courier tool name.
///
/// The courier presents `namespace/local-name`. Neither half may contain that
/// separator, or two different identities could render as one wire name and
/// the reverse resolution would be a guess. An identity that cannot be spelled
/// fails closed here, before any lease, process, or provider work.
fn courier_tool_name(id: &RegisteredToolId) -> Result<String, PreparationFailure> {
    let namespace = id.namespace().as_str();
    let local_name = id.local_name().as_str();
    for part in [namespace, local_name] {
        if part.is_empty() || part.contains(COURIER_NAME_SEPARATOR) {
            return Err(registered_preparation_failure(
                "swallowtail.grok.acp.registered_tool.identity_unprojectable",
                "Grok Build ACP registered mediation rejects an identity it cannot spell unambiguously",
            ));
        }
    }
    Ok(id.to_string())
}
