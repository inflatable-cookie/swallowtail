//! Feature-gated reference courier for Contract 063 mediated stdio.

use serde_json::{Value, json};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use swallowtail_host_local::wire::{
    REGISTERED_TOOL_PROXY_HTTP_PATH, REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES,
    REGISTERED_TOOL_PROXY_WIRE_TAG, RegisteredToolProxyRendezvousDocument,
};

const COURIER_CONNECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);
const COURIER_WRITE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);
// Contract 063 allows each call up to 60 seconds. This is only a transport
// stall backstop; the kernel remains the sole owner of caller, call, and lease
// deadlines. The margin lets a settled kernel error reach this process before
// a broken or stalled transport fails closed.
const COURIER_RESPONSE_STALL_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(65);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args_os();
    let _program = args.next();
    let wire_tag = args.next().ok_or("missing fixed courier wire tag")?;
    let rendezvous_path = args.next().ok_or("missing rendezvous path")?;
    if args.next().is_some() || wire_tag.to_string_lossy() != REGISTERED_TOOL_PROXY_WIRE_TAG {
        return Err("courier argv is outside the fixed wire shape".into());
    }
    let path = Path::new(&rendezvous_path);
    let document = read_once(path)?;
    let (host, port) = loopback_endpoint(&document.endpoint)?;
    let connect_timeout = std::time::Duration::from_millis(document.connect_timeout_ms);
    let mut stream = TcpStream::connect_timeout(
        &format!("{host}:{port}").parse()?,
        connect_timeout.min(COURIER_CONNECT_TIMEOUT),
    )?;
    stream.set_nodelay(true)?;
    stream.set_read_timeout(Some(COURIER_RESPONSE_STALL_TIMEOUT))?;
    stream.set_write_timeout(Some(COURIER_WRITE_TIMEOUT))?;
    negotiate(&mut stream, &document)?;
    let stdin = std::io::stdin();
    let mut input = BufReader::new(stdin.lock());
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    let mut record = Vec::new();
    loop {
        record.clear();
        if read_bounded_record(&mut input, &mut record)? == 0 {
            break;
        }
        if record.last() == Some(&b'\n') {
            record.pop();
            if record.last() == Some(&b'\r') {
                record.pop();
            }
        }
        if record.len() > REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES {
            return Err("courier record exceeds the fixed wire bound".into());
        }
        if record.is_empty() {
            continue;
        }
        let response = post(&mut stream, &document, &record)?;
        if !response.is_empty() {
            output.write_all(&response)?;
            output.write_all(b"\n")?;
            output.flush()?;
        }
    }
    Ok(())
}

fn negotiate(
    stream: &mut TcpStream,
    document: &RegisteredToolProxyRendezvousDocument,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "id": "swallowtail-courier",
        "method": "initialize",
        "params": { "protocolVersion": "2025-11-25" },
    }))?;
    let response = post(stream, document, &request)?;
    let response = serde_json::from_slice::<Value>(&response)?;
    if response
        .get("result")
        .and_then(|result| result.get("protocolVersion"))
        .and_then(Value::as_str)
        != Some("2025-11-25")
    {
        return Err("courier negotiation did not reach the fixed MCP protocol".into());
    }
    Ok(())
}

fn read_once(
    path: &Path,
) -> Result<RegisteredToolProxyRendezvousDocument, Box<dyn std::error::Error>> {
    let claim_path = path.with_extension("claimed");
    let _claim = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&claim_path)?;
    let mut file = OpenOptions::new().read(true).open(path)?;
    // Claiming the sidecar makes the open exclusive; unlink the rendezvous
    // immediately so no second courier can read the same authority.
    fs::remove_file(path)?;
    let body = {
        let mut body = Vec::new();
        file.read_to_end(&mut body)?;
        body
    };
    fs::remove_file(claim_path)?;
    RegisteredToolProxyRendezvousDocument::decode(&body)
        .map_err(|_| "invalid mediated stdio rendezvous".into())
}

fn loopback_endpoint(endpoint: &str) -> Result<(String, u16), Box<dyn std::error::Error>> {
    let authority = endpoint
        .strip_prefix("http://127.0.0.1:")
        .ok_or("courier endpoint is not private loopback HTTP")?;
    let (port, path) = authority
        .split_once('/')
        .ok_or("courier endpoint has no path")?;
    if format!("/{path}") != REGISTERED_TOOL_PROXY_HTTP_PATH {
        return Err("courier endpoint path is outside the fixed wire".into());
    }
    let port = port.parse::<u16>()?;
    Ok(("127.0.0.1".to_owned(), port))
}

fn post(
    stream: &mut TcpStream,
    document: &RegisteredToolProxyRendezvousDocument,
    body: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let request = format!(
        "POST {REGISTERED_TOOL_PROXY_HTTP_PATH} HTTP/1.1\r\nHost: 127.0.0.1\r\nAuthorization: Bearer {}\r\nX-Swallowtail-Lease-Generation: {}\r\nX-Swallowtail-Transport-Generation: {}\r\nX-Swallowtail-Server-Name: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
        document.bearer,
        document.lease_generation,
        document.transport_generation,
        document.server_name,
        body.len(),
    );
    stream.write_all(request.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    let (status, response) = read_response(stream)?;
    validate_response(body, status, &response)?;
    Ok(response)
}

fn read_response(stream: &mut TcpStream) -> Result<(u16, Vec<u8>), Box<dyn std::error::Error>> {
    let mut header = Vec::new();
    let mut byte = [0_u8; 1];
    loop {
        stream.read_exact(&mut byte)?;
        header.push(byte[0]);
        if header.ends_with(b"\r\n\r\n") {
            break;
        }
        if header.len() > 64 * 1024 {
            return Err("courier HTTP response headers exceed bound".into());
        }
    }
    let header_text = std::str::from_utf8(&header[..header.len() - 4])?;
    let mut lines = header_text.split("\r\n");
    let status = lines.next().ok_or("courier HTTP response has no status")?;
    let status = status
        .split_whitespace()
        .nth(1)
        .ok_or("courier HTTP response status missing")?;
    let status = status.parse::<u16>()?;
    if !(100..=599).contains(&status) {
        return Err("courier HTTP response status is outside the fixed range".into());
    }
    let mut length = None;
    for line in lines {
        let (name, value) = line
            .split_once(':')
            .ok_or("courier HTTP response header malformed")?;
        if name.eq_ignore_ascii_case("content-length") {
            if length.is_some() {
                return Err("courier HTTP response has duplicate content lengths".into());
            }
            length = Some(value.trim().parse::<usize>()?);
        } else if name.eq_ignore_ascii_case("transfer-encoding") {
            return Err("courier HTTP response uses an unsupported body encoding".into());
        }
    }
    let length = length.ok_or("courier HTTP response length missing")?;
    if length > REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES {
        return Err("courier HTTP response exceeds bound".into());
    }
    let mut body = vec![0_u8; length];
    stream.read_exact(&mut body)?;
    Ok((status, body))
}

fn read_bounded_record(input: &mut impl BufRead, record: &mut Vec<u8>) -> io::Result<usize> {
    loop {
        let available = input.fill_buf()?;
        if available.is_empty() {
            return Ok(record.len());
        }
        let end = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |index| index + 1);
        if record.len().saturating_add(end) > REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES + 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "courier record exceeds the fixed wire bound",
            ));
        }
        record.extend_from_slice(&available[..end]);
        let ended = available[end - 1] == b'\n';
        input.consume(end);
        if ended {
            return Ok(record.len());
        }
    }
}

fn validate_response(
    request: &[u8],
    status: u16,
    response: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let request = serde_json::from_slice::<Value>(request)?;
    let expected_id = request.get("id");
    if expected_id.is_none() {
        if (200..300).contains(&status) && response.is_empty() {
            return Ok(());
        }
        return Err("courier notification received an unexpected HTTP response".into());
    }
    let response = serde_json::from_slice::<Value>(response)?;
    if response.get("jsonrpc").and_then(Value::as_str) != Some("2.0")
        || response.get("id") != expected_id
    {
        return Err("courier HTTP response does not match the pending request".into());
    }
    let has_result = response.get("result").is_some_and(Value::is_object);
    let has_error = response.get("error").is_some_and(Value::is_object);
    if has_result == has_error || (!(200..300).contains(&status) && !has_error) {
        return Err("courier HTTP response is not a correlated JSON-RPC outcome".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn correlated_http_errors_are_reusable_json_rpc_responses() {
        validate_response(
            br#"{"jsonrpc":"2.0","id":7,"method":"tools/call"}"#,
            413,
            br#"{"jsonrpc":"2.0","id":7,"error":{"code":-32000,"message":"denied"}}"#,
        )
        .expect("correlated error can be returned to the SDK");
    }

    #[test]
    fn mismatched_or_malformed_responses_fail_closed() {
        assert!(
            validate_response(
                br#"{"jsonrpc":"2.0","id":7,"method":"tools/call"}"#,
                200,
                br#"{"jsonrpc":"2.0","id":8,"result":{}}"#,
            )
            .is_err()
        );
        assert!(
            validate_response(
                br#"{"jsonrpc":"2.0","id":7,"method":"tools/call"}"#,
                409,
                br#"{"jsonrpc":"2.0","id":null,"error":{"code":-32000,"message":"stale"}}"#,
            )
            .is_err()
        );
        assert!(
            validate_response(
                br#"{"jsonrpc":"2.0","id":7,"method":"tools/call"}"#,
                200,
                br#"{"jsonrpc":"2.0","id":7,"result":{},"error":{}}"#,
            )
            .is_err()
        );
        assert!(
            validate_response(
                br#"{"jsonrpc":"2.0","id":7,"method":"tools/call"}"#,
                200,
                br#"{"jsonrpc":"2.0","id":7,"error":null}"#,
            )
            .is_err()
        );
        assert!(
            validate_response(
                br#"{"jsonrpc":"2.0","id":7,"method":"tools/call"}"#,
                200,
                br#"{"jsonrpc":"2.0","id":7,"result":null}"#,
            )
            .is_err()
        );
    }

    #[test]
    fn stdio_record_limit_is_enforced_before_unbounded_growth() {
        let oversized = vec![b'x'; REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES + 3];
        let mut input = Cursor::new(oversized);
        let error = read_bounded_record(&mut input, &mut Vec::new())
            .expect_err("oversized input fails before a full record is allocated");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn stalled_partial_http_response_obeys_the_transport_backstop() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fake endpoint");
        let address = listener.local_addr().expect("endpoint address");
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("courier connects");
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 20\r\n")
                .expect("partial response header");
            thread::sleep(Duration::from_millis(100));
        });
        let mut stream = TcpStream::connect(address).expect("fake endpoint connects");
        stream
            .set_read_timeout(Some(Duration::from_millis(20)))
            .expect("bounded fixture read");
        assert!(read_response(&mut stream).is_err());
        server.join().expect("fake endpoint joins");
    }
}
