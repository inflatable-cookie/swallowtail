use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use crate::{
    ConfiguredProviderInstanceRecord, ConfiguredProviderModelRoute, PreparedOperationEvidence,
};
use swallowtail_core::PreflightPlan;

/// Requires the configured record and prepared evidence to describe one snapshot.
///
/// The comparison is exact and fails closed. Identity, instance policy, and
/// every shared access and readiness dimension must agree before any row is
/// merged; a conservative readiness summary is never a substitute.
pub(super) fn require_record_agreement(
    record: &ConfiguredProviderInstanceRecord,
    evidence: &PreparedOperationEvidence,
) -> Result<(), ConsumerRouteProjectionFailure> {
    let binding = evidence.binding();
    let plan = evidence.plan();
    if record.instance_id() != binding.instance_id()
        || record.instance_revision() != binding.instance_revision()
        || record.driver_identity() != binding.driver_identity()
        || record.protocol_facade_id() != binding.protocol_facade_id()
        || record.execution_host_id() != binding.execution_host_id()
        || record.transport_family() != binding.transport_family()
        || record.instance_policy_id() != plan.instance_policy_id()
    {
        return Err(snapshot_disagreement());
    }
    require_access_agreement(record, plan)?;
    if evidence.access().status() != plan.access_status() {
        return Err(snapshot_disagreement());
    }
    if matched_route(record, binding, plan) {
        Ok(())
    } else {
        Err(snapshot_disagreement())
    }
}

/// Requires the record and plan to agree on every shared access dimension.
fn require_access_agreement(
    record: &ConfiguredProviderInstanceRecord,
    plan: &PreflightPlan,
) -> Result<(), ConsumerRouteProjectionFailure> {
    let posture = record.credential_posture();
    let status = plan.access_status();
    if posture.profile_id() != plan.access_profile_id()
        || posture.credential_mechanism() != plan.credential_mechanism()
        || posture.endpoint_audience() != plan.endpoint_audience()
        || posture.credential_state() != status.credential()
        || posture.entitlement_state() != status.entitlement()
        || posture.endpoint_authorization() != status.endpoint_authorization()
        || posture.runtime_readiness() != status.runtime_readiness()
        || posture.support_authority() != status.support_authority()
    {
        return Err(snapshot_disagreement());
    }
    Ok(())
}

fn matched_route(
    record: &ConfiguredProviderInstanceRecord,
    binding: &crate::PreparedOperationBinding,
    plan: &PreflightPlan,
) -> bool {
    record.routes().any(|route| {
        route.driver_role() == binding.driver_role()
            && route.execution_layer() == binding.execution_layer()
            && route.operation_shape() == binding.operation_shape()
            && route
                .model_route()
                .map(ConfiguredProviderModelRoute::route_id)
                == plan.model_route_id()
            && route
                .model_route()
                .map(ConfiguredProviderModelRoute::route_revision)
                == plan.model_route_revision()
            && route
                .model_route()
                .map(ConfiguredProviderModelRoute::model_id)
                == plan.model_id()
    })
}

pub(super) fn snapshot_disagreement() -> ConsumerRouteProjectionFailure {
    failure(
        ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
        "swallowtail.consumer_route_projection.snapshot_identity_rejected",
        "Configured record, prepared evidence, and contributions do not describe one snapshot",
    )
}
