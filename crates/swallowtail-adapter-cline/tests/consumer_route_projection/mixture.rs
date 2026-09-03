#[test]
fn matching_source_cross_operation_mixtures_fail_closed_both_directions() {
    let shared = source("cline.mixture.shared");
    let (acp, _, _) = session(Scenario::Success, true, "mixture-acp");
    let acp = acp
        .consumer_route_projection_contribution(shared.clone())
        .expect("ACP contributes");
    let headless = headless_run(true)
        .consumer_route_projection_contribution(shared)
        .expect("headless contributes");
    assert_mixture_rejected(&headless, first_row(&acp).clone());
    assert_mixture_rejected(&acp, first_row(&headless).clone());
}

#[test]
fn matching_source_active_row_cannot_attach_to_headless_applicability() {
    let shared = source("cline.mixture.active");
    let (acp, host, services) = session(Scenario::ModelExact, true, "mixture-active");
    let outcome = block_on(acp.open_session_with_projection(
        source("cline.mixture.active.prepared"),
        shared.clone(),
        host.cleanup_request(),
        services.clone(),
    ))
    .unwrap_or_else(|failure| panic!("ACP open failed: {}", failure.failure()));
    let headless = headless_run(true)
        .consumer_route_projection_contribution(shared)
        .expect("headless contributes");
    let active = projection_rows(outcome.contribution())
        .find(|row| {
            row.identity().namespaced_extension().is_some_and(|extension| {
                extension.semantic_id() == "feature.active-session-plan-ack"
            })
        })
        .expect("active Plan acknowledgement");
    assert_mixture_rejected(&headless, active.clone());
    assert_eq!(
        block_on(outcome.into_parts().0.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn matching_source_cross_instance_mixture_fails_closed() {
    let shared = source("cline.mixture.instance");
    let (left, _, _) = session(Scenario::Success, false, "mixture-left");
    let (right, _, _) = session(Scenario::Success, false, "mixture-right");
    let left = left
        .consumer_route_projection_contribution(shared.clone())
        .expect("left contributes");
    let right = right
        .consumer_route_projection_contribution(shared)
        .expect("right contributes");
    assert_ne!(left.applicability().instance_id(), right.applicability().instance_id());
    assert_mixture_rejected(&right, first_row(&left).clone());
}

#[test]
fn matching_source_stale_revision_mixture_fails_closed() {
    let shared = source("cline.mixture.revision");
    let (current, _, _) = session_at_revision(Scenario::Success, false, "mixture-revision", "1");
    let (stale, _, _) = session_at_revision(Scenario::Success, false, "mixture-revision", "2");
    let current = current
        .consumer_route_projection_contribution(shared.clone())
        .expect("current contributes");
    let stale = stale
        .consumer_route_projection_contribution(shared)
        .expect("stale contributes");
    assert_ne!(
        current.applicability().instance_revision(),
        stale.applicability().instance_revision()
    );
    assert_mixture_rejected(&current, first_row(&stale).clone());
}

#[test]
fn every_exact_access_drift_stops_before_a_row_can_form() {
    let baseline = AccessStatus::new(
        AccessProfileId::new("cline.projection.local-account").expect("profile"),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let statuses = [
        AccessStatus::new(
            baseline.profile_id().clone(),
            CredentialState::Ready,
            baseline.entitlement(),
            baseline.endpoint_authorization(),
            baseline.runtime_readiness(),
            baseline.support_authority(),
        ),
        AccessStatus::new(
            baseline.profile_id().clone(),
            baseline.credential(),
            EntitlementState::Exhausted,
            baseline.endpoint_authorization(),
            baseline.runtime_readiness(),
            baseline.support_authority(),
        ),
        AccessStatus::new(
            baseline.profile_id().clone(),
            baseline.credential(),
            baseline.entitlement(),
            EndpointAuthorization::Denied,
            baseline.runtime_readiness(),
            baseline.support_authority(),
        ),
        AccessStatus::new(
            baseline.profile_id().clone(),
            baseline.credential(),
            baseline.entitlement(),
            baseline.endpoint_authorization(),
            RuntimeReadiness::Degraded,
            baseline.support_authority(),
        ),
        AccessStatus::new(
            baseline.profile_id().clone(),
            baseline.credential(),
            baseline.entitlement(),
            baseline.endpoint_authorization(),
            baseline.runtime_readiness(),
            SupportAuthority::IntegrationMaintainerSupported,
        ),
    ];
    for (index, status) in statuses.into_iter().enumerate() {
        let differences = [
            status.credential() != baseline.credential(),
            status.entitlement() != baseline.entitlement(),
            status.endpoint_authorization() != baseline.endpoint_authorization(),
            status.runtime_readiness() != baseline.runtime_readiness(),
            status.support_authority() != baseline.support_authority(),
        ];
        assert_eq!(differences.into_iter().filter(|different| *different).count(), 1);
        assert!(
            session_preparation_with_status(status, &format!("access-{index}")).is_err(),
            "drifted access must stop before a prepared contribution exists"
        );
    }
}

fn assert_mixture_rejected(
    target: &ConsumerRouteProjectionContribution,
    row: swallowtail_runtime::ConsumerRouteProjectionRow,
) {
    let rejection = ConsumerRouteProjectionContribution::new(
        target.applicability().clone(),
        [row.source().clone()],
        [row],
        [],
        [],
    )
    .expect_err("mixed applicability fails closed");
    assert_eq!(
        rejection.kind(),
        swallowtail_runtime::ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

fn first_row(
    contribution: &ConsumerRouteProjectionContribution,
) -> &swallowtail_runtime::ConsumerRouteProjectionRow {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .next()
        .expect("contribution has a row")
}
