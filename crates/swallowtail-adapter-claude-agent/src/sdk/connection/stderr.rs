//! Bounded, redacted stderr retained for one sidecar terminal diagnostic.

const MAX_STDERR_TAIL_BYTES: usize = 2048;
const MAX_SAFE_STDERR_CHARS: usize = 240;
const STDERR_TRUNCATED_SUFFIX: &str = " [stderr truncated]";

#[derive(Default)]
pub(super) struct StderrTail {
    bytes: Vec<u8>,
    truncated: bool,
}

impl StderrTail {
    pub(super) fn append(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
        if self.bytes.len() > MAX_STDERR_TAIL_BYTES {
            let overflow = self.bytes.len() - MAX_STDERR_TAIL_BYTES;
            self.bytes.drain(..overflow);
            self.truncated = true;
        }
    }

    pub(super) fn excerpt(&self) -> Option<String> {
        sanitize(&self.bytes, self.truncated)
    }
}

fn sanitize(stderr: &[u8], stderr_was_truncated: bool) -> Option<String> {
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
    // Only fixed diagnostic vocabulary crosses. A denylist cannot redact
    // arbitrary provider text or a secret split from its label by whitespace.
    !matches!(
        token,
        "native"
            | "process"
            | "exit"
            | "exited"
            | "failed"
            | "error"
            | "result"
            | "stderr"
            | "spawn"
            | "ENOENT"
            | "EACCES"
            | "EPERM"
            | "ECONNRESET"
            | "ETIMEDOUT"
            | "SIGTERM"
            | "SIGKILL"
    )
}

#[cfg(test)]
mod tests {
    use super::StderrTail;

    #[test]
    fn stderr_is_bounded_and_only_fixed_diagnostic_vocabulary_crosses() {
        let mut tail = StderrTail::default();
        tail.append(
            format!(
                "\u{1b}[31mnative /private/fixture token=secret user@example.test {}",
                "detail ".repeat(80)
            )
            .as_bytes(),
        );
        let excerpt = tail.excerpt().expect("stderr has safe words");
        assert!(excerpt.len() <= 240 + " [stderr truncated]".len());
        assert!(excerpt.contains("<path>"));
        assert!(excerpt.contains("<redacted>"));
        assert!(excerpt.ends_with("[stderr truncated]"));
        assert!(!excerpt.contains("/private/fixture"));
        assert!(!excerpt.contains("secret"));
        assert!(!excerpt.contains("user@example.test"));
    }
    #[test]
    fn free_text_and_a_truncated_secret_prefix_never_cross() {
        let mut tail = StderrTail::default();
        tail.append(format!("Bearer {} private provider detail", "s".repeat(2200)).as_bytes());
        let excerpt = tail.excerpt().expect("redaction markers remain");
        for forbidden in ["Bearer", "ssss", "private", "provider", "detail"] {
            assert!(!excerpt.contains(forbidden));
        }
    }
}
