use futures_executor::block_on;
use swallowtail_adapter_command_code::{
    COMMAND_CODE_EXECUTABLE_NAME, COMMAND_CODE_RELEASE_AXIS, CommandCodePreparationInput,
    CommandCodePreparationProbe, CommandCodePreparedRun, command_code_headless_claim,
    command_code_headless_descriptor, command_code_local_account_access_profile,
    prepare_command_code_headless,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, CapabilityProfile,
    ConfiguredInstance, ConfiguredInstanceId, CredentialState, DriverDescriptor, DriverRole,
    EndpointAuthorization, EntitlementState, ExecutionLayer, HarnessConfigurationPosture,
    HostServiceKind, InstanceRevision, IntegrationFamilyId, ModelRoute, OperationShape,
    PreflightContext, RuntimeReadiness, SupportAuthority, TransportFamilyId, preflight,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteProjectionContribution, PreparedAccessEvidence,
};

use super::assembly_support::*;
use super::{common, source, support};

const SHARED: &str = "command-code.projection.shared";
const SERVICES: [HostServiceKind; 3] = [
    HostServiceKind::Task,
    HostServiceKind::Process,
    HostServiceKind::Time,
];

#[test]
fn matching_source_cross_instance_and_stale_revision_rows_fail_closed() {
    let ready = ready_status();
    let mine = contribution(run_at("command-code.fixture.instance", "1", ready.clone()));
    let other = contribution(run_at("command-code.fixture.other", "1", ready.clone()));
    assert_ne!(
        mine.applicability().instance_id(),
        other.applicability().instance_id()
    );
    assert_same_route_access(&mine, &other);
    reject(mine.applicability().clone(), &mine, first(&other).clone());

    let stale = contribution(run_at("command-code.fixture.instance", "2", ready));
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
        assert_one_difference(&ready, &status);
        assert!(try_run_at("command-code.fixture.instance", "1", status).is_err());
    }
}

#[test]
fn synthetic_neighbour_route_mixtures_fail_closed_both_directions() {
    let mine = contribution(run_at("command-code.fixture.instance", "1", ready_status()));
    let local = applicability(&mine, false);
    assert_eq!(&local, mine.applicability());
    let neighbour = applicability(&mine, true);
    assert_ne!(
        neighbour.driver_identity(),
        mine.applicability().driver_identity()
    );
    assert_eq!(
        neighbour.operation_shape(),
        mine.applicability().operation_shape()
    );
    assert_eq!(neighbour.model(), mine.applicability().model());
    let row = first(&mine).clone();
    reject(
        mine.applicability().clone(),
        &mine,
        rebind(&row, neighbour.clone()),
    );
    reject(neighbour, &mine, row);
}

fn try_run_at(
    instance_id: &str,
    revision: &str,
    status: AccessStatus,
) -> Result<CommandCodePreparedRun, swallowtail_runtime::PreparationFailure> {
    let host_id = common::host_id();
    let host = support::FixtureHost::scripted([common::VERSION]);
    let access_id = AccessProfileId::new("command-code.fixture.access").expect("profile");
    let integration = block_on(prepare_command_code_headless(
        CommandCodePreparationInput::new(
            ConfiguredInstanceId::new(instance_id).expect("instance"),
            InstanceRevision::new(revision).expect("revision"),
            host_id.clone(),
            swallowtail_runtime::InstalledExecutableTarget::new(
                swallowtail_runtime::ExecutableRef::new(format!(
                    "/fixture/bin/{COMMAND_CODE_EXECUTABLE_NAME}"
                ))
                .expect("executable"),
                swallowtail_core::InterfaceVersionAxis::new(COMMAND_CODE_RELEASE_AXIS)
                    .expect("axis"),
            ),
            swallowtail_runtime::EnvironmentRef::new("command-code.fixture.environment")
                .expect("environment"),
            command_code_local_account_access_profile(access_id),
            PreparedAccessEvidence::caller_asserted(status),
        ),
        CommandCodePreparationProbe::new(
            swallowtail_runtime::RequestId::new("command-code.projection.probe").expect("request"),
            swallowtail_runtime::ScopeId::new("command-code.projection.probe").expect("scope"),
            common::deadline(),
            swallowtail_runtime::DiscoveryCancellation::new(),
        ),
        host.services(host_id),
    ))?;
    integration.prepare_run(common::run_input(common::model(), "assembly"))
}

fn run_at(instance_id: &str, revision: &str, status: AccessStatus) -> CommandCodePreparedRun {
    try_run_at(instance_id, revision, status).expect("run prepares")
}

fn contribution(run: CommandCodePreparedRun) -> ConsumerRouteProjectionContribution {
    run.consumer_route_projection_contribution(source(SHARED))
        .expect("run contributes")
}

fn applicability(
    contribution: &ConsumerRouteProjectionContribution,
    neighbour: bool,
) -> ConsumerRouteApplicability {
    let run = run_at("command-code.fixture.instance", "1", ready_status());
    let plan = run.plan();
    let driver_id = if neighbour {
        AdapterId::new("swallowtail.qoder.headless").expect("neighbour")
    } else {
        plan.driver_identity().id().clone()
    };
    let capabilities = CapabilityProfile::new(plan.requirements().capabilities().cloned());
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
        capabilities.clone(),
    )
    .with_interface_versions(plan.interface_versions().cloned())
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let descriptor = if neighbour {
        DriverDescriptor::new(
            AdapterIdentity::new(
                driver_id,
                command_code_headless_descriptor()
                    .identity()
                    .version()
                    .clone(),
            ),
            IntegrationFamilyId::new("qoder-headless").expect("family"),
            TransportFamilyId::new("headless-jsonl").expect("transport"),
        )
        .with_roles([DriverRole::StructuredRun])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([OperationShape::StructuredRun])
        .with_required_host_services(DriverRole::StructuredRun, SERVICES)
        .with_interface_compatibility(command_code_headless_claim())
    } else {
        command_code_headless_descriptor()
    };
    let binding = contribution.applicability().model().expect("model binding");
    let mut model = ModelRoute::new(
        binding.route_id().clone(),
        binding.route_revision().clone(),
        instance.id().clone(),
        binding.model_id().clone(),
        capabilities,
    );
    if let Some(provider) = binding.provider_id() {
        model = model.with_provider_id(provider.clone());
    }
    let profile = command_code_local_account_access_profile(
        AccessProfileId::new("command-code.fixture.access").expect("profile"),
    );
    let status = ready_status();
    let mut context = PreflightContext::new(&descriptor, &instance, &profile, &status, SERVICES);
    context = context.with_model_route(&model);
    ConsumerRouteApplicability::from_plan(
        &preflight(&context, plan.requirements()).expect("rebuilt plan forms"),
    )
}

fn ready_status() -> AccessStatus {
    common::evidence(AccessProfileId::new("command-code.fixture.access").expect("profile"))
        .status()
        .clone()
}
