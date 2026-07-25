use crate::{
    ProfilePreflightFixture, RuntimePreflightFixture, SessionAccessFixtureCase,
    SessionAccessPreflightFixture, SyntheticProfile,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, CredentialState, Diagnostic, EndpointAuthorization,
    EntitlementState, ExecutionHostId, HarnessConfigurationPosture, PreflightPlan,
    RuntimeReadiness, SafeDiagnostic, SessionAccessPolicy, SessionProviderStatePolicy,
    SupportAuthority,
};
use swallowtail_runtime::{
    AccessEvidenceProvenance, AccessEvidenceSourceId, PreparationFailure, PreparationStage,
    PreparedAccessEvidence, PreparedOperationEvidence, SessionPlanAgreement,
    validate_session_plan_agreement,
};

pub fn assert_prepared_integration_primitives() {
    assert_plan_derivation_and_mismatch();
    assert_missing_plan_echo_fails_safely();
    assert_access_provenance();
    assert_preparation_stages();
    assert_cross_shape_operation_evidence();
    assert_evidence_mismatch_fails_before_effects();
}

pub fn assert_prepared_operation_evidence_matches_plan(
    evidence: &PreparedOperationEvidence,
    plan: &PreflightPlan,
) {
    assert!(evidence.matches_plan(plan));
    assert_eq!(evidence.binding().driver_identity(), plan.driver_identity());
    assert_eq!(
        evidence.binding().driver_role(),
        plan.requirements().driver_role()
    );
    assert_eq!(
        evidence.binding().execution_layer(),
        plan.requirements().execution_layer()
    );
    assert_eq!(
        evidence.binding().operation_shape(),
        plan.requirements().operation_shape()
    );
    assert_eq!(evidence.binding().instance_id(), plan.instance_id());
    assert_eq!(
        evidence.binding().instance_revision(),
        plan.instance_revision()
    );
    assert_eq!(
        evidence.binding().execution_host_id(),
        plan.execution_host_id()
    );
    assert_eq!(
        evidence.binding().instance_target(),
        plan.instance_target_ref()
    );
    assert_eq!(
        evidence.binding().protocol_facade_id(),
        plan.protocol_facade_id()
    );
    assert_eq!(evidence.access().status(), plan.access_status());

    let expected = plan.interface_versions().collect::<Vec<_>>();
    let actual = evidence.interface_compatibility().collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert_eq!(actual.binding(), expected);
        assert_eq!(
            actual.assessment(),
            &plan.assess_interface_version(expected)
        );
    }
}

pub fn assert_prepared_failure_before_effects<T>(
    result: &Result<T, PreparationFailure>,
    provider_effect_count: usize,
    expected_stage: PreparationStage,
) {
    let Err(failure) = result else {
        panic!("prepared operation unexpectedly succeeded");
    };
    assert_eq!(failure.stage(), expected_stage);
    assert_eq!(provider_effect_count, 0);
}

fn assert_plan_derivation_and_mismatch() {
    let fixture = SessionAccessPreflightFixture::for_case(
        SessionAccessFixtureCase::ReadOnly,
        ExecutionHostId::new("fixture.prepared.host").expect("host id is valid"),
    )
    .with_session_plan_echoes(
        SessionProviderStatePolicy::Prohibited,
        HarnessConfigurationPosture::Ambient,
    );
    let plan = fixture.preflight().expect("prepared preflight succeeds");
    let agreement = SessionPlanAgreement::from_plan(&plan).expect("plan echoes derive");

    assert_eq!(agreement.access_policy(), &SessionAccessPolicy::read_only());
    assert_eq!(
        agreement.provider_state_policy(),
        Some(SessionProviderStatePolicy::Prohibited)
    );
    assert_eq!(
        agreement.harness_configuration_posture(),
        Some(HarnessConfigurationPosture::Ambient)
    );
    validate_session_plan_agreement(&plan, &agreement).expect("derived agreement matches");

    for mismatch in [
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::resource_free(),
            Some(SessionProviderStatePolicy::Prohibited),
            Some(HarnessConfigurationPosture::Ambient),
        ),
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::read_only(),
            Some(SessionProviderStatePolicy::DurableConversationDeleteOnClose),
            Some(HarnessConfigurationPosture::Ambient),
        ),
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::read_only(),
            Some(SessionProviderStatePolicy::Prohibited),
            Some(HarnessConfigurationPosture::ProviderSuppressed),
        ),
    ] {
        validate_session_plan_agreement(&plan, &mismatch)
            .expect_err("every plan-echo mismatch must fail");
    }
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

fn assert_missing_plan_echo_fails_safely() {
    let plan = RuntimePreflightFixture::canonical()
        .preflight()
        .expect("structured fixture preflight succeeds");
    let failure =
        SessionPlanAgreement::from_plan(&plan).expect_err("missing session access must fail");

    assert_eq!(failure.stage(), PreparationStage::Preflight);
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.session_request.plan_access_missing"
    );
}

fn assert_access_provenance() {
    let status = AccessStatus::new(
        AccessProfileId::new("fixture.prepared.access").expect("access id is valid"),
        CredentialState::Unknown,
        EntitlementState::Unknown,
        EndpointAuthorization::Unknown,
        RuntimeReadiness::Unknown,
        SupportAuthority::ExperimentalObserved,
    );
    let observed = PreparedAccessEvidence::observed(
        status.clone(),
        AccessEvidenceSourceId::new("fixture.prepared.observer").expect("source id is valid"),
    );
    let asserted = PreparedAccessEvidence::caller_asserted(status);

    assert_eq!(observed.status(), asserted.status());
    assert!(matches!(
        observed.provenance(),
        AccessEvidenceProvenance::Observed(_)
    ));
    assert_eq!(
        asserted.provenance(),
        &AccessEvidenceProvenance::CallerAsserted
    );
    assert!(!format!("{observed:?}").contains("fixture.prepared.observer"));
}

fn assert_preparation_stages() {
    let stages = [
        PreparationStage::TargetSelection,
        PreparationStage::ProcessSpawn,
        PreparationStage::BoundedOutput,
        PreparationStage::ProcessExit,
        PreparationStage::VersionParse,
        PreparationStage::CompatibilityClassification,
        PreparationStage::AccessEvidence,
        PreparationStage::Preflight,
        PreparationStage::Cleanup,
    ];
    let observed = stages
        .map(|stage| {
            PreparationFailure::new(
                stage,
                Diagnostic::new(SafeDiagnostic::new(
                    "swallowtail.preparation.fixture",
                    "Preparation fixture failed",
                )),
            )
            .stage()
        })
        .to_vec();
    assert_eq!(observed, stages);

    let parse = PreparationFailure::new(
        PreparationStage::VersionParse,
        Diagnostic::new(SafeDiagnostic::new(
            "swallowtail.preparation.version_parse_failed",
            "Executable version could not be parsed",
        )),
    );
    let cleanup = PreparationFailure::new(
        PreparationStage::Cleanup,
        Diagnostic::new(SafeDiagnostic::new(
            "swallowtail.preparation.cleanup_failed",
            "Preparation cleanup failed",
        )),
    )
    .with_cause(parse);
    assert_eq!(
        cleanup.cause().map(PreparationFailure::stage),
        Some(PreparationStage::VersionParse)
    );
}

fn assert_cross_shape_operation_evidence() {
    let fixtures = [
        ProfilePreflightFixture::harness_rpc_contract(),
        ProfilePreflightFixture::new(SyntheticProfile::HostedDirectApi),
        ProfilePreflightFixture::attached_runtime(),
    ];
    for fixture in fixtures {
        let plan = fixture.preflight().expect("profile preflight succeeds");
        let evidence = PreparedOperationEvidence::from_plan(
            plan.clone(),
            PreparedAccessEvidence::caller_asserted(plan.access_status().clone()),
        )
        .expect("matching prepared evidence succeeds");

        assert_prepared_operation_evidence_matches_plan(&evidence, &plan);
        assert!(
            !format!("{evidence:?}").contains(plan.instance_target_ref().as_host_value()),
            "prepared evidence must keep the host-owned target opaque"
        );
    }
}

fn assert_evidence_mismatch_fails_before_effects() {
    let fixture = RuntimePreflightFixture::canonical();
    let plan = fixture.preflight().expect("fixture preflight succeeds");
    let result = PreparedOperationEvidence::from_plan(
        plan,
        PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            AccessProfileId::new("fixture.prepared.wrong-access")
                .expect("access profile id is valid"),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        )),
    );

    assert_prepared_failure_before_effects(
        &result,
        fixture.provider_side_effect_count(),
        PreparationStage::AccessEvidence,
    );
}
