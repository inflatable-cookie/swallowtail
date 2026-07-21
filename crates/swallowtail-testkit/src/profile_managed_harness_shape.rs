use crate::profile_shape::ProfileShape;
use swallowtail_core::{
    CredentialMechanism, CredentialState, DriverRole, EntitlementMetering, ExecutionLayer,
    HostServiceKind, InstanceOwnership, OperationShape,
};

pub(crate) fn shape() -> ProfileShape {
    ProfileShape {
        adapter_id: "fixture.driver.managed-harness",
        integration_family: "fixture-managed-provider",
        transport_family: "https-sse-managed-harness",
        instance_id: "fixture.instance.managed-harness",
        route_id: "fixture.route.managed-harness",
        model_id: "fixture-model-managed-harness",
        access_profile_id: "fixture.access.managed-harness",
        audience: "fixture-managed-api",
        role: DriverRole::StructuredRun,
        layer: ExecutionLayer::HarnessInteraction,
        operation_shape: OperationShape::StructuredRun,
        ownership: InstanceOwnership::ExternalAttached,
        credential: CredentialMechanism::ApiKey,
        credential_state: CredentialState::Ready,
        metering: EntitlementMetering::PayAsYouGo,
        required_services: vec![
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    }
}
