use crate::failure::failure;
use serde_json::Value;
use std::collections::BTreeMap;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct ClaudeCodeActivityProjection {
    operation_id: ActivityOperationId,
    pending_tools: BTreeMap<String, Option<ActivityLabel>>,
    next_id: u64,
}

impl ClaudeCodeActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            pending_tools: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn assistant(
        &mut self,
        message: &Value,
        failed: bool,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let message_id = required_string(message, "id")?;
        if message.get("role").and_then(Value::as_str) != Some("assistant") {
            return Err(activity_drift());
        }
        let stop_reason = required_string(message, "stop_reason")?;
        let blocks = message
            .get("content")
            .and_then(Value::as_array)
            .ok_or_else(activity_drift)?;
        let text = blocks
            .iter()
            .filter(|block| block.get("type").and_then(Value::as_str) == Some("text"))
            .map(|block| required_string(block, "text"))
            .collect::<Result<String, _>>()?;
        let final_text = stop_reason == "end_turn" && !text.is_empty();
        let mut observations = vec![
            self.completed(
                "assistant",
                Some(message_id),
                ActivityKind::AssistantMessage,
                Some(if final_text {
                    ActivityAssistantPhase::Final
                } else {
                    ActivityAssistantPhase::ProviderUnspecified
                }),
                if final_text {
                    ActivityDisclosure::ProviderDisplayContent
                } else {
                    ActivityDisclosure::IdentityAndLifecycleOnly
                },
                if failed {
                    ActivityStatus::Failed
                } else {
                    ActivityStatus::Completed
                },
                final_text
                    .then(|| {
                        display(
                            &text,
                            ActivityContentStream::FinalAnswerText,
                            ActivityContentChangeKind::ReplacementSnapshot,
                        )
                    })
                    .transpose()?,
            )?,
        ];
        for block in blocks {
            if block.get("type").and_then(Value::as_str) != Some("tool_use") {
                continue;
            }
            let tool_id = required_string(block, "id")?;
            let name = required_string(block, "name")?;
            let label = activity_label(name);
            if self
                .pending_tools
                .insert(tool_id.to_owned(), label.clone())
                .is_some()
            {
                return Err(activity_drift());
            }
            let observation = self.completed(
                "tool-use",
                Some(tool_id),
                ActivityKind::ProviderOwnedTool,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                ActivityStatus::Completed,
                None,
            )?;
            observations.push(with_label(observation, label)?);
        }
        Ok(observations)
    }

    pub(crate) fn tool_results(
        &mut self,
        message: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if message.get("role").and_then(Value::as_str) != Some("user") {
            return Err(activity_drift());
        }
        let blocks = message
            .get("content")
            .and_then(Value::as_array)
            .ok_or_else(activity_drift)?;
        let mut observations = Vec::new();
        for block in blocks {
            if block.get("type").and_then(Value::as_str) != Some("tool_result") {
                return Err(activity_drift());
            }
            let tool_id = required_string(block, "tool_use_id")?;
            let label = self
                .pending_tools
                .remove(tool_id)
                .ok_or_else(activity_drift)?;
            let failed = block
                .get("is_error")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let observation = self.completed(
                "tool-result",
                Some(tool_id),
                ActivityKind::ProviderOwnedTool,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                if failed {
                    ActivityStatus::Failed
                } else {
                    ActivityStatus::Completed
                },
                None,
            )?;
            observations.push(with_label(observation, label)?);
        }
        Ok(observations)
    }

    pub(crate) fn stop_hook(
        &mut self,
        provider_ref: Option<&str>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        Ok(vec![self.completed(
            "hook",
            provider_ref,
            ActivityKind::Hook,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
            ActivityStatus::Completed,
            None,
        )?])
    }

    pub(crate) fn unknown(
        &mut self,
        event_type: &str,
        provider_ref: Option<&str>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        Ok(vec![self.completed(
            "unknown",
            provider_ref,
            ActivityKind::Unknown(namespace(&format!("claude-code.headless.{event_type}"))?),
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
            ActivityStatus::Completed,
            None,
        )?])
    }

    pub(crate) fn ensure_idle(&self) -> Result<(), RuntimeFailure> {
        if self.pending_tools.is_empty() {
            Ok(())
        } else {
            Err(activity_drift())
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn completed(
        &mut self,
        label: &str,
        provider_ref: Option<&str>,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        let mut observation = ActivityObservation::new(
            ActivityId::new(format!("claude-code-headless:{label}:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            kind,
            ActivityLifecyclePhase::Completed,
            status,
            assistant_phase,
            disclosure,
        )
        .map_err(|_| activity_drift())?;
        if let Some(provider_ref) = provider_ref {
            observation = observation.with_provider_activity_ref(
                ProviderActivityRef::new(provider_ref).map_err(|_| activity_drift())?,
            );
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

fn with_label(
    mut observation: ActivityObservation,
    label: Option<ActivityLabel>,
) -> Result<ActivityObservation, RuntimeFailure> {
    if let Some(label) = label {
        observation = observation
            .with_label(label)
            .map_err(|_| activity_drift())?;
    }
    Ok(observation)
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
        "swallowtail.claude_code.headless.activity_invalid",
        "Claude Code headless activity did not match the qualified stream",
    )
}

#[cfg(test)]
mod tests;
