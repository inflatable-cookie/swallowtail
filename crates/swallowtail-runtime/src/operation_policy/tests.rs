use super::{
    ExternalNetworkPolicy, ExternalSearchPolicy, OperationPolicy, ProviderExecutionPolicy,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, StreamReattachmentPolicy,
};
use std::num::NonZeroU32;

#[test]
fn external_search_never_implies_network_authority() {
    let error = OperationPolicy::new(ExternalNetworkPolicy::Denied, ExternalSearchPolicy::Enabled)
        .expect_err("search without network authority must fail");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.operation_policy_rejected"
    );
}

#[test]
fn ambient_network_authority_is_harness_only() {
    let error = OperationPolicy::new(
        ExternalNetworkPolicy::AmbientHost,
        ExternalSearchPolicy::Disabled,
    )
    .expect_err("direct operation policy must reject ambient host authority");

    assert_eq!(
        error.diagnostic().message(),
        "Ambient host network authority is valid only for a harness session"
    );
}

#[test]
fn provider_background_posture_is_explicit_and_disabled_by_default() {
    let ordinary = OperationPolicy::offline();
    assert_eq!(
        ordinary.provider_execution(),
        ProviderExecutionPolicy::Attached
    );
    assert_eq!(
        ordinary.provider_retention(),
        ProviderRetentionPolicy::Prohibited
    );
    assert_eq!(
        ordinary.provider_recovery(),
        ProviderRecoveryPolicy::Prohibited
    );
    assert_eq!(
        ordinary.stream_reattachment(),
        StreamReattachmentPolicy::Disabled
    );

    let background = ordinary
        .with_provider_execution(ProviderExecutionPolicy::Background)
        .with_provider_retention(ProviderRetentionPolicy::TemporaryAllowed)
        .with_stream_reattachment(StreamReattachmentPolicy::Bounded(
            NonZeroU32::new(1).expect("one is non-zero"),
        ));
    assert_eq!(
        background.provider_execution(),
        ProviderExecutionPolicy::Background
    );
    assert_eq!(
        background.provider_retention(),
        ProviderRetentionPolicy::TemporaryAllowed
    );
    assert_eq!(
        background.stream_reattachment(),
        StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).unwrap())
    );
}

#[test]
fn durable_retention_and_provider_recovery_are_independent_opt_ins() {
    let durable =
        OperationPolicy::offline().with_provider_retention(ProviderRetentionPolicy::DurableAllowed);
    assert_eq!(
        durable.provider_retention(),
        ProviderRetentionPolicy::DurableAllowed
    );
    assert_eq!(
        durable.provider_recovery(),
        ProviderRecoveryPolicy::Prohibited
    );

    let managed = durable.with_provider_recovery(ProviderRecoveryPolicy::ManagedAllowed);
    assert_eq!(
        managed.provider_recovery(),
        ProviderRecoveryPolicy::ManagedAllowed
    );
}
