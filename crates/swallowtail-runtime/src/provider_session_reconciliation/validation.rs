use super::{
    ProviderSessionReconciliationAgreement, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest,
};
use crate::plan_family::{
    PlanRule, check_plan_rules, failure as plan_failure, validate_agreement_matches_plan,
    validate_execution_services,
};
use crate::{HostServices, RuntimeFailure};
use swallowtail_core::{
    Capability, CapabilityConstraint, DriverRole, ExecutionLayer, HostServiceKind, OperationShape,
    PreflightPlan, ResourceAccess, SessionAccessPolicy,
};

const PLAN_MISMATCH_CODE: &str = "swallowtail.provider_session_reconciliation.plan_mismatch";
const PLAN_MISMATCH_MESSAGE: &str =
    "Provider-session reconciliation does not match its immutable binding";

const BOUND_MISMATCH_CODE: &str = "swallowtail.provider_session_reconciliation.bound_mismatch";
const BOUND_MISMATCH_MESSAGE: &str =
    "Provider-session reconciliation bounds differ from its capability plan";

const TIME_SERVICE_CODE: &str =
    "swallowtail.provider_session_reconciliation.time_service_required";
const TIME_SERVICE_MESSAGE: &str =
    "Deadline-bound provider-session reconciliation requires time service";

/// Ordered per-role validation rules for a reconciliation plan.
///
/// Reconciliation requires harness-interaction evidence, an ambient read
/// harness access policy, and a working-resource service in addition to the
/// shared role, shape, binding, and capability checks. These are the
/// explicit rules the audit asked to surface rather than hand-rolled
/// validator differences.
const RECONCILIATION_PLAN_RULES: [PlanRule<ProviderSessionReconciliationAgreement>; 9] = [
    PlanRule::new(
        PLAN_MISMATCH_CODE,
        PLAN_MISMATCH_MESSAGE,
        |preflight, _| {
            preflight.requirements().execution_layer() == ExecutionLayer::HarnessInteraction
        },
    ),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, _| {
        preflight.requirements().driver_role() == DriverRole::ProviderSessionReconciliation
    }),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, _| {
        preflight.requirements().operation_shape()
            == OperationShape::ProviderSessionReconciliation
    }),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, agreement| {
        agreement.binding().matches_plan(preflight)
    }),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, _| {
        preflight.requirements().session_access_policy()
            == Some(&SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
    }),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, _| {
        preflight
            .requirements()
            .capabilities()
            .any(|required| required.capability() == Capability::ProviderSessionReconciliation)
    }),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, _| {
        preflight
            .requirements()
            .host_services()
            .any(|required| required == HostServiceKind::WorkingResource)
    }),
    PlanRule::new(BOUND_MISMATCH_CODE, BOUND_MISMATCH_MESSAGE, |preflight, agreement| {
        let capability = preflight
            .requirements()
            .capabilities()
            .find(|required| required.capability() == Capability::ProviderSessionReconciliation)
            .expect("checked capability");
        let bounds = agreement.bounds();
        let expected = [
            CapabilityConstraint::ReplayMaximumItems(bounds.maximum_replay_items().get()),
            CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_replay_bytes().get()),
        ];
        expected.iter().all(|constraint| {
            capability.constraints().any(|actual| actual == constraint)
        }) && capability.constraints().count() == expected.len()
    }),
    PlanRule::new(TIME_SERVICE_CODE, TIME_SERVICE_MESSAGE, |preflight, agreement| {
        agreement.deadline().is_none()
            || preflight
                .requirements()
                .host_services()
                .any(|required| required == HostServiceKind::Time)
    }),
];

/// Verifies that execution input still matches its immutable plan.
pub fn validate_provider_session_reconciliation_request(
    plan: &ProviderSessionReconciliationPlan,
    request: &ProviderSessionReconciliationRequest,
) -> Result<(), RuntimeFailure> {
    validate_agreement_matches_plan(
        plan.agreement(),
        request.agreement(),
        PLAN_MISMATCH_CODE,
        "Provider-session reconciliation request does not match its immutable plan",
    )
}

/// Verifies request, execution host, and required host-service availability.
pub fn validate_provider_session_reconciliation_execution(
    plan: &ProviderSessionReconciliationPlan,
    request: &ProviderSessionReconciliationRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_provider_session_reconciliation_request(plan, request)?;
    validate_execution_services(
        plan.preflight(),
        services,
        "swallowtail.provider_session_reconciliation.service_unavailable",
        "Provider-session reconciliation host services are unavailable",
    )
}

pub(super) fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderSessionReconciliationAgreement,
) -> Result<(), RuntimeFailure> {
    check_plan_rules(preflight, agreement, &RECONCILIATION_PLAN_RULES)
}

pub(super) fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    plan_failure(code, message)
}
