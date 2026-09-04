use super::{Projection, Route};
use swallowtail_core::{AccessStatus, Capability, DriverRole};
use swallowtail_runtime::{ConsumerRouteAvailability, ConsumerRouteFeatureId};

impl Projection<'_> {
    pub(super) fn feature_for(&mut self, capability: Capability) -> Option<ConsumerRouteFeatureId> {
        if !ledger_admits(
            self.route,
            self.plan.requirements().driver_role(),
            capability,
        ) {
            return None;
        }
        Some(match capability {
            Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
            Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
            Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
            Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
            Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
            Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
            Capability::LoadSession => ConsumerRouteFeatureId::LoadSession,
            Capability::Resume => ConsumerRouteFeatureId::ResumeSession,
            Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
            Capability::WorkingResourceTextWrite => {
                ConsumerRouteFeatureId::BoundedWorkspaceTextWrite
            }
            Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
            Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
            Capability::OutputTokenLimit => ConsumerRouteFeatureId::OutputTokenLimit,
            Capability::ProviderSessionArchive => ConsumerRouteFeatureId::ProviderSessionArchive,
            Capability::ProviderSessionRestore => ConsumerRouteFeatureId::ProviderSessionRestore,
            Capability::ProviderSessionCatalogue => {
                ConsumerRouteFeatureId::ProviderSessionCatalogue
            }
            Capability::ProviderSessionImport => ConsumerRouteFeatureId::ProviderSessionImport,
            Capability::StreamReattachment => self.local_feature("feature.stream-reattachment")?,
            _ => return None,
        })
    }
}

fn ledger_admits(route: Route, role: DriverRole, capability: Capability) -> bool {
    match (route, role) {
        (Route::Acp, DriverRole::InteractiveSession) => matches!(
            capability,
            Capability::InteractiveSession
                | Capability::StreamingEvents
                | Capability::ObservableActivity
                | Capability::Interruption
                | Capability::LoadSession
                | Capability::Resume
                | Capability::WorkingResource
                | Capability::WorkingResourceTextWrite
                | Capability::ReasoningSelection
        ),
        (Route::Acp, DriverRole::ProviderSessionCatalogue) => matches!(
            capability,
            Capability::ProviderSessionCatalogue | Capability::WorkingResource
        ),
        (Route::Acp, DriverRole::ProviderSessionImport) => matches!(
            capability,
            Capability::ProviderSessionImport | Capability::WorkingResource
        ),
        (Route::Headless, DriverRole::StructuredRun) => matches!(
            capability,
            Capability::StructuredRun
                | Capability::StreamingEvents
                | Capability::ObservableActivity
                | Capability::Interruption
                | Capability::WorkingResource
        ),
        (Route::Local, DriverRole::ModelCatalog) => capability == Capability::ModelCatalog,
        (Route::Local, DriverRole::StructuredRun) => matches!(
            capability,
            Capability::StructuredRun
                | Capability::StreamingEvents
                | Capability::ObservableActivity
                | Capability::Interruption
                | Capability::WorkingResource
                | Capability::ReasoningSelection
                | Capability::StreamReattachment
        ),
        (Route::Local, DriverRole::InteractiveSession) => matches!(
            capability,
            Capability::InteractiveSession
                | Capability::StreamingEvents
                | Capability::ObservableActivity
                | Capability::Interruption
                | Capability::Resume
                | Capability::WorkingResource
                | Capability::ReasoningSelection
        ),
        (Route::Local, DriverRole::ProviderSessionManagement) => matches!(
            capability,
            Capability::ProviderSessionArchive | Capability::ProviderSessionRestore
        ),
        _ => false,
    }
}

pub(super) fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
    use swallowtail_core::{
        CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness,
    };
    if matches!(
        status.credential(),
        CredentialState::Ready | CredentialState::NotRequired
    ) && status.entitlement() == EntitlementState::Available
        && status.endpoint_authorization() == EndpointAuthorization::Allowed
        && status.runtime_readiness() == RuntimeReadiness::Ready
    {
        ConsumerRouteAvailability::Available
    } else {
        ConsumerRouteAvailability::Conditional
    }
}
