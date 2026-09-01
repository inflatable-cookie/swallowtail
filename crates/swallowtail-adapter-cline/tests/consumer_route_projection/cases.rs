#[test]
fn maximal_open_retains_exact_model_and_nine_exact_rows() {
    let (prepared, host, services) = session(Scenario::ModelExact, true, "maximal");
    let outcome = block_on(prepared.open_session_with_projection(
        source("cline.prepared.maximal"),
        source("cline.active.maximal"),
        services,
    ))
    .unwrap_or_else(|failure| panic!("projected open failed: {}", failure.failure()));
    let options = outcome.negotiated_model_options().expect("exact model options survive");
    assert_eq!(options.current_value(), "fixture-model");
    assert_eq!(
        options.options().map(|option| (option.value(), option.display_name())).collect::<Vec<_>>(),
        [("fixture-model", Some("Fixture Model")), ("other-model", None)]
    );
    assert_eq!(outcome.session().negotiated_model_options(), Some(options));
    let rows = semantic_ids(outcome.contribution());
    assert_eq!(rows.len(), 9);
    assert!(rows.contains("feature.active-session-plan-ack"));
    assert!(rows.contains("feature.negotiated-model-options-observation"));
    assert_eq!(outcome.contribution().sources().len(), 2);
    assert!(outcome.contribution().sources().any(|source| source.kind() == ConsumerRouteProjectionSourceKind::ActiveSessionObservation && source.id().as_str() == "cline.active.maximal"));
    let acknowledgement = projection_rows(outcome.contribution())
        .find(|row| row.identity().namespaced_extension().is_some_and(|extension| extension.semantic_id() == "feature.active-session-plan-ack"))
        .expect("Plan acknowledgement row");
    assert_eq!(acknowledgement.source().kind(), ConsumerRouteProjectionSourceKind::ActiveSessionObservation);
    assert_eq!(acknowledgement.evidence_strength(), swallowtail_runtime::ConsumerRouteEvidenceStrength::WireAcknowledgement);
    assert_eq!(acknowledgement.lifecycle(), swallowtail_runtime::ConsumerRouteLifecycle::PostOpenObservationOnly);
    assert!(acknowledgement.state_support().requested());
    assert!(acknowledgement.state_support().provider_effective());
    assert!(!acknowledgement.state_support().rejected());
    assert!(acknowledgement.mutation_authority().is_acknowledged());
    let value = acknowledgement.control_value().expect("acknowledgement value");
    assert_eq!(value.kind(), swallowtail_runtime::ConsumerRouteValueKind::AcknowledgementState);
    assert_eq!(value.omission(), swallowtail_runtime::ConsumerRouteOmissionSemantics::NotSelectable);
    let model = projection_rows(outcome.contribution())
        .find(|row| row.identity().namespaced_extension().is_some_and(|extension| extension.semantic_id() == "feature.negotiated-model-options-observation"))
        .expect("model observation row");
    assert!(model.state_support().observed());
    assert!(!model.state_support().provider_effective());
    assert!(!model.state_support().requested());
    assert!(model.mutation_authority().source().is_none());
    let value = model.control_value().expect("observation descriptor");
    assert_eq!(value.kind(), swallowtail_runtime::ConsumerRouteValueKind::Observation);
    assert_eq!(value.omission(), swallowtail_runtime::ConsumerRouteOmissionSemantics::NotSelectable);
    assert_eq!(block_on(outcome.into_parts().0.close()), CleanupOutcome::Clean);
    assert_eq!(host.releases(), 1);
}

fn projection_rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &swallowtail_runtime::ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}

#[test]
fn omission_has_no_active_rows_or_unused_active_source() {
    let (prepared, host, services) = session(Scenario::Success, false, "omitted");
    let outcome = block_on(prepared.open_session_with_projection(source("cline.prepared.omitted"), source("cline.active.omitted"), services)).unwrap_or_else(|failure| panic!("projected open failed: {}", failure.failure()));
    let rows = semantic_ids(outcome.contribution());
    assert_eq!(rows.len(), 6);
    assert!(!rows.contains("control.harness-mode"));
    assert!(!rows.contains("feature.active-session-plan-ack"));
    assert!(!rows.contains("feature.negotiated-model-options-observation"));
    assert_eq!(outcome.contribution().sources().len(), 1);
    assert!(outcome.negotiated_model_options().is_none());
    assert_eq!(block_on(outcome.into_parts().0.close()), CleanupOutcome::Clean);
    assert_eq!(host.releases(), 1);
}

#[test]
fn exact_act_is_rejected_without_session_or_model_row() {
    let (prepared, host, services) = session(Scenario::ModelExactPlanDrift, true, "rejected");
    let failure = block_on(prepared.open_session_with_projection(source("cline.prepared.rejected"), source("cline.active.rejected"), services)).err().expect("Act rejects Plan");
    assert_eq!(failure.failure().diagnostic().code(), "swallowtail.cline.acp.harness_mode_mismatch");
    let ClineProjectionOpenFailure::Rejected { contribution, .. } = failure else { panic!("exact admitted Act must be typed Rejected"); };
    let rows = semantic_ids(&contribution);
    assert!(rows.contains("feature.active-session-plan-ack"));
    assert!(!rows.contains("feature.negotiated-model-options-observation"));
    let acknowledgement = projection_rows(&contribution)
        .find(|row| {
            row.identity().namespaced_extension().is_some_and(|extension| {
                extension.semantic_id() == "feature.active-session-plan-ack"
            })
        })
        .expect("rejected Plan acknowledgement row");
    assert_eq!(
        acknowledgement.source().kind(),
        ConsumerRouteProjectionSourceKind::ActiveSessionObservation
    );
    assert!(acknowledgement.state_support().rejected());
    assert!(!acknowledgement.state_support().provider_effective());
    let value = acknowledgement.control_value().expect("rejected value");
    assert_eq!(
        value.kind(),
        swallowtail_runtime::ConsumerRouteValueKind::AcknowledgementState
    );
    assert_eq!(
        value.omission(),
        swallowtail_runtime::ConsumerRouteOmissionSemantics::NotSelectable
    );
    assert!(acknowledgement.mutation_authority().is_acknowledged());
    let swallowtail_runtime::ConsumerRouteValueDomain::Enumerated(values) = value.domain() else {
        panic!("rejected acknowledgement needs an exact domain");
    };
    assert_eq!(
        values.values().map(|value| value.as_str()).collect::<Vec<_>>(),
        ["act"]
    );
    assert_eq!(host.releases(), 1);
}

#[test]
fn malformed_plan_confirmation_is_runtime_with_legacy_parity() {
    for scenario in [Scenario::PlanConfirmationMissing, Scenario::PlanConfirmationMalformed, Scenario::PlanConfirmationAmbiguous, Scenario::PlanDisconnect] {
        let (projected, projected_host, services) = session(scenario, true, "projected-plan");
        let projected_failure = block_on(projected.open_session_with_projection(source("cline.prepared.plan-failure"), source("cline.active.plan-failure"), services)).err().expect("projected open fails");
        assert!(matches!(projected_failure, ClineProjectionOpenFailure::Runtime(_)));
        let (legacy, legacy_host, services) = session(scenario, true, "legacy-plan");
        let legacy_failure = block_on(legacy.open_session(services)).err().expect("legacy fails");
        assert_eq!(projected_failure.failure().diagnostic().code(), legacy_failure.diagnostic().code());
        assert_eq!(projected_host.releases(), 1);
        assert_eq!(legacy_host.releases(), 1);
    }
}

#[test]
fn invalid_model_is_ignored_by_legacy_and_closes_projected_open() {
    for scenario in [Scenario::ModelMalformed, Scenario::ModelDuplicate, Scenario::ModelUnadvertised, Scenario::ModelUnbounded] {
        let (legacy, legacy_host, services) = session(scenario, false, "legacy-model");
        let handle = block_on(legacy.open_session(services)).expect("legacy open is preserved");
        assert!(handle.negotiated_model_options().is_none());
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(legacy_host.releases(), 1);
        let (projected, projected_host, services) = session(scenario, false, "projected-model");
        let failure = block_on(projected.open_session_with_projection(source("cline.prepared.model-invalid"), source("cline.active.model-invalid"), services)).err().expect("projected open rejects invalid model evidence");
        assert!(matches!(failure, ClineProjectionOpenFailure::Runtime(_)));
        assert_eq!(failure.failure().diagnostic().code(), "swallowtail.negotiated_model_options.invalid");
        assert!(failure.rejected_contribution().is_none());
        assert_eq!(projected_host.releases(), 1);
    }
}

#[test]
fn equal_sources_fail_before_provider_or_resource_work() {
    let (prepared, host, services) = session(Scenario::Success, false, "equal-source");
    let failure = block_on(prepared.open_session_with_projection(source("cline.same-source"), source("cline.same-source"), services)).err().expect("equal sources fail");
    assert_eq!(failure.failure().diagnostic().code(), "swallowtail.cline.projection_source_identity_invalid");
    assert!(!host.process_started());
    assert_eq!(host.releases(), 0);
}
