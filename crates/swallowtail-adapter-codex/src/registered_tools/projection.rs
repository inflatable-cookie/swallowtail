//! Contract 061 registered-capability rows for the Codex app-server route.
//!
//! Every row is descriptive. It authorizes no dispatch, opens no lease, and
//! grants no runtime support. The rows exist so a consumer can read the exact
//! mediation kind this route proves — host-mediated dynamic native tools, with
//! provider-direct MCP withheld — instead of inferring parity from another
//! adapter.

use super::binding::{CODEX_REGISTERED_TOOL_ROUTE, CodexRegisteredToolBinding};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind, HostServices,
    RegisteredCapabilityProjectionInput, RegisteredToolReadiness, RegisteredToolRouteQualification,
    project_registered_capability,
};

/// Projects the registered capability one prepared Codex session bound.
pub(crate) fn registered_capability_contribution(
    plan: &PreflightPlan,
    binding: &CodexRegisteredToolBinding,
    services: &HostServices,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    let selection = binding.selection();
    let readiness = RegisteredToolReadiness::evaluate(services, selection);
    project_registered_capability(
        RegisteredCapabilityProjectionInput::new(
            ConsumerRouteApplicability::from_plan(plan),
            ConsumerRouteProjectionSourceIdentity::new(
                source_id,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            selection,
            &readiness,
        )
        .with_route_qualification(RegisteredToolRouteQualification::Qualified(
            CODEX_REGISTERED_TOOL_ROUTE,
        )),
    )
}
