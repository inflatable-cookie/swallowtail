use swallowtail_core::ProviderSessionImportAvailability;
use swallowtail_testkit::{
    ProviderSessionImportFixture, assert_provider_session_import_contract,
    provider_session_catalogue_bounds,
};

#[test]
fn public_fixture_pack_passes_provider_session_import_contract() {
    assert_provider_session_import_contract();
}

#[test]
fn public_fixture_remains_composable_without_a_provider_adapter() {
    let fixture = ProviderSessionImportFixture::remote_authoritative();
    let catalogue = fixture.catalogue_plan(
        "fixture-public-catalogue",
        provider_session_catalogue_bounds(10, 100, 128, 512, 256),
    );
    let candidate = fixture
        .candidate(
            &catalogue,
            "fixture-public-candidate",
            "provider/private/public-session",
            ProviderSessionImportAvailability::Available,
        )
        .expect("public candidate is valid");
    let import = fixture
        .import_plan(catalogue, candidate)
        .expect("public import plan is valid");

    assert_eq!(
        import.preflight().execution_host_id(),
        fixture.topology().execution_host_id()
    );
}
