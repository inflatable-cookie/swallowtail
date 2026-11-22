mod content;
mod extension;
mod item;
mod projection;
mod request;
mod subagent;

use crate::turn_state::malformed_notification;
use item::ItemIdentity;
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use swallowtail_core::{ProviderActivityRef, ProviderRequestRef};
use swallowtail_runtime::{
    ActivityCorrelation, ActivityId, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, RuntimeFailure, RuntimeTurnId,
};

/// Maximum number of tracked item/identity/label/correlation entries per turn.
const MAX_TRACKED_ACTIVITY_ENTRIES: usize = 2048;
/// Maximum number of concurrent pending server requests tracked per turn.
const MAX_TRACKED_REQUESTS: usize = 256;

struct BoundedMap<K, V> {
    entries: HashMap<K, V>,
    order: VecDeque<K>,
    capacity: usize,
}

impl<K: Eq + Hash + Clone, V> BoundedMap<K, V> {
    fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            order: VecDeque::new(),
            capacity,
        }
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.entries.get(key)
    }

    fn insert(&mut self, key: K, value: V) {
        if let Some(existing) = self.entries.get_mut(&key) {
            *existing = value;
            return;
        }
        while self.entries.len() >= self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.entries.remove(&oldest);
            } else {
                break;
            }
        }
        self.entries.insert(key.clone(), value);
        self.order.push_back(key);
    }

    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let removed = self.entries.remove(key)?;
        self.order.retain(|existing| existing.borrow() != key);
        Some(removed)
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.entries.len()
    }
}

struct TrackedItem {
    activity_id: ActivityId,
    identity: ItemIdentity,
    label: Option<ActivityLabel>,
    active: bool,
}

/// Bounded table of tracked items that preserves in-flight item identity,
/// metadata, and labels through completion. Completed items are evicted FIFO
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

    fn get(&self, key: &str) -> Option<&TrackedItem> {
        self.entries.get(key)
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
                return Err(crate::rpc::failure(
                    "swallowtail.codex.app_server.activity_capacity_exceeded",
                    "Codex app-server exceeded the bounded active activity capacity",
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

pub(crate) struct AppServerActivityProjection {
    operation_id: ActivityOperationId,
    root_provider_thread_id: String,
    identities: BoundedMap<String, ActivityId>,
    items: TrackedItemTable,
    correlations: BoundedMap<String, ActivityCorrelation>,
    requests: BoundedMap<ProviderRequestRef, RequestIdentity>,
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
    pub(crate) fn new(turn_id: RuntimeTurnId, root_provider_thread_id: String) -> Self {
        Self::with_item_capacity(
            turn_id,
            root_provider_thread_id,
            MAX_TRACKED_ACTIVITY_ENTRIES,
        )
    }

    fn with_item_capacity(
        turn_id: RuntimeTurnId,
        root_provider_thread_id: String,
        item_capacity: usize,
    ) -> Self {
        Self {
            operation_id: ActivityOperationId::Turn(turn_id),
            root_provider_thread_id,
            identities: BoundedMap::new(MAX_TRACKED_ACTIVITY_ENTRIES),
            items: TrackedItemTable::new(item_capacity),
            correlations: BoundedMap::new(MAX_TRACKED_ACTIVITY_ENTRIES),
            requests: BoundedMap::new(MAX_TRACKED_REQUESTS),
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
        let (activity_id, label) = if let Some(tracked) = self.items.get(source.identity_key) {
            (tracked.activity_id.clone(), tracked.label.clone())
        } else {
            (
                self.activity_id(&format!("item:{}", source.identity_key))?,
                None,
            )
        };
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

    fn mint_activity_id(&mut self, provider_key: &str) -> Result<ActivityId, RuntimeFailure> {
        let preferred = format!("codex-app-server:{provider_key}");
        ActivityId::new(preferred)
            .or_else(|_| {
                self.next_minted_id = self.next_minted_id.saturating_add(1);
                ActivityId::new(format!("codex-app-server:minted-{}", self.next_minted_id))
            })
            .map_err(|_| malformed_notification())
    }

    fn activity_id(&mut self, provider_key: &str) -> Result<ActivityId, RuntimeFailure> {
        if let Some(identity) = self.identities.get(provider_key) {
            return Ok(identity.clone());
        }
        let identity = self.mint_activity_id(provider_key)?;
        self.identities
            .insert(provider_key.to_owned(), identity.clone());
        Ok(identity)
    }

    fn track_item(
        &mut self,
        provider_id: &str,
        identity: ItemIdentity,
        label: Option<ActivityLabel>,
        active: bool,
    ) -> Result<(), RuntimeFailure> {
        if let Some(existing) = self.items.get_mut(provider_id) {
            if existing.identity != identity {
                return Err(malformed_notification());
            }
            if let Some(label) = label {
                existing.label = Some(label);
            }
            existing.active = active;
            return Ok(());
        }
        let activity_id = self.mint_activity_id(&format!("item:{provider_id}"))?;
        self.items.insert(
            provider_id.to_owned(),
            TrackedItem {
                activity_id,
                identity,
                label,
                active,
            },
        )
    }

    fn get_or_track_item(
        &mut self,
        provider_id: &str,
        make_identity: impl FnOnce() -> ItemIdentity,
    ) -> Result<ItemIdentity, RuntimeFailure> {
        if let Some(existing) = self.items.get(provider_id) {
            return Ok(existing.identity.clone());
        }
        let identity = make_identity();
        let activity_id = self.mint_activity_id(&format!("item:{provider_id}"))?;
        self.items.insert(
            provider_id.to_owned(),
            TrackedItem {
                activity_id,
                identity: identity.clone(),
                label: None,
                active: true,
            },
        )?;
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

#[cfg(test)]
mod bounded_map_tests {
    use super::{AppServerActivityProjection, BoundedMap};
    use serde_json::json;
    use swallowtail_runtime::RuntimeTurnId;

    #[test]
    fn bounded_map_evicts_oldest_and_updates_in_place() {
        let mut map = BoundedMap::new(3);
        map.insert("a".to_owned(), 1);
        map.insert("b".to_owned(), 2);
        map.insert("c".to_owned(), 3);
        map.insert("b".to_owned(), 20);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("b"), Some(&20));
        map.insert("d".to_owned(), 4);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("a"), None);
        assert_eq!(map.get("d"), Some(&4));
        assert_eq!(map.remove("c"), Some(3));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn in_flight_item_preserves_minted_id_and_label_across_capacity_turnover() {
        let mut projection = AppServerActivityProjection::with_item_capacity(
            RuntimeTurnId::new("turn-cap").expect("turn id"),
            "thread-root".to_owned(),
            3,
        );
        // Provider ID longer than ActivityId's max length forces a minted ID.
        let long_id = format!("mcp-{}", "x".repeat(200));
        let started = projection
            .project_notification(
                "item/started",
                &json!({
                    "item": {
                        "id": long_id,
                        "type": "mcpToolCall",
                        "server": "github",
                        "tool": "search",
                        "status": "inProgress"
                    }
                }),
            )
            .expect("long in-flight item starts");
        let started_id = started[0].activity_id().clone();
        let started_label = started[0].label().cloned();
        assert_eq!(
            started_label.as_ref().map(|label| label.as_str()),
            Some("github.search"),
            "mcpToolCall carries a label"
        );

        // Cycle more completed items than the table capacity.
        for idx in 0..5 {
            let short_id = format!("other-{idx}");
            projection
                .project_notification(
                    "item/started",
                    &json!({
                        "item": {
                            "id": short_id,
                            "type": "commandExecution",
                            "command": format!("echo {idx}"),
                            "status": "inProgress"
                        }
                    }),
                )
                .expect("unrelated item starts");
            projection
                .project_notification(
                    "item/completed",
                    &json!({
                        "item": {
                            "id": short_id,
                            "type": "commandExecution",
                            "command": format!("echo {idx}"),
                            "status": "completed"
                        }
                    }),
                )
                .expect("unrelated item completes");
        }

        // Completing the original in-flight item must preserve both its minted ID and label.
        let completed = projection
            .project_notification(
                "item/completed",
                &json!({
                    "item": {
                        "id": long_id,
                        "type": "mcpToolCall",
                        "status": "completed"
                    }
                }),
            )
            .expect("original in-flight item completes");
        assert_eq!(completed[0].activity_id(), &started_id);
        assert_eq!(completed[0].label(), started_label.as_ref());
    }

    #[test]
    fn exceeding_capacity_with_all_active_items_returns_explicit_overflow_error() {
        let mut projection = AppServerActivityProjection::with_item_capacity(
            RuntimeTurnId::new("turn-overflow").expect("turn id"),
            "thread-root".to_owned(),
            2,
        );
        for idx in 0..2 {
            projection
                .project_notification(
                    "item/started",
                    &json!({
                        "item": {
                            "id": format!("active-{idx}"),
                            "type": "commandExecution",
                            "command": "sleep 1",
                            "status": "inProgress"
                        }
                    }),
                )
                .expect("active item fits within capacity");
        }
        let overflow = projection
            .project_notification(
                "item/started",
                &json!({
                    "item": {
                        "id": "active-overflow",
                        "type": "commandExecution",
                        "command": "sleep 1",
                        "status": "inProgress"
                    }
                }),
            )
            .expect_err("all-active overflow must fail explicitly");
        assert_eq!(
            overflow.diagnostic().code(),
            "swallowtail.codex.app_server.activity_capacity_exceeded"
        );
    }
}
