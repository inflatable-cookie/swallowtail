use super::common::{
    NO_TOOL_SUCCESS, VERSION, deadline, evidence, host_id, model, preparation_input, prepare,
    probe, run_input,
};
use super::support;
use futures_executor::block_on;
use swallowtail_adapter_command_code::{
    COMMAND_CODE_RELEASE_AXIS, command_code_local_account_access_profile,
    prepare_command_code_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, CredentialMechanism, CredentialState,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, InterfaceVersionAxis, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ExecutableRef, InstalledExecutableTarget, OperationContent, OperationPolicy,
    PreparedAccessEvidence, ProviderRetentionPolicy, RequestId, StructuredRunDriver,
    StructuredRunRequest,
};

#[test]
fn facade_rejects_access_target_and_binding_drift_before_model_execution() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());

    let wrong_host = ExecutionHostId::new("command-code.fixture.other-host").unwrap();
    assert!(
        prepared
            .validate_execution_binding(&wrong_host, prepared.target())
            .is_err()
    );
    let wrong_target = InstalledExecutableTarget::new(
        ExecutableRef::new("/fixture/bin/other").unwrap(),
        InterfaceVersionAxis::new(COMMAND_CODE_RELEASE_AXIS).unwrap(),
    );
    assert!(
        prepared
            .validate_execution_binding(&host_id, &wrong_target)
            .is_err()
    );

    let access_id = AccessProfileId::new("command-code.fixture.access").unwrap();
    let host = support::FixtureHost::scripted([VERSION]);
    let wrong_access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("wrong.audience").unwrap(),
        SupportAuthority::ProviderSupported,
    );
    let error = block_on(prepare_command_code_headless(
        preparation_input(host_id.clone(), wrong_access, evidence(access_id)),
        probe(),
        host.services(host_id.clone()),
    ))
    .expect_err("access drift fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.command_code.preparation.access_profile_rejected"
    );
    assert!(!host.started());

    let access_id = AccessProfileId::new("command-code.fixture.access").unwrap();
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::Unknown,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let evidence_host = support::FixtureHost::scripted([VERSION]);
    let inaccessible = block_on(prepare_command_code_headless(
        preparation_input(
            host_id.clone(),
            command_code_local_account_access_profile(access_id),
            PreparedAccessEvidence::caller_asserted(status),
        ),
        probe(),
        evidence_host.services(host_id.clone()),
    ))
    .expect("discovery does not use account state");
    assert!(inaccessible.prepare_run(run_input(model(), "low")).is_err());
    assert_eq!(evidence_host.observations().len(), 1);
    assert_eq!(evidence_host.observations()[0].arguments, ["--version"]);

    let access_id = AccessProfileId::new("command-code.fixture.access").unwrap();
    let release_host = support::FixtureHost::scripted(["1.15.2\n"]);
    let error = block_on(prepare_command_code_headless(
        preparation_input(
            host_id.clone(),
            command_code_local_account_access_profile(access_id.clone()),
            evidence(access_id),
        ),
        probe(),
        release_host.services(host_id),
    ))
    .expect_err("different release fails");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::VersionParse
    );
    assert_eq!(release_host.observations().len(), 1);
    assert_eq!(release_host.observations()[0].arguments, ["--version"]);
}

#[test]
fn low_level_escape_hatch_rejects_missing_resource_before_process() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    let run = prepared
        .prepare_run(run_input(model(), "low"))
        .expect("run prepares");
    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::Prohibited)
        .with_harness_isolation(swallowtail_core::HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(swallowtail_core::HarnessConfigurationPosture::Ambient);
    let request = StructuredRunRequest::new(
        RequestId::new("command-code.fixture.drift").unwrap(),
        OperationContent::new("private prompt").unwrap(),
        policy,
    )
    .with_deadline(deadline());
    let host = support::FixtureHost::scripted([NO_TOOL_SUCCESS]);
    let result = block_on(run.low_level_driver().start_run(
        run.plan().clone(),
        request,
        host.services(host_id),
    ));
    let Err(error) = result else {
        panic!("missing resource must fail");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.command_code.headless.unsupported_request"
    );
    assert!(!host.started());
}
