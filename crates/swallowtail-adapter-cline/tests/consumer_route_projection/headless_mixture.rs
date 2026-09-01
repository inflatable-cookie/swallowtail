#[test]
fn headless_matching_source_cross_instance_and_stale_revision_fail_closed() {
    let shared = source("cline.headless.mixture");
    let ready = ready_status();
    let mine = headless_run_at(false, "cline.projection.headless", "1", ready.clone())
        .expect("baseline prepares")
        .consumer_route_projection_contribution(shared.clone())
        .expect("baseline contributes");
    let other = headless_run_at(false, "cline.projection.other", "1", ready.clone())
        .expect("other prepares")
        .consumer_route_projection_contribution(shared.clone())
        .expect("other contributes");
    assert_ne!(mine.applicability().instance_id(), other.applicability().instance_id());
    assert_eq!(mine.applicability().driver_identity(), other.applicability().driver_identity());
    assert_mixture_rejected(&mine, first_row(&other).clone());

    let stale = headless_run_at(false, "cline.projection.headless", "2", ready)
        .expect("stale prepares")
        .consumer_route_projection_contribution(shared)
        .expect("stale contributes");
    assert_ne!(
        mine.applicability().instance_revision(),
        stale.applicability().instance_revision()
    );
    assert_eq!(mine.applicability().driver_identity(), stale.applicability().driver_identity());
    assert_mixture_rejected(&mine, first_row(&stale).clone());
}

#[test]
fn headless_all_five_access_drifts_stop_before_any_row_can_form() {
    let ready = ready_status();
    let statuses = [
        drifted_status(&ready, Some(CredentialState::Ready), None, None, None, None),
        drifted_status(&ready, None, Some(EntitlementState::Exhausted), None, None, None),
        drifted_status(
            &ready,
            None,
            None,
            Some(EndpointAuthorization::Denied),
            None,
            None,
        ),
        drifted_status(
            &ready,
            None,
            None,
            None,
            Some(RuntimeReadiness::Degraded),
            None,
        ),
        drifted_status(
            &ready,
            None,
            None,
            None,
            None,
            Some(SupportAuthority::ExperimentalObserved),
        ),
    ];
    for status in statuses {
        let differences = [
            ready.credential() != status.credential(),
            ready.entitlement() != status.entitlement(),
            ready.endpoint_authorization() != status.endpoint_authorization(),
            ready.runtime_readiness() != status.runtime_readiness(),
            ready.support_authority() != status.support_authority(),
        ];
        assert_eq!(differences.into_iter().filter(|different| *different).count(), 1);
        assert!(
            headless_run_at(false, "cline.projection.headless", "1", status).is_err(),
            "drifted headless access must stop before a contribution exists"
        );
    }
}

fn drifted_status(
    base: &AccessStatus,
    credential: Option<CredentialState>,
    entitlement: Option<EntitlementState>,
    endpoint: Option<EndpointAuthorization>,
    readiness: Option<RuntimeReadiness>,
    support: Option<SupportAuthority>,
) -> AccessStatus {
    AccessStatus::new(
        base.profile_id().clone(),
        credential.unwrap_or(base.credential()),
        entitlement.unwrap_or(base.entitlement()),
        endpoint.unwrap_or(base.endpoint_authorization()),
        readiness.unwrap_or(base.runtime_readiness()),
        support.unwrap_or(base.support_authority()),
    )
}
