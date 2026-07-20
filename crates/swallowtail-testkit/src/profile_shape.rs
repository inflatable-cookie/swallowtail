use swallowtail_core::{
    CredentialMechanism, CredentialState, DriverRole, EntitlementMetering, ExecutionLayer,
    HostServiceKind, InstanceOwnership, OperationShape,
};

use crate::SyntheticProfile;

pub(crate) struct ProfileShape {
    pub(crate) adapter_id: &'static str,
    pub(crate) integration_family: &'static str,
    pub(crate) transport_family: &'static str,
    pub(crate) instance_id: &'static str,
    pub(crate) route_id: &'static str,
    pub(crate) model_id: &'static str,
    pub(crate) access_profile_id: &'static str,
    pub(crate) audience: &'static str,
    pub(crate) role: DriverRole,
    pub(crate) layer: ExecutionLayer,
    pub(crate) operation_shape: OperationShape,
    pub(crate) ownership: InstanceOwnership,
    pub(crate) credential: CredentialMechanism,
    pub(crate) credential_state: CredentialState,
    pub(crate) metering: EntitlementMetering,
    pub(crate) required_services: Vec<HostServiceKind>,
}

impl ProfileShape {
    pub(crate) fn for_profile(profile: SyntheticProfile) -> Self {
        match profile {
            SyntheticProfile::OneShotStructuredCli => Self::one_shot(),
            SyntheticProfile::LongLivedRpcHarness => Self::rpc(),
            SyntheticProfile::HostedDirectApi => Self::hosted(),
            SyntheticProfile::AttachedSelfHosted => Self::self_hosted(
                "fixture.driver.attached",
                "fixture.instance.attached",
                "fixture.route.attached",
                "fixture.access.attached",
                InstanceOwnership::ExternalAttached,
            ),
            SyntheticProfile::OwnedSelfHosted => Self::self_hosted(
                "fixture.driver.owned",
                "fixture.instance.owned",
                "fixture.route.owned",
                "fixture.access.owned",
                InstanceOwnership::HostOwnedEphemeral,
            ),
        }
    }

    fn one_shot() -> Self {
        Self {
            adapter_id: "fixture.driver.one-shot",
            integration_family: "fixture-harness",
            transport_family: "structured-cli",
            instance_id: "fixture.instance.one-shot",
            route_id: "fixture.route.one-shot",
            model_id: "fixture-model-cli",
            access_profile_id: "fixture.access.one-shot",
            audience: "fixture-cli",
            role: DriverRole::StructuredRun,
            layer: ExecutionLayer::HarnessInteraction,
            operation_shape: OperationShape::StructuredRun,
            ownership: InstanceOwnership::ExternalAttached,
            credential: CredentialMechanism::LocalUnauthenticated,
            credential_state: CredentialState::NotRequired,
            metering: EntitlementMetering::SubscriptionAllowance,
            required_services: vec![
                HostServiceKind::Task,
                HostServiceKind::Time,
                HostServiceKind::Process,
            ],
        }
    }

    fn rpc() -> Self {
        Self {
            adapter_id: "fixture.driver.rpc",
            integration_family: "fixture-harness",
            transport_family: "rpc",
            instance_id: "fixture.instance.rpc",
            route_id: "fixture.route.rpc",
            model_id: "fixture-model-rpc",
            access_profile_id: "fixture.access.rpc",
            audience: "fixture-rpc",
            role: DriverRole::InteractiveSession,
            layer: ExecutionLayer::HarnessInteraction,
            operation_shape: OperationShape::InteractiveSession,
            ownership: InstanceOwnership::ExternalAttached,
            credential: CredentialMechanism::LocalUnauthenticated,
            credential_state: CredentialState::NotRequired,
            metering: EntitlementMetering::SubscriptionAllowance,
            required_services: vec![
                HostServiceKind::Task,
                HostServiceKind::Time,
                HostServiceKind::Process,
            ],
        }
    }

    fn hosted() -> Self {
        Self {
            adapter_id: "fixture.driver.hosted-api",
            integration_family: "fixture-provider",
            transport_family: "http-api",
            instance_id: "fixture.instance.hosted-api",
            route_id: "fixture.route.hosted-api",
            model_id: "fixture-model-hosted",
            access_profile_id: "fixture.access.hosted-api",
            audience: "fixture-public-api",
            role: DriverRole::StructuredRun,
            layer: ExecutionLayer::DirectModelInference,
            operation_shape: OperationShape::StructuredRun,
            ownership: InstanceOwnership::ExternalAttached,
            credential: CredentialMechanism::ApiKey,
            credential_state: CredentialState::Ready,
            metering: EntitlementMetering::PayAsYouGo,
            required_services: vec![
                HostServiceKind::Task,
                HostServiceKind::Time,
                HostServiceKind::Network,
                HostServiceKind::Credential,
            ],
        }
    }

    fn self_hosted(
        adapter_id: &'static str,
        instance_id: &'static str,
        route_id: &'static str,
        access_profile_id: &'static str,
        ownership: InstanceOwnership,
    ) -> Self {
        Self {
            adapter_id,
            integration_family: "fixture-local-runtime",
            transport_family: "local-service",
            instance_id,
            route_id,
            model_id: "fixture-model-local",
            access_profile_id,
            audience: "fixture-local-runtime",
            role: DriverRole::ServingInstanceLifecycle,
            layer: ExecutionLayer::DirectModelInference,
            operation_shape: OperationShape::StructuredRun,
            ownership,
            credential: CredentialMechanism::LocalUnauthenticated,
            credential_state: CredentialState::NotRequired,
            metering: EntitlementMetering::LocalCompute,
            required_services: vec![HostServiceKind::Task, HostServiceKind::Time],
        }
    }
}
