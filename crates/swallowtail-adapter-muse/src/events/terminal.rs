use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, ProcessExit, TerminalOutcome, TerminalStatus,
};

pub(crate) struct ParsedTerminal {
    output: Option<OperationContent>,
    status: Option<TerminalStatus>,
    complete: bool,
}

impl ParsedTerminal {
    pub(super) fn complete(
        output: Option<OperationContent>,
        status: Option<TerminalStatus>,
    ) -> Self {
        Self {
            output,
            status,
            complete: true,
        }
    }

    pub(super) fn incomplete(
        output: Option<OperationContent>,
        status: Option<TerminalStatus>,
    ) -> Self {
        Self {
            output,
            status,
            complete: false,
        }
    }

    pub(crate) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        let status = if !self.complete {
            TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                "swallowtail.muse_code.headless.incomplete_stream",
                "Muse Code stream ended without its complete correlated lifecycle",
            ))
        } else if !exit.success() && self.status == Some(TerminalStatus::Completed) {
            TerminalStatus::ProviderFailed(
                SafeDiagnostic::new(
                    "swallowtail.muse_code.headless.process_failed",
                    match exit.code() {
                        Some(code) => format!("Muse Code exited with status {code}"),
                        None => "Muse Code exited unsuccessfully".to_owned(),
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
            self.status.unwrap_or_else(|| {
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.muse_code.headless.invalid_terminal",
                    "Muse Code terminal status could not be classified",
                ))
            })
        };
        let outcome = TerminalOutcome::new(status, CleanupOutcome::Clean);
        match self.output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        }
    }
}
