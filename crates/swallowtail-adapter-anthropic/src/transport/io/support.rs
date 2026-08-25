fn capture_header(line: &[u8], headers: &mut BTreeMap<String, String>) {
    let Ok(line) = std::str::from_utf8(line) else {
        return;
    };
    let Some((name, value)) = line.split_once(':') else {
        return;
    };
    let name = name.trim().to_ascii_lowercase();
    if name == "request-id" || name.starts_with("anthropic-ratelimit-") {
        headers.insert(name, value.trim().to_owned());
    }
}

fn parse_status(line: &[u8]) -> Option<u32> {
    std::str::from_utf8(line)
        .ok()?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.endpoint_invalid",
        "Host-approved Anthropic endpoint was not an eligible HTTP endpoint",
    )
}

fn response_limit() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.response_limit",
        "Anthropic response exceeded the bounded input limit",
    )
}

fn backpressure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.sse_backpressure",
        "Anthropic SSE delivery exceeded its bounded capacity",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.request_cancelled",
        "Anthropic HTTP request was cancelled",
    )
}

fn curl_failure(_: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.transport_failed",
        "Anthropic HTTP transport failed",
    )
}

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

fn attach_post_body<'easy, 'data>(
    transfer: &mut curl::easy::Transfer<'easy, 'data>,
    body: &'data [u8],
    offset: &'data Cell<usize>,
) -> Result<(), RuntimeFailure> {
    transfer
        .read_function(|into| {
            let start = offset.get();
            let rest = body.get(start..).unwrap_or(&[]);
            let n = rest.len().min(into.len());
            into[..n].copy_from_slice(&rest[..n]);
            offset.set(start + n);
            Ok(n)
        })
        .map_err(curl_failure)?;
    transfer
        .seek_function(|whence| {
            let len = body.len() as u64;
            let current = offset.get() as u64;
            let next = match whence {
                SeekFrom::Start(position) => Some(position),
                SeekFrom::Current(delta) => current.checked_add_signed(delta),
                SeekFrom::End(delta) => len.checked_add_signed(delta),
            };
            match next {
                Some(position) if position <= len => {
                    offset.set(position as usize);
                    SeekResult::Ok
                }
                _ => SeekResult::Fail,
            }
        })
        .map_err(curl_failure)
}
