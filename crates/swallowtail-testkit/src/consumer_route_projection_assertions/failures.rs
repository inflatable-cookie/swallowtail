use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailureKind,
    ConsumerRouteProjectionSourceId, ConsumerRouteStateSupport, ConsumerRouteValueDomain,
    ConsumerRouteValueKind,
};

use crate::ConsumerRouteProjectionFixture;

use super::support::*;

pub(super) fn assert_failure_kinds() {
    let fixture = ConsumerRouteProjectionFixture::canonical();
    let applicability = fixture.applicability();

    assert_kind(
        &ConsumerRouteProjectionSourceId::new("   ").expect_err("blank identity is rejected"),
        ConsumerRouteProjectionFailureKind::IdentityInvalid,
    );
    assert_kind(
        &ConsumerRouteProjectionSourceId::new("bad\u{7}id")
            .expect_err("control-bearing identity is rejected"),
        ConsumerRouteProjectionFailureKind::IdentityInvalid,
    );

    assert_kind(
        &ConsumerRouteProjectionContribution::new(
            applicability.clone(),
            [adapter_source(), adapter_source()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect_err("a repeated source identity is rejected"),
        ConsumerRouteProjectionFailureKind::DuplicateSource,
    );

    let duplicate = feature_row(
        &applicability,
        ConsumerRouteFeatureId::InteractiveSession,
        ConsumerRouteLifecycle::SelectionSummary,
    );
    assert_kind(
        &contribution(
            &applicability,
            vec![duplicate.clone(), duplicate],
            Vec::new(),
            Vec::new(),
        )
        .expect_err("a repeated row identity is rejected"),
        ConsumerRouteProjectionFailureKind::DuplicateRow,
    );

    assert_kind(
        &contribution(
            &applicability,
            vec![feature_row(
                &applicability,
                ConsumerRouteFeatureId::InteractiveSession,
                ConsumerRouteLifecycle::SessionStartOnly,
            )],
            Vec::new(),
            Vec::new(),
        )
        .expect_err("a row whose lifecycle does not admit it to the view is rejected"),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
    );

    let superseded = ConsumerRouteProjectionFixture::superseded().applicability();
    assert_kind(
        &contribution(
            &applicability,
            vec![feature_row(
                &superseded,
                ConsumerRouteFeatureId::InteractiveSession,
                ConsumerRouteLifecycle::SelectionSummary,
            )],
            Vec::new(),
            Vec::new(),
        )
        .expect_err("a row bound to another revision is rejected"),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
    );

    let foreign = contribution(
        &superseded,
        vec![feature_row(
            &superseded,
            ConsumerRouteFeatureId::InteractiveSession,
            ConsumerRouteLifecycle::SelectionSummary,
        )],
        Vec::new(),
        Vec::new(),
    )
    .expect("the foreign contribution is internally consistent");
    assert_kind(
        &compose(&fixture, &[&foreign]).expect_err("a mixed snapshot is rejected as a whole"),
        ConsumerRouteProjectionFailureKind::SnapshotIdentityDisagreement,
    );

    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            Vec::new(),
            vec![
                feature_row(
                    &applicability,
                    ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
                    ConsumerRouteLifecycle::PostOpenObservationOnly,
                )
                .with_state_support(
                    ConsumerRouteStateSupport::descriptor_only().with_provider_effective(),
                ),
            ],
        )
        .expect_err("an effective claim without acknowledgement authority is rejected"),
        ConsumerRouteProjectionFailureKind::MutationAuthorityAbsent,
    );

    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            vec![
                feature_row(
                    &applicability,
                    ConsumerRouteFeatureId::InteractiveSession,
                    ConsumerRouteLifecycle::SessionStartOnly,
                )
                .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
                .with_mutation_authority(
                    ConsumerRouteMutationAuthority::PreparedSessionStart(
                        ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE)
                            .expect("source id is valid"),
                    ),
                ),
            ],
            Vec::new(),
        )
        .expect_err("a selectable row must publish its value domain"),
        ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
    );
    assert_kind(
        &contribution(
            &applicability,
            Vec::new(),
            vec![
                feature_row(
                    &applicability,
                    ConsumerRouteFeatureId::InteractiveSession,
                    ConsumerRouteLifecycle::SessionStartOnly,
                )
                .with_control_value(ConsumerRouteControlValue::new(
                    ConsumerRouteValueKind::CapabilityState,
                    ConsumerRouteValueDomain::Descriptor,
                    ConsumerRouteOmissionSemantics::SuppliesNothing,
                )),
            ],
            Vec::new(),
        )
        .expect_err("a descriptor-only domain cannot carry selectable omission truth"),
        ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
    );

    assert_kind(
        &ConsumerRouteEnumeratedValues::new([
            ConsumerRouteEnumerableValue::new("low").expect("value is valid"),
            ConsumerRouteEnumerableValue::new("low").expect("value is valid"),
        ])
        .expect_err("a repeated admitted value is rejected"),
        ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
    );
}
