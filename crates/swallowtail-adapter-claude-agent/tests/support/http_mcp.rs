#![allow(dead_code)]

use serde_json::{Value, json};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use url::Url;

const TOOL: &str = "ping";

pub fn placement_from_session_new(params: &Value) -> Option<(String, Vec<(String, String)>)> {
    let server = params.get("mcpServers")?.as_array()?.first()?;
    if server.get("type").and_then(Value::as_str) != Some("http") {
        return None;
    }
    let url = server.get("url")?.as_str()?.to_owned();
    let headers = server
        .get("headers")
        .and_then(Value::as_array)
        .map(|headers| {
            headers
                .iter()
                .filter_map(|header| {
                    Some((
                        header.get("name")?.as_str()?.to_owned(),
                        header.get("value")?.as_str()?.to_owned(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    Some((url, headers))
}

pub fn connect_and_list(url: &str, headers: &[(String, String)]) -> Result<(), String> {
    let initialize = mcp_rpc(
        url,
        headers,
        &json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "claude-agent-acp-http-mcp-live", "version": "0.0.0"}
            }
        }),
    )?;
    if initialize.pointer("/result/protocolVersion").is_none() {
        return Err("initialize missing protocolVersion".to_owned());
    }
    let _ = mcp_rpc(
        url,
        headers,
        &json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }),
    );
    let listed = mcp_rpc(
        url,
        headers,
        &json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        }),
    )?;
    let tools = listed
        .pointer("/result/tools")
        .and_then(Value::as_array)
        .ok_or_else(|| "tools/list missing tools".to_owned())?;
    if tools.iter().any(|tool| tool["name"] == TOOL) {
        Ok(())
    } else {
        Err("tools/list omitted the deterministic tool".to_owned())
    }
}

pub fn call_ping(url: &str, headers: &[(String, String)]) -> Result<String, String> {
    let reply = mcp_rpc(
        url,
        headers,
        &json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": TOOL,
                "arguments": {}
            }
        }),
    )?;
    reply
        .pointer("/result/content/0/text")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| "tools/call missing text result".to_owned())
}

fn mcp_rpc(url: &str, headers: &[(String, String)], body: &Value) -> Result<Value, String> {
    let parsed = Url::parse(url).map_err(|error| error.to_string())?;
    let host = parsed
        .host_str()
        .ok_or_else(|| "MCP URL has no host".to_owned())?;
    let port = parsed
        .port_or_known_default()
        .ok_or_else(|| "MCP URL has no port".to_owned())?;
    let path = if parsed.path().is_empty() {
        "/"
    } else {
        parsed.path()
    };
    let payload = serde_json::to_vec(body).map_err(|error| error.to_string())?;
    let mut request = format!(
        "POST {path} HTTP/1.1\r\nHost: {host}:{port}\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n",
        payload.len()
    );
    for (name, value) in headers {
        request.push_str(name);
        request.push_str(": ");
        request.push_str(value);
        request.push_str("\r\n");
    }
    request.push_str("\r\n");
    let mut stream = TcpStream::connect((host, port)).map_err(|error| error.to_string())?;
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|error| error.to_string())?;
    stream
        .write_all(request.as_bytes())
        .map_err(|error| error.to_string())?;
    stream
        .write_all(&payload)
        .map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    stream
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    parse_http_json_rpc(&bytes)
}

fn parse_http_json_rpc(bytes: &[u8]) -> Result<Value, String> {
    let header_end = bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| "MCP HTTP response missing header terminator".to_owned())?;
    let headers = std::str::from_utf8(&bytes[..header_end]).map_err(|error| error.to_string())?;
    let status = headers
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or_else(|| "MCP HTTP response missing status".to_owned())?;
    if status == "202" {
        return Ok(json!({}));
    }
    if status != "200" {
        return Err(format!("MCP HTTP status {status}"));
    }
    let body = &bytes[header_end + 4..];
    let content_type = headers
        .lines()
        .find_map(|line| {
            line.to_ascii_lowercase()
                .strip_prefix("content-type:")
                .map(str::trim)
                .map(str::to_owned)
        })
        .unwrap_or_default();
    if content_type.contains("text/event-stream") {
        let text = std::str::from_utf8(body).map_err(|error| error.to_string())?;
        let data = text
            .lines()
            .filter_map(|line| line.strip_prefix("data:"))
            .map(str::trim)
            .find(|line| !line.is_empty())
            .ok_or_else(|| "MCP SSE response had no data".to_owned())?;
        serde_json::from_str(data).map_err(|error| error.to_string())
    } else if body.is_empty() {
        Ok(json!({}))
    } else {
        serde_json::from_slice(body).map_err(|error| error.to_string())
    }
}
