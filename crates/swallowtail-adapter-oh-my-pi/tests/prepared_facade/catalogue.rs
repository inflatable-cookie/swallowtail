#[test]
fn prepared_catalogues_ask_pi_for_configured_models_without_selecting_one() {
    for host_value in ["fixture.pi.catalogue.local", "fixture.pi.catalogue.remote"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let discovery = FixtureHost::version_probe("17.2.9");
        let prepared = block_on(prepare_oh_my_pi_rpc(
            preparation_input(host_id.clone()),
            probe(),
            discovery.services(host_id.clone()),
        ))
        .expect("OhMyPi prepares");
        let profile = prepared
            .prepare_catalogue(OhMyPiCatalogueProfileInput::new(
                RequestId::new("pi-prepared-catalogue").expect("valid request"),
            ))
            .expect("OhMyPi catalogue profile prepares");

        assert!(profile.plan().provider_id().is_none());
        assert!(profile.plan().model_id().is_none());
        assert!(profile.plan().model_route_id().is_none());
        assert!(
            profile
                .plan()
                .requirements()
                .capabilities()
                .any(|requirement| requirement.capability() == Capability::ModelCatalog)
        );
        assert!(
            !profile
                .plan()
                .requirements()
                .host_services()
                .any(|service| service == swallowtail_core::HostServiceKind::WorkingResource)
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile
                .evidence()
                .operation()
                .observable_activity()
                .availability(),
            ObservableActivityAvailability::NotApplicable
        );

        let operation = FixtureHost::new(Scenario::Complete);
        let models =
            block_on(profile.list_models(operation.services(host_id))).expect("OhMyPi catalogue loads");
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].id().as_str(), "fixture-model");
        assert_eq!(
            models[0].provider_id().map(ProviderId::as_str),
            Some("fixture-provider")
        );
        assert_eq!(
            operation.process_arguments(),
            [
                "--mode",
                "rpc",
                "--no-session",
                "--no-tools",
                "--no-extensions",
                "--no-skills",
                "--no-rules",
                "--no-prewalk",
            ]
        );
        assert_eq!(
            operation.cleanup_events(),
            [
                support::CleanupEvent::ProcessWait,
            ]
        );
        assert!(!format!("{models:?}").contains("fixture-private.invalid"));
    }
}

#[test]
fn prepared_catalogues_bound_failures_deadlines_and_cleanup() {
    for (index, scenario, code) in [
        (
            1,
            Scenario::Malformed,
            "swallowtail.oh_my_pi.rpc.catalogue_invalid",
        ),
        (
            2,
            Scenario::ProviderFailure,
            "swallowtail.oh_my_pi.rpc.catalogue_rejected",
        ),
        (
            3,
            Scenario::ResponseMismatch,
            "swallowtail.oh_my_pi.rpc.response_command_mismatch",
        ),
        (
            4,
            Scenario::Disconnect,
            "swallowtail.oh_my_pi.rpc.connection_ended",
        ),
    ] {
        let host_id = ExecutionHostId::new(format!("fixture.pi.catalogue.failure.{index}"))
            .expect("valid host");
        let profile = prepared_catalogue(host_id.clone(), None);
        let operation = FixtureHost::new(scenario);
        let error = block_on(profile.list_models(operation.services(host_id)))
            .expect_err("OhMyPi catalogue failure remains typed");
        assert_eq!(error.diagnostic().code(), code);
        assert_eq!(
            operation.cleanup_events(),
            [
                support::CleanupEvent::ProcessWait,
            ]
        );
        assert!(!format!("{error:?}").contains("fixture private"));
    }

    let host_id = ExecutionHostId::new("fixture.pi.catalogue.timeout").expect("valid host");
    let profile = prepared_catalogue(
        host_id.clone(),
        Some(Deadline::at(MonotonicInstant::from_ticks(1_001))),
    );
    let operation = FixtureHost::new(Scenario::Hold).with_immediate_time();
    let error = block_on(profile.list_models(operation.services(host_id)))
        .expect_err("OhMyPi catalogue deadline fires");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.oh_my_pi.rpc.catalogue_timed_out"
    );
    assert_eq!(
        operation.cleanup_events(),
        [
            support::CleanupEvent::ProcessWait,
        ]
    );
}

#[test]
fn catalogue_success_does_not_hide_process_cleanup_failure() {
    let host_id = ExecutionHostId::new("fixture.pi.catalogue.cleanup").expect("valid host");
    let profile = prepared_catalogue(host_id.clone(), None);
    let operation = FixtureHost::new(Scenario::Complete).with_process_wait_failure();
    let error = block_on(profile.list_models(operation.services(host_id)))
        .expect_err("OhMyPi catalogue cleanup failure wins");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.oh_my_pi.rpc.catalogue_cleanup_failed"
    );
    assert_eq!(
        operation.cleanup_events(),
        [
            support::CleanupEvent::ProcessWait,
        ]
    );
}
