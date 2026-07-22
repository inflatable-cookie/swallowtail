use super::{PreflightDimension, PreflightFailure};
use crate::{
    Capability, CapabilityConstraint, OperationRequirements, OperationShape,
    OwnedRemoteResourceKind, SessionProviderStatePolicy,
};

pub(super) fn validate_session_provider_state(
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    match (
        requirements.operation_shape(),
        requirements.session_provider_state_policy(),
    ) {
        (OperationShape::InteractiveSession, Some(policy)) => validate_policy(requirements, policy),
        (OperationShape::InteractiveSession, None) => Err(failure(
            "Interactive session provider-state policy is missing",
        )),
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
    let durable = capability(requirements, Capability::ProviderDurableRetention);
    let deletion = capability(requirements, Capability::OwnedRemoteResourceDeletion);
    match policy {
        SessionProviderStatePolicy::Prohibited => {
            if durable.is_some() || deletion.is_some_and(has_conversation_deletion) {
                Err(failure(
                    "Provider conversation capabilities conflict with prohibited session state",
                ))
            } else {
                Ok(())
            }
        }
        SessionProviderStatePolicy::DurableConversationDeleteOnClose => {
            if durable.is_none() {
                return Err(failure(
                    "Durable provider conversation policy lacks retention capability",
                ));
            }
            let deletion = deletion.ok_or_else(|| {
                failure("Provider conversation policy lacks remote deletion capability")
            })?;
            for kind in [
                OwnedRemoteResourceKind::ConversationItems,
                OwnedRemoteResourceKind::Conversation,
            ] {
                if !deletion.constraints().any(|constraint| {
                    constraint == &CapabilityConstraint::OwnedRemoteResource(kind)
                }) {
                    return Err(failure(
                        "Provider conversation policy lacks an exact deletion constraint",
                    ));
                }
            }
            Ok(())
        }
    }
}

fn capability(
    requirements: &OperationRequirements,
    expected: Capability,
) -> Option<&crate::CapabilityRequirement> {
    requirements
        .capabilities()
        .find(|requirement| requirement.capability() == expected)
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
