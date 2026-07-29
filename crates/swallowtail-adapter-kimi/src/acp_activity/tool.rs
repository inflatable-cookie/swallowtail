use super::AcpActivityProjection;
use super::content::{tool_content, tool_status};
use crate::failure::malformed;
use swallowtail_core::ActivityDisclosure;
use swallowtail_protocol_acp::{AcpToolCall, AcpToolCallUpdate};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityObservation, RuntimeFailure,
};

impl AcpActivityProjection {
    pub(super) fn tool_start(
        &mut self,
        call: &AcpToolCall,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let key = format!("tool:{}", call.tool_call_id.as_str());
        if self.open.contains_key(&key) || call.status.is_terminal() {
            return Err(malformed());
        }
        let status = tool_status(call.status);
        let activity = self.open_or_insert(
            &key,
            Some(call.tool_call_id.as_str()),
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            status,
        )?;
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Started,
            status,
            tool_content(call.title.as_str(), &call.content)?,
        )?])
    }

    pub(super) fn tool_update(
        &mut self,
        update: &AcpToolCallUpdate,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let key = format!("tool:{}", update.tool_call_id.as_str());
        let mut activity = self.open.get(&key).cloned().ok_or_else(malformed)?;
        let status = update.status.map_or(activity.status, tool_status);
        let phase = if status.is_terminal() {
            ActivityLifecyclePhase::Completed
        } else {
            ActivityLifecyclePhase::Updated
        };
        activity.status = status;
        let content = update
            .content_replacement
            .as_ref()
            .map(|content| {
                tool_content(
                    update.title.as_ref().map_or("", |title| title.as_str()),
                    content,
                )
            })
            .transpose()?
            .flatten();
        let observation = self.observation(&activity, phase, status, content)?;
        if status.is_terminal() {
            self.open.remove(&key);
        } else {
            self.open.insert(key, activity);
        }
        Ok(vec![observation])
    }
}
