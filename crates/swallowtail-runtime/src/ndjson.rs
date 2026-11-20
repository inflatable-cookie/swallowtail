//! Bounded LF-delimited line framing for newline-delimited provider streams.
//!
//! A provider stream arrives as chunks that split lines arbitrarily. Every
//! adapter that frames such a stream holds one trailing partial line between
//! chunks and bounds the bytes of any single line, so this module owns that
//! one implementation instead of one copy per adapter.
//!
//! One complete line is handed to the callback as a borrowed slice and only a
//! trailing partial line is copied into the held buffer. Every length bound is
//! checked *before* bytes are copied, so one oversized chunk can neither grow
//! the held buffer past the bound nor allocate the rejected text, and each
//! chunk is visited once instead of draining the buffer per line.

/// Scans `bytes` into complete lines, holding one trailing partial line.
///
/// `maximum` bounds the bytes of one line, excluding the `\n` delimiter. An
/// oversized line is reported through `too_long` before any of its bytes are
/// held, and the callback is then never called for it. `on_line` receives each
/// complete line without its `\n` delimiter; a caller that also tolerates
/// `\r\n` framing strips the carriage return itself.
///
/// The scan is linear in the bytes offered: each byte is visited once whether
/// it completes a line, joins the held partial line, or starts the next one.
pub fn scan_lines<E>(
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

/// Returns `line` without a trailing carriage return, for `\r\n` framing.
#[must_use]
pub fn strip_carriage_return(line: &[u8]) -> &[u8] {
    line.strip_suffix(b"\r").unwrap_or(line)
}

#[cfg(test)]
mod tests {
    use super::{scan_lines, strip_carriage_return};

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

    #[test]
    fn a_callback_error_stops_the_scan_with_the_line_already_consumed() {
        let mut pending = Vec::new();
        let mut seen = Vec::new();
        let scanned = scan_lines(&mut pending, b"one\ntwo\nthree\n", 8, limit, |line| {
            seen.push(String::from_utf8(line.to_vec()).expect("fixture line is UTF-8"));
            if line == b"two" {
                return Err("stop");
            }
            Ok(())
        });
        assert_eq!(scanned, Err("stop"));
        assert_eq!(seen, ["one".to_owned(), "two".to_owned()]);
    }

    #[test]
    fn a_carriage_return_is_stripped_only_when_the_caller_asks() {
        let mut pending = Vec::new();
        let mut lines: Vec<Vec<u8>> = Vec::new();
        scan_lines(&mut pending, b"one\r\nplain\n", 8, limit, |line| {
            lines.push(strip_carriage_return(line).to_vec());
            Ok(())
        })
        .expect("chunk is within bounds");
        assert_eq!(lines, [b"one".to_vec(), b"plain".to_vec()]);
        assert_eq!(strip_carriage_return(b"plain"), b"plain");
    }
}
