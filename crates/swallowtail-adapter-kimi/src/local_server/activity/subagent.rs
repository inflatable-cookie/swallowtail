use super::*;
use swallowtail_runtime::{SubagentId, SubagentParent, SubagentStatus};

impl KimiLocalActivityProjection {
    pub(in crate::local_server::activity) fn subagent_started(
        &mut self,
        subagent_id: &str,
        name: &str,
        parent_tool_call_id: &str,
        background: bool,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if self.subagents.contains_key(subagent_id) {
            return Err(activity_drift());
        }
        let mut activity = self.open(
            ActivityBucket::Subagent.label(),
            Some(subagent_id),
            ActivityKind::SubagentOrCollaboration,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )?;
        activity.subagent = Some(
            SubagentSnapshot::new(
                SubagentId::new(subagent_id).map_err(|_| activity_drift())?,
                SubagentParent::Operation,
                SubagentStatus::Pending,
            )
            .with_label(ActivityLabel::new(name).map_err(|_| activity_drift())?)
            .with_background(background)
            .with_originating_activity(
                ProviderActivityRef::new(parent_tool_call_id).map_err(|_| activity_drift())?,
            ),
        );
        self.subagents
            .insert(subagent_id.to_owned(), activity.clone());
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            None,
        )?])
    }

    pub(in crate::local_server::activity) fn subagent_updated(
        &mut self,
        subagent_id: &str,
        suspended: bool,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self
            .subagents
            .get_mut(subagent_id)
            .ok_or_else(activity_drift)?;
        let snapshot = activity.subagent.take().ok_or_else(activity_drift)?;
        activity.subagent = Some(snapshot.with_status(if suspended {
            SubagentStatus::Waiting
        } else {
            SubagentStatus::Running
        }));
        let activity = activity.clone();
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            None,
        )?])
    }

    pub(in crate::local_server::activity) fn subagent_ended(
        &mut self,
        subagent_id: &str,
        failed: bool,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut activity = self
            .subagents
            .remove(subagent_id)
            .ok_or_else(activity_drift)?;
        let snapshot = activity.subagent.take().ok_or_else(activity_drift)?;
        activity.subagent = Some(snapshot.with_status(if failed {
            SubagentStatus::Failed
        } else {
            SubagentStatus::Completed
        }));
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            terminal_status(failed),
            None,
        )?])
    }
}
