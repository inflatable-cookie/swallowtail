use swallowtail_core::PreflightDimension;
use swallowtail_testkit::{
    ProviderConversationPreflightCase, ProviderConversationPreflightFixture,
    run_provider_conversation_boundary_assertions,
};

#[test]
fn provider_conversation_policy_and_lifecycle_assertions_pass() {
    run_provider_conversation_boundary_assertions();
}

#[test]
fn every_provider_conversation_mismatch_rejects_before_effects() {
    for case in [
        ProviderConversationPreflightCase::PolicyProhibited,
        ProviderConversationPreflightCase::MissingDurableRequirement,
        ProviderConversationPreflightCase::MissingConversationDeletionRequirement,
        ProviderConversationPreflightCase::MissingItemDeletionRequirement,
        ProviderConversationPreflightCase::AdvertisedMissingDurable,
        ProviderConversationPreflightCase::AdvertisedMissingConversationDeletion,
        ProviderConversationPreflightCase::AdvertisedMissingItemDeletion,
    ] {
        let fixture = ProviderConversationPreflightFixture::for_case(case);
        let failure = fixture
            .preflight()
            .expect_err("mismatch must fail preflight");
        assert!(matches!(
            failure.dimension(),
            PreflightDimension::SessionProviderState
                | PreflightDimension::Capability
                | PreflightDimension::Constraint
        ));
        assert_eq!(fixture.provider_side_effect_count(), 0);
    }
}
