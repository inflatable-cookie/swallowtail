//! Bounded LF-delimited framing shared by the Claude Code stream parsers and
//! the private SDK wire decoder.
//!
//! One complete line is handed to the callback as a borrowed slice, and only a
//! trailing partial line is held for the next chunk. Every length bound is
//! checked *before* bytes are copied, so one oversized chunk can neither grow
//! the held buffer past the bound nor allocate the rejected text, and each
//! chunk is visited once instead of draining the buffer per line.

/// Scans `bytes` into complete lines, holding one trailing partial line.
///
/// `maximum` bounds the bytes of one line, excluding the delimiter. `too_long`
/// names the caller's own bound failure; `on_line` receives each complete line
/// without its `\n` delimiter.
pub(crate) fn scan_lines<E>(
    pending: &mut Vec<u8>,
    bytes: &[u8],
    maximum: usize,
    too_long: impl Fn() -> E,
    mut on_line: impl FnMut(&[u8]) -> Result<(), E>,
) -> Result<(), E> {
    let mut rest = bytes;
    loop {
        if pending.is_empty() {
            // Fast path: no partial line is held, so a complete line is parsed
            // straight out of the caller's chunk without a copy.
            let Some(end) = rest.iter().position(|byte| *byte == b'\n') else {
                break;
            };
            if end > maximum {
                return Err(too_long());
            }
            on_line(&rest[..end])?;
            rest = &rest[end + 1..];
        } else {
            let Some(end) = rest.iter().position(|byte| *byte == b'\n') else {
                break;
            };
            if pending.len() + end > maximum {
                return Err(too_long());
            }
            pending.extend_from_slice(&rest[..end]);
            let line = std::mem::take(pending);
            on_line(&line)?;
            rest = &rest[end + 1..];
        }
    }
    if pending.len() + rest.len() > maximum {
        return Err(too_long());
    }
    pending.extend_from_slice(rest);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::scan_lines;

    fn limit() -> &'static str {
        "line too long"
    }

    fn lines(chunks: &[&[u8]]) -> Vec<String> {
        let mut pending = Vec::new();
        let mut lines = Vec::new();
        for chunk in chunks {
            scan_lines(&mut pending, chunk, 8, limit, |line| {
                lines.push(String::from_utf8(line.to_vec()).expect("fixture line is UTF-8"));
                Ok(())
            })
            .expect("chunk is within bounds");
        }
        assert!(pending.is_empty(), "fixture ends on a delimiter");
        lines
    }

    #[test]
    fn complete_lines_split_across_chunks_keep_their_order() {
        assert_eq!(
            lines(&[b"one\ntw", b"o\n\nthree\n"]),
            [
                "one".to_owned(),
                "two".to_owned(),
                "".to_owned(),
                "three".to_owned()
            ]
        );
    }

    #[test]
    fn one_oversized_chunk_is_rejected_before_any_bytes_are_held() {
        let mut pending = Vec::new();
        let oversized = vec![b'x'; 4096];
        assert_eq!(
            scan_lines(&mut pending, &oversized, 8, limit, |_| Ok(())),
            Err("line too long")
        );
        assert!(
            pending.is_empty(),
            "a rejected chunk must not be held for the next call"
        );
    }

    #[test]
    fn an_oversized_completed_line_stops_before_the_remaining_copy() {
        let mut pending = Vec::new();
        let oversized = b"xxxxxxxxxx\nshort\n";
        assert_eq!(
            scan_lines(&mut pending, oversized, 8, limit, |_| Ok(())),
            Err("line too long")
        );
        assert!(pending.is_empty());
    }

    #[test]
    fn a_partial_line_exactly_at_the_bound_is_still_accepted() {
        let mut pending = Vec::new();
        scan_lines(&mut pending, b"12345678", 8, limit, |_| Ok(())).expect("bound is inclusive");
        assert_eq!(pending.len(), 8);
        scan_lines(&mut pending, b"\n", 8, limit, |_| Ok(())).expect("completed line is accepted");
        assert!(pending.is_empty());
    }

    #[test]
    fn a_partial_line_over_the_bound_is_rejected_after_the_cursor_scan() {
        let mut pending = Vec::new();
        assert_eq!(
            scan_lines(&mut pending, b"123456789", 8, limit, |_| Ok(())),
            Err("line too long")
        );
        assert!(pending.is_empty());
    }
}
