//! Contract 061 disposition proof for the exact Anthropic rows.

#![allow(dead_code, unused_imports)]

mod support;

#[path = "prepared_facade/fixtures.rs"]
mod direct_fixtures;

use direct_fixtures::PreparedFixture;
use std::collections::BTreeSet;
use std::num::{NonZeroU32, NonZeroU64};
use std::sync::Arc;
use support::{ManagedFixtureServer, ManagedStreamFixture, ThreadServices};
use swallowtail_adapter_anthropic::{
    AnthropicCatalogueProfileInput, AnthropicInferenceAttemptInput, AnthropicManagedAgentRunInput,
    AnthropicManagedModelSelection, AnthropicManagedPreparationInput, AnthropicModelSelection,
    AnthropicSessionProfileInput, AnthropicThinkingMode, AnthropicWebSearchInput,
    anthropic_managed_agent_descriptor, anthropic_managed_requirements,
    prepare_anthropic_managed_agent,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, ProtocolFacadeId,
    ProviderAgentBinding, ProviderAgentId, ProviderAgentVersion, ProviderId, RuntimeReadiness,
    SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRole, BlockingWorkService, CleanupOutcome,
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteNamespacedExtension,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionRow, ConsumerRouteRowIdentity, CredentialRef, CredentialService,
    Deadline, EndpointRef, HostServices, MonotonicInstant, NetworkPolicyService, OperationContent,
    OperationPolicy, PreparedAccessEvidence, ProviderRecoveryPolicy, ProviderRetentionPolicy,
    RequestId, SchemaDocument, ScopedTaskService, StreamReattachmentPolicy, StructuredRunRequest,
    TimeService, ToolDeclaration,
};
use swallowtail_testkit::ExecutionTopologyFixture;

include!("managed_driver/fixture.rs");
include!("managed_driver/prepared_fixture.rs");

const MESSAGES_ROUTE: &str = "anthropic.messages";
const MANAGED_ROUTE: &str = "anthropic.managed-agent";
const CATALOGUE: &str = "AnthropicPreparedCatalogue";
const INFERENCE: &str = "AnthropicPreparedInferenceAttempt[maximal]";
const SESSION: &str = "AnthropicPreparedSession";
const MANAGED: &str = "AnthropicPreparedManagedAgentRun[maximal]";
const PROFILES: [&str; 4] = [CATALOGUE, INFERENCE, SESSION, MANAGED];

struct LedgerEntry {
    route: &'static str,
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
}

const ANTHROPIC_TRANCHE: [LedgerEntry; 23] = [
    entry(
        MESSAGES_ROUTE,
        "model-catalogue",
        "feature.model-catalogue",
        &[CATALOGUE],
    ),
    entry(
        MESSAGES_ROUTE,
        "structured-run",
        "feature.structured-run",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "interactive-session",
        "feature.interactive-session",
        &[SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-observation",
        "feature.streaming-events",
        &[INFERENCE, SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-observation",
        "feature.usage-evidence",
        &[INFERENCE, SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-capability",
        "feature.output-token-limit",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-capability",
        "feature.reasoning-selection",
        &[INFERENCE, SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-capability",
        "feature.attachments",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-capability",
        "feature.consumer-tool-exchange",
        &[SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-capability",
        "feature.cancellation-or-interruption",
        &[SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-capability",
        "feature.external-search",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-capability",
        "feature.prepared-facade",
        &[CATALOGUE, INFERENCE, SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "route-observation",
        "feature.activity-observation",
        &[INFERENCE, SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "structured-run",
        "control.model-selection",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "interactive-session",
        "control.model-selection",
        &[SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "structured-run",
        "control.reasoning-selection",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "interactive-session",
        "control.reasoning-selection",
        &[SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "structured-run",
        "control.maximum-output-tokens",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "structured-run",
        "control.attachments",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "structured-run",
        "control.web-search-allowlist",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "interactive-session",
        "control.session-tools-and-reasoning",
        &[SESSION],
    ),
    entry(
        MESSAGES_ROUTE,
        "structured-run",
        "control.thinking-mode",
        &[INFERENCE],
    ),
    entry(
        MESSAGES_ROUTE,
        "interactive-session",
        "control.thinking-mode",
        &[SESSION],
    ),
];

// The managed-agent rows are kept in a second table so the two route-local
// dialects cannot be mistaken for one shared route.
const MANAGED_TRANCHE: [LedgerEntry; 17] = [
    entry(
        MANAGED_ROUTE,
        "structured-run",
        "feature.structured-run",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "route-observation",
        "feature.streaming-events",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "route-observation",
        "feature.usage-evidence",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "route-capability",
        "feature.consumer-tool-exchange",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "route-capability",
        "feature.cancellation-or-interruption",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "session-lifecycle",
        "feature.stream-reattachment",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "session-lifecycle",
        "feature.provider-managed-recovery",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "route-capability",
        "feature.owned-remote-resource-cleanup",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "session-lifecycle",
        "feature.persistent-session-posture",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "route-capability",
        "feature.prepared-facade",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "route-observation",
        "feature.activity-observation",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "structured-run",
        "control.model-selection",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "structured-run",
        "control.consumer-tool-exchange",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "structured-run",
        "control.provider-retention-policy",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "structured-run",
        "control.provider-recovery-policy",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "structured-run",
        "control.stream-reattachment",
        &[MANAGED],
    ),
    entry(
        MANAGED_ROUTE,
        "structured-run",
        "control.cross-process-recovery",
        &[MANAGED],
    ),
];

const fn entry(
    route: &'static str,
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
) -> LedgerEntry {
    LedgerEntry {
        route,
        operation_shape,
        semantic_id,
        emitted_by,
    }
}

#[test]
fn coverage_ledgers_preserve_the_exact_messages_and_managed_counts() {
    assert_eq!(ANTHROPIC_TRANCHE.len(), 23);
    assert_eq!(MANAGED_TRANCHE.len(), 17);
    assert_eq!(
        ANTHROPIC_TRANCHE
            .iter()
            .chain(MANAGED_TRANCHE.iter())
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        40
    );
    assert_eq!(
        ANTHROPIC_TRANCHE
            .iter()
            .filter(|entry| entry.route == MESSAGES_ROUTE)
            .count(),
        23
    );
    assert_eq!(
        ANTHROPIC_TRANCHE
            .iter()
            .chain(MANAGED_TRANCHE.iter())
            .map(|entry| (entry.route, entry.operation_shape, entry.semantic_id))
            .collect::<BTreeSet<_>>()
            .len(),
        40
    );
}

#[test]
fn every_prepared_anthropic_profile_matches_its_ledger_disposition() {
    let observed = [
        (CATALOGUE, catalogue()),
        (INFERENCE, inference()),
        (SESSION, session()),
        (MANAGED, managed()),
    ]
    .into_iter()
    .map(|(profile, contribution)| {
        let identities = all_rows(&contribution)
            .map(|row| row_identity(row, &contribution))
            .collect::<BTreeSet<_>>();
        (profile, identities)
    })
    .collect::<Vec<_>>();

    assert_eq!(observed.len(), PROFILES.len());
    for (profile, identities) in observed {
        let expected = ANTHROPIC_TRANCHE
            .iter()
            .chain(MANAGED_TRANCHE.iter())
            .filter(|entry| entry.emitted_by.contains(&profile))
            .map(|entry| (entry.route, entry.operation_shape, entry.semantic_id))
            .collect::<BTreeSet<_>>();
        assert_eq!(identities, expected, "{profile} disposition differs");
    }
}

#[test]
fn optional_controls_are_negative_without_request_evidence_and_mixed_routes_cannot_assemble() {
    let fixture =
        PreparedFixture::new(ExecutionHostId::new("anthropic.projection.negative").unwrap());
    let minimal = fixture
        .prepared()
        .prepare_inference_attempt(fixture.attempt_input("anthropic.projection.minimal"))
        .expect("minimal inference prepares")
        .consumer_route_projection_contribution(source("anthropic.projection.minimal"))
        .expect("minimal inference contributes");
    let minimal_semantics = all_rows(&minimal)
        .map(|row| semantic_id(row.identity()))
        .collect::<BTreeSet<_>>();
    for withheld in [
        "feature.attachments",
        "feature.external-search",
        "feature.reasoning-selection",
        "control.attachments",
        "control.web-search-allowlist",
        "control.reasoning-selection",
        "control.thinking-mode",
    ] {
        assert!(
            !minimal_semantics.contains(withheld),
            "{withheld} needs retained evidence"
        );
    }

    let direct = session();
    let managed = managed();
    let rejection = ConsumerRouteProjectionContribution::new(
        managed.applicability().clone(),
        managed.sources().cloned().collect::<Vec<_>>(),
        [all_rows(&direct)
            .next()
            .expect("direct session publishes")
            .clone()],
        [],
        [],
    )
    .expect_err("Messages evidence cannot assemble a Managed Agents snapshot");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

#[test]
fn managed_tool_exchange_is_the_only_per_turn_row_and_uses_consumer_authority() {
    let published = managed();
    let per_turn = published
        .session_start_rows()
        .find(|row| row.lifecycle() == swallowtail_runtime::ConsumerRouteLifecycle::PerTurn)
        .expect("managed tool exchange is per-turn");
    assert_eq!(
        semantic_id(per_turn.identity()),
        "control.consumer-tool-exchange"
    );
    assert!(
        per_turn
            .mutation_authority()
            .is_consumer_mediated_per_turn()
    );
    assert!(!per_turn.state_support().prepared());
    assert!(!per_turn.state_support().provider_effective());
    assert!(!per_turn.state_support().rejected());

    let fixture = Fixture::new();
    let no_tools = fixture
        .prepared_run_input("anthropic.projection.no-tools", [])
        .with_cross_process_recovery();
    let no_tools =
        prepare_anthropic_managed_agent(fixture.preparation_input(), &fixture.services())
            .expect("managed integration prepares")
            .prepare_managed_run(no_tools)
            .expect("managed no-tool run prepares")
            .consumer_route_projection_contribution(source("anthropic.projection.no-tools"))
            .expect("managed no-tool run contributes");
    assert!(
        !all_rows(&no_tools)
            .any(|row| { row.lifecycle() == swallowtail_runtime::ConsumerRouteLifecycle::PerTurn })
    );
}

fn row_identity(
    row: &ConsumerRouteProjectionRow,
    contribution: &ConsumerRouteProjectionContribution,
) -> (&'static str, &'static str, &'static str) {
    (
        row_route(contribution),
        census_shape(
            row.identity(),
            contribution.applicability().operation_shape(),
        ),
        semantic_id(row.identity()),
    )
}

fn all_rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}

fn row_route(contribution: &ConsumerRouteProjectionContribution) -> &'static str {
    if contribution.applicability().protocol_facade_id().as_str() == "managed-agents-2026-04-01" {
        MANAGED_ROUTE
    } else {
        MESSAGES_ROUTE
    }
}

fn semantic_id(identity: &ConsumerRouteRowIdentity) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        return match extension.semantic_id() {
            "feature.stream-reattachment" => "feature.stream-reattachment",
            "feature.provider-managed-recovery" => "feature.provider-managed-recovery",
            "feature.owned-remote-resource-cleanup" => "feature.owned-remote-resource-cleanup",
            "control.attachments" => "control.attachments",
            "control.web-search-allowlist" => "control.web-search-allowlist",
            "control.session-tools-and-reasoning" => "control.session-tools-and-reasoning",
            "control.thinking-mode" => "control.thinking-mode",
            "control.provider-retention-policy" => "control.provider-retention-policy",
            "control.provider-recovery-policy" => "control.provider-recovery-policy",
            "control.stream-reattachment" => "control.stream-reattachment",
            "control.cross-process-recovery" => "control.cross-process-recovery",
            other => panic!("unexpected Anthropic descriptor {other}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "feature.model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            ConsumerRouteFeatureId::UsageEvidence => "feature.usage-evidence",
            ConsumerRouteFeatureId::OutputTokenLimit => "feature.output-token-limit",
            ConsumerRouteFeatureId::ReasoningSelection => "feature.reasoning-selection",
            ConsumerRouteFeatureId::Attachments => "feature.attachments",
            ConsumerRouteFeatureId::ConsumerToolExchange => "feature.consumer-tool-exchange",
            ConsumerRouteFeatureId::CancellationOrInterruption => {
                "feature.cancellation-or-interruption"
            }
            ConsumerRouteFeatureId::ExternalSearch => "feature.external-search",
            ConsumerRouteFeatureId::PersistentSessionPosture => {
                "feature.persistent-session-posture"
            }
            ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected Anthropic feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection => "control.model-selection",
            ConsumerRouteControlId::ReasoningSelection => "control.reasoning-selection",
            ConsumerRouteControlId::MaximumOutputTokens => "control.maximum-output-tokens",
            ConsumerRouteControlId::UserInputExchange => "control.consumer-tool-exchange",
            other => panic!("unexpected Anthropic control {other:?}"),
        },
    }
}

fn census_shape(
    identity: &ConsumerRouteRowIdentity,
    operation_shape: swallowtail_core::OperationShape,
) -> &'static str {
    if let Some(extension) = identity.namespaced_extension() {
        return match extension.semantic_id() {
            "feature.stream-reattachment"
            | "feature.provider-managed-recovery"
            | "feature.persistent-session-posture" => "session-lifecycle",
            "feature.owned-remote-resource-cleanup" => "route-capability",
            "control.attachments" | "control.web-search-allowlist" => "structured-run",
            "control.session-tools-and-reasoning" => "interactive-session",
            "control.thinking-mode" => match operation_shape {
                swallowtail_core::OperationShape::StructuredRun => "structured-run",
                swallowtail_core::OperationShape::InteractiveSession => "interactive-session",
                other => panic!("unexpected Anthropic thinking shape {other:?}"),
            },
            "control.provider-retention-policy"
            | "control.provider-recovery-policy"
            | "control.stream-reattachment"
            | "control.cross-process-recovery" => "structured-run",
            other => panic!("unexpected Anthropic descriptor {other}"),
        };
    }
    match identity {
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            ConsumerRouteFeatureId::ModelCatalogue => "model-catalogue",
            ConsumerRouteFeatureId::StructuredRun => "structured-run",
            ConsumerRouteFeatureId::InteractiveSession => "interactive-session",
            ConsumerRouteFeatureId::StreamingEvents | ConsumerRouteFeatureId::UsageEvidence => {
                "route-observation"
            }
            ConsumerRouteFeatureId::OutputTokenLimit
            | ConsumerRouteFeatureId::ReasoningSelection
            | ConsumerRouteFeatureId::Attachments
            | ConsumerRouteFeatureId::ConsumerToolExchange
            | ConsumerRouteFeatureId::CancellationOrInterruption
            | ConsumerRouteFeatureId::ExternalSearch
            | ConsumerRouteFeatureId::PreparedFacade => "route-capability",
            ConsumerRouteFeatureId::PersistentSessionPosture => "session-lifecycle",
            ConsumerRouteFeatureId::ActivityObservation => "route-observation",
            other => panic!("unexpected Anthropic feature {other:?}"),
        },
        ConsumerRouteRowIdentity::Control(control) => match control {
            ConsumerRouteControlId::ModelSelection | ConsumerRouteControlId::ReasoningSelection => {
                match operation_shape {
                    swallowtail_core::OperationShape::StructuredRun => "structured-run",
                    swallowtail_core::OperationShape::InteractiveSession => "interactive-session",
                    other => panic!("unexpected Anthropic control shape {other:?}"),
                }
            }
            ConsumerRouteControlId::MaximumOutputTokens => "structured-run",
            ConsumerRouteControlId::UserInputExchange => "structured-run",
            other => panic!("unexpected Anthropic control {other:?}"),
        },
    }
}

fn source(id: &str) -> swallowtail_runtime::ConsumerRouteProjectionSourceId {
    swallowtail_runtime::ConsumerRouteProjectionSourceId::new(id).expect("source id")
}

fn catalogue() -> ConsumerRouteProjectionContribution {
    let fixture =
        PreparedFixture::new(ExecutionHostId::new("anthropic.projection.catalogue").unwrap());
    fixture
        .prepared()
        .prepare_catalogue(AnthropicCatalogueProfileInput::new(
            RequestId::new("anthropic.projection.catalogue").unwrap(),
        ))
        .expect("catalogue prepares")
        .consumer_route_projection_contribution(source("anthropic.projection.catalogue"))
        .expect("catalogue contributes")
}

fn inference() -> ConsumerRouteProjectionContribution {
    let fixture =
        PreparedFixture::new(ExecutionHostId::new("anthropic.projection.inference").unwrap());
    let image =
        AttachmentDescriptor::new(fixture.attachment_ref(), "image/png", AttachmentRole::Input)
            .expect("image descriptor")
            .with_known_length(8);
    fixture
        .prepared()
        .prepare_inference_attempt(
            fixture
                .attempt_input_for_model("anthropic.projection.inference", "claude-opus-4-7")
                .with_attachments([image])
                .with_web_search(AnthropicWebSearchInput::new(["example.com"]))
                .with_reasoning_mode(swallowtail_core::ReasoningMode::new("high").unwrap())
                .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect("maximal inference prepares")
        .consumer_route_projection_contribution(source("anthropic.projection.inference"))
        .expect("inference contributes")
}

fn session() -> ConsumerRouteProjectionContribution {
    let fixture =
        PreparedFixture::new(ExecutionHostId::new("anthropic.projection.session").unwrap());
    fixture
        .prepared()
        .prepare_session(
            AnthropicSessionProfileInput::new(
                RequestId::new("anthropic.projection.session").unwrap(),
                AnthropicModelSelection::new(
                    ModelRouteId::new("anthropic.projection.route").unwrap(),
                    ModelRouteRevision::new("1").unwrap(),
                    ModelId::new("claude-opus-4-7").unwrap(),
                ),
                [fixture_tool()],
            )
            .with_reasoning_mode(swallowtail_core::ReasoningMode::new("high").unwrap())
            .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect("session prepares")
        .consumer_route_projection_contribution(source("anthropic.projection.session"))
        .expect("session contributes")
}

fn managed() -> ConsumerRouteProjectionContribution {
    let fixture = Fixture::new();
    let prepared =
        prepare_anthropic_managed_agent(fixture.preparation_input(), &fixture.services())
            .expect("managed integration prepares")
            .prepare_managed_run(
                fixture
                    .prepared_run_input("anthropic.projection.managed", [fixture_tool()])
                    .with_cross_process_recovery(),
            )
            .expect("managed run prepares");
    prepared
        .consumer_route_projection_contribution(source("anthropic.projection.managed"))
        .expect("managed contributes")
}

fn fixture_tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_fixture",
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"key":{"type":"string"}},"required":["key"]}"#
                .to_vec(),
            1024,
        )
        .expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool is valid")
}
