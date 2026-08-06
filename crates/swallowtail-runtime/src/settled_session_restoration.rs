#![deny(missing_docs)]

use crate::{
    BoxFuture, HostServices, InteractiveSessionHandle, InterruptedTurnState, LoadedSession,
    ProviderSessionReconciliationOutcome, RuntimeFailure,
};
use std::error::Error;
use std::fmt;
use swallowtail_core::PreflightPlan;

/// Stateful attachment method prepared beside read-only reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettledSessionAttachmentKind {
    /// Load bounded replay before returning a live session.
    Load,
    /// Resume a live session without replay.
    Resume,
}

/// Live attachment returned only after eligible settled reconciliation.
pub enum SettledSessionAttachment {
    /// Loaded session with bounded replay.
    Loaded(LoadedSession),
    /// Resumed session without replay.
    Resumed(Box<dyn InteractiveSessionHandle>),
}

impl SettledSessionAttachment {
    /// Returns the attachment method represented by this outcome.
    #[must_use]
    pub const fn kind(&self) -> SettledSessionAttachmentKind {
        match self {
            Self::Loaded(_) => SettledSessionAttachmentKind::Load,
            Self::Resumed(_) => SettledSessionAttachmentKind::Resume,
        }
    }
}

/// Successful reconciliation retained beside its distinct live attachment.
pub struct SettledSessionAttachmentOutcome {
    reconciliation: ProviderSessionReconciliationOutcome,
    attachment: SettledSessionAttachment,
}

impl SettledSessionAttachmentOutcome {
    /// Returns the complete first-phase reconciliation result.
    #[must_use]
    pub const fn reconciliation(&self) -> &ProviderSessionReconciliationOutcome {
        &self.reconciliation
    }

    #[must_use]
    /// Returns the distinct live attachment result.
    pub const fn attachment(&self) -> &SettledSessionAttachment {
        &self.attachment
    }

    #[must_use]
    /// Separates first-phase observation from second-phase attachment.
    pub fn into_parts(
        self,
    ) -> (
        ProviderSessionReconciliationOutcome,
        SettledSessionAttachment,
    ) {
        (self.reconciliation, self.attachment)
    }
}

/// Truth-preserving successful outcome of observe-then-attach restoration.
pub enum SettledSessionRestorationOutcome {
    /// Reconciliation state was not eligible for attachment.
    Observed(ProviderSessionReconciliationOutcome),
    /// Eligible reconciliation was followed by a live attachment.
    Attached(SettledSessionAttachmentOutcome),
}

impl SettledSessionRestorationOutcome {
    /// Returns the complete reconciliation result from either successful path.
    #[must_use]
    pub const fn reconciliation(&self) -> &ProviderSessionReconciliationOutcome {
        match self {
            Self::Observed(reconciliation) => reconciliation,
            Self::Attached(attached) => attached.reconciliation(),
        }
    }
}

/// Phase in which observe-then-attach restoration failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettledSessionRestorationFailurePhase {
    /// Read-only reconciliation failed; attachment never started.
    Reconciliation,
    /// Attachment failed after successful reconciliation.
    Attachment,
}

/// Phase-aware failure which retains completed reconciliation on partial loss.
pub enum SettledSessionRestorationFailure {
    /// First-phase reconciliation failure.
    Reconciliation(RuntimeFailure),
    /// Second-phase failure retaining the successful reconciliation result.
    Attachment {
        /// Complete successful first-phase observation.
        reconciliation: ProviderSessionReconciliationOutcome,
        /// Attachment failure.
        failure: RuntimeFailure,
    },
}

impl SettledSessionRestorationFailure {
    /// Returns the phase that failed.
    #[must_use]
    pub const fn phase(&self) -> SettledSessionRestorationFailurePhase {
        match self {
            Self::Reconciliation(_) => SettledSessionRestorationFailurePhase::Reconciliation,
            Self::Attachment { .. } => SettledSessionRestorationFailurePhase::Attachment,
        }
    }

    #[must_use]
    /// Returns the underlying safe runtime failure.
    pub const fn failure(&self) -> &RuntimeFailure {
        match self {
            Self::Reconciliation(failure) | Self::Attachment { failure, .. } => failure,
        }
    }

    #[must_use]
    /// Returns successful reconciliation retained after attachment failure.
    pub const fn reconciliation(&self) -> Option<&ProviderSessionReconciliationOutcome> {
        match self {
            Self::Reconciliation(_) => None,
            Self::Attachment { reconciliation, .. } => Some(reconciliation),
        }
    }

    #[must_use]
    /// Separates retained reconciliation evidence from the runtime failure.
    pub fn into_parts(self) -> (Option<ProviderSessionReconciliationOutcome>, RuntimeFailure) {
        match self {
            Self::Reconciliation(failure) => (None, failure),
            Self::Attachment {
                reconciliation,
                failure,
            } => (Some(reconciliation), failure),
        }
    }
}

impl fmt::Debug for SettledSessionRestorationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SettledSessionRestorationFailure")
            .field("phase", &self.phase())
            .field("diagnostic", self.failure().diagnostic())
            .finish()
    }
}

impl fmt::Display for SettledSessionRestorationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.failure().fmt(formatter)
    }
}

impl Error for SettledSessionRestorationFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.failure())
    }
}

/// One exact prepared read-only reconciliation operation.
pub trait SettledSessionReconciliationOperation: Send + Sync {
    /// Consumes and runs the prepared read-only reconciliation phase.
    fn reconcile(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>>;
}

/// One independently prepared stateful attachment operation.
pub trait SettledSessionAttachmentOperation: Send + Sync {
    /// Returns the independently prepared attachment method.
    fn kind(&self) -> SettledSessionAttachmentKind;

    /// Consumes and runs the prepared stateful attachment phase.
    fn attach(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<SettledSessionAttachment, RuntimeFailure>>;
}

/// Consuming sequence over one prepared reconciliation and one attachment.
pub struct PreparedSettledSessionRestoration {
    reconciliation: Box<dyn SettledSessionReconciliationOperation>,
    attachment: Box<dyn SettledSessionAttachmentOperation>,
}

impl PreparedSettledSessionRestoration {
    /// Combines independently prepared reconciliation and attachment operations.
    #[must_use]
    pub fn new(
        reconciliation: impl SettledSessionReconciliationOperation + 'static,
        attachment: impl SettledSessionAttachmentOperation + 'static,
    ) -> Self {
        Self {
            reconciliation: Box::new(reconciliation),
            attachment: Box::new(attachment),
        }
    }

    #[must_use]
    /// Returns the second-phase attachment method without executing it.
    pub fn attachment_kind(&self) -> SettledSessionAttachmentKind {
        self.attachment.kind()
    }

    /// Reconciles first and attaches only when the observed state is eligible.
    pub fn restore(
        self,
        services: HostServices,
    ) -> BoxFuture<
        'static,
        Result<SettledSessionRestorationOutcome, SettledSessionRestorationFailure>,
    > {
        Box::pin(async move {
            let reconciliation = self
                .reconciliation
                .reconcile(services.clone())
                .await
                .map_err(SettledSessionRestorationFailure::Reconciliation)?;
            if !is_attachment_eligible(reconciliation.state()) {
                return Ok(SettledSessionRestorationOutcome::Observed(reconciliation));
            }
            let expected = self.attachment.kind();
            let attachment = match self.attachment.attach(services).await {
                Ok(attachment) => attachment,
                Err(failure) => {
                    return Err(SettledSessionRestorationFailure::Attachment {
                        reconciliation,
                        failure,
                    });
                }
            };
            if attachment.kind() != expected {
                return Err(SettledSessionRestorationFailure::Attachment {
                    reconciliation,
                    failure: RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.settled_session_restoration.attachment_kind_mismatch",
                        "Settled-session attachment returned a different prepared method",
                    )),
                });
            }
            Ok(SettledSessionRestorationOutcome::Attached(
                SettledSessionAttachmentOutcome {
                    reconciliation,
                    attachment,
                },
            ))
        })
    }
}

const fn is_attachment_eligible(state: InterruptedTurnState) -> bool {
    matches!(
        state,
        InterruptedTurnState::Completed
            | InterruptedTurnState::Failed
            | InterruptedTurnState::Cancelled
            | InterruptedTurnState::InactiveUnresolved
    )
}

/// Checks that independently prepared reconciliation and attachment plans
/// retain the same immutable provider-route binding.
///
/// Operation requirements remain intentionally distinct: reconciliation is
/// read-only observation while attachment retains its own access policy.
#[must_use]
pub fn settled_session_plans_share_binding(
    reconciliation: &PreflightPlan,
    attachment: &PreflightPlan,
) -> bool {
    reconciliation.driver_identity() == attachment.driver_identity()
        && reconciliation.integration_family() == attachment.integration_family()
        && reconciliation.transport_family() == attachment.transport_family()
        && reconciliation.instance_id() == attachment.instance_id()
        && reconciliation.instance_revision() == attachment.instance_revision()
        && reconciliation.instance_target_ref() == attachment.instance_target_ref()
        && reconciliation.protocol_facade_id() == attachment.protocol_facade_id()
        && reconciliation.instance_policy_id() == attachment.instance_policy_id()
        && reconciliation.model_route_id() == attachment.model_route_id()
        && reconciliation.model_route_revision() == attachment.model_route_revision()
        && reconciliation.model_id() == attachment.model_id()
        && reconciliation.provider_id() == attachment.provider_id()
        && reconciliation.provider_agent() == attachment.provider_agent()
        && reconciliation
            .interface_versions()
            .eq(attachment.interface_versions())
        && reconciliation.attached_model_observation() == attachment.attached_model_observation()
        && reconciliation.harness_rpc_policy() == attachment.harness_rpc_policy()
        && reconciliation.harness_configuration_posture()
            == attachment.harness_configuration_posture()
        && reconciliation.access_profile_id() == attachment.access_profile_id()
        && reconciliation.access_status() == attachment.access_status()
        && reconciliation.credential_mechanism() == attachment.credential_mechanism()
        && reconciliation.credential_reference() == attachment.credential_reference()
        && reconciliation.endpoint_audience() == attachment.endpoint_audience()
        && reconciliation.ownership() == attachment.ownership()
        && reconciliation.execution_host_id() == attachment.execution_host_id()
}

#[cfg(test)]
#[path = "settled_session_restoration/tests.rs"]
mod tests;
