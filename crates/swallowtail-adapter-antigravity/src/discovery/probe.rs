use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::{DiscoveryOutcome, DiscoveryStatus, SafeDiagnostic};
use swallowtail_runtime::{BoxFuture, ProcessHandle, RuntimeFailure};

use crate::antigravity_release_binding;

pub(super) enum ProbeSignal {
    Output(Result<Option<swallowtail_runtime::ProcessOutputChunk>, RuntimeFailure>),
    TimedOut,
    Cancelled,
}

pub(super) async fn next_output(
    process: &dyn ProcessHandle,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancelled: &mut BoxFuture<'static, ()>,
) -> ProbeSignal {
    let mut output = process.read_output();
    poll_fn(|context| {
        if cancelled.as_mut().poll(context).is_ready() {
            return Poll::Ready(ProbeSignal::Cancelled);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(ProbeSignal::TimedOut);
        }
        output.as_mut().poll(context).map(ProbeSignal::Output)
    })
    .await
}

pub(super) async fn stop_and_classify(
    process: &dyn ProcessHandle,
    desired: DiscoveryOutcome,
) -> DiscoveryOutcome {
    let graceful = process.request_stop().await;
    let forced = process.force_stop().await;
    let waited = process.wait().await;
    if graceful.is_err() || forced.is_err() || waited.is_err() {
        outcome(DiscoveryStatus::CleanupFailed)
    } else {
        desired
    }
}

pub(super) fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    if value.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return None;
    }
    antigravity_release_binding(value)
}

pub(super) fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "Antigravity installed discovery did not produce a compatible observation",
        )),
    )
}

pub(super) fn staged_outcome(status: DiscoveryStatus, stage: &'static str) -> DiscoveryOutcome {
    let (code, message) = match stage {
        "spawn_failed" => (
            "swallowtail.antigravity.discovery_spawn_failed",
            "Antigravity version probe could not start",
        ),
        "output_failed" => (
            "swallowtail.antigravity.discovery_output_failed",
            "Antigravity version probe output could not be read",
        ),
        "output_limit" => (
            "swallowtail.antigravity.discovery_output_limit",
            "Antigravity version probe exceeded its output limit",
        ),
        _ => (
            "swallowtail.antigravity.discovery_failed",
            "Antigravity version probe failed",
        ),
    };
    DiscoveryOutcome::new(status, Some(SafeDiagnostic::new(code, message)))
}

pub(super) fn exit_failed(
    exit_code: Option<i32>,
    stderr: &[u8],
    stderr_was_truncated: bool,
) -> DiscoveryOutcome {
    let mut message = match exit_code {
        Some(code) => format!("Antigravity version probe exited with status {code}"),
        None => "Antigravity version probe did not exit successfully".to_owned(),
    };
    if let Some(stderr) = sanitized_stderr(stderr, stderr_was_truncated) {
        message.push_str("; stderr: ");
        message.push_str(&stderr);
    }
    DiscoveryOutcome::new(
        DiscoveryStatus::Failed,
        Some(SafeDiagnostic::new(
            "swallowtail.antigravity.discovery_exit_failed",
            message,
        )),
    )
}

pub(crate) fn sanitized_stderr(stderr: &[u8], stderr_was_truncated: bool) -> Option<String> {
    let normalized = normalized_ascii(stderr);
    let mut excerpt = String::new();
    let mut truncated = stderr_was_truncated;
    for token in normalized.split_whitespace() {
        let token = if token_is_sensitive(token) {
            if token.contains('/') || token.contains('\\') {
                "<path>"
            } else {
                "<redacted>"
            }
        } else {
            token
        };
        let separator = usize::from(!excerpt.is_empty());
        let remaining = super::MAX_SAFE_STDERR_CHARS.saturating_sub(excerpt.chars().count());
        if separator + token.chars().count() > remaining {
            truncated = true;
            break;
        }
        if separator == 1 {
            excerpt.push(' ');
        }
        excerpt.push_str(token);
    }
    if excerpt.is_empty() {
        return None;
    }
    if truncated {
        excerpt.push_str(" [stderr truncated]");
    }
    Some(excerpt)
}

pub(super) fn normalized_ascii(stderr: &[u8]) -> String {
    let mut normalized = String::new();
    let mut ansi_state = 0_u8;
    for character in String::from_utf8_lossy(stderr).chars() {
        match ansi_state {
            1 if character == '[' => ansi_state = 2,
            1 => ansi_state = 0,
            2 if ('@'..='~').contains(&character) => ansi_state = 0,
            2 => {}
            _ if character == '\u{1b}' => ansi_state = 1,
            _ if character.is_ascii_graphic() => normalized.push(character),
            _ if character.is_whitespace() => normalized.push(' '),
            _ => normalized.push('?'),
        }
    }
    normalized
}

pub(super) fn token_is_sensitive(token: &str) -> bool {
    let token = token.trim_matches(|character: char| {
        matches!(
            character,
            '\'' | '"' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';'
        )
    });
    let lower = token.to_ascii_lowercase();
    token.contains('/')
        || token.contains('\\')
        || token.contains('@')
        || token.contains('=')
        || token.chars().count() > 64
        || [
            "authorization",
            "api_key",
            "apikey",
            "bearer",
            "credential",
            "password",
            "secret",
            "token",
        ]
        .iter()
        .any(|shape| lower.contains(shape))
}

pub(super) const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.antigravity.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.antigravity.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.antigravity.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.antigravity.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.antigravity.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.antigravity.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.antigravity.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.antigravity.discovery_cleanup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::{exit_failed, parse_version};

    #[test]
    fn parser_accepts_only_bare_antigravity_semver() {
        assert_eq!(
            parse_version(b"1.1.9\n")
                .expect("version parses")
                .version()
                .as_str(),
            "1.1.9"
        );
        for output in [
            b"agy 1.1.9".as_slice(),
            b"1.1.9 extra".as_slice(),
            b" 1.1.9\n".as_slice(),
            b"1.1.9\n\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }

    #[test]
    fn exit_failure_keeps_only_status_and_sanitized_bounded_stderr() {
        let stderr = format!(
            "\u{1b}[31mwrapper failed at /Users/private/bin/agy \
             token=private user@example.com {}\u{1b}[0m",
            "detail ".repeat(80)
        );
        let outcome = exit_failed(Some(126), stderr.as_bytes(), false);
        let diagnostic = outcome.diagnostic().expect("failure is diagnosed");
        assert_eq!(
            diagnostic.code(),
            "swallowtail.antigravity.discovery_exit_failed"
        );
        assert!(diagnostic.message().contains("status 126"));
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
}
