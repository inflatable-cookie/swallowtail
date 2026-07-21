use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, OwnedRemoteResourceKind,
};

pub(super) fn capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(Capability::ProviderManagedRecovery, []),
        CapabilityRequirement::new(
            Capability::OwnedRemoteResourceDeletion,
            [
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Environment),
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Session),
            ],
        ),
        CapabilityRequirement::new(
            Capability::StreamReattachment,
            [CapabilityConstraint::ReattachmentMaximumCount(1)],
        ),
        CapabilityRequirement::new(
            Capability::ToolCalls,
            [
                CapabilityConstraint::ToolSchemaDialect("json-schema-2020-12".to_owned()),
                CapabilityConstraint::ToolMaximumSchemaBytes(16_384),
                CapabilityConstraint::ToolMaximumCount(8),
            ],
        ),
    ]
}
