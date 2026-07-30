use super::{
    ActivitySource, AppServerActivityProjection, ObservationDetail, content, item::ItemIdentity,
    required_text,
};
use crate::turn_state::malformed_notification;
use serde_json::Value;
use swallowtail_core::ActivityDisclosure;
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityNamespace, ActivityObservation, ActivityStatus,
    RuntimeFailure,
};

impl AppServerActivityProjection {
    pub(super) fn project_hook(
        &mut self,
        params: &Value,
        phase: ActivityLifecyclePhase,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let run = params.get("run").ok_or_else(malformed_notification)?;
        let id = required_text(run, "id")?;
        let status = if phase == ActivityLifecyclePhase::Completed {
            ActivityStatus::Completed
        } else {
            ActivityStatus::InProgress
        };
        Ok(vec![self.observation(
            ActivitySource::new(&format!("hook:{id}"), Some(id)),
            ItemIdentity::new(
                ActivityKind::Hook,
                None,
                ActivityDisclosure::AdapterNormalizedSummary,
            ),
            phase,
            status,
            ObservationDetail::primary(None, content::hook_summary(run, phase)?),
        )?])
    }

    pub(super) fn project_unknown_update(
        &mut self,
        method: &str,
        params: &Value,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if params.get("itemId").and_then(Value::as_str).is_none() {
            return Ok(Vec::new());
        }
        let namespace =
            ActivityNamespace::new(format!("codex.app-server.{}", method.replace('/', ".")))
                .map_err(|_| malformed_notification())?;
        self.project_delta(
            params,
            ActivityKind::Unknown(namespace),
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
            None,
        )
    }
}
