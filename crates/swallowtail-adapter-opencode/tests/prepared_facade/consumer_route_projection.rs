use super::fixture::PreparedFixture;
use swallowtail_adapter_opencode::{OpenCodeRunProfileInput, OpenCodeSessionProfileInput};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
    OperationContent, RequestId,
};

fn rows(contribution: &ConsumerRouteProjectionContribution) -> Vec<&ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .collect()
}

fn semantic_id(row: &ConsumerRouteProjectionRow) -> &str {
    match row.identity() {
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Namespaced(extension)) => {
            extension.semantic_id()
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Attachments) => {
            "feature.attachments"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::QuestionExchange) => {
            "feature.question-exchange"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::StructuredRun) => {
            "feature.structured-run"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation) => {
            "feature.activity-observation"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade) => {
            "feature.prepared-facade"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::Namespaced(extension)) => {
            extension.semantic_id()
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::LoadSession) => {
            "control.load-session"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ResumeSession) => {
            "control.resume-session"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection) => {
            "control.model-selection"
        }
        _ => "other",
    }
}

fn find<'a>(
    contribution: &'a ConsumerRouteProjectionContribution,
    semantic: &str,
) -> &'a ConsumerRouteProjectionRow {
    rows(contribution)
        .into_iter()
        .find(|row| semantic_id(row) == semantic)
        .unwrap_or_else(|| panic!("missing {semantic}"))
}

#[test]
fn retained_opencode_plan_evidence_publishes_callbacks_and_attachments_per_turn() {
    let fixture = PreparedFixture::new("opencode.projection.callbacks", "1.18.10");
    let prepared = fixture.prepared();
    let run = prepared
        .prepare_run(
            OpenCodeRunProfileInput::new(
                RequestId::new("projection-run").unwrap(),
                fixture.model(),
                OperationContent::new("projection prompt").unwrap(),
                fixture.resource.clone(),
            )
            .with_attachments([fixture.attachment()])
            .with_provider_callbacks(),
        )
        .expect("callback-bearing run prepares");
    let run_contribution = run
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("opencode-projection-run").unwrap(),
        )
        .expect("run contribution admits");
    let permission = find(&run_contribution, "feature.permission-exchange");
    assert_eq!(
        permission
            .identity()
            .namespaced_extension()
            .expect("permission is namespaced")
            .version_segment(),
        "opencode/permission"
    );
    let attachment = find(&run_contribution, "control.attachments");
    assert_eq!(
        attachment.lifecycle(),
        swallowtail_runtime::ConsumerRouteLifecycle::SessionStartOnly
    );
    assert!(attachment.mutation_authority().is_prepared_session_start());
    assert!(attachment.state_support().prepared());
    let callback = find(&run_contribution, "control.provider-callbacks");
    assert_eq!(
        callback.lifecycle(),
        swallowtail_runtime::ConsumerRouteLifecycle::PerTurn
    );
    assert!(
        callback
            .mutation_authority()
            .is_consumer_mediated_per_turn()
    );
    assert!(!callback.state_support().prepared());
    assert!(!callback.state_support().provider_effective());
    assert!(!callback.state_support().rejected());

    let session = prepared
        .prepare_session(
            OpenCodeSessionProfileInput::new(
                RequestId::new("projection-session").unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            )
            .with_image_attachments()
            .with_provider_callbacks(),
        )
        .expect("callback-bearing session prepares");
    let session_contribution = session
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("opencode-projection-session").unwrap(),
        )
        .expect("session contribution admits");
    assert!(
        find(&session_contribution, "control.attachments")
            .mutation_authority()
            .is_consumer_mediated_per_turn()
    );
    assert!(
        find(&session_contribution, "control.provider-callbacks")
            .mutation_authority()
            .is_consumer_mediated_per_turn()
    );
}

#[test]
fn opencode_negative_profiles_withhold_matrix_rows_and_unmediated_turns() {
    let fixture = PreparedFixture::new("opencode.projection.negative", "1.18.10");
    let prepared = fixture.prepared();
    let ordinary = prepared
        .prepare_session(OpenCodeSessionProfileInput::new(
            RequestId::new("projection-ordinary").unwrap(),
            fixture.model(),
            fixture.resource.clone(),
        ))
        .expect("ordinary session prepares");
    let contribution = ordinary
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("opencode-projection-negative").unwrap(),
        )
        .expect("ordinary contribution admits");
    assert!(rows(&contribution).into_iter().all(|row| !matches!(
        semantic_id(row),
        "control.provider-turn-reference" | "control.reasoning-selection"
    )));
    assert!(
        rows(&contribution)
            .into_iter()
            .all(|row| !row.mutation_authority().is_consumer_mediated_per_turn())
    );

    let failure = prepared
        .prepare_session(
            OpenCodeSessionProfileInput::new(
                RequestId::new("projection-invalid-combination").unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            )
            .with_provider_callbacks()
            .with_active_turn_detachment(),
        )
        .expect_err("detachment and callbacks are mutually exclusive");
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.opencode.preparation.detachment_callbacks_unsupported"
    );
}

#[test]
fn opencode_mixed_applicability_is_rejected_before_composition() {
    let first = PreparedFixture::new("opencode.projection.first", "1.18.10");
    let second = PreparedFixture::new("opencode.projection.second", "1.18.10");
    let first_run = first
        .prepared()
        .prepare_run(OpenCodeRunProfileInput::new(
            RequestId::new("projection-first").unwrap(),
            first.model(),
            OperationContent::new("first").unwrap(),
            first.resource.clone(),
        ))
        .expect("first run prepares");
    let second_run = second
        .prepared()
        .prepare_run(OpenCodeRunProfileInput::new(
            RequestId::new("projection-second").unwrap(),
            second.model(),
            OperationContent::new("second").unwrap(),
            second.resource.clone(),
        ))
        .expect("second run prepares");
    let first_contribution = first_run
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("opencode-projection-mixed").unwrap(),
        )
        .expect("first contribution admits");
    let second_contribution = second_run
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("opencode-projection-mixed").unwrap(),
        )
        .expect("second contribution admits");
    let row = rows(&second_contribution)
        .into_iter()
        .find(|row| semantic_id(row) == "feature.prepared-facade")
        .expect("second row exists");
    let rebound = ConsumerRouteProjectionRow::new(
        row.identity().clone(),
        second_contribution.applicability().clone(),
        first_contribution.sources().next().unwrap().clone(),
        row.source_class(),
        row.evidence_strength(),
        row.lifecycle(),
    );
    let failure = ConsumerRouteProjectionContribution::new(
        first_contribution.applicability().clone(),
        first_contribution.sources().cloned().collect::<Vec<_>>(),
        [rebound],
        [],
        [],
    )
    .expect_err("mixed applicability is rejected");
    assert_eq!(
        failure.kind(),
        swallowtail_runtime::ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}
