use crate::output::failure;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use swallowtail_runtime::{
    RuntimeFailure, WATCHER_BRIDGE_MAX_BODY_BYTES, WATCHER_BRIDGE_MAX_HEADER_BYTES,
    WATCHER_BRIDGE_MAX_HEADER_COUNT,
};
use zeroize::Zeroizing;

const READ_TIMEOUT: Duration = Duration::from_secs(5);
const HEADER_TERMINATOR: &[u8] = b"\r\n\r\n";

pub(crate) struct HttpRequest {
    pub(crate) bearer: Option<Zeroizing<String>>,
    pub(crate) body: Vec<u8>,
}

pub(crate) fn configure_stream(stream: &TcpStream) -> Result<(), RuntimeFailure> {
    stream.set_nodelay(true).map_err(|_| transport_failure())?;
    stream
        .set_read_timeout(Some(READ_TIMEOUT))
        .map_err(|_| transport_failure())?;
    stream
        .set_write_timeout(Some(READ_TIMEOUT))
        .map_err(|_| transport_failure())?;
    Ok(())
}

pub(crate) fn read_request(
    stream: &mut TcpStream,
    expected_path: &str,
) -> Result<HttpRequest, HttpReject> {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 512];
    let header_end = loop {
        if buffer.len() > WATCHER_BRIDGE_MAX_HEADER_BYTES {
            return Err(HttpReject::Oversized);
        }
        let read = stream.read(&mut chunk).map_err(|_| HttpReject::Malformed)?;
        if read == 0 {
            return Err(HttpReject::Malformed);
        }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(index) = find_subslice(&buffer, HEADER_TERMINATOR) {
            break index;
        }
    };
    if header_end > WATCHER_BRIDGE_MAX_HEADER_BYTES {
        return Err(HttpReject::Oversized);
    }
    let header_text =
        std::str::from_utf8(&buffer[..header_end]).map_err(|_| HttpReject::Malformed)?;
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().ok_or(HttpReject::Malformed)?;
    let mut parts = request_line.split(' ');
    let method = parts.next().ok_or(HttpReject::Malformed)?;
    let path = parts.next().ok_or(HttpReject::Malformed)?;
    let version = parts.next().ok_or(HttpReject::Malformed)?;
    if parts.next().is_some() {
        return Err(HttpReject::Malformed);
    }
    if method != "POST" {
        return Err(HttpReject::Method);
    }
    if path != expected_path {
        return Err(HttpReject::Malformed);
    }
    if version != "HTTP/1.0" && version != "HTTP/1.1" {
        return Err(HttpReject::Malformed);
    }

    let mut bearer = None;
    let mut content_type = None;
    let mut content_length = None;
    let mut header_count = 0_usize;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        header_count += 1;
        if header_count > WATCHER_BRIDGE_MAX_HEADER_COUNT {
            return Err(HttpReject::Oversized);
        }
        let (name, value) = line.split_once(':').ok_or(HttpReject::Malformed)?;
        let name = name.trim();
        let value = value.trim();
        if name.eq_ignore_ascii_case("authorization") {
            bearer = Some(parse_bearer(value)?);
        } else if name.eq_ignore_ascii_case("content-type") {
            content_type = Some(value.to_ascii_lowercase());
        } else if name.eq_ignore_ascii_case("content-length") {
            content_length = Some(parse_content_length(value)?);
        } else if name.eq_ignore_ascii_case("transfer-encoding") {
            return Err(HttpReject::Malformed);
        }
    }
    let content_type = content_type.ok_or(HttpReject::Malformed)?;
    if content_type != "application/json" && content_type != "application/json; charset=utf-8" {
        return Err(HttpReject::Malformed);
    }
    let content_length = content_length.ok_or(HttpReject::Malformed)?;
    let mut body = buffer.split_off(header_end + HEADER_TERMINATOR.len());
    if content_length > WATCHER_BRIDGE_MAX_BODY_BYTES {
        drain_body(stream, &mut body, content_length, &mut chunk);
        return Err(HttpReject::Oversized);
    }

    while body.len() < content_length {
        let read = stream.read(&mut chunk).map_err(|_| HttpReject::Malformed)?;
        if read == 0 {
            return Err(HttpReject::Malformed);
        }
        body.extend_from_slice(&chunk[..read]);
        if body.len() > WATCHER_BRIDGE_MAX_BODY_BYTES {
            return Err(HttpReject::Oversized);
        }
    }
    body.truncate(content_length);
    Ok(HttpRequest { bearer, body })
}

pub(crate) fn write_json(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    body: &str,
) -> Result<(), RuntimeFailure> {
    write_response(stream, status, reason, body.as_bytes(), "close")
}

pub(crate) fn write_response(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    body: &[u8],
    connection: &str,
) -> Result<(), RuntimeFailure> {
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: {connection}\r\n\r\n",
        body.len(),
    );
    stream
        .write_all(response.as_bytes())
        .map_err(|_| transport_failure())?;
    stream.write_all(body).map_err(|_| transport_failure())?;
    let _ = stream.flush();
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HttpReject {
    Malformed,
    Oversized,
    Method,
}

fn parse_bearer(value: &str) -> Result<Zeroizing<String>, HttpReject> {
    let secret = value.strip_prefix("Bearer ").ok_or(HttpReject::Malformed)?;
    if secret.is_empty() || secret.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(HttpReject::Malformed);
    }
    Ok(Zeroizing::new(secret.to_owned()))
}

pub(crate) fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .fold(0_u8, |acc, (left, right)| acc | (left ^ right))
            == 0
}

fn parse_content_length(value: &str) -> Result<usize, HttpReject> {
    value.parse().map_err(|_| HttpReject::Malformed)
}

fn drain_body(stream: &mut TcpStream, body: &mut Vec<u8>, content_length: usize, chunk: &mut [u8]) {
    let drain_to = content_length.min(WATCHER_BRIDGE_MAX_BODY_BYTES.saturating_add(4096));
    while body.len() < drain_to {
        match stream.read(chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => body.extend_from_slice(&chunk[..read]),
        }
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn transport_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.transport_failed",
        "Watcher bridge transport failed",
    )
}
