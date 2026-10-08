//! Projection of qualified sidecar events onto runtime events, activity, and
//! exactly one terminal outcome.

use super::{
    SdkActiveTurn, TurnEndedDiagnostic, provider_diagnostic, provider_turn_ended_diagnostic,
};
use crate::sdk::failure::failure;
use crate::sdk::wire::ClaudeAgentSdkEvent;
use std::sync::atomic::Ordering;
use swallowtail_runtime::{
    ActivityStatus, ProviderObservation, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    TerminalStatus, TokenUsage,
};

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
                self.finish(TerminalStatus::ProviderFailed(
                    self.add_stderr_to_diagnostic(provider_diagnostic()),
                ));
                Ok(())
            }
            ClaudeAgentSdkEvent::TurnEnded {
                usage,
                stop_reason,
                failed,
                subtype,
                num_turns,
                duration_ms,
                error_text_present,
                error_text_type,
                result_field_presence,
                api_error_status,
                terminal_reason,
                rate_limit_status,
            } => {
                if let Some(usage) = usage {
                    let usage_event = RuntimeEvent::new(
                        self.next_sequence(),
                        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(
                            TokenUsage::new(Some(usage.input_tokens), Some(usage.output_tokens))
                                .with_cache_tokens(
                                    usage.cache_read_input_tokens,
                                    usage.cache_write_input_tokens,
                                ),
                        )),
                    );
                    self.events.send(usage_event)?;
                }
                let (status, activity_status) = if self.timed_out.load(Ordering::SeqCst) {
                    (TerminalStatus::TimedOut, ActivityStatus::Failed)
                } else if failed || stop_reason != "success" {
                    (
                        TerminalStatus::ProviderFailed(self.add_stderr_to_diagnostic(
                            provider_turn_ended_diagnostic(TurnEndedDiagnostic {
                                stop_reason,
                                failed,
                                subtype,
                                num_turns,
                                duration_ms,
                                error_text_present,
                                error_text_type,
                                result_field_presence,
                                api_error_status,
                                terminal_reason,
                                rate_limit_status,
                            }),
                        )),
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
