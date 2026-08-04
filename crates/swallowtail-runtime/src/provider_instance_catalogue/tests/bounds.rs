use super::super::{
    ConfiguredProviderInstanceCatalogue, ConfiguredProviderInstanceCatalogueFailureKind,
    ConfiguredProviderInstanceRecord, ConfiguredProviderModelCatalogueInput,
    MAX_CONFIGURED_PROVIDER_INSTANCES, MAX_CONFIGURED_PROVIDER_MODELS_PER_INSTANCE,
    MAX_CONFIGURED_PROVIDER_ROUTES_PER_INSTANCE,
};
use super::Fixture;
use swallowtail_core::DriverRole;

#[test]
fn duplicates_and_portable_limits_are_rejected() {
    let fixture = Fixture::ready("fixture.bounds");
    let source = fixture.prepared(DriverRole::ModelCatalog);
    let duplicate_route = ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone(), source.clone()]),
    )
    .expect_err("duplicate routes fail");
    assert_eq!(
        duplicate_route.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::DuplicateRoute
    );

    let duplicate_model = fixture.model("model-a", Some("provider-a"));
    let duplicate_model_error = ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                source.clone(),
                [duplicate_model.clone(), duplicate_model],
            )),
    )
    .expect_err("duplicate provider-model identities fail");
    assert_eq!(
        duplicate_model_error.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::DuplicateModel
    );

    let route_limit_error =
        ConfiguredProviderInstanceRecord::admit(fixture.admission().with_prepared_routes(
            std::iter::repeat_n(
                source.clone(),
                MAX_CONFIGURED_PROVIDER_ROUTES_PER_INSTANCE + 1,
            ),
        ))
        .expect_err("route limit fails before duplicate projection");
    assert_eq!(
        route_limit_error.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::LimitExceeded
    );

    let model_limit_error = ConfiguredProviderInstanceRecord::admit(
        fixture
            .admission()
            .with_prepared_routes([source.clone()])
            .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
                source,
                (0..=MAX_CONFIGURED_PROVIDER_MODELS_PER_INSTANCE)
                    .map(|index| fixture.model(&format!("model-{index}"), None)),
            )),
    )
    .expect_err("model limit fails");
    assert_eq!(
        model_limit_error.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::LimitExceeded
    );
}

#[test]
fn catalogue_preserves_unavailable_instances_and_rejects_duplicate_ids() {
    let fixture = Fixture::ready("fixture.catalogue");
    let record = ConfiguredProviderInstanceRecord::admit(fixture.admission())
        .expect("configured record is admitted");
    let catalogue =
        ConfiguredProviderInstanceCatalogue::new([record.clone()]).expect("catalogue is admitted");
    assert_eq!(catalogue.instances().len(), 1);
    assert_eq!(catalogue.get(record.instance_id()), Some(&record));

    let duplicate = ConfiguredProviderInstanceCatalogue::new([record.clone(), record.clone()])
        .expect_err("duplicate configured instances fail");
    assert_eq!(
        duplicate.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::DuplicateInstance
    );

    let limit = ConfiguredProviderInstanceCatalogue::new(std::iter::repeat_n(
        record,
        MAX_CONFIGURED_PROVIDER_INSTANCES + 1,
    ))
    .expect_err("instance limit fails before duplicate admission");
    assert_eq!(
        limit.kind(),
        ConfiguredProviderInstanceCatalogueFailureKind::LimitExceeded
    );
}
