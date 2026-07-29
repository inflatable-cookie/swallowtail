mod content;

use crate::exec_events::malformed_stream;
use semver::Version;
use serde_json::Value;
use std::collections::HashMap;
use swallowtail_core::{ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, RuntimeFailure, RuntimeRunId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ItemIdentity {
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
}

pub(crate) struct ExecActivityProjection {
    operation_id: ActivityOperationId,
    qualified_version: Version,
    identities: HashMap<String, ItemIdentity>,
    next_minted_id: u64,
}

impl ExecActivityProjection {
    pub(crate) fn new(run_id: RuntimeRunId, qualified_version: Version) -> Self {
        Self {
            operation_id: ActivityOperationId::Run(run_id),
            qualified_version,
            identities: HashMap::new(),
            next_minted_id: 0,
        }
    }

    pub(crate) fn project(
        &mut self,
        event_type: &str,
        payload: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let phase = match event_type {
            "item.started" => Some(ActivityLifecyclePhase::Started),
            "item.updated" => Some(ActivityLifecyclePhase::Updated),
            "item.completed" => Some(ActivityLifecyclePhase::Completed),
            _ => None,
        };
        if let Some(phase) = phase {
            return Ok(vec![self.project_item(payload, phase)?]);
        }
        match event_type {
            "thread.started" | "turn.started" | "turn.completed" | "turn.failed" | "error" => {
                Ok(Vec::new())
            }
            unknown => Ok(vec![self.project_unknown_event(unknown)?]),
        }
    }

    fn project_item(
        &mut self,
        payload: &Value,
        phase: ActivityLifecyclePhase,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let item = payload.get("item").ok_or_else(malformed_stream)?;
        let provider_id = required_text(item, "id")?;
        let projection = content::item_projection(item, phase, &self.qualified_version)?;
        if let Some(existing) = self.identities.get(provider_id) {
            if existing != &projection.identity {
                return Err(malformed_stream());
            }
        } else {
            self.identities
                .insert(provider_id.to_owned(), projection.identity.clone());
        }
        let activity_id = ActivityId::new(format!("codex-exec:item:{provider_id}"))
            .map_err(|_| malformed_stream())?;
        let mut observation = ActivityObservation::new(
            activity_id,
            self.operation_id.clone(),
            projection.identity.kind,
            phase,
            projection.status,
            projection.identity.assistant_phase,
            projection.identity.disclosure,
        )
        .map_err(|_| malformed_stream())?
        .with_provider_activity_ref(
            ProviderActivityRef::new(provider_id).map_err(|_| malformed_stream())?,
        );
        if let Some(content) = projection.content {
            observation = observation
                .with_content(content)
                .map_err(|_| malformed_stream())?;
        }
        Ok(observation)
    }

    fn project_unknown_event(
        &mut self,
        event_type: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.next_minted_id = self.next_minted_id.saturating_add(1);
        let namespace = ActivityNamespace::new(format!("codex.exec.event.{event_type}"))
            .map_err(|_| malformed_stream())?;
        ActivityObservation::new(
            ActivityId::new(format!("codex-exec:event:{}", self.next_minted_id))
                .map_err(|_| malformed_stream())?,
            self.operation_id.clone(),
            ActivityKind::Unknown(namespace),
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .map_err(|_| malformed_stream())
    }
}

fn required_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed_stream)
}

#[cfg(test)]
mod tests;
