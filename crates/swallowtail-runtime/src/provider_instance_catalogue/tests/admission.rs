use super::super::{
    ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceCatalogueFailureKind,
    ConfiguredProviderInstanceRecord, ConfiguredProviderModelCatalogueInput,
};
use super::Fixture;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AdapterId, AdapterIdentity, AdapterVersion,
    CredentialMechanism, DriverDescriptor, DriverRole, EndpointAudience, EntitlementMetering,
    IntegrationFamilyId, SupportAuthority, TransportFamilyId,
};

#[test]
fn cross_instance_route_and_model_sources_fail_closed() {
    let fixture = Fixture::ready("fixture.instance-a");
    let foreign = Fixture::ready("fixture.instance-b");
    let foreign_route = foreign.prepared(DriverRole::ModelCatalog);
    let route_error = ConfiguredProviderInstanceRecord::admit(
        fixture.admission().with_prepared_routes([foreign_route]),
    )
    .expect_err("a foreign route must fail");
    assert_eq!(
        route_error.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::RouteMismatch
    );

    let source = fixture.prepared(DriverRole::ModelCatalog);
    let source_error = ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_model_catalogue(fixture.available_model_catalogue(source)),
    )
    .expect_err("a catalogue source must also be an admitted route");
    assert_eq!(
        source_error.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::ModelCatalogueSourceMissing
    );
}

#[test]
fn driver_and_access_identity_mismatches_fail_closed() {
    let fixture = Fixture::ready("fixture.identity");
    let foreign_driver = DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new("fixture.foreign").expect("adapter id"),
            AdapterVersion::new("1").expect("adapter version"),
        ),
        IntegrationFamilyId::new("fixture-family").expect("family id"),
        TransportFamilyId::new("fixture-transport").expect("transport id"),
    );
    let driver_error =
        ConfiguredProviderInstanceRecord::admit(ConfiguredProviderInstanceAdmission::new(
            foreign_driver,
            fixture.instance.clone(),
            fixture.access_profile.clone(),
            fixture.access_evidence.clone(),
        ))
        .expect_err("a foreign driver must fail");
    assert_eq!(
        driver_error.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::DriverMismatch
    );

    let foreign_access = AccessProfile::new(
        AccessProfileId::new("fixture.foreign-access").expect("access id"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new("fixture-audience").expect("audience"),
        SupportAuthority::ProviderSupported,
    );
    let access_error =
        ConfiguredProviderInstanceRecord::admit(ConfiguredProviderInstanceAdmission::new(
            fixture.driver.clone(),
            fixture.instance.clone(),
            foreign_access,
            fixture.access_evidence.clone(),
        ))
        .expect_err("a foreign access profile must fail");
    assert_eq!(
        access_error.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::AccessMismatch
    );
}

#[test]
fn non_catalogue_routes_cannot_source_model_entries() {
    let fixture = Fixture::ready("fixture.invalid-source");
    let source = fixture.prepared(DriverRole::StructuredRun);
    let error = ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                source,
                [fixture.model("model-a", None)],
            )),
    )
    .expect_err("a structured route cannot source a model catalogue");
    assert_eq!(
        error.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::ModelCatalogueSourceInvalid
    );
}
