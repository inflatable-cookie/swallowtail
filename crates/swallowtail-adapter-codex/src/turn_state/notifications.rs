use super::{
    ActiveTurn, MAX_ADMITTED_CHILD_THREADS, activity_notification, child_lifecycle_after_terminal,
    child_turn_mismatch, malformed_notification, required_text,
};
use crate::rpc::failure;
use serde_json::Value;
use std::collections::BTreeSet;
use std::sync::atomic::Ordering;
use swallowtail_runtime::{
    ActivityActor, ActivityLifecyclePhase, ActivityObservation, ActivityStatus, CleanupOutcome,
    OperationContent, RuntimeEventKind, RuntimeFailure, SubagentId, TerminalStatus,
};

impl ActiveTurn {
    pub(crate) fn handle_notification(
        &self,
        method: &str,
        params: &Value,
    ) -> Result<(), RuntimeFailure> {
        if matches!(method, "turn/started" | "turn/completed")
            && let Some(child) = self.verify_child_lifecycle_owner(params)?
        {
            return self.handle_child_lifecycle(method, params, child);
        }
        let mut activity_owner = None;
        let activities = if activity_notification(method) {
            if method == "serverRequest/resolved" {
                let thread_id = required_text(params, "threadId")?;
                if thread_id != self.provider_thread_id {
                    return Err(failure(
                        "swallowtail.codex.app_server.session_id_mismatch",
                        "Codex app-server event belongs to a different provider session",
                    ));
                }
            } else {
                activity_owner = self.verify_activity_owner(params)?;
            }
            let activities = self
                .activity
                .lock()
                .expect("activity lock poisoned")
                .project_notification(method, params)?;
            let activities = match activity_owner.clone() {
                Some(child) => activities
                    .into_iter()
                    .map(|activity| activity.with_actor(ActivityActor::Subagent(child.clone())))
                    .collect(),
                None => activities,
            };
            self.admit_spawned_children(&activities)?;
            activities
        } else {
            Vec::new()
        };
        let emitted_activity = !activities.is_empty();
        for observation in activities {
            self.emit(RuntimeEventKind::Activity(observation), None)?;
        }
        match method {
            "turn/started" => {
                self.verify_turn(params)?;
                self.emit(RuntimeEventKind::Progress, None)
            }
            "item/agentMessage/delta" => {
                if activity_owner.is_some() {
                    return if emitted_activity {
                        Ok(())
                    } else {
                        self.emit(RuntimeEventKind::ProgressSnapshot, None)
                    };
                }
                self.verify_turn(params)?;
                let delta = required_text(params, "delta")?;
                self.delta_output
                    .lock()
                    .expect("turn delta lock poisoned")
                    .push_str(delta);
                match OperationContent::new(delta) {
                    Ok(content) => self.emit(RuntimeEventKind::OutputDelta, Some(content)),
                    Err(_) if delta.trim().is_empty() => Ok(()),
                    Err(_) => Err(malformed_notification()),
                }
            }
            "item/completed" => {
                if activity_owner.is_some() {
                    return if emitted_activity {
                        Ok(())
                    } else {
                        self.emit(RuntimeEventKind::ProgressSnapshot, None)
                    };
                }
                self.verify_turn(params)?;
                let item = params.get("item").ok_or_else(malformed_notification)?;
                if item.get("type").and_then(Value::as_str) == Some("agentMessage") {
                    let text = required_text(item, "text")?;
                    match OperationContent::new(text) {
                        Ok(content) => {
                            *self.final_output.lock().expect("turn output lock poisoned") =
                                Some(content.clone());
                            self.emit(RuntimeEventKind::OutputAvailable, Some(content))
                        }
                        Err(_) if text.trim().is_empty() => {
                            self.emit(RuntimeEventKind::Progress, None)
                        }
                        Err(_) => Err(malformed_notification()),
                    }
                } else if emitted_activity {
                    Ok(())
                } else {
                    self.emit(RuntimeEventKind::Progress, None)
                }
            }
            "turn/completed" => {
                self.verify_turn(params)?;
                let turn = params.get("turn").ok_or_else(malformed_notification)?;
                let status = required_text(turn, "status")?;
                let terminal = match status {
                    "completed" => TerminalStatus::Completed,
                    "interrupted" if self.timed_out.load(Ordering::SeqCst) => {
                        TerminalStatus::TimedOut
                    }
                    "interrupted" => TerminalStatus::Cancelled,
                    "failed" => {
                        TerminalStatus::ProviderFailed(swallowtail_core::SafeDiagnostic::new(
                            "swallowtail.codex.app_server.turn_failed",
                            "Codex app-server turn failed",
                        ))
                    }
                    _ => TerminalStatus::RuntimeFailed(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.codex.app_server.unknown_turn_status",
                        "Codex app-server returned an unknown turn status",
                    )),
                };
                self.finish(terminal, CleanupOutcome::NotApplicable);
                Ok(())
            }
            "error" => {
                if let Some(child) = self.verify_child_lifecycle_owner(params)? {
                    self.verify_child_turn(&child, required_text(params, "turnId")?)?;
                    return self.emit(RuntimeEventKind::ProgressSnapshot, None);
                }
                self.verify_turn(params)?;
                self.finish(
                    TerminalStatus::ProviderFailed(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.codex.app_server.provider_error",
                        "Codex app-server reported a provider error",
                    )),
                    CleanupOutcome::NotApplicable,
                );
                Ok(())
            }
            _ if emitted_activity => Ok(()),
            _ => self.emit(RuntimeEventKind::ProgressSnapshot, None),
        }
    }

    fn verify_activity_owner(&self, params: &Value) -> Result<Option<SubagentId>, RuntimeFailure> {
        let thread_id = required_text(params, "threadId")?;
        let turn_id = required_text(params, "turnId")?;
        if thread_id == self.provider_thread_id {
            self.set_provider_id(turn_id)?;
            Ok(None)
        } else if self
            .admitted_child_threads
            .lock()
            .expect("admitted child threads lock poisoned")
            .contains(thread_id)
        {
            let child = SubagentId::new(thread_id).map_err(|_| malformed_notification())?;
            self.verify_child_turn(&child, turn_id)?;
            Ok(Some(child))
        } else {
            Err(failure(
                "swallowtail.codex.app_server.session_id_mismatch",
                "Codex app-server event belongs to a different provider session",
            ))
        }
    }

    fn verify_child_lifecycle_owner(
        &self,
        params: &Value,
    ) -> Result<Option<SubagentId>, RuntimeFailure> {
        let thread_id = required_text(params, "threadId")?;
        if thread_id == self.provider_thread_id {
            return Ok(None);
        }
        if self.is_finished() {
            return Err(child_lifecycle_after_terminal());
        }
        if self
            .admitted_child_threads
            .lock()
            .expect("admitted child threads lock poisoned")
            .contains(thread_id)
        {
            return SubagentId::new(thread_id)
                .map(Some)
                .map_err(|_| malformed_notification());
        }
        Err(failure(
            "swallowtail.codex.app_server.lifecycle_owner_mismatch",
            "Codex app-server lifecycle belongs to an unknown operation thread",
        ))
    }

    fn handle_child_lifecycle(
        &self,
        method: &str,
        params: &Value,
        child: SubagentId,
    ) -> Result<(), RuntimeFailure> {
        let turn = params.get("turn").ok_or_else(malformed_notification)?;
        let provider_turn_id = required_text(turn, "id")?;
        let provider_status = required_text(turn, "status")?;
        let (phase, status, child_status) = match (method, provider_status) {
            ("turn/started", "inProgress") => (
                ActivityLifecyclePhase::Started,
                ActivityStatus::InProgress,
                swallowtail_runtime::SubagentStatus::Running,
            ),
            ("turn/completed", "completed") => (
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Completed,
                swallowtail_runtime::SubagentStatus::Completed,
            ),
            ("turn/completed", "failed") => (
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Failed,
                swallowtail_runtime::SubagentStatus::Failed,
            ),
            ("turn/completed", "interrupted") => (
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Cancelled,
                swallowtail_runtime::SubagentStatus::Interrupted,
            ),
            _ => return Err(malformed_notification()),
        };

        match phase {
            ActivityLifecyclePhase::Started => {
                let mut turns = self
                    .active_child_turns
                    .lock()
                    .expect("active child turns lock poisoned");
                if self.is_finished() {
                    return Err(child_lifecycle_after_terminal());
                }
                if turns.contains_key(child.as_str()) {
                    return Err(child_turn_mismatch());
                }
                turns.insert(child.as_str().to_owned(), provider_turn_id.to_owned());
            }
            ActivityLifecyclePhase::Completed => {
                self.verify_child_turn(&child, provider_turn_id)?;
            }
            ActivityLifecyclePhase::Updated => unreachable!("child lifecycle has no update phase"),
        }

        let observation = self
            .activity
            .lock()
            .expect("activity lock poisoned")
            .project_child_turn_lifecycle(
                child.clone(),
                provider_turn_id,
                phase,
                status,
                child_status,
            )?;
        self.emit(RuntimeEventKind::Activity(observation), None)?;

        if phase == ActivityLifecyclePhase::Completed {
            self.active_child_turns
                .lock()
                .expect("active child turns lock poisoned")
                .remove(child.as_str());
        }
        Ok(())
    }

    fn verify_child_turn(
        &self,
        child: &SubagentId,
        provider_turn_id: &str,
    ) -> Result<(), RuntimeFailure> {
        if self.is_finished() {
            return Err(child_lifecycle_after_terminal());
        }
        if self
            .active_child_turns
            .lock()
            .expect("active child turns lock poisoned")
            .get(child.as_str())
            .is_some_and(|active| active == provider_turn_id)
        {
            Ok(())
        } else {
            Err(child_turn_mismatch())
        }
    }

    fn admit_spawned_children(
        &self,
        activities: &[ActivityObservation],
    ) -> Result<(), RuntimeFailure> {
        let candidates = activities
            .iter()
            .filter(|activity| {
                activity.subagent_control()
                    == Some(swallowtail_core::SubagentControlActionKind::Spawn)
                    && activity.phase() == ActivityLifecyclePhase::Completed
                    && activity.status() == ActivityStatus::Completed
            })
            .flat_map(ActivityObservation::subagents)
            .map(|child| child.id().as_str().to_owned())
            .collect::<BTreeSet<_>>();
        if candidates.is_empty() {
            return Ok(());
        }
        let mut admitted = self
            .admitted_child_threads
            .lock()
            .expect("admitted child threads lock poisoned");
        let additional = candidates
            .iter()
            .filter(|child| !admitted.contains(*child))
            .count();
        if admitted.len().saturating_add(additional) > MAX_ADMITTED_CHILD_THREADS {
            return Err(failure(
                "swallowtail.codex.app_server.child_thread_limit_exceeded",
                "Codex app-server exceeded the operation child-thread ownership bound",
            ));
        }
        admitted.extend(candidates);
        Ok(())
    }
}
