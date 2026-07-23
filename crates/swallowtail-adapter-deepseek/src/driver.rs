mod access;
mod catalogue;
mod history;
mod lifecycle;
mod session;
mod turn;

use crate::failure::failure;
use crate::selection::{DEEPSEEK_AUDIENCE, DEEPSEEK_PROVIDER_ID, deepseek_facade_claim};
use crate::transport::CurlTransport;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, InstanceOwnership, IntegrationFamilyId, OperationShape,
    PreflightPlan, TransportFamilyId,
};
use swallowtail_runtime::RuntimeFailure;

pub(crate) const DRIVER_ID: &str = "swallowtail.deepseek.direct";

#[derive(Clone, Default)]
pub struct DeepSeekDirectDriver {
    transport: CurlTransport,
}

impl DeepSeekDirectDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID
            || plan.requirements().execution_layer() != ExecutionLayer::DirectModelInference
            || plan.ownership() != InstanceOwnership::ExternalAttached
            || plan
                .provider_id()
                .is_some_and(|provider| provider.as_str() != DEEPSEEK_PROVIDER_ID)
        {
            return Err(failure(
                "swallowtail.deepseek.plan_binding_rejected",
                "Preflight plan is not bound to the exact DeepSeek direct driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference().is_none()
            || plan.endpoint_audience().as_str() != DEEPSEEK_AUDIENCE
        {
            return Err(failure(
                "swallowtail.deepseek.access_binding_rejected",
                "DeepSeek requires the public Open Platform API-key access profile",
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn deepseek_direct_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new(DEEPSEEK_PROVIDER_ID).expect("static family id is valid"),
        TransportFamilyId::new("openai-chat-http-sse").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::ModelCatalog, DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
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
    .with_interface_compatibility(deepseek_facade_claim())
}
