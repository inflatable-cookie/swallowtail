//! Projection of qualified sidecar events onto runtime events, activity, and
//! exactly one terminal outcome.

use super::{SdkActiveTurn, provider_diagnostic};
use crate::sdk::failure::failure;
use crate::sdk::wire::ClaudeAgentSdkEvent;
use std::sync::atomic::Ordering;
use swallowtail_runtime::{ActivityStatus, RuntimeFailure, TerminalStatus};

impl SdkActiveTurn {
    pub(crate) fn handle_event(&self, event: ClaudeAgentSdkEvent) -> Result<(), RuntimeFailure> {
        if self.is_finished() {
            return Err(failure(
                "swallowtail.claude-agent.sdk.event_after_terminal",
                "Claude Agent SDK sidecar emitted an event after the active turn terminated",
            ));
        }
        self.project_activity(&event)?;
        match event {
            ClaudeAgentSdkEvent::TurnStarted | ClaudeAgentSdkEvent::Progress => self.progress(),
            ClaudeAgentSdkEvent::OutputDelta(delta) => self.output_delta(delta),
            ClaudeAgentSdkEvent::ToolStarted { .. } | ClaudeAgentSdkEvent::ToolEnded { .. } => {
                Ok(())
            }
            ClaudeAgentSdkEvent::TurnFailed => {
                self.complete_activity(ActivityStatus::Failed)?;
                self.finish(TerminalStatus::ProviderFailed(provider_diagnostic()));
                Ok(())
            }
            ClaudeAgentSdkEvent::TurnEnded {
                stop_reason,
                failed,
            } => {
                let (status, activity_status) = if self.timed_out.load(Ordering::SeqCst) {
                    (TerminalStatus::TimedOut, ActivityStatus::Failed)
                } else if failed || stop_reason != "success" {
                    (
                        TerminalStatus::ProviderFailed(provider_diagnostic()),
                        ActivityStatus::Failed,
                    )
                } else if self.cancelled.load(Ordering::SeqCst) {
                    (TerminalStatus::Cancelled, ActivityStatus::Cancelled)
                } else {
                    (TerminalStatus::Completed, ActivityStatus::Completed)
                };
                self.complete_activity(activity_status)?;
                self.finish(status);
                Ok(())
            }
        }
    }
}
