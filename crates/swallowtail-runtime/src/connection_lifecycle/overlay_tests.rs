use super::{ModelPresentationOverlayFailureKind, apply_model_presentation_overlay};
use crate::{
    ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceRecord,
    ConfiguredProviderInstanceSelectionReadiness, ConfiguredProviderModelCatalogueInput,
    PreparedAccessEvidence, PreparedOperationEvidence,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverDescriptor,
    DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId, InstanceRevision,
    InstanceTargetRef, IntegrationFamilyId, ModelCatalogEntry, ModelId, ModelMetadata,
    OperationRequirements, OperationShape, OverlayMarker, PreflightContext, ProtocolFacadeId,
    ProviderId, RuntimeReadiness, SupportAuthority, TransportFamilyId, preflight,
};

struct Fixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl Fixture {
    fn ready(instance_id: &str) -> Self {
        Self::with_status(
            instance_id,
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        )
    }

    fn not_ready(instance_id: &str) -> Self {
        Self::with_status(
            instance_id,
            CredentialState::Required,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        )
    }

    fn with_status(
        instance_id: &str,
        credential: CredentialState,
        entitlement: EntitlementState,
        endpoint: EndpointAuthorization,
        runtime: RuntimeReadiness,
    ) -> Self {
        let adapter_id = AdapterId::new("fixture.provider").expect("adapter id");
        let host_id = ExecutionHostId::new("fixture.host").expect("host id");
        let access_id = AccessProfileId::new("fixture.access").expect("access id");
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(
                adapter_id.clone(),
                AdapterVersion::new("1").expect("adapter version"),
            ),
            IntegrationFamilyId::new("fixture-family").expect("family id"),
            TransportFamilyId::new("fixture-transport").expect("transport id"),
        )
        .with_roles([DriverRole::ModelCatalog, DriverRole::StructuredRun])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([
            OperationShape::InteractiveSession,
            OperationShape::StructuredRun,
        ]);
        let capabilities = CapabilityProfile::new([
            CapabilityRequirement::new(Capability::ModelCatalog, []),
            CapabilityRequirement::new(Capability::StructuredRun, []),
        ]);
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new(instance_id).expect("instance id"),
            InstanceRevision::new("revision-1").expect("instance revision"),
            adapter_id,
            host_id,
            InstanceTargetRef::new("private-target").expect("target"),
            InstanceOwnership::HostOwnedPersistent,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("fixture-facade").expect("facade"),
            InstancePolicyId::new("fixture-policy").expect("policy"),
            capabilities,
        );
        let access_profile = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            EndpointAudience::new("fixture-audience").expect("audience"),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(
            CredentialRef::new("private-credential").expect("credential reference"),
        );
        let access_evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access_id,
            credential,
            entitlement,
            endpoint,
            runtime,
            SupportAuthority::ProviderSupported,
        ));
        Self {
            driver,
            instance,
            access_profile,
            access_evidence,
        }
    }

    fn admission(&self) -> ConfiguredProviderInstanceAdmission {
        ConfiguredProviderInstanceAdmission::new(
            self.driver.clone(),
            self.instance.clone(),
            self.access_profile.clone(),
            self.access_evidence.clone(),
        )
    }

    fn prepared(&self) -> PreparedOperationEvidence {
        let status = self.access_evidence.status();
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::ModelCatalog,
            self.instance.execution_host_id().clone(),
            AccessRequirement::new(self.access_profile.id().clone())
                .with_credential_states([status.credential()])
                .with_entitlement_states([status.entitlement()])
                .with_endpoint_authorizations([status.endpoint_authorization()])
                .with_runtime_readiness([status.runtime_readiness()])
                .with_support_authorities([status.support_authority()]),
        )
        .with_ownership_modes([self.instance.ownership()])
        .with_capabilities([CapabilityRequirement::new(Capability::ModelCatalog, [])]);
        let plan = preflight(
            &PreflightContext::new(
                &self.driver,
                &self.instance,
                &self.access_profile,
                status,
                [],
            ),
            &requirements,
        )
        .expect("fixture preflight succeeds");
        PreparedOperationEvidence::from_plan(plan, self.access_evidence.clone())
            .expect("fixture evidence prepares")
    }

    fn model(model_id: &str, provider_id: &str, provider_default: bool) -> ModelCatalogEntry {
        ModelCatalogEntry::new(
            ModelId::new(model_id).expect("model id"),
            ModelMetadata::default().with_default(provider_default),
        )
        .with_provider_id(ProviderId::new(provider_id).expect("provider id"))
    }
}

fn ready_record() -> ConfiguredProviderInstanceRecord {
    let fixture = Fixture::ready("work");
    let source = fixture.prepared();
    ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                source,
                [
                    Fixture::model("opus", "anthropic", true),
                    Fixture::model("sonnet", "anthropic", false),
                    Fixture::model("haiku", "anthropic", false),
                ],
            )),
    )
    .expect("ready catalogue is admitted")
}

fn not_ready_record() -> ConfiguredProviderInstanceRecord {
    let fixture = Fixture::not_ready("work");
    let source = fixture.prepared();
    ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                source,
                [Fixture::model("opus", "anthropic", true)],
            )),
    )
    .expect("not-ready catalogue remains visible")
}

fn marker(instance_id: &str, model_id: &str) -> OverlayMarker {
    OverlayMarker::new(
        ConfiguredInstanceId::new(instance_id).expect("instance id"),
        ProviderId::new("anthropic").expect("provider id"),
        ModelId::new(model_id).expect("model id"),
    )
}

#[test]
fn overlay_projects_hide_ordinal_consumer_default_and_favourite() {
    let record = ready_record();
    let before = record.clone();
    let overlay = apply_model_presentation_overlay(
        &record,
        &[
            marker("work", "haiku")
                .with_ordinal(Some(0))
                .with_favourite(true),
            marker("work", "opus")
                .with_hidden(true)
                .with_ordinal(Some(1))
                .with_consumer_default(true),
        ],
    )
    .expect("matching markers apply");

    let ids: Vec<_> = overlay
        .entries()
        .map(|entry| entry.model_id().as_str())
        .collect();
    assert_eq!(ids, ["haiku", "opus", "sonnet"]);

    let haiku = overlay.entries().next().expect("haiku is first");
    assert!(!haiku.hidden());
    assert_eq!(haiku.ordinal(), Some(0));
    assert!(haiku.favourite());
    assert!(!haiku.consumer_default());
    assert!(!haiku.provider_default());

    let opus = overlay.entries().nth(1).expect("opus is second");
    assert!(opus.hidden());
    assert_eq!(opus.ordinal(), Some(1));
    assert!(opus.consumer_default());
    assert!(opus.provider_default());
    assert!(!opus.favourite());

    let sonnet = overlay.entries().nth(2).expect("unmarked sonnet remains");
    assert!(!sonnet.hidden());
    assert_eq!(sonnet.ordinal(), None);
    assert!(!sonnet.consumer_default());
    assert!(!sonnet.favourite());
    assert!(!sonnet.provider_default());

    assert_eq!(
        overlay.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    assert_eq!(overlay.instance_id(), record.instance_id());
    assert_eq!(record, before);
    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
}

#[test]
fn provider_default_stays_distinct_from_consumer_default() {
    let record = ready_record();
    let overlay = apply_model_presentation_overlay(
        &record,
        &[marker("work", "sonnet").with_consumer_default(true)],
    )
    .expect("consumer default applies");

    let opus = overlay
        .entries()
        .find(|entry| entry.model_id().as_str() == "opus")
        .expect("opus remains");
    let sonnet = overlay
        .entries()
        .find(|entry| entry.model_id().as_str() == "sonnet")
        .expect("sonnet remains");

    assert!(opus.provider_default());
    assert!(!opus.consumer_default());
    assert!(!sonnet.provider_default());
    assert!(sonnet.consumer_default());
    assert!(
        record
            .model_catalogue()
            .expect("catalogue")
            .entries()
            .next()
            .expect("opus")
            .metadata()
            .is_default()
    );
}

#[test]
fn overlay_does_not_change_ready_or_not_ready() {
    let ready = ready_record();
    let ready_overlay =
        apply_model_presentation_overlay(&ready, &[marker("work", "opus").with_hidden(true)])
            .expect("ready overlay applies");
    assert_eq!(
        ready.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    assert_eq!(
        ready_overlay.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );

    let not_ready = not_ready_record();
    let not_ready_overlay = apply_model_presentation_overlay(
        &not_ready,
        &[marker("work", "opus")
            .with_consumer_default(true)
            .with_favourite(true)],
    )
    .expect("not-ready overlay applies");
    assert_eq!(
        not_ready.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );
    assert_eq!(
        not_ready_overlay.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );
}

#[test]
fn unknown_model_ids_are_rejected() {
    let record = ready_record();
    let failure = apply_model_presentation_overlay(&record, &[marker("work", "invented")])
        .expect_err("unknown model fails closed");
    assert_eq!(
        failure.kind(),
        ModelPresentationOverlayFailureKind::UnknownModel
    );
    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
}

#[test]
fn cross_instance_markers_are_rejected() {
    let record = ready_record();
    let failure = apply_model_presentation_overlay(&record, &[marker("personal", "opus")])
        .expect_err("cross-instance marker fails closed");
    assert_eq!(
        failure.kind(),
        ModelPresentationOverlayFailureKind::CrossInstance
    );
}

#[test]
fn overlay_cannot_change_not_ready_to_ready() {
    let record = not_ready_record();
    let overlay = apply_model_presentation_overlay(
        &record,
        &[marker("work", "opus")
            .with_consumer_default(true)
            .with_favourite(true)
            .with_hidden(false)],
    )
    .expect("preference markers do not select a not-ready instance");
    assert_eq!(
        overlay.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );
    assert_ne!(
        overlay.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
}
