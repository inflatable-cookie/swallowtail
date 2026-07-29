use super::{
    ActivitySource, AppServerActivityProjection, content, item::ItemIdentity, required_text,
};
use crate::turn_state::malformed_notification;
use serde_json::Value;
use swallowtail_core::ActivityDisclosure;
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContentChangeKind, ActivityContentStream, ActivityKind,
    ActivityLifecyclePhase, ActivityObservation, ActivityStatus, RuntimeFailure,
};

impl AppServerActivityProjection {
    pub(crate) fn project_notification(
        &mut self,
        method: &str,
        params: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        match method {
            "item/started" => self.project_item(params, ActivityLifecyclePhase::Started),
            "item/completed" => self.project_item(params, ActivityLifecyclePhase::Completed),
            "item/agentMessage/delta" => self.project_assistant_delta(params),
            "item/reasoning/summaryPartAdded" => self.project_delta(
                params,
                ActivityKind::ReasoningSummary,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                None,
            ),
            "item/reasoning/summaryTextDelta" => self.project_delta(
                params,
                ActivityKind::ReasoningSummary,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                content::text_delta(params, ActivityContentStream::ReasoningSummaryText)?,
            ),
            "item/reasoning/textDelta" => Ok(Vec::new()),
            "item/plan/delta" => self.project_delta(
                params,
                ActivityKind::Plan,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                content::text_delta(params, ActivityContentStream::PlanText)?,
            ),
            "item/commandExecution/outputDelta" => self.project_delta(
                params,
                ActivityKind::CommandExecution,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                content::text_delta(params, ActivityContentStream::CommandOutput)?,
            ),
            "item/fileChange/outputDelta" => self.project_delta(
                params,
                ActivityKind::FileChange,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                content::text_delta(params, ActivityContentStream::FileChangeOutput)?,
            ),
            "item/fileChange/patchUpdated" => self.project_delta(
                params,
                ActivityKind::FileChange,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                content::file_changes(
                    params.get("changes"),
                    ActivityContentChangeKind::ReplacementSnapshot,
                )?,
            ),
            "item/mcpToolCall/progress" => self.project_delta(
                params,
                ActivityKind::ProviderOwnedTool,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                content::text_field(
                    params,
                    "message",
                    ActivityContentStream::ProviderToolDisplay,
                    ActivityContentChangeKind::ReplacementSnapshot,
                )?,
            ),
            "turn/plan/updated" => self.project_turn_snapshot(
                "turn-plan",
                ActivityKind::Plan,
                ActivityDisclosure::ProviderDisplayContent,
                content::plan_snapshot(params)?,
            ),
            "turn/diff/updated" => self.project_turn_snapshot(
                "turn-diff",
                ActivityKind::FileChange,
                ActivityDisclosure::ProviderDisplayContent,
                content::text_field(
                    params,
                    "diff",
                    ActivityContentStream::FileChangeOutput,
                    ActivityContentChangeKind::ReplacementSnapshot,
                )?,
            ),
            "thread/compacted" => self.project_completion(
                "thread-compaction",
                ActivityKind::ContextCompaction,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
                None,
            ),
            "hook/started" => self.project_hook(params, ActivityLifecyclePhase::Started),
            "hook/completed" => self.project_hook(params, ActivityLifecyclePhase::Completed),
            "serverRequest/resolved" => self.project_request_resolution(params),
            _ if method.starts_with("item/") => self.project_unknown_update(method, params),
            _ => Ok(Vec::new()),
        }
    }

    fn project_item(
        &mut self,
        params: &Value,
        phase: ActivityLifecyclePhase,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let item = params.get("item").ok_or_else(malformed_notification)?;
        let provider_id = required_text(item, "id")?;
        let projection = super::item::item_projection(item, phase)?;
        if let Some(existing) = self.items.get(provider_id) {
            if existing != &projection.identity {
                return Err(malformed_notification());
            }
        } else {
            self.items
                .insert(provider_id.to_owned(), projection.identity.clone());
        }
        let correlation = self.correlations.get(provider_id).cloned();
        Ok(vec![self.observation(
            ActivitySource::new(provider_id, Some(provider_id)),
            projection.identity,
            phase,
            projection.status,
            correlation,
            projection.content,
        )?])
    }

    pub(super) fn project_delta(
        &mut self,
        params: &Value,
        fallback_kind: ActivityKind,
        fallback_assistant_phase: Option<ActivityAssistantPhase>,
        fallback_disclosure: ActivityDisclosure,
        content: Option<swallowtail_runtime::ActivityContentUpdate>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let provider_id = required_text(params, "itemId")?;
        let fallback =
            ItemIdentity::new(fallback_kind, fallback_assistant_phase, fallback_disclosure);
        let identity = self
            .items
            .entry(provider_id.to_owned())
            .or_insert(fallback)
            .clone();
        let correlation = self.correlations.get(provider_id).cloned();
        Ok(vec![self.observation(
            ActivitySource::new(provider_id, Some(provider_id)),
            identity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            correlation,
            content,
        )?])
    }

    fn project_assistant_delta(
        &mut self,
        params: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let provider_id = required_text(params, "itemId")?;
        let identity = self
            .items
            .entry(provider_id.to_owned())
            .or_insert_with(|| {
                ItemIdentity::new(
                    ActivityKind::AssistantMessage,
                    Some(ActivityAssistantPhase::ProviderUnspecified),
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                )
            })
            .clone();
        let content = match identity.assistant_phase {
            Some(ActivityAssistantPhase::Intermediate) => {
                content::text_delta(params, ActivityContentStream::IntermediateAssistantText)?
            }
            Some(ActivityAssistantPhase::Final) => {
                content::text_delta(params, ActivityContentStream::FinalAnswerText)?
            }
            Some(ActivityAssistantPhase::ProviderUnspecified) => None,
            None => return Err(malformed_notification()),
        };
        let correlation = self.correlations.get(provider_id).cloned();
        Ok(vec![self.observation(
            ActivitySource::new(provider_id, Some(provider_id)),
            identity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            correlation,
            content,
        )?])
    }

    fn project_turn_snapshot(
        &mut self,
        key: &str,
        kind: ActivityKind,
        disclosure: ActivityDisclosure,
        content: Option<swallowtail_runtime::ActivityContentUpdate>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        Ok(vec![self.observation(
            ActivitySource::new(key, None),
            ItemIdentity::new(kind, None, disclosure),
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            None,
            content,
        )?])
    }

    fn project_completion(
        &mut self,
        key: &str,
        kind: ActivityKind,
        disclosure: ActivityDisclosure,
        provider_ref: Option<&str>,
        content: Option<swallowtail_runtime::ActivityContentUpdate>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        Ok(vec![self.observation(
            ActivitySource::new(key, provider_ref),
            ItemIdentity::new(kind, None, disclosure),
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            content,
        )?])
    }
}
