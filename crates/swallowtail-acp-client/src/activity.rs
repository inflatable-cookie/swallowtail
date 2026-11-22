use crate::diagnostics::{AcpActivityConfig, MessageClassification, PlanProjection};
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_protocol_acp::{
    AcpContentBlock, AcpMessageChunk, AcpMessageRole, AcpPlanEntry, AcpPlanEntryPriority,
    AcpPlanEntryStatus, AcpSessionUpdate, AcpToolCall, AcpToolCallContent, AcpToolCallStatus,
    AcpToolCallUpdate, AcpToolKind,
};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure,
    RuntimeTurnId, TaskListItem, TaskListItemPriority, TaskListItemStatus, TaskListSnapshot,
    TerminalStatus,
};

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

/// Projects ACP v1 `session/update` payloads into portable [`ActivityObservation`] values.
pub struct AcpActivityProjection {
    config: AcpActivityConfig,
    operation_id: ActivityOperationId,
    open: BTreeMap<String, OpenActivity>,
    closed: BTreeSet<String>,
    next_id: u64,
}

#[derive(Clone)]
struct OpenActivity {
    id: ActivityId,
    provider_ref: Option<ProviderActivityRef>,
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    status: ActivityStatus,
    label: Option<ActivityLabel>,
    tool_kind: Option<AcpToolKind>,
}

impl AcpActivityProjection {
    /// Creates an empty activity projection bound to one runtime turn and adapter config.
    #[must_use]
    pub fn new(turn_id: RuntimeTurnId, config: AcpActivityConfig) -> Self {
        Self {
            config,
            operation_id: ActivityOperationId::Turn(turn_id),
            open: BTreeMap::new(),
            closed: BTreeSet::new(),
            next_id: 0,
        }
    }

    /// Projects one decoded ACP session update into zero or more activity observations.
    pub fn project(
        &mut self,
        update: &AcpSessionUpdate,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        match update {
            AcpSessionUpdate::Message(message) => self.message(message),
            AcpSessionUpdate::ToolCall(call) => self.tool_start(call),
            AcpSessionUpdate::ToolCallUpdate(update) => self.tool_update(update),
            AcpSessionUpdate::Plan(entries)
                if self.config.plan_projection == PlanProjection::TaskList =>
            {
                self.plan(entries)
            }
            AcpSessionUpdate::Unknown { namespace } => self.unknown(namespace.as_str()),
            AcpSessionUpdate::Plan(_)
            | AcpSessionUpdate::AvailableCommands(_)
            | AcpSessionUpdate::CurrentMode(_)
            | AcpSessionUpdate::ConfigOptions(_)
            | AcpSessionUpdate::SessionInfo { .. }
            | AcpSessionUpdate::Usage(_) => Ok(Vec::new()),
        }
    }

    /// Completes all still-open activities when the owning turn reaches a terminal status.
    pub fn complete(
        &mut self,
        terminal: &TerminalStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let Some(status) = terminal_status(terminal) else {
            return Ok(Vec::new());
        };
        std::mem::take(&mut self.open)
            .into_values()
            .map(|activity| {
                self.observation(&activity, ActivityLifecyclePhase::Completed, status, None)
            })
            .collect()
    }

    /// Returns whether the given tool call is currently open in `Pending` status with `Execute` kind.
    #[must_use]
    pub fn is_pending_execute(&self, tool_call_id: &str) -> bool {
        let key = format!("tool:{tool_call_id}");
        self.open.get(&key).is_some_and(|activity| {
            activity.status == ActivityStatus::Pending
                && activity.tool_kind == Some(AcpToolKind::Execute)
        })
    }

    pub(crate) fn open_count(&self) -> usize {
        self.open.len()
    }

    /// Computes `(observation_count, next_open_count)` for `update` without
    /// mutating or cloning `open` / `closed` history.
    pub(crate) fn projected_capacity(&self, update: &AcpSessionUpdate) -> (usize, usize) {
        let open = self.open.len();
        match update {
            AcpSessionUpdate::Message(message) => {
                if message.role == AcpMessageRole::User {
                    return (0, open);
                }
                let Ok(text) = text_content(&message.content, self.config.malformed) else {
                    return (0, open);
                };
                let label = match message.role {
                    AcpMessageRole::Agent
                        if self.config.message_classification
                            == MessageClassification::ModeAndWarning
                            && text.starts_with("[MODE_UPDATE]") =>
                    {
                        "mode-display"
                    }
                    AcpMessageRole::Agent => "assistant",
                    AcpMessageRole::Thought
                        if self.config.message_classification
                            == MessageClassification::ModeAndWarning
                            && text.starts_with("Warning:") =>
                    {
                        "warning"
                    }
                    AcpMessageRole::Thought => "thought",
                    AcpMessageRole::User => unreachable!("user messages returned above"),
                };
                let provider_id = message.message_id.as_ref().map(|value| value.as_str());
                let key = format!("{label}:{}", provider_id.unwrap_or("turn"));
                let next_open = open.saturating_add(usize::from(!self.open.contains_key(&key)));
                (1, next_open)
            }
            AcpSessionUpdate::Plan(_)
                if self.config.plan_projection == PlanProjection::TaskList =>
            {
                let next_open = open.saturating_add(usize::from(!self.open.contains_key("plan")));
                (1, next_open)
            }
            AcpSessionUpdate::ToolCall(call) => {
                let key = format!("tool:{}", call.tool_call_id.as_str());
                if self.closed.contains(&key) {
                    return (0, open);
                }
                let status = tool_status(call.status);
                if let Some(existing) = self.open.get(&key) {
                    let reconciled = reconcile_status(existing.status, status);
                    let next_open = if reconciled.is_terminal() {
                        open.saturating_sub(1)
                    } else {
                        open
                    };
                    (1, next_open)
                } else if status.is_terminal() {
                    (2, open)
                } else {
                    (1, open.saturating_add(1))
                }
            }
            AcpSessionUpdate::ToolCallUpdate(update) => {
                let key = format!("tool:{}", update.tool_call_id.as_str());
                if self.closed.contains(&key) {
                    return (0, open);
                }
                if let Some(existing) = self.open.get(&key) {
                    let reconciled = update.status.map_or(existing.status, |status| {
                        reconcile_status(existing.status, tool_status(status))
                    });
                    let next_open = if reconciled.is_terminal() {
                        open.saturating_sub(1)
                    } else {
                        open
                    };
                    (1, next_open)
                } else if update
                    .status
                    .map(tool_status)
                    .is_some_and(ActivityStatus::is_terminal)
                {
                    (2, open)
                } else {
                    (0, open)
                }
            }
            AcpSessionUpdate::Unknown { .. } => (1, open),
            AcpSessionUpdate::Plan(_)
            | AcpSessionUpdate::AvailableCommands(_)
            | AcpSessionUpdate::CurrentMode(_)
            | AcpSessionUpdate::ConfigOptions(_)
            | AcpSessionUpdate::SessionInfo { .. }
            | AcpSessionUpdate::Usage(_) => (0, open),
        }
    }

    fn malformed(&self) -> RuntimeFailure {
        (self.config.malformed)()
    }

    fn message(
        &mut self,
        message: &AcpMessageChunk,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if message.role == AcpMessageRole::User {
            return Ok(Vec::new());
        }
        let text = text_content(&message.content, self.config.malformed)?;
        let provider_id = message.message_id.as_ref().map(|value| value.as_str());
        let (kind, phase, disclosure, stream, label) = match message.role {
            AcpMessageRole::Agent
                if self.config.message_classification == MessageClassification::ModeAndWarning
                    && text.starts_with("[MODE_UPDATE]") =>
            {
                (
                    ActivityKind::AssistantMessage,
                    Some(ActivityAssistantPhase::ProviderUnspecified),
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                    None,
                    "mode-display",
                )
            }
            AcpMessageRole::Agent => (
                ActivityKind::AssistantMessage,
                Some(ActivityAssistantPhase::Final),
                ActivityDisclosure::ProviderDisplayContent,
                Some(ActivityContentStream::FinalAnswerText),
                "assistant",
            ),
            AcpMessageRole::Thought
                if self.config.message_classification == MessageClassification::ModeAndWarning
                    && text.starts_with("Warning:") =>
            {
                (
                    ActivityKind::WarningOrError,
                    None,
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                    None,
                    "warning",
                )
            }
            AcpMessageRole::Thought => (
                ActivityKind::ReasoningSummary,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                Some(ActivityContentStream::ReasoningSummaryText),
                "thought",
            ),
            AcpMessageRole::User => unreachable!("user messages returned above"),
        };
        let key = format!("{label}:{}", provider_id.unwrap_or("turn"));
        let (activity, is_new, next_id) = self.prepare_activity(
            &key,
            provider_id,
            kind,
            phase,
            disclosure,
            ActivityStatus::InProgress,
        )?;
        let content = stream
            .map(|stream| {
                content_update(
                    text,
                    ActivityContentChangeKind::Delta,
                    stream,
                    self.config.malformed,
                )
            })
            .transpose()?
            .flatten();
        let observation = self.observation(
            &activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            content,
        )?;
        self.next_id = next_id;
        if is_new {
            self.open.insert(key, activity);
        }
        Ok(vec![observation])
    }

    fn plan(
        &mut self,
        entries: &[AcpPlanEntry],
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let (activity, is_new, next_id) = self.prepare_activity(
            "plan",
            None,
            ActivityKind::Plan,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            ActivityStatus::InProgress,
        )?;
        let display = entries
            .iter()
            .map(|entry| entry.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let content = content_update(
            &display,
            ActivityContentChangeKind::ReplacementSnapshot,
            ActivityContentStream::PlanText,
            self.config.malformed,
        )?;
        let tasks = task_list(entries, self.config.malformed)?;
        let observation = self
            .observation(
                &activity,
                ActivityLifecyclePhase::Updated,
                ActivityStatus::InProgress,
                content,
            )?
            .with_task_list(tasks)
            .map_err(|_| self.malformed())?;
        self.next_id = next_id;
        if is_new {
            self.open.insert("plan".to_owned(), activity);
        }
        Ok(vec![observation])
    }

    fn tool_start(
        &mut self,
        call: &AcpToolCall,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let key = format!("tool:{}", call.tool_call_id.as_str());
        if self.closed.contains(&key) {
            return Ok(Vec::new());
        }
        let status = tool_status(call.status);
        let label = activity_label(call.title.as_str());
        let content = tool_content(&call.content, self.config.malformed)?;
        if let Some(mut activity) = self.open.get(&key).cloned() {
            if label.is_some() {
                activity.label = label;
            }
            activity.tool_kind = Some(call.kind.clone());
            let status = reconcile_status(activity.status, status);
            let phase = if status.is_terminal() {
                ActivityLifecyclePhase::Completed
            } else {
                ActivityLifecyclePhase::Updated
            };
            activity.status = status;
            let observation = self.observation(&activity, phase, status, content)?;
            if status.is_terminal() {
                self.open.remove(&key);
                self.closed.insert(key);
            } else {
                self.open.insert(key, activity);
            }
            return Ok(vec![observation]);
        }
        if status.is_terminal() {
            let (mut activity, _, next_id) = self.prepare_activity(
                &key,
                Some(call.tool_call_id.as_str()),
                ActivityKind::ProviderOwnedTool,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                ActivityStatus::InProgress,
            )?;
            activity.label = label;
            activity.tool_kind = Some(call.kind.clone());
            let started = self.observation(
                &activity,
                ActivityLifecyclePhase::Started,
                ActivityStatus::InProgress,
                None,
            )?;
            let completed = self.observation(
                &activity,
                ActivityLifecyclePhase::Completed,
                status,
                content,
            )?;
            self.next_id = next_id;
            self.closed.insert(key);
            return Ok(vec![started, completed]);
        }
        let (mut activity, _, next_id) = self.prepare_activity(
            &key,
            Some(call.tool_call_id.as_str()),
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            status,
        )?;
        activity.label = label;
        activity.tool_kind = Some(call.kind.clone());
        let observation =
            self.observation(&activity, ActivityLifecyclePhase::Started, status, content)?;
        self.next_id = next_id;
        self.open.insert(key, activity);
        Ok(vec![observation])
    }

    fn tool_update(
        &mut self,
        update: &AcpToolCallUpdate,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let key = format!("tool:{}", update.tool_call_id.as_str());
        if self.closed.contains(&key) {
            return Ok(Vec::new());
        }
        let activity = self.open.get(&key).cloned();
        let orphan_terminal_status = if activity.is_none() {
            let Some(status) = update.status.map(tool_status) else {
                return Ok(Vec::new());
            };
            if !status.is_terminal() {
                return Ok(Vec::new());
            }
            Some(status)
        } else {
            None
        };
        let label = update
            .title
            .as_ref()
            .and_then(|title| activity_label(title.as_str()));
        let content = update
            .content_replacement
            .as_ref()
            .map(|content| tool_content(content, self.config.malformed))
            .transpose()?
            .flatten();
        let Some(mut activity) = activity else {
            let Some(status) = orphan_terminal_status else {
                return Ok(Vec::new());
            };
            let (mut activity, _, next_id) = self.prepare_activity(
                &key,
                Some(update.tool_call_id.as_str()),
                ActivityKind::ProviderOwnedTool,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                ActivityStatus::InProgress,
            )?;
            activity.label = label;
            activity.tool_kind = update.kind.clone();
            let started = self.observation(
                &activity,
                ActivityLifecyclePhase::Started,
                ActivityStatus::InProgress,
                None,
            )?;
            let completed = self.observation(
                &activity,
                ActivityLifecyclePhase::Completed,
                status,
                content,
            )?;
            self.next_id = next_id;
            self.closed.insert(key);
            return Ok(vec![started, completed]);
        };
        if label.is_some() {
            activity.label = label;
        }
        if update.kind.is_some() {
            activity.tool_kind = update.kind.clone();
        }
        let status = update.status.map_or(activity.status, |status| {
            reconcile_status(activity.status, tool_status(status))
        });
        let phase = if status.is_terminal() {
            ActivityLifecyclePhase::Completed
        } else {
            ActivityLifecyclePhase::Updated
        };
        activity.status = status;
        let observation = self.observation(&activity, phase, status, content)?;
        if status.is_terminal() {
            self.open.remove(&key);
            self.closed.insert(key);
        } else {
            self.open.insert(key, activity);
        }
        Ok(vec![observation])
    }

    fn unknown(&mut self, namespace: &str) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let next_id = self.next_id.saturating_add(1);
        let activity = OpenActivity {
            id: ActivityId::new(format!("{}:unknown:{next_id}", self.config.id_prefix))
                .map_err(|_| self.malformed())?,
            provider_ref: None,
            kind: ActivityKind::Unknown(
                ActivityNamespace::new(format!("{}.{namespace}", self.config.namespace_prefix))
                    .map_err(|_| self.malformed())?,
            ),
            assistant_phase: None,
            disclosure: ActivityDisclosure::IdentityAndLifecycleOnly,
            status: ActivityStatus::Completed,
            label: None,
            tool_kind: None,
        };
        let observation = self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
        )?;
        self.next_id = next_id;
        Ok(vec![observation])
    }

    fn prepare_activity(
        &self,
        key: &str,
        provider_ref: Option<&str>,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
        status: ActivityStatus,
    ) -> Result<(OpenActivity, bool, u64), RuntimeFailure> {
        if let Some(activity) = self.open.get(key) {
            return Ok((activity.clone(), false, self.next_id));
        }
        let mut next_id = self.next_id;
        let id = ActivityId::new(format!("{}:{key}", self.config.id_prefix))
            .or_else(|_| {
                next_id = next_id.saturating_add(1);
                ActivityId::new(format!("{}:minted:{next_id}", self.config.id_prefix))
            })
            .map_err(|_| self.malformed())?;
        let provider_ref = provider_ref
            .map(ProviderActivityRef::new)
            .transpose()
            .map_err(|_| self.malformed())?;
        let activity = OpenActivity {
            id,
            provider_ref,
            kind,
            assistant_phase,
            disclosure,
            status,
            label: None,
            tool_kind: None,
        };
        Ok((activity, true, next_id))
    }

    fn observation(
        &self,
        activity: &OpenActivity,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let mut observation = ActivityObservation::new(
            activity.id.clone(),
            self.operation_id.clone(),
            activity.kind.clone(),
            phase,
            status,
            activity.assistant_phase,
            activity.disclosure,
        )
        .map_err(|_| self.malformed())?;
        if let Some(reference) = activity.provider_ref.clone() {
            observation = observation.with_provider_activity_ref(reference);
        }
        if let Some(label) = activity.label.clone() {
            observation = observation
                .with_label(label)
                .map_err(|_| self.malformed())?;
        }
        if let Some(content) = content {
            observation = observation
                .with_content(content)
                .map_err(|_| self.malformed())?;
        }
        Ok(observation)
    }
}

/// Extracts text from an [`AcpContentBlock`], rejecting non-text blocks with `malformed`.
pub fn text_content(
    content: &AcpContentBlock,
    malformed: fn() -> RuntimeFailure,
) -> Result<&str, RuntimeFailure> {
    match content {
        AcpContentBlock::Text(text) => Ok(text.as_str()),
        _ => Err(malformed()),
    }
}

fn tool_content(
    content: &[AcpToolCallContent],
    malformed: fn() -> RuntimeFailure,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let mut display = String::new();
    for item in content {
        let text = match item {
            AcpToolCallContent::Content(AcpContentBlock::Text(text)) => text.as_str(),
            AcpToolCallContent::Diff { new_text, .. } => new_text.as_str(),
            _ => continue,
        };
        if !display.is_empty() {
            display.push('\n');
        }
        display.push_str(text);
    }
    content_update(
        &display,
        ActivityContentChangeKind::ReplacementSnapshot,
        ActivityContentStream::ProviderToolDisplay,
        malformed,
    )
}

fn content_update(
    text: &str,
    change: ActivityContentChangeKind,
    stream: ActivityContentStream,
    malformed: fn() -> RuntimeFailure,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let text = bounded_content(text);
    if text.trim().is_empty() {
        return Ok(None);
    }
    let content = ActivityContent::new(
        OperationContent::new(text).map_err(|_| malformed())?,
        MAXIMUM_ACTIVITY_CONTENT_BYTES,
    )
    .map_err(|_| malformed())?;
    Ok(Some(ActivityContentUpdate::new(change, stream, content)))
}

fn bounded_content(text: &str) -> &str {
    let mut end = text.len().min(MAXIMUM_ACTIVITY_CONTENT_BYTES);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}

fn task_list(
    entries: &[AcpPlanEntry],
    malformed: fn() -> RuntimeFailure,
) -> Result<TaskListSnapshot, RuntimeFailure> {
    let items = entries
        .iter()
        .map(|entry| {
            let content = OperationContent::new(entry.content.as_str()).map_err(|_| malformed())?;
            let status = match entry.status {
                AcpPlanEntryStatus::Pending => TaskListItemStatus::Pending,
                AcpPlanEntryStatus::InProgress => TaskListItemStatus::InProgress,
                AcpPlanEntryStatus::Completed => TaskListItemStatus::Completed,
            };
            let priority = match entry.priority {
                AcpPlanEntryPriority::Low => TaskListItemPriority::Low,
                AcpPlanEntryPriority::Medium => TaskListItemPriority::Medium,
                AcpPlanEntryPriority::High => TaskListItemPriority::High,
            };
            Ok(TaskListItem::new(content, status).with_priority(priority))
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    TaskListSnapshot::new(items, 256, 4 * 1024 * 1024).map_err(|_| malformed())
}

fn tool_status(status: AcpToolCallStatus) -> ActivityStatus {
    match status {
        AcpToolCallStatus::Pending => ActivityStatus::Pending,
        AcpToolCallStatus::InProgress => ActivityStatus::InProgress,
        AcpToolCallStatus::Completed => ActivityStatus::Completed,
        AcpToolCallStatus::Failed => ActivityStatus::Failed,
    }
}

fn terminal_status(status: &TerminalStatus) -> Option<ActivityStatus> {
    match status {
        TerminalStatus::Detached => None,
        TerminalStatus::Completed => Some(ActivityStatus::Completed),
        TerminalStatus::Cancelled
        | TerminalStatus::TimedOut
        | TerminalStatus::ProviderRequestObserved(_) => Some(ActivityStatus::Cancelled),
        TerminalStatus::ProviderFailed(_)
        | TerminalStatus::HostFailed(_)
        | TerminalStatus::RuntimeFailed(_) => Some(ActivityStatus::Failed),
    }
}

fn activity_label(value: &str) -> Option<ActivityLabel> {
    ActivityLabel::new(value.trim()).ok()
}

fn reconcile_status(current: ActivityStatus, next: ActivityStatus) -> ActivityStatus {
    if current == ActivityStatus::InProgress && next == ActivityStatus::Pending {
        current
    } else {
        next
    }
}
