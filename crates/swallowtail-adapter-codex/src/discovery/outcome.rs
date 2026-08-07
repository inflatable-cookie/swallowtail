use crate::safe_excerpt::sanitize_stderr;
use swallowtail_core::{DiscoveryOutcome, DiscoveryStatus, SafeDiagnostic};

pub(super) fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "Codex installed executable discovery did not produce a compatible observation",
        )),
    )
}

fn staged_outcome(
    status: DiscoveryStatus,
    code: &'static str,
    message: impl Into<String>,
) -> DiscoveryOutcome {
    DiscoveryOutcome::new(status, Some(SafeDiagnostic::new(code, message)))
}

pub(super) fn spawn_failed() -> DiscoveryOutcome {
    staged_outcome(
        DiscoveryStatus::Failed,
        "swallowtail.codex.discovery_spawn_failed",
        "Codex version probe could not start",
    )
}

pub(super) fn output_failed() -> DiscoveryOutcome {
    staged_outcome(
        DiscoveryStatus::Failed,
        "swallowtail.codex.discovery_output_failed",
        "Codex version probe output could not be read",
    )
}

pub(super) fn output_limit() -> DiscoveryOutcome {
    staged_outcome(
        DiscoveryStatus::Malformed,
        "swallowtail.codex.discovery_output_limit",
        "Codex version probe exceeded its output limit",
    )
}

pub(super) fn exit_failed(
    exit_code: Option<i32>,
    stderr: &[u8],
    stderr_was_truncated: bool,
) -> DiscoveryOutcome {
    let mut message = match exit_code {
        Some(code) => format!("Codex version probe exited with status {code}"),
        None => "Codex version probe did not exit successfully".to_owned(),
    };
    if let Some(stderr) = sanitize_stderr(stderr, stderr_was_truncated) {
        message.push_str("; stderr: ");
        message.push_str(&stderr);
    }
    staged_outcome(
        DiscoveryStatus::Failed,
        "swallowtail.codex.discovery_exit_failed",
        message,
    )
}

const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.codex.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.codex.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.codex.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.codex.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.codex.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.codex.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.codex.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.codex.discovery_cleanup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::{exit_failed, sanitize_stderr};

    #[test]
    fn exit_failure_keeps_code_status_and_only_sanitized_bounded_stderr() {
        let stderr = format!(
            "\u{1b}[31mcmux wrapper failed at /Users/private/bin/codex \
             token=private user@example.com {}\u{1b}[0m",
            "detail ".repeat(80)
        );
        let outcome = exit_failed(Some(126), stderr.as_bytes(), false);
        let diagnostic = outcome.diagnostic().expect("failure is diagnosed");

        assert_eq!(diagnostic.code(), "swallowtail.codex.discovery_exit_failed");
        assert!(diagnostic.message().contains("status 126"));
        assert!(diagnostic.message().contains("cmux wrapper failed"));
        assert!(diagnostic.message().contains("<path>"));
        assert!(diagnostic.message().contains("<redacted>"));
        assert!(diagnostic.message().contains("[stderr truncated]"));
        for private in [
            "/Users/private",
            "token=private",
            "user@example.com",
            "\u{1b}",
        ] {
            assert!(!diagnostic.message().contains(private));
        }
    }

    #[test]
    fn absent_or_unsafe_stderr_does_not_create_raw_detail() {
        assert!(sanitize_stderr(&[], false).is_none());
        assert_eq!(
            sanitize_stderr(b"/private/path", false).as_deref(),
            Some("<path>")
        );
        let status_only = exit_failed(Some(9), &[], false);
        assert_eq!(
            status_only
                .diagnostic()
                .expect("failure is diagnosed")
                .message(),
            "Codex version probe exited with status 9"
        );
    }
}
