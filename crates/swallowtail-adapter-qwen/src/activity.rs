use crate::validation::failure;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct QwenActivityProjection {
    operation_id: ActivityOperationId,
    message: Option<(String, OpenActivity)>,
    blocks: BTreeMap<u64, OpenActivity>,
    completed_messages: BTreeSet<String>,
    completed_tools: BTreeSet<String>,
    next_id: u64,
}

#[derive(Clone)]
struct OpenActivity {
    id: ActivityId,
    provider_ref: Option<ProviderActivityRef>,
    provider_key: Option<String>,
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    label: Option<ActivityLabel>,
}

impl QwenActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            message: None,
            blocks: BTreeMap::new(),
            completed_messages: BTreeSet::new(),
            completed_tools: BTreeSet::new(),
            next_id: 0,
        }
    }

    pub(crate) fn message_started(
        &mut self,
        message: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if self.message.is_some() || !self.blocks.is_empty() {
            return Err(activity_drift());
        }
        let message_id = required_string(message, "id")?;
        if message.get("role").and_then(Value::as_str) != Some("assistant")
            || self.completed_messages.contains(message_id)
        {
            return Err(activity_drift());
        }
        let activity = self.open(
            "message",
            Some(message_id),
            ActivityKind::AssistantMessage,
            Some(ActivityAssistantPhase::ProviderUnspecified),
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )?;
        self.message = Some((message_id.to_owned(), activity.clone()));
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            None,
        )?])
    }

    pub(crate) fn block_started(
        &mut self,
        index: u64,
        block: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let (message_id, _) = self.message.as_ref().ok_or_else(activity_drift)?;
        if self.blocks.contains_key(&index) {
            return Err(activity_drift());
        }
        let block_type = required_string(block, "type")?;
        let (provider_ref, kind, assistant_phase, disclosure, content) = match block_type {
            "text" => (
                Some(format!("{message_id}:content:{index}")),
                ActivityKind::AssistantMessage,
                Some(ActivityAssistantPhase::Intermediate),
                ActivityDisclosure::ProviderDisplayContent,
                None,
            ),
            "thinking" => (
                Some(format!("{message_id}:content:{index}")),
                ActivityKind::ReasoningSummary,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                None,
            ),
            "tool_use" => {
                let tool_id = required_string(block, "id")?;
                required_string(block, "name")?;
                if self.completed_tools.contains(tool_id) {
                    return Err(activity_drift());
                }
                (
                    Some(tool_id.to_owned()),
                    ActivityKind::ProviderOwnedTool,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                    None,
                )
            }
            other => (
                Some(format!("{message_id}:content:{index}")),
                ActivityKind::Unknown(namespace(&format!("qwen.headless.content-block.{other}"))?),
                None,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
            ),
        };
        let mut activity = self.open(
            "content",
            provider_ref.as_deref(),
            kind,
            assistant_phase,
            disclosure,
        )?;
        if block_type == "tool_use" {
            activity.label = activity_label(required_string(block, "name")?);
        }
        self.blocks.insert(index, activity.clone());
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            content,
        )?])
    }

    pub(crate) fn block_updated(
        &self,
        index: u64,
        delta: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self.blocks.get(&index).ok_or_else(activity_drift)?;
        let delta_type = required_string(delta, "type")?;
        let content = match (&activity.kind, delta_type) {
            (ActivityKind::AssistantMessage, "text_delta") => Some(display(
                required_string(delta, "text")?,
                ActivityContentStream::IntermediateAssistantText,
                ActivityContentChangeKind::Delta,
            )?),
            (ActivityKind::ReasoningSummary, "thinking_delta") => Some(display(
                required_string(delta, "thinking")?,
                ActivityContentStream::ReasoningSummaryText,
                ActivityContentChangeKind::Delta,
            )?),
            (ActivityKind::ProviderOwnedTool, "input_json_delta") => {
                required_string(delta, "partial_json")?;
                None
            }
            (ActivityKind::Unknown(_), _) => None,
            _ => return Err(activity_drift()),
        };
        Ok(vec![self.observation(
            activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            content,
        )?])
    }

    pub(crate) fn block_completed(
        &mut self,
        index: u64,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self.blocks.remove(&index).ok_or_else(activity_drift)?;
        if matches!(activity.kind, ActivityKind::ProviderOwnedTool)
            && let Some(provider_key) = activity.provider_key.as_ref()
        {
            self.completed_tools.insert(provider_key.clone());
        }
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
        )?])
    }

    pub(crate) fn message_completed(&mut self) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if !self.blocks.is_empty() {
            return Err(activity_drift());
        }
        let (message_id, activity) = self.message.take().ok_or_else(activity_drift)?;
        self.completed_messages.insert(message_id);
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
        )?])
    }

    pub(crate) fn completed_assistant(
        &mut self,
        message: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let message_id = required_string(message, "id")?;
        if message.get("role").and_then(Value::as_str) != Some("assistant") {
            return Err(activity_drift());
        }
        let blocks = message
            .get("content")
            .and_then(Value::as_array)
            .ok_or_else(activity_drift)?;
        let mut observations = Vec::new();
        if !self.completed_messages.contains(message_id) {
            let text = blocks
                .iter()
                .filter(|block| block.get("type").and_then(Value::as_str) == Some("text"))
                .map(|block| required_string(block, "text"))
                .collect::<Result<String, _>>()?;
            let activity = self.open(
                "assistant",
                Some(message_id),
                ActivityKind::AssistantMessage,
                Some(ActivityAssistantPhase::Final),
                ActivityDisclosure::ProviderDisplayContent,
            )?;
            observations.push(
                self.observation(
                    &activity,
                    ActivityLifecyclePhase::Completed,
                    ActivityStatus::Completed,
                    (!text.is_empty())
                        .then(|| {
                            display(
                                &text,
                                ActivityContentStream::FinalAnswerText,
                                ActivityContentChangeKind::ReplacementSnapshot,
                            )
                        })
                        .transpose()?,
                )?,
            );
            self.completed_messages.insert(message_id.to_owned());
        }
        for block in blocks {
            if block.get("type").and_then(Value::as_str) != Some("tool_use") {
                continue;
            }
            let tool_id = required_string(block, "id")?;
            let name = required_string(block, "name")?;
            if self.completed_tools.insert(tool_id.to_owned()) {
                let mut activity = self.open(
                    "tool",
                    Some(tool_id),
                    ActivityKind::ProviderOwnedTool,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                )?;
                activity.label = activity_label(name);
                observations.push(self.observation(
                    &activity,
                    ActivityLifecyclePhase::Completed,
                    ActivityStatus::Completed,
                    None,
                )?);
            }
        }
        Ok(observations)
    }

    pub(crate) fn unknown(
        &mut self,
        event_type: &str,
        provider_ref: Option<&str>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self.open(
            "unknown",
            provider_ref,
            ActivityKind::Unknown(namespace(&format!("qwen.headless.{event_type}"))?),
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )?;
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
        )?])
    }

    pub(crate) fn ensure_idle(&self) -> Result<(), RuntimeFailure> {
        if self.message.is_none() && self.blocks.is_empty() {
            Ok(())
        } else {
            Err(activity_drift())
        }
    }

    fn open(
        &mut self,
        label: &str,
        provider_ref: Option<&str>,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
    ) -> Result<OpenActivity, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        Ok(OpenActivity {
            id: ActivityId::new(format!("qwen-headless:{label}:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            provider_ref: provider_ref
                .map(ProviderActivityRef::new)
                .transpose()
                .map_err(|_| activity_drift())?,
            provider_key: provider_ref.map(str::to_owned),
            kind,
            assistant_phase,
            disclosure,
            label: None,
        })
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
        .map_err(|_| activity_drift())?;
        if let Some(provider_ref) = activity.provider_ref.clone() {
            observation = observation.with_provider_activity_ref(provider_ref);
        }
        if let Some(label) = activity.label.clone() {
            observation = observation
                .with_label(label)
                .map_err(|_| activity_drift())?;
        }
        if let Some(content) = content {
            observation = observation
                .with_content(content)
                .map_err(|_| activity_drift())?;
        }
        Ok(observation)
    }
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(activity_drift)
}

fn namespace(value: &str) -> Result<ActivityNamespace, RuntimeFailure> {
    ActivityNamespace::new(value).map_err(|_| activity_drift())
}

fn activity_label(value: &str) -> Option<ActivityLabel> {
    ActivityLabel::new(value.trim()).ok()
}

fn display(
    value: &str,
    stream: ActivityContentStream,
    change: ActivityContentChangeKind,
) -> Result<ActivityContentUpdate, RuntimeFailure> {
    let value = OperationContent::new(value.to_owned()).map_err(|_| activity_drift())?;
    let value = ActivityContent::new(value, MAXIMUM_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| activity_drift())?;
    Ok(ActivityContentUpdate::new(change, stream, value))
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.qwen.headless.activity_invalid",
        "Qwen headless activity did not match the qualified stream",
    )
}

#[cfg(test)]
mod tests;
