use super::{PreflightDimension, PreflightFailure};
use crate::{
    Capability, CapabilityConstraint, DriverRole, OperationRequirements, OperationShape,
    OwnedRemoteResourceKind, SessionProviderStatePolicy,
};

pub(super) fn validate_session_provider_state(
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    match (
        requirements.operation_shape(),
        requirements.session_provider_state_policy(),
    ) {
        (
            OperationShape::InteractiveSession
            | OperationShape::ProviderSessionImport
            | OperationShape::ProviderSessionReconciliation
            | OperationShape::ProviderSessionHistory,
            Some(policy),
        ) => validate_policy(requirements, policy),
        (OperationShape::InteractiveSession, None)
            if requirements.driver_role() != DriverRole::ModelCatalog =>
        {
            Err(failure(
                "Interactive session provider-state policy is missing",
            ))
        }
        (OperationShape::ProviderSessionImport, None) => Err(failure(
            "Provider-session import provider-state policy is missing",
        )),
        (OperationShape::ProviderSessionReconciliation, None) => Err(failure(
            "Provider-session reconciliation provider-state policy is missing",
        )),
        (OperationShape::ProviderSessionHistory, None) => Err(failure(
            "Provider-session history provider-state policy is missing",
        )),
        (OperationShape::InteractiveSession, None) => Ok(()),
        (_, Some(_)) => Err(failure(
            "Session provider-state policy is bound to a non-interactive operation",
        )),
        (_, None) => Ok(()),
    }
}

fn validate_policy(
    requirements: &OperationRequirements,
    policy: SessionProviderStatePolicy,
) -> Result<(), PreflightFailure> {
    let durable = capabilities(requirements, Capability::ProviderDurableRetention);
    let deletion = capabilities(requirements, Capability::OwnedRemoteResourceDeletion);
    let deletion_deletes_conversation = deletion
        .iter()
        .any(|requirement| has_conversation_deletion(requirement));
    match policy {
        SessionProviderStatePolicy::Prohibited => {
            if !durable.is_empty() || deletion_deletes_conversation {
                Err(failure(
                    "Provider conversation capabilities conflict with prohibited session state",
                ))
            } else {
                Ok(())
            }
        }
        SessionProviderStatePolicy::DurableProviderSessionPreserved => {
            if durable.is_empty() {
                Err(failure(
                    "Durable provider session policy lacks retention capability",
                ))
            } else if deletion_deletes_conversation {
                Err(failure(
                    "Preserved provider session policy conflicts with delete-on-close capability",
                ))
            } else {
                Ok(())
            }
        }
        SessionProviderStatePolicy::DurableConversationDeleteOnClose => {
            if durable.is_empty() {
                return Err(failure(
                    "Durable provider conversation policy lacks retention capability",
                ));
            }
            if deletion.is_empty() {
                return Err(failure(
                    "Provider conversation policy lacks remote deletion capability",
                ));
            }
            // Every deletion declaration must carry both exact constraints, so a
            // duplicate entry cannot satisfy the policy beside a weaker one.
            for requirement in deletion {
                for kind in [
                    OwnedRemoteResourceKind::ConversationItems,
                    OwnedRemoteResourceKind::Conversation,
                ] {
                    if !requirement.constraints().any(|constraint| {
                        constraint == &CapabilityConstraint::OwnedRemoteResource(kind)
                    }) {
                        return Err(failure(
                            "Provider conversation policy lacks an exact deletion constraint",
                        ));
                    }
                }
            }
            Ok(())
        }
    }
}

fn capabilities(
    requirements: &OperationRequirements,
    expected: Capability,
) -> Vec<&crate::CapabilityRequirement> {
    requirements
        .capabilities()
        .filter(|requirement| requirement.capability() == expected)
        .collect()
}

fn has_conversation_deletion(requirement: &crate::CapabilityRequirement) -> bool {
    requirement.constraints().any(|constraint| {
        matches!(
            constraint,
            CapabilityConstraint::OwnedRemoteResource(
                OwnedRemoteResourceKind::Conversation | OwnedRemoteResourceKind::ConversationItems
            )
        )
    })
}

fn failure(message: &'static str) -> PreflightFailure {
    PreflightFailure::new(PreflightDimension::SessionProviderState, message)
}

#[cfg(test)]
mod tests {
    use super::validate_session_provider_state;
    use crate::{
        AccessProfileId, AccessRequirement, Capability, CapabilityConstraint,
        CapabilityRequirement, DriverRole, ExecutionHostId, ExecutionLayer, OperationRequirements,
        OperationShape, OwnedRemoteResourceKind, PreflightDimension, SessionProviderStatePolicy,
    };

    fn requirements(
        capabilities: Vec<CapabilityRequirement>,
        policy: SessionProviderStatePolicy,
    ) -> OperationRequirements {
        OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            ExecutionHostId::new("fixture.host.local").expect("host id is valid"),
            AccessRequirement::new(AccessProfileId::new("fixture.access.local").expect("profile")),
        )
        .with_capabilities(capabilities)
        .with_session_provider_state_policy(policy)
    }

    fn durable_retention() -> CapabilityRequirement {
        CapabilityRequirement::new(Capability::ProviderDurableRetention, [])
    }

    fn deletion(
        constraints: impl IntoIterator<Item = OwnedRemoteResourceKind>,
    ) -> CapabilityRequirement {
        CapabilityRequirement::new(
            Capability::OwnedRemoteResourceDeletion,
            constraints
                .into_iter()
                .map(CapabilityConstraint::OwnedRemoteResource),
        )
    }

    #[test]
    fn a_later_conflicting_deletion_declaration_is_not_ignored() {
        let requirements = requirements(
            vec![
                durable_retention(),
                deletion([OwnedRemoteResourceKind::Environment]),
                deletion([OwnedRemoteResourceKind::Conversation]),
            ],
            SessionProviderStatePolicy::Prohibited,
        );
        let failure = validate_session_provider_state(&requirements)
            .expect_err("any conversation deletion conflicts with prohibited session state");
        assert_eq!(
            failure.dimension(),
            PreflightDimension::SessionProviderState
        );
    }

    #[test]
    fn every_delete_on_close_declaration_carries_both_constraints() {
        let complete = deletion([
            OwnedRemoteResourceKind::ConversationItems,
            OwnedRemoteResourceKind::Conversation,
        ]);
        validate_session_provider_state(&requirements(
            vec![durable_retention(), complete.clone()],
            SessionProviderStatePolicy::DurableConversationDeleteOnClose,
        ))
        .expect("a complete declaration satisfies the policy");

        let failure = validate_session_provider_state(&requirements(
            vec![
                durable_retention(),
                complete,
                deletion([OwnedRemoteResourceKind::Conversation]),
            ],
            SessionProviderStatePolicy::DurableConversationDeleteOnClose,
        ))
        .expect_err("a weaker duplicate declaration cannot satisfy the policy");
        assert_eq!(
            failure.dimension(),
            PreflightDimension::SessionProviderState
        );
    }
}
