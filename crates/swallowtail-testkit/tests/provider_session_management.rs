use swallowtail_testkit::{
    ProviderSessionManagementFixture, ProviderSessionManagementFixtureCase,
    assert_provider_session_management_contract,
};

#[test]
fn public_fixture_pack_passes_provider_session_management_contract() {
    assert_provider_session_management_contract();
}

#[test]
fn public_fixture_remains_composable_without_a_provider_adapter() {
    let fixture = ProviderSessionManagementFixture::local(
        ProviderSessionManagementFixtureCase::Qualified,
        swallowtail_core::ProviderSessionManagementAction::Delete(
            swallowtail_core::ProviderSessionDeletionStrength::HistoryRemoved,
        ),
    );

    let plan = fixture.plan(None).expect("public fixture plan is valid");
    assert_eq!(
        plan.agreement().action().deletion_strength(),
        Some(swallowtail_core::ProviderSessionDeletionStrength::HistoryRemoved)
    );
}
