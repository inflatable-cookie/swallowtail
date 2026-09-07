//! Feature-gated reference courier for Contract 063 mediated stdio.

use serde_json::{Value, json};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use swallowtail_host_local::wire::{
    REGISTERED_TOOL_PROXY_HTTP_PATH, REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES,
    REGISTERED_TOOL_PROXY_WIRE_TAG, RegisteredToolProxyRendezvousDocument,
};

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
    let mut stream = TcpStream::connect((host.as_str(), port))?;
    stream.set_nodelay(true)?;
    negotiate(&mut stream, &document.bearer)?;
    let stdin = std::io::stdin();
    let mut input = BufReader::new(stdin.lock());
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    let mut record = Vec::new();
    loop {
        record.clear();
        if input.read_until(b'\n', &mut record)? == 0 {
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
        let response = post(&mut stream, &document.bearer, &record)?;
        if !response.is_empty() {
            output.write_all(&response)?;
            output.write_all(b"\n")?;
            output.flush()?;
        }
    }
    Ok(())
}

fn negotiate(stream: &mut TcpStream, bearer: &str) -> Result<(), Box<dyn std::error::Error>> {
    let request = serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "id": "swallowtail-courier",
        "method": "initialize",
        "params": { "protocolVersion": "2025-11-25" },
    }))?;
    let response = post(stream, bearer, &request)?;
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
    bearer: &str,
    body: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let request = format!(
        "POST {REGISTERED_TOOL_PROXY_HTTP_PATH} HTTP/1.1\r\nHost: 127.0.0.1\r\nAuthorization: Bearer {bearer}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
        body.len()
    );
    stream.write_all(request.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    read_response(stream)
}

fn read_response(stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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
    let mut length = None;
    for line in lines {
        let (name, value) = line
            .split_once(':')
            .ok_or("courier HTTP response header malformed")?;
        if name.eq_ignore_ascii_case("content-length") {
            length = Some(value.trim().parse::<usize>()?);
        }
    }
    let length = length.ok_or("courier HTTP response length missing")?;
    if length > REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES {
        return Err("courier HTTP response exceeds bound".into());
    }
    let mut body = vec![0_u8; length];
    stream.read_exact(&mut body)?;
    if !(200..300).contains(&status) {
        let _ = serde_json::from_slice::<Value>(&body);
        return Err("courier HTTP request failed".into());
    }
    Ok(body)
}
