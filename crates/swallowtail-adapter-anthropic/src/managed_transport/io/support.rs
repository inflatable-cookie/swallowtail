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
        "swallowtail.anthropic.managed.endpoint_invalid",
        "Host-approved Anthropic Managed Agents endpoint was not eligible",
    )
}

fn response_limit() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.response_limit",
        "Anthropic Managed Agents response exceeded the bounded input limit",
    )
}

fn backpressure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.sse_backpressure",
        "Anthropic Managed Agents SSE delivery exceeded its bounded capacity",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.request_cancelled",
        "Anthropic Managed Agents HTTP request was cancelled",
    )
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.protocol_invalid",
        "Anthropic Managed Agents event stream framing was invalid",
    )
}

fn provider_failure(operation: &str) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.provider_failed",
        format!("Anthropic Managed Agents {operation} failed"),
    )
}

fn curl_failure(_: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.transport_failed",
        "Anthropic Managed Agents HTTP transport failed",
    )
}

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}
