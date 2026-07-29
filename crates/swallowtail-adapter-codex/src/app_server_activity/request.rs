use super::{AppServerActivityProjection, RequestIdentity, required_text};
use crate::turn_state::malformed_notification;
use serde_json::Value;
use swallowtail_core::{ActivityDisclosure, ProviderActivityRef, ProviderRequestRef};
use swallowtail_runtime::{
    ActivityCorrelation, ActivityKind, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityStatus, CallbackId, RuntimeFailure,
};

impl AppServerActivityProjection {
    pub(crate) fn register_callback(&mut self, provider_call_id: &str, callback_id: CallbackId) {
        self.correlations.insert(
            provider_call_id.to_owned(),
            ActivityCorrelation::Callback(callback_id),
        );
    }

    pub(crate) fn provider_request_started(
        &mut self,
        request_id: ProviderRequestRef,
        item_id: Option<&str>,
        namespace: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let request_key = request_id.as_provider_value().to_owned();
        let identity = self.activity_id(&format!("request:{request_key}"))?;
        let provider_ref = item_id
            .map(ProviderActivityRef::new)
            .transpose()
            .map_err(|_| malformed_notification())?;
        let namespace = ActivityNamespace::new(format!("codex.app-server.request.{namespace}"))
            .map_err(|_| malformed_notification())?;
        self.requests.insert(
            request_key,
            RequestIdentity {
                activity_id: identity.clone(),
                namespace: namespace.clone(),
                provider_ref: provider_ref.clone(),
            },
        );
        let mut observation = ActivityObservation::new(
            identity,
            self.operation_id.clone(),
            ActivityKind::Unknown(namespace),
            ActivityLifecyclePhase::Started,
            ActivityStatus::Pending,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .map_err(|_| malformed_notification())?
        .with_correlation(ActivityCorrelation::ProviderRequest(request_id));
        if let Some(provider_ref) = provider_ref {
            observation = observation.with_provider_activity_ref(provider_ref);
        }
        Ok(observation)
    }

    pub(super) fn project_request_resolution(
        &mut self,
        params: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let request_id = required_text(params, "requestId")?;
        let Some(identity) = self.requests.remove(request_id) else {
            return Ok(Vec::new());
        };
        let request_ref =
            ProviderRequestRef::new(request_id).map_err(|_| malformed_notification())?;
        let observation = ActivityObservation::new(
            identity.activity_id,
            self.operation_id.clone(),
            ActivityKind::Unknown(identity.namespace),
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .map_err(|_| malformed_notification())?
        .with_correlation(ActivityCorrelation::ProviderRequest(request_ref));
        Ok(vec![match identity.provider_ref {
            Some(reference) => observation.with_provider_activity_ref(reference),
            None => observation,
        }])
    }
}
