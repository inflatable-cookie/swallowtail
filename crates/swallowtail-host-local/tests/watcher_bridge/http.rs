use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub(super) fn post_json(endpoint: &str, bearer: Option<&str>, body: &str) -> (u16, String) {
    post_json_result(endpoint, bearer, body).expect("http round-trip")
}

pub(super) fn post_json_result(
    endpoint: &str,
    bearer: Option<&str>,
    body: &str,
) -> Result<(u16, String), std::io::Error> {
    let without_scheme = endpoint
        .strip_prefix("http://")
        .expect("loopback http endpoint");
    let (host, path) = without_scheme.split_once('/').expect("host and path");
    let mut stream = TcpStream::connect(host)?;
    stream.set_read_timeout(Some(Duration::from_secs(8)))?;
    stream.set_write_timeout(Some(Duration::from_secs(8)))?;
    let authorization = bearer
        .map(|value| format!("Authorization: Bearer {value}\r\n"))
        .unwrap_or_default();
    let request = format!(
        "POST /{path} HTTP/1.1\r\nHost: {host}\r\n{authorization}Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(request.as_bytes())?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    let text = String::from_utf8_lossy(&response);
    let status = text
        .split_whitespace()
        .nth(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);
    let body = text.split("\r\n\r\n").nth(1).unwrap_or_default().to_owned();
    Ok((status, body))
}
