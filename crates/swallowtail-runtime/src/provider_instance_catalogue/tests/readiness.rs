use super::super::{
    ConfiguredProviderInstanceRecord, ConfiguredProviderInstanceSelectionReadiness,
    ConfiguredProviderModelCatalogueInput, ConfiguredProviderModelCatalogueState,
};
use super::Fixture;
use swallowtail_core::{
    CredentialMechanism, CredentialState, DriverRole, EndpointAuthorization, EntitlementState,
    InstanceLabel, ProviderId, RuntimeReadiness, SafeDiagnostic,
};

#[test]
fn exact_available_evidence_produces_a_ready_portable_record() {
    let fixture = Fixture::ready("fixture.instance");
    let source = fixture.prepared(DriverRole::ModelCatalog);
    let record = ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                source,
                [fixture.model("model-a", Some("provider-a"))],
            )),
    )
    .expect("exact evidence is admitted");

    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    assert_eq!(record.instance_id(), fixture.instance.id());
    assert_eq!(
        record.protocol_facade_id(),
        fixture.instance.protocol_facade_id()
    );
    assert_eq!(record.driver_identity(), fixture.driver.identity());
    assert_eq!(record.routes().len(), 1);
    let catalogue = record.model_catalogue().expect("catalogue is retained");
    assert_eq!(
        catalogue.state(),
        ConfiguredProviderModelCatalogueState::Available
    );
    let model = catalogue.entries().next().expect("model is retained");
    assert_eq!(model.id().as_str(), "model-a");
    assert_eq!(
        model.provider_id().map(ProviderId::as_str),
        Some("provider-a")
    );
}

#[test]
fn optional_instance_label_is_projected_without_changing_readiness() {
    let ready_fixture = Fixture::ready("fixture.label-ready");
    let ready_source = ready_fixture.prepared(DriverRole::ModelCatalog);
    let ready = ConfiguredProviderInstanceRecord::admit(
        ready_fixture
            .admission()
            .with_label(InstanceLabel::new("Work").expect("label is valid"))
            .with_prepared_routes([ready_source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                ready_source,
                [ready_fixture.model("model-a", None)],
            )),
    )
    .expect("labelled ready record is admitted");
    assert_eq!(ready.label().map(InstanceLabel::as_str), Some("Work"));
    assert_eq!(
        ready.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );

    let not_ready_fixture = Fixture::with_status(
        "fixture.label-not-ready",
        CredentialState::Expired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
    );
    let not_ready_source = not_ready_fixture.prepared(DriverRole::ModelCatalog);
    let not_ready = ConfiguredProviderInstanceRecord::admit(
        not_ready_fixture
            .admission()
            .with_label(InstanceLabel::new("Expired").expect("label is valid"))
            .with_prepared_routes([not_ready_source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                not_ready_source,
                [not_ready_fixture.model("model-a", None)],
            )),
    )
    .expect("labelled not-ready record is admitted");
    assert_eq!(
        not_ready.label().map(InstanceLabel::as_str),
        Some("Expired")
    );
    assert_eq!(
        not_ready.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );
}

#[test]
fn unavailable_empty_and_absent_catalogues_remain_visible_but_not_ready() {
    let fixture = Fixture::with_status(
        "fixture.expired",
        CredentialState::Expired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
    );
    let source = fixture.prepared(DriverRole::ModelCatalog);
    let unavailable = ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::unavailable(
                source,
                SafeDiagnostic::new("fixture.catalogue.unavailable", "Catalogue unavailable"),
            )),
    )
    .expect("unavailable evidence remains admissible");
    assert_eq!(
        unavailable.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );
    assert_eq!(
        unavailable.credential_posture().credential_state(),
        CredentialState::Expired
    );
    let catalogue = unavailable
        .model_catalogue()
        .expect("failure remains visible");
    assert_eq!(
        catalogue.state(),
        ConfiguredProviderModelCatalogueState::Unavailable
    );
    assert_eq!(catalogue.entries().len(), 0);
    assert_eq!(
        catalogue
            .unavailable_diagnostic()
            .expect("diagnostic")
            .code(),
        "fixture.catalogue.unavailable"
    );

    let ready_fixture = Fixture::ready("fixture.empty");
    let empty_source = ready_fixture.prepared(DriverRole::ModelCatalog);
    let empty = ConfiguredProviderInstanceRecord::admit(
        ready_fixture
            .admission()
            .with_prepared_routes([empty_source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                empty_source,
                [],
            )),
    )
    .expect("empty catalogue remains visible");
    let absent = ConfiguredProviderInstanceRecord::admit(ready_fixture.admission())
        .expect("unprepared instance remains visible");
    assert_eq!(
        empty.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );
    assert_eq!(
        absent.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );
    assert!(absent.model_catalogue().is_none());
}

#[test]
fn every_non_positive_access_dimension_is_non_selectable() {
    let cases = [
        (
            CredentialState::Unknown,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        ),
        (
            CredentialState::Required,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        ),
        (
            CredentialState::Rejected,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        ),
        (
            CredentialState::Ready,
            EntitlementState::Unknown,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        ),
        (
            CredentialState::Ready,
            EntitlementState::Unavailable,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        ),
        (
            CredentialState::Ready,
            EntitlementState::Exhausted,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        ),
        (
            CredentialState::Ready,
            EntitlementState::Restricted,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
        ),
        (
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Unknown,
            RuntimeReadiness::Ready,
        ),
        (
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Denied,
            RuntimeReadiness::Ready,
        ),
        (
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Unknown,
        ),
        (
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Degraded,
        ),
        (
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Unavailable,
        ),
    ];
    for (index, (credential, entitlement, endpoint, runtime)) in cases.into_iter().enumerate() {
        let fixture = Fixture::with_status(
            &format!("fixture.not-ready-{index}"),
            credential,
            entitlement,
            endpoint,
            runtime,
        );
        let source = fixture.prepared(DriverRole::ModelCatalog);
        let record = ConfiguredProviderInstanceRecord::admit(
            fixture
                .admission()
                .with_prepared_routes([source.clone()])
                .with_model_catalogue(fixture.available_model_catalogue(source)),
        )
        .expect("negative access evidence remains observable");
        assert_eq!(
            record.selection_readiness(),
            ConfiguredProviderInstanceSelectionReadiness::NotReady,
            "case {index} must not become selectable"
        );
    }
}

#[test]
fn projection_drops_credential_and_target_authority() {
    let fixture = Fixture::ready("fixture.redacted");
    let source = fixture.prepared(DriverRole::ModelCatalog);
    let record = ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone()])
            .with_model_catalogue(fixture.available_model_catalogue(source)),
    )
    .expect("record is admitted");
    let debug = format!("{record:?}");
    assert!(!debug.contains("private-credential"));
    assert!(!debug.contains("private-target"));
    assert_eq!(
        record.credential_posture().credential_mechanism(),
        &CredentialMechanism::ApiKey
    );
}
