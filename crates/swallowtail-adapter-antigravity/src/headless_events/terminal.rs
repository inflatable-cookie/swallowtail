use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{CleanupOutcome, OperationContent, ProcessExit, TerminalOutcome, TerminalStatus};

pub(crate) struct ParsedTerminal {
    final_output: Option<OperationContent>,
    status: Option<TerminalStatus>,
    terminal_seen: bool,
    conversation_id: Option<String>,
}

impl ParsedTerminal {
    pub(super) const fn new(
        final_output: Option<OperationContent>,
        status: Option<TerminalStatus>,
        terminal_seen: bool,
        conversation_id: Option<String>,
    ) -> Self {
        Self {
            final_output,
            status,
            terminal_seen,
            conversation_id,
        }
    }

    pub(crate) fn conversation_id(&self) -> Option<&str> {
        self.conversation_id.as_deref()
    }

    pub(crate) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        let status = if !self.terminal_seen {
            TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                "swallowtail.antigravity.headless.incomplete_stream",
                "Antigravity headless stream ended without one terminal result",
            ))
        } else {
            let parsed = self.status.unwrap_or_else(|| {
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.antigravity.headless.invalid_terminal_status",
                    "Antigravity terminal result could not be classified",
                ))
            });
            if parsed == TerminalStatus::Completed && !exit.success() {
                TerminalStatus::ProviderFailed(
                    SafeDiagnostic::new(
                        "swallowtail.antigravity.headless.process_failed",
                        match exit.code() {
                            Some(code) => {
                                format!("Antigravity headless process exited with status {code}")
                            }
                            None => "Antigravity headless process exited unsuccessfully".to_owned(),
                        },
                    )
                    .with_failure_classification(
                        swallowtail_core::FailureClassification::new(
                            swallowtail_core::FailureOrigin::Harness,
                            swallowtail_core::FailureKind::Unknown,
                            swallowtail_core::FailureRecovery::Unknown,
                        ),
                    ),
                )
            } else {
                parsed
            }
        };
        let outcome = TerminalOutcome::new(status, CleanupOutcome::Clean);
        match self.final_output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        }
    }
}

