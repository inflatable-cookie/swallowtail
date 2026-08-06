use crate::ExecutionTopologyFixture;
use std::num::NonZeroU32;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, CredentialMechanism, CredentialState, DriverDescriptor, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionLayer,
    HarnessIsolation, HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision,
    IntegrationFamilyId, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
    InterfaceVersionSegment, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, ProtocolFacadeId,
    ProviderSessionActivityState, ProviderSessionCatalogueBounds, ProviderSessionDisplayContent,
    ProviderSessionImportAvailability, ResourceAccess, ResourceRepresentation, RuntimeReadiness,
    SessionAccessPolicy, SessionProviderStatePolicy, SessionRef, SupportAuthority,
    TransportFamilyId, preflight,
};
use swallowtail_runtime::{
    AccessEvidenceSourceId, Deadline, MonotonicInstant, PreparedAccessEvidence,
    PreparedProviderSessionCatalogueEvidence, PreparedProviderSessionImportEvidence,
    ProviderSessionCandidate, ProviderSessionCandidateId, ProviderSessionCatalogueAgreement,
    ProviderSessionCatalogueId, ProviderSessionCataloguePlan, ProviderSessionCatalogueScope,
    ProviderSessionImportAgreement, ProviderSessionImportPlan, RuntimeFailure,
    SessionPlanAgreement,
};

const VERSION_AXIS: &str = "fixture.provider-session-rpc";
const VERSION: &str = "1.0.0";

/// Provider-neutral catalogue/import fixture bound to one execution topology.
pub struct ProviderSessionImportFixture {
    topology: ExecutionTopologyFixture,
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    access_profile: AccessProfile,
    access_status: AccessStatus,
}

impl ProviderSessionImportFixture {
    /// Builds the fixture for a local execution topology.
    #[must_use]
    pub fn local() -> Self {
        Self::for_topology(ExecutionTopologyFixture::local())
    }

    /// Builds the fixture for an authoritative remote execution topology.
    #[must_use]
    pub fn remote_authoritative() -> Self {
        Self::for_topology(ExecutionTopologyFixture::remote_authoritative())
    }

    /// Builds the fixture for an explicit execution topology.
    #[must_use]
    pub fn for_topology(topology: ExecutionTopologyFixture) -> Self {
        let adapter_id = value(AdapterId::new, "fixture.provider-session-import");
        let access_id = value(AccessProfileId::new, "fixture.provider-session-access");
        let axis = value(InterfaceVersionAxis::new, VERSION_AXIS);
        let version = value(InterfaceVersion::new, VERSION);
        let capabilities = capabilities();
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(
                adapter_id.clone(),
                value(AdapterVersion::new, "fixture-driver-1"),
            ),
            value(IntegrationFamilyId::new, "fixture-provider-session"),
            value(TransportFamilyId::new, "fixture-provider-session-rpc"),
        )
        .with_roles([
            DriverRole::ProviderSessionCatalogue,
            DriverRole::ProviderSessionImport,
            DriverRole::InteractiveSession,
        ])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([
            OperationShape::ProviderSessionCatalogue,
            OperationShape::ProviderSessionImport,
            OperationShape::InteractiveSession,
        ])
        .with_interface_compatibility(
            InterfaceCompatibilityClaim::new(
                value(
                    InterfaceCompatibilityClaimId::new,
                    "fixture-provider-session-support",
                ),
                axis.clone(),
                InterfaceVersionScheme::Semantic,
                InterfaceNewerVersionPosture::QualifiedOnly,
                [InterfaceVersionSegment::exact(
                    version.clone(),
                    value(
                        InterfaceBehaviorRevision::new,
                        "fixture-provider-session-v1",
                    ),
                    InterfaceSupportStatus::Maintained,
                )],
                [],
            )
            .expect("fixture compatibility claim is valid"),
        );
        let instance = ConfiguredInstance::new(
            topology.configured_instance_id().clone(),
            value(InstanceRevision::new, "fixture-revision-1"),
            adapter_id,
            topology.execution_host_id().clone(),
            topology.instance_target().clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::IntegrationMaintainerSupported,
            value(ProtocolFacadeId::new, "fixture.provider-session-facade"),
            value(InstancePolicyId::new, "fixture.provider-session-policy"),
            capabilities.clone(),
        )
        .with_interface_versions([InterfaceVersionBinding::new(axis, version)]);
        let route = ModelRoute::new(
            value(ModelRouteId::new, "fixture.provider-session-route"),
            value(ModelRouteRevision::new, "fixture-route-1"),
            topology.configured_instance_id().clone(),
            value(ModelId::new, "fixture-provider-session-model"),
            capabilities,
        );
        let access_profile = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::Unauthenticated,
            EntitlementMetering::Unknown,
            value(EndpointAudience::new, "fixture-provider-session"),
            SupportAuthority::IntegrationMaintainerSupported,
        );
        let access_status = AccessStatus::new(
            access_id,
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        );
        Self {
            topology,
            driver,
            instance,
            route,
            access_profile,
            access_status,
        }
    }

    /// Builds a provider-session catalogue plan with explicit result bounds.
    pub fn catalogue_plan(
        &self,
        catalogue_id: &str,
        bounds: ProviderSessionCatalogueBounds,
    ) -> ProviderSessionCataloguePlan {
        let requirements = self
            .requirements(
                OperationShape::ProviderSessionCatalogue,
                DriverRole::ProviderSessionCatalogue,
            )
            .with_capabilities([
                CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []),
                working_resource_requirement(),
            ]);
        let preflight = self.preflight(requirements, false);
        ProviderSessionCataloguePlan::new(
            preflight,
            ProviderSessionCatalogueAgreement::new(
                value(ProviderSessionCatalogueId::new, catalogue_id),
                ProviderSessionCatalogueScope::working_resource(
                    self.topology.working_resource().clone(),
                ),
                bounds,
                Some(deadline()),
            ),
        )
        .expect("fixture catalogue plan is valid")
    }

    /// Builds a catalogue candidate bound to `plan`.
    pub fn candidate(
        &self,
        plan: &ProviderSessionCataloguePlan,
        candidate_id: &str,
        provider_session_ref: &str,
        availability: ProviderSessionImportAvailability,
    ) -> Result<ProviderSessionCandidate, RuntimeFailure> {
        ProviderSessionCandidate::new(
            plan,
            value(ProviderSessionCandidateId::new, candidate_id),
            value(SessionRef::new, provider_session_ref),
            ProviderSessionDisplayContent::new(
                Some("private provider title".to_owned()),
                Some("private provider preview".to_owned()),
            )
            .expect("fixture display content is valid"),
            Some(1_775_000_000_000),
            ProviderSessionActivityState::Inactive,
            availability,
        )
    }

    /// Builds an import plan for a candidate from the source catalogue.
    pub fn import_plan(
        &self,
        source: ProviderSessionCataloguePlan,
        candidate: ProviderSessionCandidate,
    ) -> Result<ProviderSessionImportPlan, RuntimeFailure> {
        let requirements = self
            .requirements(
                OperationShape::ProviderSessionImport,
                DriverRole::ProviderSessionImport,
            )
            .with_capabilities([
                CapabilityRequirement::new(Capability::ProviderSessionImport, []),
                CapabilityRequirement::new(Capability::LoadSession, []),
                CapabilityRequirement::new(Capability::Resume, []),
                CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
                working_resource_requirement(),
            ])
            .require_model_route()
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_session_access_policy(self.access_policy())
            .with_session_provider_state_policy(
                SessionProviderStatePolicy::DurableProviderSessionPreserved,
            );
        let preflight = self.preflight(requirements, true);
        let session = SessionPlanAgreement::from_plan(&preflight)
            .expect("fixture session agreement is valid");
        ProviderSessionImportPlan::new(
            preflight,
            source,
            ProviderSessionImportAgreement::new(
                candidate,
                self.topology.working_resource().clone(),
                session,
                Some(deadline()),
            ),
        )
    }

    /// Produces prepared catalogue evidence from a validated plan.
    pub fn prepared_catalogue(
        &self,
        plan: ProviderSessionCataloguePlan,
    ) -> PreparedProviderSessionCatalogueEvidence {
        PreparedProviderSessionCatalogueEvidence::from_plan(plan, self.access_evidence())
            .expect("fixture catalogue evidence is valid")
    }

    /// Produces prepared import evidence from a validated plan.
    pub fn prepared_import(
        &self,
        plan: ProviderSessionImportPlan,
    ) -> PreparedProviderSessionImportEvidence {
        PreparedProviderSessionImportEvidence::from_plan(plan, self.access_evidence())
            .expect("fixture import evidence is valid")
    }

    #[must_use]
    /// Returns the execution topology used by this fixture.
    pub const fn topology(&self) -> &ExecutionTopologyFixture {
        &self.topology
    }

    #[must_use]
    /// Returns the fixture driver descriptor.
    pub const fn driver(&self) -> &DriverDescriptor {
        &self.driver
    }

    #[must_use]
    /// Returns the ambient-harness access policy used for imported sessions.
    pub fn access_policy(&self) -> SessionAccessPolicy {
        SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
    }

    fn requirements(&self, shape: OperationShape, role: DriverRole) -> OperationRequirements {
        OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            shape,
            role,
            self.topology.execution_host_id().clone(),
            AccessRequirement::new(self.access_profile.id().clone())
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services([
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::WorkingResource,
        ])
        .with_interface_versions(self.instance.interface_versions().cloned())
    }

    fn preflight(
        &self,
        requirements: OperationRequirements,
        include_route: bool,
    ) -> swallowtail_core::PreflightPlan {
        let context = PreflightContext::new(
            &self.driver,
            &self.instance,
            &self.access_profile,
            &self.access_status,
            [
                HostServiceKind::Task,
                HostServiceKind::Time,
                HostServiceKind::WorkingResource,
            ],
        );
        let context = if include_route {
            context.with_model_route(&self.route)
        } else {
            context
        };
        preflight(&context, &requirements).expect("fixture preflight is valid")
    }

    fn access_evidence(&self) -> PreparedAccessEvidence {
        PreparedAccessEvidence::observed(
            self.access_status.clone(),
            value(
                AccessEvidenceSourceId::new,
                "fixture.private.provider-session-access-observation",
            ),
        )
    }
}

#[must_use]
/// Builds validated catalogue bounds from ordinary integer inputs.
pub fn provider_session_catalogue_bounds(
    page: u32,
    total: u32,
    cursor_bytes: u32,
    content_bytes: u32,
    reference_bytes: u32,
) -> ProviderSessionCatalogueBounds {
    ProviderSessionCatalogueBounds::new(
        NonZeroU32::new(page).expect("page bound is non-zero"),
        NonZeroU32::new(total).expect("total bound is non-zero"),
        NonZeroU32::new(cursor_bytes).expect("cursor bound is non-zero"),
        NonZeroU32::new(content_bytes).expect("content bound is non-zero"),
        NonZeroU32::new(reference_bytes).expect("reference bound is non-zero"),
    )
    .expect("catalogue bounds are valid")
}

fn capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []),
        CapabilityRequirement::new(Capability::ProviderSessionImport, []),
        CapabilityRequirement::new(Capability::LoadSession, []),
        CapabilityRequirement::new(Capability::Resume, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        working_resource_requirement(),
    ])
}

fn working_resource_requirement() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    )
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(100))
}

fn value<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, input: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(input.to_owned()).expect("static provider-session fixture text is valid")
}
