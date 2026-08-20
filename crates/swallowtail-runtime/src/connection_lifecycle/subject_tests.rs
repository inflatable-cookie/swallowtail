use super::{
    ConnectionLifecycleStore, ConnectionLifecycleStoreFailure, SubjectObservationFailureKind,
    observe_authenticated_subject,
};
use crate::ConfiguredProviderInstanceSelectionReadiness;
use std::collections::BTreeMap;
use std::sync::Mutex;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, AddableRouteId, AdmittedInstanceRecord,
    AuthenticatedSubjectObservation, ConfiguredInstanceId, IntegrationFamilyId, OverlayMarker,
    RouteTopology, SubjectDisclosure,
};

struct MemoryStore {
    instances: Mutex<BTreeMap<String, AdmittedInstanceRecord>>,
}

impl MemoryStore {
    fn new() -> Self {
        Self {
            instances: Mutex::new(BTreeMap::new()),
        }
    }
}

impl ConnectionLifecycleStore for MemoryStore {
    fn put_instance(
        &self,
        record: AdmittedInstanceRecord,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        self.instances
            .lock()
            .expect("store lock poisoned")
            .insert(record.id().as_str().to_owned(), record);
        Ok(())
    }

    fn get_instance(
        &self,
        id: &ConfiguredInstanceId,
    ) -> Result<Option<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .instances
            .lock()
            .expect("store lock poisoned")
            .get(id.as_str())
            .cloned())
    }

    fn list_instances(
        &self,
    ) -> Result<Vec<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .instances
            .lock()
            .expect("store lock poisoned")
            .values()
            .cloned()
            .collect())
    }

    fn put_overlay_marker(
        &self,
        _marker: OverlayMarker,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        Ok(())
    }

    fn list_overlay_markers(&self) -> Result<Vec<OverlayMarker>, ConnectionLifecycleStoreFailure> {
        Ok(Vec::new())
    }
}

fn instance(id: &str) -> AdmittedInstanceRecord {
    AdmittedInstanceRecord::new(
        ConfiguredInstanceId::new(id).expect("instance id is valid"),
        IntegrationFamilyId::new("fixture-family").expect("family is valid"),
        AddableRouteId::new("fixture-hosted-messages").expect("route id is valid"),
        AdapterIdentity::new(
            AdapterId::new("swallowtail-adapter-fixture-hosted").expect("adapter id is valid"),
            AdapterVersion::new("0.0.0").expect("adapter version is valid"),
        ),
        RouteTopology::Hosted,
    )
}

#[test]
fn default_subject_observation_is_not_revealed() {
    let store = MemoryStore::new();
    let record = instance("work");
    store.put_instance(record.clone()).expect("put instance");
    let snapshot = ConfiguredProviderInstanceSelectionReadiness::Ready;
    let reported = AuthenticatedSubjectObservation::undisclosed()
        .with_email_disclosed()
        .with_login_absent()
        .reveal_plan("pro")
        .expect("plan is valid")
        .reveal_email("user@example.com")
        .expect("email is valid");

    let observation = observe_authenticated_subject(&store, record.id(), reported)
        .expect("subject observation succeeds");

    assert_eq!(observation.email(), &SubjectDisclosure::Redacted);
    assert_eq!(observation.login(), &SubjectDisclosure::Absent);
    assert_eq!(observation.plan(), &SubjectDisclosure::Redacted);
    assert!(observation.is_redacted());
    assert!(!format!("{observation:?}").contains("user@example.com"));
    assert!(!format!("{observation:?}").contains("pro"));
    assert_eq!(
        store
            .get_instance(record.id())
            .expect("get instance")
            .expect("instance exists"),
        record
    );
    assert_eq!(
        snapshot,
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    let stored_debug = format!("{record:?}");
    assert!(!stored_debug.contains("user@example.com"));
    assert!(!stored_debug.contains("email"));
}

#[test]
fn revealed_email_stays_out_of_debug_and_failure_diagnostics() {
    let store = MemoryStore::new();
    let email = "user@example.com";
    let revealed = AuthenticatedSubjectObservation::redacted()
        .reveal_email(email)
        .expect("email is valid");

    assert!(!format!("{revealed:?}").contains(email));

    let error = observe_authenticated_subject(
        &store,
        &ConfiguredInstanceId::new("missing").expect("instance id is valid"),
        revealed,
    )
    .expect_err("absent instance must fail");

    assert_eq!(error.kind(), SubjectObservationFailureKind::InstanceAbsent);
    assert!(!format!("{error:?}").contains(email));
    assert!(!error.diagnostic().message().contains(email));
}

#[test]
fn forty_seven_types_have_no_subject_fields() {
    let readiness = ConfiguredProviderInstanceSelectionReadiness::Ready;
    let debug = format!("{readiness:?}");

    assert!(!debug.contains("email"));
    assert!(!debug.contains("login"));
    assert!(!debug.contains("plan"));
    assert_eq!(
        readiness,
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
}
