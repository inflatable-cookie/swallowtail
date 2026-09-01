use super::*;

mod lifecycle;

#[test]
fn prepared_driver_binds_exact_target_version_access_and_fixed_instance_facts() {
    for driver in [
        CodexPreparedDriver::StructuredExec,
        CodexPreparedDriver::AppServer,
    ] {
        let fixture = fixture(driver, "host.remote", "codex.remote");
        let (process, state) = FakeProcessService::completed("codex-cli 0.145.0\n");
        let prepared = block_on(prepare_codex(
            fixture.input,
            fixture.probe,
            services(fixture.host.clone(), process),
        ))
        .expect("qualified executable prepares");

        assert_eq!(prepared.driver(), driver);
        assert_eq!(
            prepared.observation().version().version().as_str(),
            "0.145.0"
        );
        assert_eq!(prepared.observation().execution_host_id(), &fixture.host);
        assert_eq!(prepared.instance().execution_host_id(), &fixture.host);
        assert_eq!(
            prepared.instance().target_reference().as_host_value(),
            "codex.remote"
        );
        assert_eq!(
            prepared.instance().interface_versions().next(),
            Some(prepared.observation().version())
        );
        assert_eq!(
            prepared.access_evidence().provenance(),
            &AccessEvidenceProvenance::Observed(
                AccessEvidenceSourceId::new("host-access-observer").unwrap()
            )
        );
        assert_eq!(state.request().executable, "codex.remote");
        assert_eq!(state.request().arguments, ["--version"]);
        assert!(state.request().environments.is_empty());
        assert!(state.waited());

        match driver {
            CodexPreparedDriver::StructuredExec => {
                assert_eq!(
                    prepared.instance().ownership(),
                    InstanceOwnership::HostOwnedEphemeral
                );
                assert_eq!(
                    prepared.instance().harness_configuration_posture(),
                    Some(HarnessConfigurationPosture::ProviderSuppressed)
                );
                assert_eq!(
                    prepared.instance().protocol_facade_id().as_str(),
                    "codex-exec-jsonl"
                );
            }
            CodexPreparedDriver::AppServer => {
                assert_eq!(
                    prepared.instance().ownership(),
                    InstanceOwnership::HostOwnedPersistent
                );
                assert_eq!(
                    prepared.instance().harness_configuration_posture(),
                    Some(HarnessConfigurationPosture::Ambient)
                );
                assert_eq!(
                    prepared.instance().protocol_facade_id().as_str(),
                    "codex-app-server-v2"
                );
            }
        }
    }
}

#[test]
fn prepared_compatibility_keeps_deprecated_and_unverified_newer_visible() {
    for (version, expected_status) in [
        ("0.100.0", Some(InterfaceSupportStatus::Deprecated)),
        ("0.145.0", Some(InterfaceSupportStatus::Maintained)),
        ("0.146.0", Some(InterfaceSupportStatus::Maintained)),
        ("0.147.0", Some(InterfaceSupportStatus::Maintained)),
        ("0.148.0", Some(InterfaceSupportStatus::Maintained)),
        ("0.149.0", Some(InterfaceSupportStatus::Maintained)),
        ("0.149.1", Some(InterfaceSupportStatus::Maintained)),
        ("0.150.0", Some(InterfaceSupportStatus::Maintained)),
        ("0.150.1", Some(InterfaceSupportStatus::Maintained)),
        ("0.151.0", Some(InterfaceSupportStatus::Maintained)),
        ("0.152.0", Some(InterfaceSupportStatus::Maintained)),
        ("0.152.1", None),
    ] {
        let fixture = fixture(CodexPreparedDriver::StructuredExec, "host.local", "codex");
        let (process, _) = FakeProcessService::completed(&format!("codex-cli {version}\n"));
        let prepared = block_on(prepare_codex(
            fixture.input,
            fixture.probe,
            services(fixture.host, process),
        ))
        .expect("permitted version prepares");

        match (prepared.observation().compatibility(), expected_status) {
            (InstalledExecutableCompatibility::Qualified(matched), Some(expected)) => {
                assert_eq!(matched.support_status(), expected);
            }
            (InstalledExecutableCompatibility::UnverifiedNewer(newer), None) => {
                assert_eq!(newer.version().as_str(), version);
                assert_eq!(newer.latest_qualified().as_str(), "0.152.0");
            }
            (actual, expected) => {
                panic!("unexpected compatibility {actual:?} for expected {expected:?}")
            }
        }
    }
}

#[test]
fn incompatible_malformed_and_bounded_output_fail_at_distinct_stages() {
    for (output, expected_stage, expected_code) in [
        (
            "codex-cli 0.108.0\n".to_owned(),
            PreparationStage::CompatibilityClassification,
            "swallowtail.codex.preparation.discovery_rejected",
        ),
        (
            "not-a-version\n".to_owned(),
            PreparationStage::VersionParse,
            "swallowtail.codex.discovery_malformed",
        ),
        (
            format!("codex-cli {}\n", "1".repeat(140)),
            PreparationStage::BoundedOutput,
            "swallowtail.codex.discovery_output_limit",
        ),
    ] {
        let fixture = fixture(CodexPreparedDriver::StructuredExec, "host.local", "codex");
        let (process, _) = FakeProcessService::completed(&output);
        let failure = block_on(prepare_codex(
            fixture.input,
            fixture.probe,
            services(fixture.host, process),
        ))
        .expect_err("observation must not promote");

        assert_eq!(failure.stage(), expected_stage);
        assert_eq!(failure.diagnostic().safe().code(), expected_code);
        assert!(!format!("{failure:?}").contains(&output));
    }
}
