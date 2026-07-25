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
    message: &'static str,
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

pub(super) fn exit_failed() -> DiscoveryOutcome {
    staged_outcome(
        DiscoveryStatus::Failed,
        "swallowtail.codex.discovery_exit_failed",
        "Codex version probe did not exit successfully",
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
