use super::namespace;
use swallowtail_core::ActivityDisclosure;
use swallowtail_runtime::{ActivityKind, ActivityObservation, ActivityStatus, RuntimeFailure};

impl super::ClaudeCodeActivityProjection {
    pub(crate) fn stop_hook(
        &mut self,
        phase: &str,
        session: Option<&str>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let provider_ref = session.map(|session| format!("{session}|{phase}"));
        Ok(vec![self.completed(
            "hook",
            provider_ref.as_deref(),
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
}
