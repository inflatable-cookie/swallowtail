//! Preflight-plan checks shared by the headless and response-only routes.
//!
//! Both Claude Code routes are gated by the same plan vocabulary: required
//! host services, required capabilities, and the exact capability constraints
//! that pin a request to the route it was prepared for. Only the route label
//! and the failure codes differ, so each route supplies one
//! [`PlanRoute`] descriptor and the checks themselves live here once.

use crate::failure::failure;
use swallowtail_core::{Capability, CapabilityConstraint, HostServiceKind, PreflightPlan};
use swallowtail_runtime::RuntimeFailure;

/// Route identity stamped on every plan-check failure.
pub(crate) struct PlanRoute {
    /// Human label used in failure messages.
    pub(crate) label: &'static str,
    /// Failure code for a missing preflight-bound host service.
    pub(crate) host_service_missing: &'static str,
    /// Failure code for a request that does not match its prepared plan.
    pub(crate) request_plan_mismatch: &'static str,
    /// Failure code for an input the route does not support.
    pub(crate) unsupported_input: &'static str,
}

/// Route identity for the structured headless route.
pub(crate) const HEADLESS_PLAN_ROUTE: PlanRoute = PlanRoute {
    label: "Claude Code headless",
    host_service_missing: "swallowtail.claude_code.headless.host_service_missing",
    request_plan_mismatch: "swallowtail.claude_code.headless.request_plan_mismatch",
    unsupported_input: "swallowtail.claude_code.headless.unsupported_input",
};

/// Route identity for the tool-free response-only route.
pub(crate) const RESPONSE_ONLY_PLAN_ROUTE: PlanRoute = PlanRoute {
    label: "Claude Code response-only execution",
    host_service_missing: "swallowtail.claude_code.response_only.host_service_missing",
    request_plan_mismatch: "swallowtail.claude_code.response_only.request_plan_mismatch",
    unsupported_input: "swallowtail.claude_code.response_only.unsupported_input",
};

/// Requires the plan to bind `service` and the host to provide it.
pub(crate) fn require_service(
    route: &PlanRoute,
    plan: &PreflightPlan,
    available: bool,
    service: HostServiceKind,
    name: &str,
) -> Result<(), RuntimeFailure> {
    if !binds_host_service(plan, service) {
        Err(plan_mismatch(route, name))
    } else if !available {
        Err(missing_service(route, name))
    } else {
        Ok(())
    }
}

/// Requires the host to provide `service` only when the plan binds it.
pub(crate) fn require_optional_service(
    route: &PlanRoute,
    plan: &PreflightPlan,
    available: bool,
    service: HostServiceKind,
    name: &str,
) -> Result<(), RuntimeFailure> {
    if !binds_host_service(plan, service) {
        Ok(())
    } else if !available {
        Err(missing_service(route, name))
    } else {
        Ok(())
    }
}

/// Requires the plan to request `capability`.
pub(crate) fn require_capability(
    route: &PlanRoute,
    plan: &PreflightPlan,
    capability: Capability,
) -> Result<(), RuntimeFailure> {
    if requests_capability(plan, capability) {
        Ok(())
    } else {
        Err(plan_mismatch(route, "capability"))
    }
}

/// Requires the plan to request `capability` with exactly `constraint`.
pub(crate) fn require_constraint(
    route: &PlanRoute,
    plan: &PreflightPlan,
    capability: Capability,
    constraint: CapabilityConstraint,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().capabilities().any(|required| {
        required.capability() == capability
            && required
                .constraints()
                .any(|required| required == &constraint)
    }) {
        Ok(())
    } else {
        Err(plan_mismatch(route, "capability constraint"))
    }
}

/// Rejects a plan that requests `capability` at all.
pub(crate) fn reject_capability(
    route: &PlanRoute,
    plan: &PreflightPlan,
    capability: Capability,
) -> Result<(), RuntimeFailure> {
    if requests_capability(plan, capability) {
        Err(plan_mismatch(route, "forbidden capability"))
    } else {
        Ok(())
    }
}

/// Reports a request that does not match its preflight-bound `dimension`.
pub(crate) fn plan_mismatch(route: &PlanRoute, dimension: &str) -> RuntimeFailure {
    failure(
        route.request_plan_mismatch,
        format!(
            "{} request does not match its preflight-bound {dimension}",
            route.label
        ),
    )
}

/// Reports an input the route does not support.
pub(crate) fn unsupported(route: &PlanRoute, feature: &str) -> RuntimeFailure {
    failure(
        route.unsupported_input,
        format!("{} does not support {feature}", route.label),
    )
}

fn missing_service(route: &PlanRoute, name: &str) -> RuntimeFailure {
    failure(
        route.host_service_missing,
        format!(
            "{} requires the preflight-bound {name} service",
            route.label
        ),
    )
}

fn binds_host_service(plan: &PreflightPlan, service: HostServiceKind) -> bool {
    plan.requirements()
        .host_services()
        .any(|required| required == service)
}

fn requests_capability(plan: &PreflightPlan, capability: Capability) -> bool {
    plan.requirements()
        .capabilities()
        .any(|required| required.capability() == capability)
}
