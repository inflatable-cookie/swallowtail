use crate::{fixture, server, services};

use fixture::Fixture;
use server::ServerMode;
use services::TimeMode;
use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroU64;
use swallowtail_adapter_openai::{
    OPENAI_BACKGROUND_MODEL_ID, OPENAI_BACKGROUND_MODEL_ROUTE_ID, OpenAiBackgroundModelSelection,
    OpenAiBackgroundRunProfileInput, OpenAiBackgroundServiceTier, OpenAiPreparedBackgroundRun,
    prepare_openai_background,
};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision, OperationShape, ReasoningMode};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, Deadline, MonotonicInstant, OperationContent, RequestId,
    SchemaDocument, StructuredOutputDescriptor,
};

use super::ledger::*;
use super::naming::*;

/// Prepares one background run carrying only the required inputs.
pub(super) fn minimal() -> OpenAiPreparedBackgroundRun {
    run(profile_input("projection-background-minimal"))
}

/// Prepares one background run with reasoning, schema, and service tier.
pub(super) fn tiered() -> OpenAiPreparedBackgroundRun {
    run(profile_input("projection-background-tiered")
        .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning mode is valid"))
        .with_structured_output(schema())
        .with_service_tier(OpenAiBackgroundServiceTier::standard()))
}

/// Prepares one background run that requests active-run detachment.
pub(super) fn detached() -> OpenAiPreparedBackgroundRun {
    run(profile_input("projection-background-detached").with_active_run_detachment())
}

fn run(input: OpenAiBackgroundRunProfileInput) -> OpenAiPreparedBackgroundRun {
    let fixture = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares")
        .prepare_background_run(input)
        .expect("background run prepares")
}

/// Prepares one background run bound to a different exact route revision.
pub(super) fn alternate_revision() -> OpenAiPreparedBackgroundRun {
    run(profile_with(
        "projection-background-alternate",
        "projection-2",
    ))
}

pub(super) fn profile_input(id: &str) -> OpenAiBackgroundRunProfileInput {
    profile_with(id, "projection-1")
}

fn profile_with(id: &str, revision: &str) -> OpenAiBackgroundRunProfileInput {
    OpenAiBackgroundRunProfileInput::background_with_temporary_retention_and_one_reattachment(
        RequestId::new(id).expect("request id is valid"),
        OpenAiBackgroundModelSelection::new(
            ModelRouteId::new(OPENAI_BACKGROUND_MODEL_ROUTE_ID).expect("route id is valid"),
            ModelRouteRevision::new(revision).expect("route revision is valid"),
            ModelId::new(OPENAI_BACKGROUND_MODEL_ID).expect("model id is valid"),
        ),
        OperationContent::new("Say hello").expect("content is valid"),
        NonZeroU64::new(64).expect("limit is non-zero"),
        Deadline::at(MonotonicInstant::from_ticks(100_000)),
    )
}

fn schema() -> StructuredOutputDescriptor {
    StructuredOutputDescriptor::new(
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1024).expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("structured output descriptor is valid")
}

pub(super) fn contribution(
    run: &OpenAiPreparedBackgroundRun,
    source_id: &str,
) -> ConsumerRouteProjectionContribution {
    run.consumer_route_projection_contribution(source(source_id))
        .expect("prepared background run contributes")
}

/// Collects the exact census identities each background profile emits.
pub(super) fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([
        (
            MINIMAL,
            identities(&contribution(&minimal(), "openai.background.minimal")),
        ),
        (
            TIERED,
            identities(&contribution(&tiered(), "openai.background.tiered")),
        ),
        (
            DETACHED,
            identities(&contribution(&detached(), "openai.background.detached")),
        ),
    ])
}

/// Returns the exact prepared operation shape each profile binds its rows to.
pub(super) fn prepared_operation_shapes() -> BTreeMap<&'static str, OperationShape> {
    BTreeMap::from([
        (
            MINIMAL,
            operation_shape_of(&contribution(&minimal(), "openai.background.shape-minimal")),
        ),
        (
            TIERED,
            operation_shape_of(&contribution(&tiered(), "openai.background.shape-tiered")),
        ),
        (
            DETACHED,
            operation_shape_of(&contribution(
                &detached(),
                "openai.background.shape-detached",
            )),
        ),
    ])
}

/// Returns the operation shape every row of one contribution is bound to.
fn operation_shape_of(contribution: &ConsumerRouteProjectionContribution) -> OperationShape {
    for row in contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
    {
        assert_eq!(
            row.applicability(),
            contribution.applicability(),
            "{:?} is not bound to the contribution's exact applicability",
            row.identity()
        );
    }
    contribution.applicability().operation_shape()
}
