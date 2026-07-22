use crate::profile_shape::ProfileShape;
use swallowtail_core::{
    CredentialMechanism, CredentialState, DriverRole, EntitlementMetering, ExecutionLayer,
    HostServiceKind, InstanceOwnership, OperationShape,
};

pub(crate) fn shape() -> ProfileShape {
    ProfileShape {
        adapter_id: "fixture.driver.realtime-media",
        integration_family: "fixture-provider",
        transport_family: "websocket-realtime-media",
        instance_id: "fixture.instance.realtime-media",
        route_id: "fixture.route.realtime-media",
        model_id: "fixture-realtime-model",
        access_profile_id: "fixture.access.realtime-media",
        audience: "fixture-realtime-api",
        role: DriverRole::RealtimeMediaSession,
        layer: ExecutionLayer::DirectModelInference,
        operation_shape: OperationShape::InteractiveSession,
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
