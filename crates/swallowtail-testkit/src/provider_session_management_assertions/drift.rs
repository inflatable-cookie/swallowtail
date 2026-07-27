use crate::{
    ProviderSessionManagementBindingDrift, ProviderSessionManagementFixture,
    ProviderSessionManagementFixtureCase, RecordingHostServices, RecordingOutcome,
};
use swallowtail_core::{
    ProviderSessionAffectedScope, ProviderSessionCancellationPosture,
    ProviderSessionDeletionStrength, ProviderSessionInitialStateRequirement,
    ProviderSessionManagementAction,
};
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, ProviderSessionManagementAgreement,
    ProviderSessionManagementPlan, validate_provider_session_management_request,
};

pub(super) fn assert_request_drift_stops_before_dispatch() {
    let local = ProviderSessionManagementFixture::local(
        ProviderSessionManagementFixtureCase::Qualified,
        ProviderSessionManagementAction::Archive,
    );
    let remote = ProviderSessionManagementFixture::remote_authoritative(
        ProviderSessionManagementFixtureCase::Qualified,
        ProviderSessionManagementAction::Archive,
    );
    let local_plan = local.plan(None).expect("local plan is valid");
    let remote_plan = remote.plan(None).expect("remote plan is valid");
    let request = ArchiveProviderSessionRequest::from_plan(
        swallowtail_runtime::RequestId::new("fixture-drift").expect("request id is valid"),
        &remote_plan,
    )
    .expect("remote request is valid");
    let host = RecordingHostServices::for_host(
        local_plan.preflight().execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let failure = validate_provider_session_management_request(
        &local_plan,
        request.agreement(),
        host.services(),
    )
    .expect_err("cross-topology request must fail");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.provider_session_management.plan_mismatch"
    );
    assert!(host.calls().is_empty());

    for drift in [
        ProviderSessionManagementBindingDrift::ProviderSessionReference,
        ProviderSessionManagementBindingDrift::DriverIdentity,
        ProviderSessionManagementBindingDrift::IntegrationFamily,
        ProviderSessionManagementBindingDrift::TransportFamily,
        ProviderSessionManagementBindingDrift::ConfiguredInstance,
        ProviderSessionManagementBindingDrift::InstanceRevision,
        ProviderSessionManagementBindingDrift::ExecutionHost,
        ProviderSessionManagementBindingDrift::InstanceTarget,
        ProviderSessionManagementBindingDrift::ProtocolFacade,
        ProviderSessionManagementBindingDrift::AccessProfile,
        ProviderSessionManagementBindingDrift::InterfaceVersion,
        ProviderSessionManagementBindingDrift::Capabilities,
        ProviderSessionManagementBindingDrift::WorkingResource,
        ProviderSessionManagementBindingDrift::Origin,
    ] {
        let binding = local
            .drifted_binding(drift)
            .expect("drifted binding remains internally valid");
        let agreement = local_plan.agreement();
        let drifted = ProviderSessionManagementAgreement::new(
            binding,
            agreement.action(),
            agreement.initial_state(),
            agreement.affected_scope(),
            agreement.activity(),
            agreement.cancellation(),
            agreement.deadline(),
        );
        match ProviderSessionManagementPlan::new(local_plan.preflight().clone(), drifted) {
            Ok(drifted_plan) => {
                assert_agreement_drift_fails(&local_plan, &drifted_plan, host.services());
            }
            Err(failure) => {
                assert_eq!(
                    failure.diagnostic().code(),
                    "swallowtail.provider_session_management.plan_mismatch"
                );
                assert!(!format!("{failure:?}").contains("fixture.private"));
            }
        }
    }

    assert_plan_field_drift_fails(&local_plan, host.services());
}

fn assert_plan_field_drift_fails(
    plan: &ProviderSessionManagementPlan,
    services: &swallowtail_runtime::HostServices,
) {
    let agreement = plan.agreement();
    let alternate = |action, initial_state, affected_scope, cancellation, deadline| {
        ProviderSessionManagementAgreement::new(
            agreement.binding().clone(),
            action,
            initial_state,
            affected_scope,
            agreement.activity(),
            cancellation,
            deadline,
        )
    };

    let action_failure = ProviderSessionManagementPlan::new(
        plan.preflight().clone(),
        alternate(
            ProviderSessionManagementAction::Restore,
            ProviderSessionInitialStateRequirement::Archived,
            agreement.affected_scope(),
            agreement.cancellation(),
            agreement.deadline(),
        ),
    )
    .expect_err("action drift must fail preflight");
    assert_eq!(
        action_failure.diagnostic().code(),
        "swallowtail.provider_session_management.capability_mismatch"
    );

    let state_failure = ProviderSessionManagementPlan::new(
        plan.preflight().clone(),
        alternate(
            agreement.action(),
            ProviderSessionInitialStateRequirement::Archived,
            agreement.affected_scope(),
            agreement.cancellation(),
            agreement.deadline(),
        ),
    )
    .expect_err("initial-state drift must fail preflight");
    assert_eq!(
        state_failure.diagnostic().code(),
        "swallowtail.provider_session_management.initial_state_mismatch"
    );

    for drifted in [
        alternate(
            agreement.action(),
            agreement.initial_state(),
            ProviderSessionAffectedScope::ProviderDefinedDescendants,
            agreement.cancellation(),
            agreement.deadline(),
        ),
        alternate(
            agreement.action(),
            agreement.initial_state(),
            agreement.affected_scope(),
            ProviderSessionCancellationPosture::ProviderNative,
            agreement.deadline(),
        ),
        alternate(
            agreement.action(),
            agreement.initial_state(),
            agreement.affected_scope(),
            agreement.cancellation(),
            Some(swallowtail_runtime::Deadline::at(
                swallowtail_runtime::MonotonicInstant::from_ticks(100),
            )),
        ),
    ] {
        let drifted_plan = ProviderSessionManagementPlan::new(plan.preflight().clone(), drifted)
            .expect("alternate explicit plan is valid");
        assert_agreement_drift_fails(plan, &drifted_plan, services);
    }

    assert_deletion_strength_drift_fails();
}

fn assert_deletion_strength_drift_fails() {
    let delete = ProviderSessionManagementFixture::local(
        ProviderSessionManagementFixtureCase::Qualified,
        ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        ),
    );
    let delete_plan = delete.plan(None).expect("delete plan is valid");
    let deletion_strength_drift = ProviderSessionManagementPlan::new(
        delete_plan.preflight().clone(),
        ProviderSessionManagementAgreement::new(
            delete_plan.agreement().binding().clone(),
            ProviderSessionManagementAction::Delete(
                ProviderSessionDeletionStrength::ProviderHardDeleted,
            ),
            delete_plan.agreement().initial_state(),
            delete_plan.agreement().affected_scope(),
            delete_plan.agreement().activity(),
            delete_plan.agreement().cancellation(),
            delete_plan.agreement().deadline(),
        ),
    )
    .expect("alternate deletion-strength plan is valid");
    let host = RecordingHostServices::for_host(
        delete_plan.preflight().execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let request = swallowtail_runtime::DeleteProviderSessionRequest::from_plan(
        swallowtail_runtime::RequestId::new("fixture-deletion-strength-drift")
            .expect("request id is valid"),
        &deletion_strength_drift,
    )
    .expect("drifted delete request is valid");
    let failure = validate_provider_session_management_request(
        &delete_plan,
        request.agreement(),
        host.services(),
    )
    .expect_err("deletion-strength drift must fail");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.provider_session_management.plan_mismatch"
    );
    assert!(host.calls().is_empty());
}

fn assert_agreement_drift_fails(
    expected: &ProviderSessionManagementPlan,
    drifted: &ProviderSessionManagementPlan,
    services: &swallowtail_runtime::HostServices,
) {
    let request = ArchiveProviderSessionRequest::from_plan(
        swallowtail_runtime::RequestId::new("fixture-agreement-drift")
            .expect("request id is valid"),
        drifted,
    )
    .expect("drifted request is internally valid");
    let failure =
        validate_provider_session_management_request(expected, request.agreement(), services)
            .expect_err("agreement drift must fail");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.provider_session_management.plan_mismatch"
    );
}
