use super::{
    ProviderSessionHistoryAgreement, ProviderSessionHistoryPlan, ProviderSessionHistoryRequest,
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

const PLAN_MISMATCH_CODE: &str = "swallowtail.provider_session_history.plan_mismatch";
const PLAN_MISMATCH_MESSAGE: &str =
    "Provider-session history does not match its immutable binding";

const BOUND_MISMATCH_CODE: &str = "swallowtail.provider_session_history.bound_mismatch";
const BOUND_MISMATCH_MESSAGE: &str =
    "Provider-session history bounds differ from its capability plan";

const TIME_SERVICE_CODE: &str = "swallowtail.provider_session_history.time_service_required";
const TIME_SERVICE_MESSAGE: &str = "Deadline-bound provider-session history requires time service";

/// Ordered per-role validation rules for a history-page plan.
const HISTORY_PLAN_RULES: [PlanRule<ProviderSessionHistoryAgreement>; 9] = [
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, _| {
        matches!(
            preflight.requirements().execution_layer(),
            ExecutionLayer::HarnessInteraction | ExecutionLayer::DirectModelInference
        )
    }),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, _| {
        preflight.requirements().driver_role() == DriverRole::ProviderSessionHistory
    }),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, _| {
        preflight.requirements().operation_shape() == OperationShape::ProviderSessionHistory
    }),
    PlanRule::new(
        PLAN_MISMATCH_CODE,
        PLAN_MISMATCH_MESSAGE,
        |preflight, agreement| agreement.binding().matches_plan(preflight),
    ),
    PlanRule::new(
        PLAN_MISMATCH_CODE,
        PLAN_MISMATCH_MESSAGE,
        |preflight, agreement| {
            let policy = preflight.requirements().session_access_policy();
            let working_resource = preflight
                .requirements()
                .host_services()
                .any(|required| required == HostServiceKind::WorkingResource);
            if agreement.binding().is_resource_free() {
                policy == Some(&SessionAccessPolicy::resource_free())
                    && agreement.binding().access_policy() == &SessionAccessPolicy::resource_free()
                    && !working_resource
            } else {
                policy == Some(&SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
                    && agreement.binding().access_policy()
                        == &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
                    && working_resource
            }
        },
    ),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, _| {
        preflight
            .requirements()
            .capabilities()
            .any(|required| required.capability() == Capability::ProviderSessionHistory)
    }),
    PlanRule::new(PLAN_MISMATCH_CODE, PLAN_MISMATCH_MESSAGE, |preflight, agreement| {
        let declares_resource = preflight
            .requirements()
            .capabilities()
            .any(|required| required.capability() == Capability::WorkingResource);
        if agreement.binding().is_resource_free() {
            !declares_resource
        } else {
            declares_resource
        }
    }),
    PlanRule::new(
        BOUND_MISMATCH_CODE,
        BOUND_MISMATCH_MESSAGE,
        |preflight, agreement| {
            let bounds = agreement.bounds();
            if bounds.maximum_snapshot_items().get() < bounds.maximum_page_items().get() {
                return false;
            }
            let capability = preflight
                .requirements()
                .capabilities()
                .find(|required| required.capability() == Capability::ProviderSessionHistory)
                .expect("checked capability");
            let expected = [
                CapabilityConstraint::ReplayMaximumItems(bounds.maximum_page_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_page_bytes().get()),
            ];
            expected
                .iter()
                .all(|constraint| capability.constraints().any(|actual| actual == constraint))
                && capability.constraints().count() == expected.len()
        },
    ),
    PlanRule::new(
        TIME_SERVICE_CODE,
        TIME_SERVICE_MESSAGE,
        |preflight, agreement| {
            agreement.deadline().is_none()
                || preflight
                    .requirements()
                    .host_services()
                    .any(|required| required == HostServiceKind::Time)
        },
    ),
];

/// Verifies that execution input still matches its immutable plan.
pub fn validate_provider_session_history_request(
    plan: &ProviderSessionHistoryPlan,
    request: &ProviderSessionHistoryRequest,
) -> Result<(), RuntimeFailure> {
    validate_agreement_matches_plan(
        plan.agreement(),
        request.agreement(),
        PLAN_MISMATCH_CODE,
        "Provider-session history request does not match its immutable plan",
    )
}

/// Verifies request, execution host, and required host-service availability.
pub fn validate_provider_session_history_execution(
    plan: &ProviderSessionHistoryPlan,
    request: &ProviderSessionHistoryRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_provider_session_history_request(plan, request)?;
    validate_execution_services(
        plan.preflight(),
        services,
        "swallowtail.provider_session_history.service_unavailable",
        "Provider-session history host services are unavailable",
    )
}

pub(super) fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderSessionHistoryAgreement,
) -> Result<(), RuntimeFailure> {
    check_plan_rules(preflight, agreement, &HISTORY_PLAN_RULES)
}

pub(super) fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    plan_failure(code, message)
}
