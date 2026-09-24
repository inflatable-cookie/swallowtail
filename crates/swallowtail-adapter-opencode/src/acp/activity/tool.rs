use super::{
    AcpActivityProjection,
    content::{tool_content, tool_status},
};
use swallowtail_core::ActivityDisclosure;
use swallowtail_protocol_acp::{AcpToolCall, AcpToolCallUpdate};
use swallowtail_runtime::{
    ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityObservation, ActivityStatus,
    RuntimeFailure,
};

impl AcpActivityProjection {
    pub(super) fn tool_start(
        &mut self,
        call: &AcpToolCall,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let key = format!("tool:{}", call.tool_call_id.as_str());
        if self.closed.contains(&key) {
            return Ok(Vec::new());
        }
        let status = tool_status(call.status);
        let label = activity_label(call.title.as_str());
        let content = tool_content(&call.content)?;
        if let Some(mut activity) = self.open.get(&key).cloned() {
            if label.is_some() {
                activity.label = label;
            }
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
            let mut activity = self.open_or_insert(
                &key,
                Some(call.tool_call_id.as_str()),
                ActivityKind::ProviderOwnedTool,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                ActivityStatus::InProgress,
            )?;
            activity.label = label;
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
            self.open.remove(&key);
            self.closed.insert(key);
            return Ok(vec![started, completed]);
        }
        let mut activity = self.open_or_insert(
            &key,
            Some(call.tool_call_id.as_str()),
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            status,
        )?;
        activity.label = label;
        self.open.insert(key, activity.clone());
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Started,
            status,
            content,
        )?])
    }

    pub(super) fn tool_update(
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
            .map(|content| tool_content(content))
            .transpose()?
            .flatten();
        let Some(mut activity) = activity else {
            let Some(status) = orphan_terminal_status else {
                return Ok(Vec::new());
            };
            let mut activity = self.open_or_insert(
                &key,
                Some(update.tool_call_id.as_str()),
                ActivityKind::ProviderOwnedTool,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                ActivityStatus::InProgress,
            )?;
            activity.label = label;
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
            self.open.remove(&key);
            self.closed.insert(key);
            return Ok(vec![started, completed]);
        };
        if label.is_some() {
            activity.label = label;
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
