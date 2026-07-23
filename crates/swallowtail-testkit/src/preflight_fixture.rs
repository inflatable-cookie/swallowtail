use std::cell::Cell;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, CancellationScope, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer, ExtensionNamespace,
    HarnessIsolation, HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision,
    InstanceTargetRef, IntegrationFamilyId, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightFailure, PreflightPlan,
    ProtocolFacadeId, ReasoningMode, RuntimeReadiness, SupportAuthority, TransportFamilyId,
    preflight,
};

mod harness_configuration;

const DRIVER_ID: &str = "fixture.harness.structured-cli";
const INSTANCE_ID: &str = "fixture.instance.local";
const INSTANCE_REVISION: &str = "revision-1";
const ACCESS_PROFILE_ID: &str = "fixture.access.subscription";
const HOST_ID: &str = "fixture.host.local";
const ROUTE_ID: &str = "fixture.route.model";

macro_rules! valid_text {
    ($type:ident, $value:expr) => {
        $type::new($value).expect("static fixture text must be valid")
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreflightFixtureCase {
    Canonical,
    MissingRole,
    MissingHostService,
    MissingCapability,
    MissingConstraint,
    MissingReasoningMode,
    MissingExternalSearch,
    MissingSchemaService,
    MissingModelRoute,
    RejectedAccess,
    RejectedSupportAuthority,
    RejectedOwnership,
    WrongExecutionHost,
    MissingExtension,
    HarnessIsolationAmbient,
    DirectInferenceHarnessIsolation,
    HarnessConfigurationAmbient,
    HarnessConfigurationMismatch,
    ProviderSuppressedWithoutVersionEvidence,
    DirectInferenceHarnessConfiguration,
    HostScopedHarnessConfiguration,
}

/// Canonical pure-preflight fixture with a provider-side-effect recorder.
pub struct RuntimePreflightFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    model_route: Option<ModelRoute>,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    requirements: OperationRequirements,
    host_services: Vec<HostServiceKind>,
    provider_side_effects: Cell<usize>,
}

impl RuntimePreflightFixture {
    #[must_use]
    pub fn canonical() -> Self {
        Self::for_case(PreflightFixtureCase::Canonical)
    }

    #[must_use]
    pub fn for_case(case: PreflightFixtureCase) -> Self {
        let extension = extension_namespace();
        let constraint = CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun);
        let reasoning = reasoning_mode();
        let complete_capabilities =
            capability_profile(Some(constraint.clone()), Some(reasoning.clone()), true);

        let execution_layer = if matches!(
            case,
            PreflightFixtureCase::DirectInferenceHarnessIsolation
                | PreflightFixtureCase::DirectInferenceHarnessConfiguration
        ) {
            ExecutionLayer::DirectModelInference
        } else {
            ExecutionLayer::HarnessInteraction
        };
        let mut driver = DriverDescriptor::new(
            AdapterIdentity::new(
                adapter_id(),
                valid_text!(AdapterVersion, "fixture-version-1"),
            ),
            valid_text!(IntegrationFamilyId, "fixture-harness"),
            valid_text!(TransportFamilyId, "structured-cli"),
        )
        .with_roles([DriverRole::StructuredRun])
        .with_execution_layers([execution_layer])
        .with_operation_shapes([OperationShape::StructuredRun])
        .with_required_host_services(
            DriverRole::StructuredRun,
            [HostServiceKind::Task, HostServiceKind::Process],
        )
        .with_extension_namespaces([extension.clone()]);

        if case == PreflightFixtureCase::MissingRole {
            driver = DriverDescriptor::new(
                driver.identity().clone(),
                driver.integration_family().clone(),
                driver.transport_family().clone(),
            )
            .with_execution_layers([ExecutionLayer::HarnessInteraction])
            .with_operation_shapes([OperationShape::StructuredRun]);
        } else if case == PreflightFixtureCase::MissingExtension {
            driver = DriverDescriptor::new(
                driver.identity().clone(),
                driver.integration_family().clone(),
                driver.transport_family().clone(),
            )
            .with_roles([DriverRole::StructuredRun])
            .with_execution_layers([ExecutionLayer::HarnessInteraction])
            .with_operation_shapes([OperationShape::StructuredRun])
            .with_required_host_services(
                DriverRole::StructuredRun,
                [HostServiceKind::Task, HostServiceKind::Process],
            );
        }

        let instance_capabilities = match case {
            PreflightFixtureCase::MissingCapability => {
                CapabilityProfile::new([CapabilityRequirement::new(Capability::StructuredRun, [])])
            }
            PreflightFixtureCase::MissingConstraint => {
                capability_profile(None, Some(reasoning.clone()), true)
            }
            PreflightFixtureCase::MissingReasoningMode => {
                capability_profile(Some(constraint.clone()), None, true)
            }
            PreflightFixtureCase::MissingExternalSearch => {
                capability_profile(Some(constraint.clone()), Some(reasoning.clone()), false)
            }
            _ => complete_capabilities.clone(),
        };
        let ownership = InstanceOwnership::ExternalAttached;
        let support_authority = if case == PreflightFixtureCase::RejectedSupportAuthority {
            SupportAuthority::ExperimentalObserved
        } else {
            SupportAuthority::ProviderSupported
        };
        let mut instance = configured_instance(
            valid_text!(InstanceRevision, INSTANCE_REVISION),
            instance_capabilities,
            ownership,
            support_authority,
        );
        let instance_configuration = harness_configuration::instance_posture(case);
        if let Some(posture) = instance_configuration {
            instance = instance.with_harness_configuration_posture(posture);
        }

        let model_route = (case != PreflightFixtureCase::MissingModelRoute).then(|| {
            ModelRoute::new(
                valid_text!(ModelRouteId, ROUTE_ID),
                valid_text!(ModelRouteRevision, "route-revision-1"),
                instance.id().clone(),
                valid_text!(ModelId, "fixture-model"),
                complete_capabilities,
            )
        });

        let access_profile = AccessProfile::new(
            access_profile_id(),
            CredentialMechanism::InteractiveOauth,
            EntitlementMetering::SubscriptionAllowance,
            valid_text!(EndpointAudience, "fixture-product"),
            support_authority,
        );
        let credential = if case == PreflightFixtureCase::RejectedAccess {
            CredentialState::Required
        } else {
            CredentialState::Ready
        };
        let access_status = AccessStatus::new(
            access_profile_id(),
            credential,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            support_authority,
        );

        let required_host = if case == PreflightFixtureCase::WrongExecutionHost {
            valid_text!(ExecutionHostId, "fixture.host.remote")
        } else {
            execution_host_id()
        };
        let accepted_ownership = if case == PreflightFixtureCase::RejectedOwnership {
            InstanceOwnership::HostOwnedPersistent
        } else {
            ownership
        };
        let access_requirement = AccessRequirement::new(access_profile_id())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]);
        let requirements = OperationRequirements::new(
            execution_layer,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            required_host,
            access_requirement,
        )
        .with_ownership_modes([accepted_ownership])
        .with_host_services([HostServiceKind::Task, HostServiceKind::Schema])
        .with_capabilities([
            CapabilityRequirement::new(Capability::StructuredRun, []),
            CapabilityRequirement::new(Capability::Interruption, [constraint]),
            CapabilityRequirement::new(
                Capability::Attachments,
                [
                    CapabilityConstraint::attachment_media_type("image/png")
                        .expect("fixture media type is valid"),
                    CapabilityConstraint::AttachmentMaximumBytes(1_048_576),
                    CapabilityConstraint::AttachmentMaximumCount(4),
                ],
            ),
            CapabilityRequirement::new(
                Capability::StructuredOutput,
                [CapabilityConstraint::schema_dialect("json-schema-2020-12")
                    .expect("fixture schema dialect is valid")],
            ),
            CapabilityRequirement::new(
                Capability::ReasoningSelection,
                [CapabilityConstraint::reasoning_mode(reasoning)],
            ),
            CapabilityRequirement::new(Capability::ExternalSearch, []),
        ])
        .with_extension_namespaces([extension])
        .require_model_route();
        let requirements = if matches!(
            case,
            PreflightFixtureCase::HarnessIsolationAmbient
                | PreflightFixtureCase::DirectInferenceHarnessIsolation
        ) {
            requirements.with_harness_isolation(HarnessIsolation::AmbientHost)
        } else {
            requirements
        };
        let requirements = harness_configuration::bind_requirement(case, requirements);

        let host_services = if case == PreflightFixtureCase::MissingHostService {
            vec![HostServiceKind::Task]
        } else if case == PreflightFixtureCase::MissingSchemaService {
            vec![HostServiceKind::Task, HostServiceKind::Process]
        } else {
            vec![
                HostServiceKind::Task,
                HostServiceKind::Process,
                HostServiceKind::Schema,
            ]
        };

        Self {
            driver,
            instance,
            model_route,
            access_profile,
            access_status,
            requirements,
            host_services,
            provider_side_effects: Cell::new(0),
        }
    }

    pub fn preflight(&self) -> Result<PreflightPlan, PreflightFailure> {
        preflight(&self.context(), &self.requirements)
    }

    #[must_use]
    pub const fn driver_descriptor(&self) -> &DriverDescriptor {
        &self.driver
    }

    #[must_use]
    pub fn context(&self) -> PreflightContext<'_> {
        let context = PreflightContext::new(
            &self.driver,
            &self.instance,
            &self.access_profile,
            &self.access_status,
            self.host_services.iter().copied(),
        );
        if let Some(route) = &self.model_route {
            context.with_model_route(route)
        } else {
            context
        }
    }

    #[must_use]
    pub fn with_instance_revision(mut self, revision: &str) -> Self {
        let mut instance = configured_instance(
            valid_text!(InstanceRevision, revision),
            self.instance.capabilities().clone(),
            self.instance.ownership(),
            self.instance.support_authority(),
        );
        if let Some(posture) = self.instance.harness_configuration_posture() {
            instance = instance.with_harness_configuration_posture(posture);
        }
        self.instance = instance;
        self
    }

    /// Simulates the provider boundary. Preflight itself never calls this.
    pub fn record_provider_side_effect(&self) {
        self.provider_side_effects
            .set(self.provider_side_effects.get() + 1);
    }

    #[must_use]
    pub fn provider_side_effect_count(&self) -> usize {
        self.provider_side_effects.get()
    }
}

impl Default for RuntimePreflightFixture {
    fn default() -> Self {
        Self::canonical()
    }
}

fn capability_profile(
    interruption_constraint: Option<CapabilityConstraint>,
    reasoning_mode: Option<ReasoningMode>,
    external_search: bool,
) -> CapabilityProfile {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::Interruption, interruption_constraint),
        CapabilityRequirement::new(
            Capability::Attachments,
            [
                CapabilityConstraint::attachment_media_type("image/png")
                    .expect("fixture media type is valid"),
                CapabilityConstraint::AttachmentMaximumBytes(1_048_576),
                CapabilityConstraint::AttachmentMaximumCount(4),
            ],
        ),
        CapabilityRequirement::new(
            Capability::StructuredOutput,
            [CapabilityConstraint::schema_dialect("json-schema-2020-12")
                .expect("fixture schema dialect is valid")],
        ),
        CapabilityRequirement::new(
            Capability::ReasoningSelection,
            reasoning_mode.map(CapabilityConstraint::reasoning_mode),
        ),
    ];
    if external_search {
        capabilities.push(CapabilityRequirement::new(Capability::ExternalSearch, []));
    }
    CapabilityProfile::new(capabilities)
}

fn reasoning_mode() -> ReasoningMode {
    valid_text!(ReasoningMode, "low")
}

fn configured_instance(
    revision: InstanceRevision,
    capabilities: CapabilityProfile,
    ownership: InstanceOwnership,
    support_authority: SupportAuthority,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        valid_text!(ConfiguredInstanceId, INSTANCE_ID),
        revision,
        adapter_id(),
        execution_host_id(),
        valid_text!(InstanceTargetRef, "fixture-host-target"),
        ownership,
        access_profile_id(),
        support_authority,
        valid_text!(ProtocolFacadeId, "fixture-protocol"),
        valid_text!(InstancePolicyId, "fixture-policy"),
        capabilities,
    )
}

fn adapter_id() -> AdapterId {
    valid_text!(AdapterId, DRIVER_ID)
}

fn access_profile_id() -> AccessProfileId {
    valid_text!(AccessProfileId, ACCESS_PROFILE_ID)
}

fn execution_host_id() -> ExecutionHostId {
    valid_text!(ExecutionHostId, HOST_ID)
}

fn extension_namespace() -> ExtensionNamespace {
    valid_text!(ExtensionNamespace, "fixture.runtime/v1")
}
