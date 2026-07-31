use swallowtail_core::{DiscoveryOutcome, DiscoveryStatus, SafeDiagnostic};

const MAX_SAFE_STDERR_CHARS: usize = 240;

pub(super) fn outcome(status: DiscoveryStatus) -> DiscoveryOutcome {
    DiscoveryOutcome::new(
        status,
        Some(SafeDiagnostic::new(
            status_code(status),
            "Cursor installed discovery did not produce a compatible observation",
        )),
    )
}

pub(super) fn staged_outcome(status: DiscoveryStatus, stage: &'static str) -> DiscoveryOutcome {
    let (code, message) = match stage {
        "spawn_failed" => (
            "swallowtail.cursor.discovery_spawn_failed",
            "Cursor version probe could not start",
        ),
        "output_failed" => (
            "swallowtail.cursor.discovery_output_failed",
            "Cursor version probe output could not be read",
        ),
        "output_limit" => (
            "swallowtail.cursor.discovery_output_limit",
            "Cursor version probe exceeded its output limit",
        ),
        _ => (
            "swallowtail.cursor.discovery_failed",
            "Cursor version probe failed",
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
        Some(code) => format!("Cursor version probe exited with status {code}"),
        None => "Cursor version probe did not exit successfully".to_owned(),
    };
    if let Some(stderr) = sanitize_stderr(stderr, stderr_was_truncated) {
        message.push_str("; stderr: ");
        message.push_str(&stderr);
    }
    DiscoveryOutcome::new(
        DiscoveryStatus::Failed,
        Some(SafeDiagnostic::new(
            "swallowtail.cursor.discovery_exit_failed",
            message,
        )),
    )
}

fn sanitize_stderr(stderr: &[u8], stderr_was_truncated: bool) -> Option<String> {
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
        let remaining = MAX_SAFE_STDERR_CHARS.saturating_sub(excerpt.chars().count());
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

fn normalized_ascii(stderr: &[u8]) -> String {
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

fn token_is_sensitive(token: &str) -> bool {
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

const fn status_code(status: DiscoveryStatus) -> &'static str {
    match status {
        DiscoveryStatus::Absent => "swallowtail.cursor.discovery_absent",
        DiscoveryStatus::Discovered => "swallowtail.cursor.discovery_discovered",
        DiscoveryStatus::Incompatible => "swallowtail.cursor.discovery_incompatible",
        DiscoveryStatus::Malformed => "swallowtail.cursor.discovery_malformed",
        DiscoveryStatus::TimedOut => "swallowtail.cursor.discovery_timed_out",
        DiscoveryStatus::Cancelled => "swallowtail.cursor.discovery_cancelled",
        DiscoveryStatus::Failed => "swallowtail.cursor.discovery_failed",
        DiscoveryStatus::CleanupFailed => "swallowtail.cursor.discovery_cleanup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::exit_failed;

    #[test]
    fn exit_failure_keeps_only_status_and_sanitized_bounded_stderr() {
        let stderr = format!(
            "\u{1b}[31mwrapper failed at /Users/private/bin/cursor-agent \
             token=private user@example.com {}\u{1b}[0m",
            "detail ".repeat(80)
        );
        let outcome = exit_failed(Some(126), stderr.as_bytes(), false);
        let diagnostic = outcome.diagnostic().expect("failure is diagnosed");
        assert_eq!(
            diagnostic.code(),
            "swallowtail.cursor.discovery_exit_failed"
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
