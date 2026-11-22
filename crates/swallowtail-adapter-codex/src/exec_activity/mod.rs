mod content;
mod subagent;

use crate::exec_events::malformed_stream;
use semver::Version;
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use swallowtail_core::{ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase,
    ActivityNamespace, ActivityObservation, ActivityOperationId, ActivityStatus, RuntimeFailure,
    RuntimeRunId,
};

/// Maximum number of tracked item records retained per exec run.
const MAX_TRACKED_ACTIVITY_ENTRIES: usize = 2048;

#[derive(Clone, Debug, Eq, PartialEq)]
struct ItemIdentity {
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
}

struct TrackedItem {
    activity_id: ActivityId,
    identity: ItemIdentity,
    label: Option<ActivityLabel>,
    active: bool,
}

/// Bounded table of tracked exec items that preserves in-flight item identity,
/// minted IDs, and labels through completion. Completed items are evicted FIFO
/// when capacity is reached; if every slot is still active, inserting a new
/// item fails explicitly rather than evicting an in-flight record.
struct TrackedItemTable {
    entries: HashMap<String, TrackedItem>,
    order: VecDeque<String>,
    capacity: usize,
}

impl TrackedItemTable {
    fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            order: VecDeque::new(),
            capacity,
        }
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut TrackedItem> {
        self.entries.get_mut(key)
    }

    fn insert(&mut self, key: String, item: TrackedItem) -> Result<(), RuntimeFailure> {
        if let Some(existing) = self.entries.get_mut(&key) {
            *existing = item;
            return Ok(());
        }
        while self.entries.len() >= self.capacity {
            let Some(evict_index) = self
                .order
                .iter()
                .position(|candidate| self.entries.get(candidate).is_some_and(|e| !e.active))
            else {
                return Err(crate::exec::failure(
                    "swallowtail.codex.exec.activity_capacity_exceeded",
                    "Codex exec exceeded the bounded active activity capacity",
                ));
            };
            if let Some(evicted_key) = self.order.remove(evict_index) {
                self.entries.remove(&evicted_key);
            }
        }
        self.entries.insert(key.clone(), item);
        self.order.push_back(key);
        Ok(())
    }
}

pub(crate) struct ExecActivityProjection {
    operation_id: ActivityOperationId,
    qualified_version: Version,
    items: TrackedItemTable,
    provider_thread_id: Option<String>,
    next_minted_id: u64,
}

impl ExecActivityProjection {
    pub(crate) fn new(run_id: RuntimeRunId, qualified_version: Version) -> Self {
        Self::with_item_capacity(run_id, qualified_version, MAX_TRACKED_ACTIVITY_ENTRIES)
    }

    fn with_item_capacity(
        run_id: RuntimeRunId,
        qualified_version: Version,
        item_capacity: usize,
    ) -> Self {
        Self {
            operation_id: ActivityOperationId::Run(run_id),
            qualified_version,
            items: TrackedItemTable::new(item_capacity),
            provider_thread_id: None,
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
            "thread.started" => {
                self.provider_thread_id = payload
                    .get("thread_id")
                    .and_then(Value::as_str)
                    .filter(|value| !value.trim().is_empty())
                    .map(str::to_owned);
                Ok(Vec::new())
            }
            "turn.started" | "turn.completed" | "turn.failed" | "error" => Ok(Vec::new()),
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
        let projection = content::item_projection(
            item,
            phase,
            &self.qualified_version,
            self.provider_thread_id.as_deref(),
        )?;
        let active = phase != ActivityLifecyclePhase::Completed;
        let (activity_id, label) = if let Some(existing) = self.items.get_mut(provider_id) {
            if existing.identity != projection.identity {
                return Err(malformed_stream());
            }
            if let Some(label) = projection.label {
                existing.label = Some(label);
            }
            existing.active = active;
            (existing.activity_id.clone(), existing.label.clone())
        } else {
            let activity_id = self.mint_activity_id(provider_id)?;
            let label = projection.label;
            self.items.insert(
                provider_id.to_owned(),
                TrackedItem {
                    activity_id: activity_id.clone(),
                    identity: projection.identity.clone(),
                    label: label.clone(),
                    active,
                },
            )?;
            (activity_id, label)
        };
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
        if let Some(label) = label {
            observation = observation
                .with_label(label)
                .map_err(|_| malformed_stream())?;
        }
        if let Some(content) = projection.content {
            observation = observation
                .with_content(content)
                .map_err(|_| malformed_stream())?;
        }
        if item.get("type").and_then(Value::as_str) == Some("todo_list") {
            observation = observation
                .with_task_list(content::task_list_snapshot(item)?)
                .map_err(|_| malformed_stream())?;
        }
        observation = observation.with_actor(projection.subagent.actor);
        if !projection.subagent.snapshots.is_empty() {
            observation = observation
                .with_subagents(projection.subagent.snapshots)
                .map_err(|_| malformed_stream())?;
        }
        if let Some(action) = projection.subagent.control {
            observation = observation
                .with_subagent_control(action)
                .map_err(|_| malformed_stream())?;
        }
        Ok(observation)
    }

    fn mint_activity_id(&mut self, provider_id: &str) -> Result<ActivityId, RuntimeFailure> {
        let preferred = format!("codex-exec:item:{provider_id}");
        ActivityId::new(preferred)
            .or_else(|_| {
                self.next_minted_id = self.next_minted_id.saturating_add(1);
                ActivityId::new(format!("codex-exec:minted-{}", self.next_minted_id))
            })
            .map_err(|_| malformed_stream())
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

#[cfg(test)]
mod bounded_tests {
    use super::ExecActivityProjection;
    use semver::Version;
    use serde_json::json;
    use swallowtail_runtime::RuntimeRunId;

    #[test]
    fn oversized_provider_item_id_uses_minted_id_and_preserves_identity_across_turnover() {
        let mut projection = ExecActivityProjection::with_item_capacity(
            RuntimeRunId::new("exec-run-cap").expect("run id"),
            Version::new(0, 164, 0),
            3,
        );
        let long_id = format!("mcp-{}", "x".repeat(300));
        let started = projection
            .project(
                "item.started",
                &json!({
                    "item": {
                        "id": long_id,
                        "type": "mcp_tool_call",
                        "server": "github",
                        "tool": "search",
                        "status": "in_progress"
                    }
                }),
            )
            .expect("oversized provider item id must mint fallback activity id");
        let started_id = started[0].activity_id().clone();
        assert!(
            started_id.as_str().starts_with("codex-exec:minted-"),
            "expected minted id, got {}",
            started_id.as_str()
        );
        let started_label = started[0].label().cloned();
        assert_eq!(
            started_label.as_ref().map(|label| label.as_str()),
            Some("github.search")
        );

        // Cycle more completed items than table capacity to trigger FIFO eviction.
        for idx in 0..5 {
            let short_id = format!("cmd-{idx}");
            projection
                .project(
                    "item.started",
                    &json!({
                        "item": {
                            "id": short_id,
                            "type": "command_execution",
                            "command": format!("echo {idx}"),
                            "status": "in_progress"
                        }
                    }),
                )
                .expect("unrelated item starts");
            projection
                .project(
                    "item.completed",
                    &json!({
                        "item": {
                            "id": short_id,
                            "type": "command_execution",
                            "command": format!("echo {idx}"),
                            "status": "completed",
                            "exit_code": 0
                        }
                    }),
                )
                .expect("unrelated item completes");
        }

        let completed = projection
            .project(
                "item.completed",
                &json!({
                    "item": {
                        "id": long_id,
                        "type": "mcp_tool_call",
                        "server": "github",
                        "tool": "search",
                        "status": "completed"
                    }
                }),
            )
            .expect("in-flight oversized item completes cleanly");
        assert_eq!(completed[0].activity_id(), &started_id);
        assert_eq!(completed[0].label(), started_label.as_ref());
    }

    #[test]
    fn exceeding_capacity_with_all_active_items_returns_explicit_overflow_error() {
        let mut projection = ExecActivityProjection::with_item_capacity(
            RuntimeRunId::new("exec-run-overflow").expect("run id"),
            Version::new(0, 164, 0),
            2,
        );
        for idx in 0..2 {
            projection
                .project(
                    "item.started",
                    &json!({
                        "item": {
                            "id": format!("active-{idx}"),
                            "type": "command_execution",
                            "command": "sleep 1",
                            "status": "in_progress"
                        }
                    }),
                )
                .expect("active item fits within capacity");
        }
        let overflow = projection
            .project(
                "item.started",
                &json!({
                    "item": {
                        "id": "active-overflow",
                        "type": "command_execution",
                        "command": "sleep 1",
                        "status": "in_progress"
                    }
                }),
            )
            .expect_err("all-active overflow must fail explicitly");
        assert_eq!(
            overflow.diagnostic().code(),
            "swallowtail.codex.exec.activity_capacity_exceeded"
        );
    }
}
