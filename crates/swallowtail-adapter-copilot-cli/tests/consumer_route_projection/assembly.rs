use swallowtail_adapter_copilot_cli::{
    copilot_cli_acp_claim, copilot_cli_acp_descriptor, copilot_cli_host_account_access_profile,
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

const SHARED: &str = "copilot-cli.projection.shared";
const HOST_SERVICES: [HostServiceKind; 3] = [
    HostServiceKind::Task,
    HostServiceKind::Process,
    HostServiceKind::WorkingResource,
];

#[test]
fn matching_source_cross_instance_and_stale_revision_rows_fail_closed() {
    let baseline = ready_status();
    let mine = contribution(
        prepared_session_at("copilot-cli.projection.instance", "1", baseline.clone())
            .expect("baseline prepares"),
    );
    let other_instance = contribution(
        prepared_session_at("copilot-cli.projection.other", "1", baseline.clone())
            .expect("other instance prepares"),
    );
    assert_ne!(
        mine.applicability().instance_id(),
        other_instance.applicability().instance_id()
    );
    assert_shared_dimensions(&mine, &other_instance);
    assert_rejects(&mine, first_row(&other_instance).clone());

    let stale = contribution(
        prepared_session_at("copilot-cli.projection.instance", "2", baseline)
            .expect("stale revision prepares"),
    );
    assert_ne!(
        mine.applicability().instance_revision(),
        stale.applicability().instance_revision()
    );
    assert_shared_dimensions(&mine, &stale);
    assert_rejects(&mine, first_row(&stale).clone());
}

#[test]
fn all_five_access_drifts_stop_before_any_row_can_form() {
    let ready = ready_status();
    let statuses = [
        status(&ready, Some(CredentialState::Ready), None, None, None, None),
        status(
            &ready,
            None,
            Some(EntitlementState::Exhausted),
            None,
            None,
            None,
        ),
        status(
            &ready,
            None,
            None,
            Some(EndpointAuthorization::Denied),
            None,
            None,
        ),
        status(
            &ready,
            None,
            None,
            None,
            Some(RuntimeReadiness::Degraded),
            None,
        ),
        status(
            &ready,
            None,
            None,
            None,
            None,
            Some(SupportAuthority::ProviderSupported),
        ),
    ];
    for shifted in statuses {
        assert_one_access_difference(&ready, &shifted);
        assert!(
            prepared_session_at("copilot-cli.projection.instance", "1", shifted).is_err(),
            "drifted access must stop before a contribution exists"
        );
    }
}

#[test]
fn synthetic_neighbour_route_mixtures_fail_closed_both_directions() {
    let mine = contribution(prepared_session());
    let local = rebuilt_applicability(&mine, false);
    assert_eq!(&local, mine.applicability());
    let neighbour = rebuilt_applicability(&mine, true);
    assert_ne!(
        neighbour.driver_identity(),
        mine.applicability().driver_identity()
    );
    assert_shared_applicability(&neighbour, mine.applicability());

    let row = first_row(&mine).clone();
    let rebound = rebind(&row, neighbour.clone());
    assert_rejects(&mine, rebound);
    assert_rejects_applicability(neighbour, &mine, row);
}

fn contribution(
    session: swallowtail_adapter_copilot_cli::CopilotCliPreparedSession,
) -> ConsumerRouteProjectionContribution {
    session
        .consumer_route_projection_contribution(source(SHARED))
        .expect("session contributes")
}

fn rebuilt_applicability(
    contribution: &ConsumerRouteProjectionContribution,
    neighbour: bool,
) -> ConsumerRouteApplicability {
    let plan = contribution.applicability();
    let driver_id = if neighbour {
        AdapterId::new("swallowtail.goose.acp").expect("neighbour id")
    } else {
        plan.driver_identity().id().clone()
    };
    let capabilities = CapabilityProfile::new(
        prepared_session()
            .plan()
            .requirements()
            .capabilities()
            .cloned(),
    );
    let prepared = prepared_session();
    let base = prepared.plan();
    let instance = ConfiguredInstance::new(
        base.instance_id().clone(),
        base.instance_revision().clone(),
        driver_id.clone(),
        base.execution_host_id().clone(),
        base.instance_target_ref().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.access_status().support_authority(),
        base.protocol_facade_id().clone(),
        base.instance_policy_id().clone(),
        capabilities,
    )
    .with_interface_versions(base.interface_versions().cloned())
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let descriptor = if neighbour {
        DriverDescriptor::new(
            AdapterIdentity::new(
                driver_id,
                copilot_cli_acp_descriptor().identity().version().clone(),
            ),
            IntegrationFamilyId::new("goose-acp").expect("family"),
            TransportFamilyId::new("acp-v1-stdio").expect("transport"),
        )
        .with_roles([DriverRole::InteractiveSession])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([OperationShape::InteractiveSession])
        .with_required_host_services(DriverRole::InteractiveSession, HOST_SERVICES)
        .with_interface_compatibility(copilot_cli_acp_claim())
    } else {
        copilot_cli_acp_descriptor()
    };
    let profile = copilot_cli_host_account_access_profile(
        AccessProfileId::new("copilot-cli.projection.host-account").expect("profile"),
    );
    let status = ready_status();
    let context = PreflightContext::new(&descriptor, &instance, &profile, &status, HOST_SERVICES);
    let rebuilt = preflight(&context, base.requirements()).expect("rebuilt plan forms");
    ConsumerRouteApplicability::from_plan(&rebuilt)
}
