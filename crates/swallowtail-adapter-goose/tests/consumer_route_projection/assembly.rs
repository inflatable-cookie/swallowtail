use swallowtail_adapter_goose::{
    goose_acp_claim, goose_acp_descriptor, goose_local_config_access_profile,
};
use swallowtail_core::{
    AccessProfileId, AdapterId, AdapterIdentity, CapabilityProfile, ConfiguredInstance,
    CredentialState, DriverDescriptor, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HarnessConfigurationPosture, HostServiceKind, IntegrationFamilyId,
    OperationShape, PreflightContext, RuntimeReadiness, SupportAuthority, TransportFamilyId,
    preflight,
};
use swallowtail_runtime::{ConsumerRouteApplicability, ConsumerRouteProjectionContribution};

use super::assembly_support::*;
use super::{prepared_session, prepared_session_at, ready_status, source};

const SHARED: &str = "goose.projection.shared";
const SERVICES: [HostServiceKind; 3] = [
    HostServiceKind::Task,
    HostServiceKind::Process,
    HostServiceKind::WorkingResource,
];

#[test]
fn matching_source_cross_instance_and_stale_revision_rows_fail_closed() {
    let ready = ready_status();
    let mine = contribution(
        prepared_session_at("goose.projection.instance", "1", ready.clone())
            .expect("baseline prepares"),
    );
    let other = contribution(
        prepared_session_at("goose.projection.other", "1", ready.clone())
            .expect("other instance prepares"),
    );
    assert_ne!(
        mine.applicability().instance_id(),
        other.applicability().instance_id()
    );
    assert_same_route_access(&mine, &other);
    reject(mine.applicability().clone(), &mine, first(&other).clone());

    let stale = contribution(
        prepared_session_at("goose.projection.instance", "2", ready)
            .expect("stale revision prepares"),
    );
    assert_ne!(
        mine.applicability().instance_revision(),
        stale.applicability().instance_revision()
    );
    assert_same_route_access(&mine, &stale);
    reject(mine.applicability().clone(), &mine, first(&stale).clone());
}

#[test]
fn all_five_access_drifts_stop_before_any_row_can_form() {
    let ready = ready_status();
    let shifted = [
        access(&ready, Some(CredentialState::Ready), None, None, None, None),
        access(
            &ready,
            None,
            Some(EntitlementState::Exhausted),
            None,
            None,
            None,
        ),
        access(
            &ready,
            None,
            None,
            Some(EndpointAuthorization::Denied),
            None,
            None,
        ),
        access(
            &ready,
            None,
            None,
            None,
            Some(RuntimeReadiness::Degraded),
            None,
        ),
        access(
            &ready,
            None,
            None,
            None,
            None,
            Some(SupportAuthority::ExperimentalObserved),
        ),
    ];
    for status in shifted {
        let differences = [
            ready.credential() != status.credential(),
            ready.entitlement() != status.entitlement(),
            ready.endpoint_authorization() != status.endpoint_authorization(),
            ready.runtime_readiness() != status.runtime_readiness(),
            ready.support_authority() != status.support_authority(),
        ];
        assert_eq!(differences.into_iter().filter(|value| *value).count(), 1);
        assert!(prepared_session_at("goose.projection.instance", "1", status).is_err());
    }
}

#[test]
fn synthetic_neighbour_route_mixtures_fail_closed_both_directions() {
    let mine = contribution(prepared_session());
    let local = applicability(false);
    assert_eq!(&local, mine.applicability());
    let neighbour = applicability(true);
    assert_ne!(
        neighbour.driver_identity(),
        mine.applicability().driver_identity()
    );
    assert_eq!(
        neighbour.operation_shape(),
        mine.applicability().operation_shape()
    );
    assert_eq!(
        neighbour.access_profile_id(),
        mine.applicability().access_profile_id()
    );
    let row = first(&mine).clone();
    reject(
        mine.applicability().clone(),
        &mine,
        rebind(&row, neighbour.clone()),
    );
    reject(neighbour, &mine, row);
}

fn applicability(neighbour: bool) -> ConsumerRouteApplicability {
    let prepared = prepared_session();
    let plan = prepared.plan();
    let driver_id = if neighbour {
        AdapterId::new("swallowtail.copilot-cli.acp").expect("neighbour")
    } else {
        plan.driver_identity().id().clone()
    };
    let instance = ConfiguredInstance::new(
        plan.instance_id().clone(),
        plan.instance_revision().clone(),
        driver_id.clone(),
        plan.execution_host_id().clone(),
        plan.instance_target_ref().clone(),
        plan.ownership(),
        plan.access_profile_id().clone(),
        plan.access_status().support_authority(),
        plan.protocol_facade_id().clone(),
        plan.instance_policy_id().clone(),
        CapabilityProfile::new(plan.requirements().capabilities().cloned()),
    )
    .with_interface_versions(plan.interface_versions().cloned())
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let descriptor = if neighbour {
        DriverDescriptor::new(
            AdapterIdentity::new(
                driver_id,
                goose_acp_descriptor().identity().version().clone(),
            ),
            IntegrationFamilyId::new("copilot-cli").expect("family"),
            TransportFamilyId::new("acp-v1-stdio").expect("transport"),
        )
        .with_roles([DriverRole::InteractiveSession])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([OperationShape::InteractiveSession])
        .with_required_host_services(DriverRole::InteractiveSession, SERVICES)
        .with_interface_compatibility(goose_acp_claim())
    } else {
        goose_acp_descriptor()
    };
    let profile = goose_local_config_access_profile(
        AccessProfileId::new("goose.projection.local-config").expect("profile"),
    );
    let status = ready_status();
    let context = PreflightContext::new(&descriptor, &instance, &profile, &status, SERVICES);
    ConsumerRouteApplicability::from_plan(
        &preflight(&context, plan.requirements()).expect("rebuilt plan forms"),
    )
}

fn contribution(
    session: swallowtail_adapter_goose::GoosePreparedSession,
) -> ConsumerRouteProjectionContribution {
    session
        .consumer_route_projection_contribution(source(SHARED))
        .expect("session contributes")
}
