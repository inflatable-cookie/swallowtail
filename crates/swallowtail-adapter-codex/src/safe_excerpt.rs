pub(crate) const MAX_SAFE_STDERR_CHARS: usize = 240;
pub(crate) const STDERR_TRUNCATED_SUFFIX: &str = " [stderr truncated]";

pub(crate) fn sanitize_stderr(stderr: &[u8], stderr_was_truncated: bool) -> Option<String> {
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
