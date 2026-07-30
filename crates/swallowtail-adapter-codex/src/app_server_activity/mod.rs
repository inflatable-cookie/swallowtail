mod content;
mod extension;
mod item;
mod projection;
mod request;
mod subagent;

use crate::turn_state::malformed_notification;
use item::ItemIdentity;
use serde_json::Value;
use std::collections::HashMap;
use swallowtail_core::ProviderActivityRef;
use swallowtail_runtime::{
    ActivityCorrelation, ActivityId, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, RuntimeFailure, RuntimeTurnId,
};

pub(crate) struct AppServerActivityProjection {
    operation_id: ActivityOperationId,
    identities: HashMap<String, ActivityId>,
    items: HashMap<String, ItemIdentity>,
    labels: HashMap<String, ActivityLabel>,
    correlations: HashMap<String, ActivityCorrelation>,
    requests: HashMap<String, RequestIdentity>,
    next_minted_id: u64,
}

pub(super) struct ActivitySource<'a> {
    identity_key: &'a str,
    provider_ref: Option<&'a str>,
}

struct ObservationDetail {
    correlation: Option<ActivityCorrelation>,
    content: Option<swallowtail_runtime::ActivityContentUpdate>,
    subagent: subagent::SubagentProjection,
}

impl ObservationDetail {
    const fn primary(
        correlation: Option<ActivityCorrelation>,
        content: Option<swallowtail_runtime::ActivityContentUpdate>,
    ) -> Self {
        Self {
            correlation,
            content,
            subagent: subagent::SubagentProjection::primary(),
        }
    }

    const fn with_subagent(
        correlation: Option<ActivityCorrelation>,
        content: Option<swallowtail_runtime::ActivityContentUpdate>,
        subagent: subagent::SubagentProjection,
    ) -> Self {
        Self {
            correlation,
            content,
            subagent,
        }
    }
}

impl<'a> ActivitySource<'a> {
    pub(super) const fn new(identity_key: &'a str, provider_ref: Option<&'a str>) -> Self {
        Self {
            identity_key,
            provider_ref,
        }
    }
}

impl AppServerActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self {
            operation_id: ActivityOperationId::Turn(turn_id),
            identities: HashMap::new(),
            items: HashMap::new(),
            labels: HashMap::new(),
            correlations: HashMap::new(),
            requests: HashMap::new(),
            next_minted_id: 0,
        }
    }

    fn observation(
        &mut self,
        source: ActivitySource<'_>,
        identity: ItemIdentity,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        detail: ObservationDetail,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let activity_id = self.activity_id(&format!("item:{}", source.identity_key))?;
        let label = self.labels.get(source.identity_key).cloned();
        let mut observation = ActivityObservation::new(
            activity_id,
            self.operation_id.clone(),
            identity.kind,
            phase,
            status,
            identity.assistant_phase,
            identity.disclosure,
        )
        .map_err(|_| malformed_notification())?;
        if let Some(provider_ref) = source.provider_ref {
            observation = observation.with_provider_activity_ref(
                ProviderActivityRef::new(provider_ref).map_err(|_| malformed_notification())?,
            );
        }
        if let Some(correlation) = detail.correlation {
            observation = observation.with_correlation(correlation);
        }
        if let Some(label) = label {
            observation = observation
                .with_label(label)
                .map_err(|_| malformed_notification())?;
        }
        if let Some(content) = detail.content {
            observation = observation
                .with_content(content)
                .map_err(|_| malformed_notification())?;
        }
        observation = observation.with_actor(detail.subagent.actor);
        if !detail.subagent.snapshots.is_empty() {
            observation = observation
                .with_subagents(detail.subagent.snapshots)
                .map_err(|_| malformed_notification())?;
        }
        if let Some(action) = detail.subagent.control {
            observation = observation
                .with_subagent_control(action)
                .map_err(|_| malformed_notification())?;
        }
        Ok(observation)
    }

    fn activity_id(&mut self, provider_key: &str) -> Result<ActivityId, RuntimeFailure> {
        if let Some(identity) = self.identities.get(provider_key) {
            return Ok(identity.clone());
        }
        let preferred = format!("codex-app-server:{provider_key}");
        let identity = ActivityId::new(preferred)
            .or_else(|_| {
                self.next_minted_id = self.next_minted_id.saturating_add(1);
                ActivityId::new(format!("codex-app-server:minted-{}", self.next_minted_id))
            })
            .map_err(|_| malformed_notification())?;
        self.identities
            .insert(provider_key.to_owned(), identity.clone());
        Ok(identity)
    }
}

struct RequestIdentity {
    activity_id: ActivityId,
    namespace: ActivityNamespace,
    provider_ref: Option<ProviderActivityRef>,
}

fn required_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed_notification)
}

#[cfg(test)]
mod tests;
