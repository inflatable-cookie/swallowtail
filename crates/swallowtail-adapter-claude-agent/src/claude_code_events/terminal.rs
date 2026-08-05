use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, SafeDiagnostic,
};
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, ProcessExit, TerminalOutcome, TerminalStatus,
};

pub(crate) struct ParsedTerminal {
    pub(super) final_output: Option<OperationContent>,
    pub(super) provider_failure: Option<SafeDiagnostic>,
    pub(super) initialized: bool,
    pub(super) terminal_seen: bool,
}

impl ParsedTerminal {
    pub(crate) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        let status = match exit.code() {
            Some(130) => provider_failure(
                "swallowtail.claude_code.headless.process_interrupted",
                "Claude Code was interrupted outside Swallowtail cancellation",
                FailureKind::TransportInterrupted,
                FailureRecovery::RetryMaySucceed,
            ),
            _ if self.provider_failure.is_some() => TerminalStatus::ProviderFailed(
                self.provider_failure.expect("checked provider failure"),
            ),
            _ if !exit.success() => provider_failure(
                "swallowtail.claude_code.headless.process_failed",
                "Claude Code exited unsuccessfully",
                FailureKind::Unknown,
                FailureRecovery::Unknown,
            ),
            _ if !self.initialized || !self.terminal_seen => {
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.claude_code.headless.incomplete_stream",
                    "Claude Code ended without complete init and result evidence",
                ))
            }
            _ => TerminalStatus::Completed,
        };
        let outcome = TerminalOutcome::new(status, CleanupOutcome::Clean);
        match self.final_output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        }
    }
}

fn provider_failure(
    code: &'static str,
    message: &'static str,
    kind: FailureKind,
    recovery: FailureRecovery,
) -> TerminalStatus {
    TerminalStatus::ProviderFailed(
        SafeDiagnostic::new(code, message).with_failure_classification(FailureClassification::new(
            FailureOrigin::Harness,
            kind,
            recovery,
        )),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_exit_and_provider_error_keep_distinct_origins() {
        let process = ParsedTerminal {
            final_output: None,
            provider_failure: None,
            initialized: true,
            terminal_seen: true,
        }
        .outcome(ProcessExit::new(false, Some(130)));
        let provider = ParsedTerminal {
            final_output: None,
            provider_failure: Some(
                SafeDiagnostic::new("fixture.provider", "Provider failed")
                    .with_failure_classification(FailureClassification::new(
                        FailureOrigin::Provider,
                        FailureKind::Unknown,
                        FailureRecovery::Unknown,
                    )),
            ),
            initialized: true,
            terminal_seen: true,
        }
        .outcome(ProcessExit::new(true, Some(0)));

        assert_eq!(
            process
                .failure()
                .expect("process failure")
                .diagnostic()
                .failure_classification()
                .origin(),
            FailureOrigin::Harness
        );
        assert_eq!(
            provider
                .failure()
                .expect("provider failure")
                .diagnostic()
                .failure_classification()
                .origin(),
            FailureOrigin::Provider
        );
    }
}
