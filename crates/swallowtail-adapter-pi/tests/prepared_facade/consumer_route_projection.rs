use super::support::{FixtureHost, reasoning_options};
use swallowtail_adapter_pi::{
    PiRunProfileInput, PiSdkSidecarPreparedSession, PiSdkSidecarSessionPreparation,
    PiSessionProfileInput, prepare_pi_rpc, prepare_pi_sdk_sidecar_session,
};
use swallowtail_core::{
    AccessProfileId, ConfiguredInstanceId, CredentialRef, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId,
};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
    Deadline, EnvironmentRef, MonotonicInstant, OperationContent, RequestId, SessionOptions,
    WorkingResourceRef,
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
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Attachments) => {
            "feature.attachments"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::QuestionExchange) => {
            "feature.question-exchange"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ModelCatalogue) => {
            "feature.model-catalogue"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation) => {
            "feature.activity-observation"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::UsageEvidence) => {
            "feature.usage-evidence"
        }
        ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade) => {
            "feature.prepared-facade"
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::Namespaced(extension)) => {
            extension.semantic_id()
        }
        ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ReasoningSelection) => {
            "control.reasoning-selection"
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

fn pi_prepared() -> swallowtail_adapter_pi::PiPreparedIntegration {
    let host_id = super::ExecutionHostId::new("pi.projection.host").unwrap();
    let discovery = FixtureHost::version_probe("0.84.4");
    futures_executor::block_on(prepare_pi_rpc(
        super::preparation_input(host_id.clone()),
        super::probe(),
        discovery.services(host_id),
    ))
    .expect("Pi RPC prepares")
}

fn sidecar_prepared(
    image_attachments: bool,
    options: SessionOptions,
) -> PiSdkSidecarPreparedSession {
    let input = PiSdkSidecarSessionPreparation::new(
        ConfiguredInstanceId::new("pi.projection.sidecar").unwrap(),
        InstanceRevision::new("1").unwrap(),
        super::ExecutionHostId::new("pi.projection.sidecar.host").unwrap(),
        InstanceTargetRef::new("pi.projection.launch").unwrap(),
        EnvironmentRef::new("pi.projection.environment").unwrap(),
        CredentialRef::new("pi.projection.credential").unwrap(),
        AccessProfileId::new("pi.projection.access").unwrap(),
        ModelRouteId::new("pi.projection.route").unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ProviderId::new("anthropic").unwrap(),
        ModelId::new("claude-opus-4-5").unwrap(),
        WorkingResourceRef::new("pi.projection.workspace").unwrap(),
        RequestId::new("pi-projection-sidecar").unwrap(),
    );
    let input = if image_attachments {
        input.with_image_attachments()
    } else {
        input
    };
    prepare_pi_sdk_sidecar_session(input, options).expect("sidecar prepares")
}

#[test]
fn pi_rpc_attachments_are_conditional_and_per_turn_authority_is_exact() {
    let prepared = pi_prepared();
    let run = prepared
        .prepare_run(PiRunProfileInput::new(
            RequestId::new("pi-projection-run").unwrap(),
            super::model("pi.projection.run.route"),
            OperationContent::new("projection run").unwrap(),
            WorkingResourceRef::new("pi.projection.workspace").unwrap(),
            Deadline::at(MonotonicInstant::from_ticks(100_000)),
        ))
        .expect("Pi run prepares");
    let run_contribution = run
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("pi-projection-run").unwrap(),
        )
        .expect("Pi run contribution admits");
    assert!(
        rows(&run_contribution)
            .into_iter()
            .all(|row| semantic_id(row) != "feature.attachments"
                && semantic_id(row) != "control.attachments")
    );

    let session = prepared
        .prepare_session(
            PiSessionProfileInput::new(
                RequestId::new("pi-projection-session").unwrap(),
                super::model("pi.projection.session.route"),
                WorkingResourceRef::new("pi.projection.workspace").unwrap(),
                SessionOptions::default(),
            )
            .with_image_attachments(),
        )
        .expect("Pi image session prepares");
    let contribution = session
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("pi-projection-session").unwrap(),
        )
        .expect("Pi session contribution admits");
    let attachment = find(&contribution, "control.attachments");
    assert!(
        attachment
            .mutation_authority()
            .is_consumer_mediated_per_turn()
    );
    assert_eq!(
        attachment.lifecycle(),
        swallowtail_runtime::ConsumerRouteLifecycle::PerTurn
    );
    assert!(!attachment.state_support().prepared());
    assert!(!attachment.state_support().provider_effective());
    assert!(
        rows(&contribution)
            .into_iter()
            .any(|row| semantic_id(row) == "feature.question-exchange")
    );
}

#[test]
fn pi_sdk_sidecar_projection_keeps_reasoning_and_attachment_evidence_route_local() {
    let sidecar = sidecar_prepared(true, reasoning_options("medium"));
    let contribution = sidecar
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("pi-sidecar-projection").unwrap(),
        )
        .expect("sidecar contribution admits");
    for semantic in [
        "feature.model-catalogue",
        "feature.activity-observation",
        "feature.usage-evidence",
        "feature.attachments",
        "control.reasoning-selection",
        "control.session-options",
    ] {
        find(&contribution, semantic);
    }
    assert_eq!(rows(&contribution).len(), 19);
    let attachment = find(&contribution, "control.attachments");
    assert!(
        attachment
            .mutation_authority()
            .is_consumer_mediated_per_turn()
    );
    assert_eq!(
        attachment.lifecycle(),
        swallowtail_runtime::ConsumerRouteLifecycle::PerTurn
    );
    assert!(!attachment.state_support().prepared());
    assert!(!attachment.state_support().provider_effective());

    let plain = sidecar_prepared(false, SessionOptions::default());
    let plain_contribution = plain
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("pi-sidecar-plain").unwrap(),
        )
        .expect("plain sidecar contribution admits");
    assert!(
        rows(&plain_contribution)
            .into_iter()
            .all(|row| semantic_id(row) != "feature.attachments"
                && semantic_id(row) != "control.attachments")
    );
}
