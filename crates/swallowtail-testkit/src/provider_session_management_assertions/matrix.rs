use super::{
    driver::{FixtureBehavior, execute},
    trace::FixtureEvent,
};
use crate::{
    ProviderSessionManagementFixture, ProviderSessionManagementFixtureCase, RecordedHostCall,
    RecordingHostServices, RecordingOutcome,
};
use swallowtail_core::{
    Capability, ProviderSessionAffectedScope, ProviderSessionDeletionStrength,
    ProviderSessionEffectTruth, ProviderSessionManagementAction,
};
use swallowtail_runtime::ProviderSessionManagementPlan;

pub(super) fn assert_action_and_version_matrix() {
    let actions = [
        ProviderSessionManagementAction::Archive,
        ProviderSessionManagementAction::Restore,
        ProviderSessionManagementAction::Delete(ProviderSessionDeletionStrength::HistoryRemoved),
        ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        ),
        ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderHardDeleted,
        ),
    ];
    for action in actions {
        ProviderSessionManagementFixture::local(
            ProviderSessionManagementFixtureCase::Qualified,
            action,
        )
        .plan(None)
        .expect("qualified action must plan");
        let unverified = ProviderSessionManagementFixture::local(
            ProviderSessionManagementFixtureCase::UnverifiedNewer,
            action,
        )
        .plan(None)
        .expect("unverified-newer action remains visible and permitted");
        assert!(
            unverified
                .agreement()
                .binding()
                .interface_compatibility()
                .all(|evidence| evidence.assessment().is_permitted())
        );
    }

    let incompatible = ProviderSessionManagementFixture::local(
        ProviderSessionManagementFixtureCase::Incompatible,
        ProviderSessionManagementAction::Archive,
    );
    assert!(incompatible.preflight().is_err());
    assert!(incompatible.binding().is_err());

    let unsupported = ProviderSessionManagementFixture::local(
        ProviderSessionManagementFixtureCase::Unsupported,
        ProviderSessionManagementAction::Archive,
    );
    assert!(unsupported.preflight().is_err());
    assert!(unsupported.binding().is_err());
    assert!(
        !unsupported
            .instance()
            .capabilities()
            .supports(Capability::ProviderSessionArchive)
    );
    assert!(
        unsupported
            .instance()
            .capabilities()
            .supports(Capability::ProviderNativeSessionClose)
    );
}

pub(super) fn assert_topologies_and_cleanup() {
    for fixture in [
        ProviderSessionManagementFixture::local(
            ProviderSessionManagementFixtureCase::Qualified,
            ProviderSessionManagementAction::Archive,
        ),
        ProviderSessionManagementFixture::remote_authoritative(
            ProviderSessionManagementFixtureCase::Qualified,
            ProviderSessionManagementAction::Restore,
        ),
    ] {
        let plan = fixture.plan(None).expect("topology plan is valid");
        let host = RecordingHostServices::for_host(
            plan.preflight().execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        let (outcome, events) = execute(
            &fixture,
            plan,
            host.services().clone(),
            FixtureBehavior::Apply,
            false,
        );
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );
        assert_eq!(
            events,
            vec![
                FixtureEvent::TaskStarted,
                FixtureEvent::CredentialAcquired,
                FixtureEvent::ResourceAcquired,
                FixtureEvent::Dispatched,
                FixtureEvent::TaskJoined,
                FixtureEvent::ResourceReleased,
                FixtureEvent::CredentialReleased,
            ]
        );
        assert_eq!(
            host.calls(),
            vec![
                RecordedHostCall::TaskSpawn,
                RecordedHostCall::CredentialAcquire,
                RecordedHostCall::WorkingResourceResolve,
                RecordedHostCall::TaskJoin,
                RecordedHostCall::WorkingResourceRelease,
                RecordedHostCall::CredentialRelease,
            ]
        );
    }

    let descendant = ProviderSessionManagementFixture::local(
        ProviderSessionManagementFixtureCase::Qualified,
        ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        ),
    );
    let agreement = descendant
        .plan(None)
        .expect("delete plan is valid")
        .agreement()
        .clone();
    let agreement = swallowtail_runtime::ProviderSessionManagementAgreement::new(
        agreement.binding().clone(),
        agreement.action(),
        agreement.initial_state(),
        ProviderSessionAffectedScope::ProviderDefinedDescendants,
        agreement.activity(),
        agreement.cancellation(),
        agreement.deadline(),
    );
    let plan = ProviderSessionManagementPlan::new(
        descendant.preflight().expect("preflight is valid"),
        agreement,
    )
    .expect("descendant scope remains explicit");
    assert_eq!(
        plan.agreement().affected_scope(),
        ProviderSessionAffectedScope::ProviderDefinedDescendants
    );
}
