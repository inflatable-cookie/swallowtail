//! Qualification of one registered selection against the Codex tool seam.

use crate::rpc::failure;
use std::collections::BTreeMap;
use swallowtail_runtime::{
    RegisteredToolExecutionKind, RegisteredToolId, RegisteredToolPreparation,
    RegisteredToolProgressMode, RegisteredToolQualifiedRoute, RegisteredToolSelection,
    RegisteredToolSkillDelivery, RegisteredToolTransport, RuntimeFailure, SchemaDocument,
    ToolDeclaration,
};

/// Separator that joins a producer namespace to its local tool name.
///
/// Codex app-server rejects a namespaced dynamic tool callback, so one flat
/// wire name has to carry the whole namespaced identity. Neither half may
/// contain this separator, which keeps the encoding unambiguous in both
/// directions instead of relying on a first or last match.
pub const CODEX_REGISTERED_TOOL_NAME_SEPARATOR: &str = "__";

/// Exact registered-capability dimensions this route proves.
///
/// The Deny path is the released dynamic-tool failure result the provider
/// actually receives, so it is an exact route-supported denial rather than a
/// simulated acknowledgement. Codex projects no consumer tool progress for a
/// dynamic tool call, and this card transports no selected skill bundle.
pub const CODEX_REGISTERED_TOOL_ROUTE: RegisteredToolQualifiedRoute =
    RegisteredToolQualifiedRoute::new(
        swallowtail_runtime::RegisteredToolPermissionStrength::ExactOneShot,
        RegisteredToolProgressMode::NoProgress,
        RegisteredToolSkillDelivery::NotCarried,
    );

const JSON_SCHEMA_MEDIA_TYPE: &str = "application/schema+json";
/// Longest dynamic tool name the Codex function-tool wire accepts.
const MAX_WIRE_NAME_BYTES: usize = 64;

/// One registered selection qualified for the Codex dynamic native tool seam.
///
/// Qualification happens at preparation, before any process, connection, or
/// provider work. It owns no runtime resource and opens no lease.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexRegisteredToolBinding {
    preparation: RegisteredToolPreparation,
    declarations: Vec<ToolDeclaration>,
    wire_names: BTreeMap<String, RegisteredToolId>,
}

impl CodexRegisteredToolBinding {
    /// Qualifies one registered selection for Codex dynamic native tools.
    ///
    /// Every selected tool must be a native client tool carried by the
    /// host-mediated callback carrier with an inline JSON Schema input. An MCP,
    /// app, or provider-owned tool, an unqualified carrier, or an identity the
    /// Codex wire cannot express fails here rather than at dispatch.
    pub fn qualify(preparation: RegisteredToolPreparation) -> Result<Self, RuntimeFailure> {
        let selection = preparation.selection();
        if selection.transport() != RegisteredToolTransport::HostMediatedCallback {
            return Err(unsupported_transport(selection.transport()));
        }
        let snapshot = selection.snapshot();
        let mut declarations = Vec::with_capacity(selection.selected().len());
        let mut wire_names = BTreeMap::new();
        for id in selection.selected() {
            let declaration = snapshot.declaration(id).ok_or_else(|| {
                failure(
                    "swallowtail.codex.app_server.registered_tool_unknown",
                    "Codex registered selection names a tool the snapshot does not declare",
                )
            })?;
            require_native_kind(declaration.kind())?;
            let schema = declaration.input_schema();
            if schema.media_type().as_str() != JSON_SCHEMA_MEDIA_TYPE {
                return Err(failure(
                    "swallowtail.codex.app_server.registered_schema_unsupported",
                    "Codex registered tools require a JSON Schema input document",
                ));
            }
            let body = schema.document().expose_for_execution();
            if serde_json::from_str::<serde_json::Value>(body).is_err() {
                return Err(failure(
                    "swallowtail.codex.app_server.registered_schema_invalid",
                    "Codex registered tool schema is not valid JSON",
                ));
            }
            let name = wire_name(id)?;
            if wire_names.insert(name.clone(), id.clone()).is_some() {
                return Err(failure(
                    "swallowtail.codex.app_server.registered_identity_unsupported",
                    "Codex registered tools collide on one dynamic tool name",
                ));
            }
            declarations.push(
                ToolDeclaration::new(
                    name,
                    SchemaDocument::Inline(body.as_bytes().to_vec()),
                    JSON_SCHEMA_MEDIA_TYPE,
                    schema.dialect().as_str(),
                )
                .map_err(|_| {
                    failure(
                        "swallowtail.codex.app_server.registered_identity_unsupported",
                        "Codex registered tool declaration is not transportable",
                    )
                })?,
            );
        }
        Ok(Self {
            preparation,
            declarations,
            wire_names,
        })
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

    /// Returns the dynamic tool declarations this binding transports.
    #[must_use]
    pub fn declarations(&self) -> &[ToolDeclaration] {
        &self.declarations
    }

    /// Returns the namespaced identity one Codex dynamic tool name binds.
    #[must_use]
    pub fn tool_for(&self, wire_name: &str) -> Option<&RegisteredToolId> {
        self.wire_names.get(wire_name)
    }

    /// Returns every Codex dynamic tool name this binding declares.
    pub(crate) fn wire_names(&self) -> impl Iterator<Item = &str> {
        self.wire_names.keys().map(String::as_str)
    }
}

fn require_native_kind(kind: RegisteredToolExecutionKind) -> Result<(), RuntimeFailure> {
    match kind {
        RegisteredToolExecutionKind::NativeClient => Ok(()),
        // Codex app-server projects provider-owned MCP tool activity, which is
        // observation, not consumer registration or result authority. Binding a
        // registered MCP tool needs its own app-server surface and corpus.
        RegisteredToolExecutionKind::Mcp => Err(failure(
            "swallowtail.codex.app_server.registered_mcp_withheld",
            "Codex app-server provider-direct MCP registration is withheld without exact surface evidence",
        )),
        RegisteredToolExecutionKind::App | RegisteredToolExecutionKind::ProviderOwned => {
            Err(failure(
                "swallowtail.codex.app_server.registered_kind_unsupported",
                "Codex app-server binds registered native client tools only",
            ))
        }
    }
}

fn unsupported_transport(transport: RegisteredToolTransport) -> RuntimeFailure {
    let _ = transport;
    failure(
        "swallowtail.codex.app_server.registered_transport_unsupported",
        "Codex app-server carries registered tools over host-mediated callbacks only",
    )
}

/// Renders one namespaced identity as a single Codex dynamic tool name.
fn wire_name(id: &RegisteredToolId) -> Result<String, RuntimeFailure> {
    let namespace = id.namespace().as_str();
    let local_name = id.local_name().as_str();
    if !is_wire_segment(namespace) || !is_wire_segment(local_name) {
        return Err(failure(
            "swallowtail.codex.app_server.registered_identity_unsupported",
            "Codex registered tool identities must be unseparated alphanumeric segments",
        ));
    }
    let name = format!("{namespace}{CODEX_REGISTERED_TOOL_NAME_SEPARATOR}{local_name}");
    if name.len() > MAX_WIRE_NAME_BYTES {
        return Err(failure(
            "swallowtail.codex.app_server.registered_identity_unsupported",
            "Codex registered tool name exceeds the dynamic tool name bound",
        ));
    }
    Ok(name)
}

fn is_wire_segment(value: &str) -> bool {
    !value.is_empty()
        && !value.contains(CODEX_REGISTERED_TOOL_NAME_SEPARATOR)
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

#[cfg(test)]
#[path = "binding_tests.rs"]
mod tests;
