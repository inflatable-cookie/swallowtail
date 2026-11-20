pub(crate) const MAX_SAFE_STDERR_CHARS: usize = 240;
pub(crate) const STDERR_TRUNCATED_SUFFIX: &str = " [stderr truncated]";

pub(crate) fn sanitize_stderr(stderr: &[u8], stderr_was_truncated: bool) -> Option<String> {
    let normalized = normalized_ascii(stderr);
    let mut excerpt = String::new();
    let mut truncated = stderr_was_truncated;
    // A secret label such as `Authorization:` or `Bearer` makes the token after
    // it the secret too: `Authorization: Bearer sk-...` is two labels and one value.
    let mut redact_next = false;

    for raw in normalized.split_whitespace() {
        let redact = redact_next || token_is_sensitive(raw);
        redact_next = redact && is_secret_label(raw);
        let token = if redact {
            if raw.contains('/') || raw.contains('\\') {
                "<path>"
            } else {
                "<redacted>"
            }
        } else {
            raw
        };
        let separator_len = usize::from(!excerpt.is_empty());
        let remaining = MAX_SAFE_STDERR_CHARS.saturating_sub(excerpt.chars().count());
        if separator_len + token.chars().count() > remaining {
            truncated = true;
            break;
        }
        if separator_len == 1 {
            excerpt.push(' ');
        }
        excerpt.push_str(token);
    }

    if excerpt.is_empty() {
        return None;
    }
    if truncated {
        excerpt.push_str(STDERR_TRUNCATED_SUFFIX);
    }
    Some(excerpt)
}

pub(crate) fn normalized_ascii(stderr: &[u8]) -> String {
    let mut normalized = String::new();
    let mut ansi_state = 0_u8;
    for character in String::from_utf8_lossy(stderr).chars() {
        match ansi_state {
            1 if character == '[' => {
                ansi_state = 2;
            }
            1 => {
                ansi_state = 0;
            }
            2 if ('@'..='~').contains(&character) => {
                ansi_state = 0;
            }
            2 => {}
            _ if character == '\u{1b}' => {
                ansi_state = 1;
            }
            _ if character.is_ascii_graphic() => normalized.push(character),
            _ if character.is_whitespace() => normalized.push(' '),
            _ => normalized.push('?'),
        }
    }
    normalized
}

fn token_is_sensitive(token: &str) -> bool {
    let token = trim_token(token);
    let lower = token.to_ascii_lowercase();
    token.contains('/')
        || token.contains('\\')
        || token.contains('@')
        || token.contains('=')
        || token.chars().count() > 64
        || [
            "authorization",
            "api_key",
            "api-key",
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

/// A token whose following token carries the secret: a `label:` such as
/// `Authorization:` or `token:`, or an auth scheme such as `Bearer` or `Basic`.
fn is_secret_label(token: &str) -> bool {
    let lower = trim_token(token).to_ascii_lowercase();
    lower.ends_with(':')
        || matches!(lower.as_str(), "bearer" | "basic")
        || lower
            .rsplit_once(':')
            .is_some_and(|(_, scheme)| matches!(scheme, "bearer" | "basic"))
}

fn trim_token(token: &str) -> &str {
    token.trim_matches(|character: char| {
        matches!(
            character,
            '\'' | '"' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';'
        )
    })
}

#[cfg(test)]
mod tests {
    use super::sanitize_stderr;

    fn excerpt(input: &str) -> String {
        sanitize_stderr(input.as_bytes(), false).expect("non-empty excerpt")
    }

    #[test]
    fn bearer_value_after_authorization_label_is_redacted() {
        let output = excerpt("Authorization: Bearer sk-live-1234567890abcdef");
        assert_eq!(output, "<redacted> <redacted> <redacted>");
        assert!(!output.contains("sk-live"));
    }

    #[test]
    fn value_after_token_label_is_redacted_and_following_text_kept() {
        assert_eq!(
            excerpt("token: abc123 request failed"),
            "<redacted> <redacted> request failed"
        );
    }

    #[test]
    fn value_after_api_key_and_password_labels_is_redacted() {
        assert_eq!(excerpt("api-key: xyz789"), "<redacted> <redacted>");
        assert_eq!(
            excerpt("password: hunter2 rejected"),
            "<redacted> <redacted> rejected"
        );
    }

    #[test]
    fn basic_scheme_value_is_redacted() {
        let output = excerpt("Authorization: Basic dXNlcjpwYXNz");
        assert_eq!(output, "<redacted> <redacted> <redacted>");
        assert!(!output.contains("dXNlcjpwYXNz"));
    }

    #[test]
    fn joined_authorization_scheme_value_is_redacted() {
        let output = excerpt("Authorization:Bearer sk-live-1234567890abcdef");
        assert!(!output.contains("sk-live"));
    }

    #[test]
    fn unlabelled_text_is_unchanged() {
        assert_eq!(
            excerpt("codex app-server ready on stdio"),
            "codex app-server ready on stdio"
        );
    }
}
