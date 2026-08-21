use std::collections::BTreeMap;
use std::sync::Mutex;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, AdmittedInstanceRecord, Capability, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, IntegrationFamilyId, ModelCatalogEntry, ModelId,
    ModelMetadata, OperationRequirements, OperationShape, OverlayMarker, PreflightContext,
    ProtocolFacadeId, ProviderId, RuntimeReadiness, SupportAuthority, TransportFamilyId, preflight,
};
use swallowtail_runtime::{
    ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceRecord,
    ConfiguredProviderInstanceSelectionReadiness, ConfiguredProviderModelCatalogueInput,
    ConnectionLifecycleStore, ConnectionLifecycleStoreFailure, ModelPresentationOverlayFailureKind,
    PreparedAccessEvidence, PreparedOperationEvidence, apply_model_presentation_overlay,
    apply_stored_model_presentation_overlay,
};

struct Fixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl Fixture {
    fn with_status(instance_id: &str, credential: CredentialState) -> Self {
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
        .with_roles([DriverRole::ModelCatalog])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([OperationShape::InteractiveSession]);
        let capabilities =
            CapabilityProfile::new([CapabilityRequirement::new(Capability::ModelCatalog, [])]);
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
        );
        let access_evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access_id,
            credential,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
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
}

fn admit(
    instance_id: &str,
    credential: CredentialState,
    entries: impl IntoIterator<Item = ModelCatalogEntry>,
) -> ConfiguredProviderInstanceRecord {
    let fixture = Fixture::with_status(instance_id, credential);
    let source = fixture.prepared();
    ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                source, entries,
            )),
    )
    .expect("fixture catalogue is admitted")
}

fn model(model_id: &str, provider_id: Option<&str>) -> ModelCatalogEntry {
    let entry = ModelCatalogEntry::new(
        ModelId::new(model_id).expect("model id"),
        ModelMetadata::default(),
    );
    match provider_id {
        Some(provider_id) => {
            entry.with_provider_id(ProviderId::new(provider_id).expect("provider id"))
        }
        None => entry,
    }
}

fn marker(instance_id: &str, model_id: &str) -> OverlayMarker {
    OverlayMarker::new(
        ConfiguredInstanceId::new(instance_id).expect("instance id"),
        ProviderId::new("anthropic").expect("provider id"),
        ModelId::new(model_id).expect("model id"),
    )
}

fn unmarked_marker(instance_id: &str, model_id: &str) -> OverlayMarker {
    OverlayMarker::without_provider(
        ConfiguredInstanceId::new(instance_id).expect("instance id"),
        ModelId::new(model_id).expect("model id"),
    )
}

type OverlayKey = (String, Option<String>, String);

struct MemoryStore {
    overlays: Mutex<BTreeMap<OverlayKey, OverlayMarker>>,
}

impl MemoryStore {
    fn new() -> Self {
        Self {
            overlays: Mutex::new(BTreeMap::new()),
        }
    }
}

impl ConnectionLifecycleStore for MemoryStore {
    fn put_instance(
        &self,
        _record: AdmittedInstanceRecord,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        Ok(())
    }

    fn get_instance(
        &self,
        _id: &ConfiguredInstanceId,
    ) -> Result<Option<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(None)
    }

    fn list_instances(
        &self,
    ) -> Result<Vec<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(Vec::new())
    }

    fn put_overlay_marker(
        &self,
        marker: OverlayMarker,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        self.overlays.lock().expect("store lock poisoned").insert(
            (
                marker.instance_id().as_str().to_owned(),
                marker
                    .provider_id()
                    .map(|provider| provider.as_str().to_owned()),
                marker.model_id().as_str().to_owned(),
            ),
            marker,
        );
        Ok(())
    }

    fn list_overlay_markers(&self) -> Result<Vec<OverlayMarker>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .overlays
            .lock()
            .expect("store lock poisoned")
            .values()
            .cloned()
            .collect())
    }
}

#[test]
fn stored_overlay_filters_other_instances_and_keeps_readiness() {
    let record = admit(
        "work",
        CredentialState::Ready,
        [model("opus", Some("anthropic"))],
    );
    let store = MemoryStore::new();
    store
        .put_overlay_marker(marker("work", "opus").with_favourite(true))
        .expect("put work marker");
    store
        .put_overlay_marker(marker("personal", "opus").with_hidden(true))
        .expect("put personal marker");

    let overlay = apply_stored_model_presentation_overlay(&store, &record)
        .expect("stored overlay filters by instance");
    let opus = overlay.entries().next().expect("opus remains");
    assert!(opus.favourite());
    assert!(!opus.hidden());
    assert_eq!(
        overlay.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
}

#[test]
fn unfiltered_cross_instance_markers_fail_closed() {
    let record = admit(
        "work",
        CredentialState::Ready,
        [model("opus", Some("anthropic"))],
    );
    let failure = apply_model_presentation_overlay(&record, &[marker("personal", "opus")])
        .expect_err("unfiltered store list fails closed");
    assert_eq!(
        failure.kind(),
        ModelPresentationOverlayFailureKind::CrossInstance
    );
}

#[test]
fn overlay_cannot_mark_not_ready_selectable_or_invent_a_model() {
    let not_ready = admit(
        "work",
        CredentialState::Required,
        [model("opus", Some("anthropic"))],
    );
    let overlay = apply_model_presentation_overlay(
        &not_ready,
        &[marker("work", "opus").with_consumer_default(true)],
    )
    .expect("preference markers do not select a not-ready instance");
    assert_eq!(
        overlay.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );

    let ready = admit(
        "work",
        CredentialState::Ready,
        [model("opus", Some("anthropic"))],
    );
    let unknown = apply_model_presentation_overlay(&ready, &[marker("work", "invented")])
        .expect_err("unknown model fails closed");
    assert_eq!(
        unknown.kind(),
        ModelPresentationOverlayFailureKind::UnknownModel
    );
}

#[test]
fn catalogue_rows_without_provider_id_key_instance_and_model() {
    let record = admit("work", CredentialState::Ready, [model("opus", None)]);
    let overlay = apply_model_presentation_overlay(
        &record,
        &[unmarked_marker("work", "opus").with_favourite(true)],
    )
    .expect("instance-plus-model marker applies");
    let entry = overlay.entries().next().expect("opus");
    assert_eq!(entry.provider_id(), None);
    assert!(entry.favourite());

    let failure = apply_model_presentation_overlay(&record, &[marker("work", "opus")])
        .expect_err("marker cannot invent a catalogue provider id");
    assert_eq!(
        failure.kind(),
        ModelPresentationOverlayFailureKind::UnknownModel
    );
}
