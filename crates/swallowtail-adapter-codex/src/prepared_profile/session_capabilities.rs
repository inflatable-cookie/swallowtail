use super::CodexPreparedSessionKind;
use super::plan::failure;
use crate::{CodexPreparedIntegration, codex_bounded_workspace_access_policy};
use std::collections::BTreeSet;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, InstalledExecutableCompatibility,
    SessionAccessPolicy,
};
use swallowtail_runtime::{PreparationFailure, SchemaDocument, SessionOptions};

const JSON_SCHEMA_MEDIA_TYPE: &str = "application/schema+json";
const MAXIMUM_DYNAMIC_TOOLS: u32 = 4;
const MAXIMUM_TOOL_SCHEMA_BYTES: u64 = 4096;

pub(super) fn session_capabilities(
    kind: CodexPreparedSessionKind,
    options: &SessionOptions,
) -> Result<
    (
        Vec<CapabilityRequirement>,
        BTreeSet<swallowtail_core::ExtensionNamespace>,
        SessionAccessPolicy,
    ),
    PreparationFailure,
> {
    let mut capabilities = vec![CapabilityRequirement::new(
        Capability::InteractiveSession,
        [],
    )];
    if let Some(mode) = options.reasoning_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::reasoning_mode(mode.clone())],
        ));
    }
    let tools = options.tools().collect::<Vec<_>>();
    if !tools.is_empty() {
        capabilities.push(tool_capability(tools)?);
    }
    let access_policy = match kind {
        CodexPreparedSessionKind::ReadOnly => SessionAccessPolicy::read_only(),
        CodexPreparedSessionKind::BoundedWorkspace => codex_bounded_workspace_access_policy(),
    };
    Ok((capabilities, BTreeSet::new(), access_policy))
}

fn tool_capability(
    tools: Vec<&swallowtail_runtime::ToolDeclaration>,
) -> Result<CapabilityRequirement, PreparationFailure> {
    if tools.len() > usize::try_from(MAXIMUM_DYNAMIC_TOOLS).unwrap_or(usize::MAX) {
        return Err(failure(
            "swallowtail.codex.preparation.tool_limit",
            "Codex prepared session declares too many dynamic tools",
        ));
    }
    let mut names = BTreeSet::new();
    let mut constraints = BTreeSet::from([
        CapabilityConstraint::ToolMaximumCount(MAXIMUM_DYNAMIC_TOOLS),
        CapabilityConstraint::ToolMaximumSchemaBytes(MAXIMUM_TOOL_SCHEMA_BYTES),
    ]);
    for tool in tools {
        if !names.insert(tool.name()) || tool.schema_media_type() != JSON_SCHEMA_MEDIA_TYPE {
            return Err(failure(
                "swallowtail.codex.preparation.tool_unsupported",
                "Codex prepared session tools must have unique names and JSON Schema input",
            ));
        }
        let SchemaDocument::Inline(bytes) = tool.input_schema() else {
            return Err(failure(
                "swallowtail.codex.preparation.tool_unsupported",
                "Codex prepared session tools require inline JSON Schema input",
            ));
        };
        if serde_json::from_slice::<serde_json::Value>(bytes).is_err() {
            return Err(failure(
                "swallowtail.codex.preparation.tool_schema_invalid",
                "Codex prepared session tool schema is invalid",
            ));
        }
        if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAXIMUM_TOOL_SCHEMA_BYTES {
            return Err(failure(
                "swallowtail.codex.preparation.tool_schema_limit",
                "Codex prepared session tool schema exceeds its supported bound",
            ));
        }
        constraints.insert(
            CapabilityConstraint::tool_schema_dialect(tool.schema_dialect())
                .expect("tool dialect is non-empty"),
        );
    }
    Ok(CapabilityRequirement::new(
        Capability::ToolCalls,
        constraints,
    ))
}

pub(super) fn behavior_revision(prepared: &CodexPreparedIntegration) -> Option<&str> {
    match prepared.observation().compatibility() {
        InstalledExecutableCompatibility::Qualified(assessment) => {
            Some(assessment.behavior_revision().as_str())
        }
        InstalledExecutableCompatibility::UnverifiedNewer(assessment) => {
            Some(assessment.behavior_revision().as_str())
        }
        InstalledExecutableCompatibility::Incompatible => None,
    }
}
