use super::super::*;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_codex::CodexPreparedExec;
use swallowtail_core::OperationShape;
use swallowtail_runtime::ConsumerRouteProjectionContribution;

use super::ledger::*;
use super::naming::*;

/// Prepares one exec run carrying every optional exec input the route admits.
pub(super) fn maximal() -> CodexPreparedExec {
    let recording = RecordingHostServices::default();
    let exec = prepared(
        CodexPreparedDriver::StructuredExec,
        FIXTURE_VERSION,
        &recording,
        true,
    );
    let attachment = AttachmentDescriptor::new(
        AttachmentRef::new("image").unwrap(),
        "image/png",
        AttachmentRole::Input,
    )
    .unwrap()
    .with_known_length(512);
    let output = StructuredOutputDescriptor::new(
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1024).unwrap(),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .unwrap();
    exec.prepare_structured_exec(
        CodexExecProfileInput::new(
            RequestId::new("projection-exec-maximal").unwrap(),
            OperationContent::new("private prompt").unwrap(),
            model(),
            working_resource(),
            ExternalNetworkPolicy::HostApproved,
            ExternalSearchPolicy::Enabled,
        )
        .with_reasoning_mode(ReasoningMode::new("low").unwrap())
        .with_model_verbosity(CodexModelVerbosity::High)
        .with_attachments([attachment])
        .with_structured_output(output),
    )
    .expect("maximal exec run prepares")
}

/// Prepares one exec run carrying only the inputs exec preparation requires.
pub(super) fn minimal() -> CodexPreparedExec {
    exec_run("projection-exec-minimal", model())
}

/// Prepares one exec run bound to a different exact model route.
pub(super) fn foreign_model() -> CodexPreparedExec {
    exec_run(
        "projection-exec-foreign",
        CodexModelSelection::new(
            ModelRouteId::new("codex-model").unwrap(),
            ModelRouteRevision::new("1").unwrap(),
            ModelId::new("gpt-5.4").unwrap(),
        ),
    )
}

fn exec_run(request_id: &str, selection: CodexModelSelection) -> CodexPreparedExec {
    let recording = RecordingHostServices::default();
    let exec = prepared(
        CodexPreparedDriver::StructuredExec,
        FIXTURE_VERSION,
        &recording,
        false,
    );
    exec.prepare_structured_exec(CodexExecProfileInput::new(
        RequestId::new(request_id).unwrap(),
        OperationContent::new("private prompt").unwrap(),
        selection,
        working_resource(),
        ExternalNetworkPolicy::Denied,
        ExternalSearchPolicy::Disabled,
    ))
    .expect("exec run prepares")
}

pub(super) fn contribution(
    run: &CodexPreparedExec,
    source_id: &str,
) -> ConsumerRouteProjectionContribution {
    run.consumer_route_projection_contribution(source(source_id))
        .expect("prepared exec contributes")
}

/// Collects the exact census identities each prepared exec profile emits.
pub(super) fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([
        (
            MAXIMAL,
            identities(&contribution(&maximal(), "codex.exec.maximal")),
        ),
        (
            MINIMAL,
            identities(&contribution(&minimal(), "codex.exec.minimal")),
        ),
    ])
}

/// Returns the exact prepared operation shape each profile binds its rows to.
pub(super) fn prepared_operation_shapes() -> BTreeMap<&'static str, OperationShape> {
    BTreeMap::from([
        (
            MAXIMAL,
            operation_shape_of(&contribution(&maximal(), "codex.exec.shape-maximal")),
        ),
        (
            MINIMAL,
            operation_shape_of(&contribution(&minimal(), "codex.exec.shape-minimal")),
        ),
    ])
}

/// Returns the operation shape every row of one contribution is bound to.
fn operation_shape_of(contribution: &ConsumerRouteProjectionContribution) -> OperationShape {
    let shape = contribution.applicability().operation_shape();
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
    shape
}
