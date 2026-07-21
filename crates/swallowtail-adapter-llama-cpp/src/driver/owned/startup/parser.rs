use crate::failure::failure;
use swallowtail_runtime::{ProcessOutputChunk, RuntimeFailure};
use url::Url;

const STARTUP_LIMIT: usize = 16 * 1024;
const LISTENING_MARKER: &str = "listening on ";

pub(super) fn extend_bounded(
    output: &mut Vec<u8>,
    chunk: &ProcessOutputChunk,
) -> Result<(), RuntimeFailure> {
    if chunk.bytes().len() > STARTUP_LIMIT.saturating_sub(output.len()) {
        return Err(failure(
            "swallowtail.llama_cpp.serving_output_limit",
            "Owned llama.cpp startup output exceeded its bounded fixture",
        ));
    }
    output.extend_from_slice(chunk.bytes());
    Ok(())
}

pub(super) fn parse_listening_endpoint(output: &[u8]) -> Result<Option<String>, RuntimeFailure> {
    let text = std::str::from_utf8(output).map_err(|_| endpoint_failure())?;
    let matches = text.match_indices(LISTENING_MARKER).collect::<Vec<_>>();
    if matches.len() > 1 {
        return Err(failure(
            "swallowtail.llama_cpp.serving_endpoint_duplicate",
            "Owned llama.cpp reported more than one listening endpoint",
        ));
    }
    let Some((offset, _)) = matches.first().copied() else {
        return Ok(None);
    };
    let record = &text[offset + LISTENING_MARKER.len()..];
    let Some(record_end) = record.find('\n') else {
        return Ok(None);
    };
    let value = record[..record_end]
        .split_whitespace()
        .next()
        .ok_or_else(endpoint_failure)?;
    let url = Url::parse(value).map_err(|_| endpoint_failure())?;
    if url.scheme() != "http"
        || url.host_str() != Some("127.0.0.1")
        || url.port().is_none_or(|port| port == 0)
        || url.path() != "/"
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(endpoint_failure());
    }
    Ok(Some(value.to_owned()))
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.llama_cpp.serving_endpoint_invalid",
        "Owned llama.cpp reported an invalid loopback endpoint",
    )
}

#[cfg(test)]
mod tests {
    use super::parse_listening_endpoint;

    #[test]
    fn parses_only_one_exact_loopback_http_endpoint() {
        assert_eq!(
            parse_listening_endpoint(b"srv  listening on http://127.0.0.1:49152\n")
                .expect("endpoint parses")
                .as_deref(),
            Some("http://127.0.0.1:49152")
        );
        assert_eq!(
            parse_listening_endpoint(b"srv listening on http://127.0.0.1:")
                .expect("partial record remains pending"),
            None
        );
        for invalid in [
            "srv listening on http://0.0.0.0:49152\n",
            "srv listening on https://127.0.0.1:49152\n",
            "srv listening on http://127.0.0.1:0\n",
            "srv listening on not-a-url\n",
        ] {
            assert!(parse_listening_endpoint(invalid.as_bytes()).is_err());
        }
    }

    #[test]
    fn rejects_duplicate_listening_records() {
        let output = b"listening on http://127.0.0.1:49152\nlistening on http://127.0.0.1:49153\n";
        let error = parse_listening_endpoint(output).expect_err("duplicate fails");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.llama_cpp.serving_endpoint_duplicate"
        );
    }
}
