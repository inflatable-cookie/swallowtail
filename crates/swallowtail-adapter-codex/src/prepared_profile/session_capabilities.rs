use super::CodexPreparedSessionKind;
use super::plan::failure;
use crate::{
    CodexPreparedIntegration, codex_approval_request_extension,
    codex_bounded_workspace_access_policy, codex_user_input_request_extension,
};
use std::collections::BTreeSet;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, InstalledExecutableCompatibility,
    ProviderRequestPolicy, SessionAccessPolicy,
};
use swallowtail_runtime::{PreparationFailure, SchemaDocument, SessionOptions};

const JSON_SCHEMA_MEDIA_TYPE: &str = "application/schema+json";
const PLAN_MODE_MINIMUM_VERSION: semver::Version = semver::Version::new(0, 88, 0);

pub(super) fn session_capabilities(
    kind: CodexPreparedSessionKind,
    options: &SessionOptions,
    user_input_exchange: bool,
) -> Result<
    (
        Vec<CapabilityRequirement>,
        BTreeSet<swallowtail_core::ExtensionNamespace>,
        SessionAccessPolicy,
    ),
    PreparationFailure,
> {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(
            Capability::LoadSession,
            [
                CapabilityConstraint::ReplayMaximumItems(
                    crate::session_replay::MAXIMUM_REPLAY_ITEMS as u32,
                ),
                CapabilityConstraint::ReplayMaximumBytes(
                    crate::session_replay::MAXIMUM_REPLAY_BYTES as u64,
                ),
            ],
        ),
        CapabilityRequirement::new(Capability::Resume, []),
    ];
    if let Some(mode) = options.reasoning_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::reasoning_mode(mode.clone())],
        ));
    }
    if let Some(mode) = options.harness_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::HarnessModeSelection,
            [CapabilityConstraint::harness_mode(mode)],
        ));
    }
    let tools = options.tools().collect::<Vec<_>>();
    if !tools.is_empty() {
        capabilities.push(tool_capability(tools)?);
    }
    let mut access_policy = match kind {
        CodexPreparedSessionKind::ReadOnly => SessionAccessPolicy::read_only(),
        CodexPreparedSessionKind::BoundedWorkspace => codex_bounded_workspace_access_policy(),
    };
    if user_input_exchange {
        let provider_requests = match kind {
            CodexPreparedSessionKind::ReadOnly => {
                ProviderRequestPolicy::exchange([codex_user_input_request_extension()])
            }
            CodexPreparedSessionKind::BoundedWorkspace => {
                ProviderRequestPolicy::observe_and_exchange(
                    [codex_approval_request_extension()],
                    [codex_user_input_request_extension()],
                )
            }
        };
        access_policy = access_policy.with_provider_requests(provider_requests);
    }
    Ok((capabilities, BTreeSet::new(), access_policy))
}

fn tool_capability(
    tools: Vec<&swallowtail_runtime::ToolDeclaration>,
) -> Result<CapabilityRequirement, PreparationFailure> {
    let tool_count = u32::try_from(tools.len()).map_err(|_| {
        failure(
            "swallowtail.codex.preparation.tool_limit",
            "Codex prepared session tool count cannot be represented",
        )
    })?;
    let mut names = BTreeSet::new();
    let mut maximum_schema_bytes = 0;
    let mut constraints = BTreeSet::new();
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
        maximum_schema_bytes =
            maximum_schema_bytes.max(u64::try_from(bytes.len()).map_err(|_| {
                failure(
                    "swallowtail.codex.preparation.tool_schema_limit",
                    "Codex prepared session tool schema size cannot be represented",
                )
            })?);
        constraints.insert(
            CapabilityConstraint::tool_schema_dialect(tool.schema_dialect())
                .expect("tool dialect is non-empty"),
        );
    }
    constraints.insert(CapabilityConstraint::ToolMaximumCount(tool_count));
    constraints.insert(CapabilityConstraint::ToolMaximumSchemaBytes(
        maximum_schema_bytes,
    ));
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

pub(super) fn supports_harness_mode(prepared: &CodexPreparedIntegration) -> bool {
    prepared.observation().is_permitted()
        && semver::Version::parse(prepared.observation().version().version().as_str())
            .is_ok_and(|version| version >= PLAN_MODE_MINIMUM_VERSION)
}
