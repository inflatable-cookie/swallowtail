mod access;
mod lifecycle;
mod session;
mod turn;

use crate::failure::failure;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan,
    TransportFamilyId,
};

pub(super) const DRIVER_ID: &str = "swallowtail.xai.websocket";
pub(super) const PROVIDER_ID: &str = "xai";

#[derive(Clone, Default)]
pub struct XaiWebSocketDriver;

impl XaiWebSocketDriver {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn validate_plan(plan: &PreflightPlan) -> Result<(), swallowtail_runtime::RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.xai.plan_driver_mismatch",
                "xAI WebSocket preflight plan belongs to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference().is_none()
        {
            return Err(failure(
                "swallowtail.xai.credential_binding_rejected",
                "xAI WebSocket requires a bound public API-key credential",
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn xai_websocket_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new(PROVIDER_ID).expect("static family id is valid"),
        TransportFamilyId::new("responses-websocket").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
}
