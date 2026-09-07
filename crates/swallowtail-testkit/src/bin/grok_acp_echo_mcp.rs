//! Disposable non-mutating stdio MCP server with one echo tool.

#![forbid(unsafe_code)]

use std::env;
use std::io::{self, BufRead, Write};
use swallowtail_testkit::{
    ECHO_MCP_TRANSCRIPT_ENV, append_echo_mcp_transcript, grok_acp_echo_mcp_reply,
    grok_acp_echo_mcp_stdio_frame,
};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    loop {
        let Some(request) = read_mcp_value(&mut stdin)? else {
            return Ok(());
        };
        if let Some(method) = request.get("method").and_then(serde_json::Value::as_str) {
            record_method(method);
        }
        if let Some(reply) = grok_acp_echo_mcp_reply(&request) {
            write_mcp_value(&mut stdout, &reply)?;
        }
    }
}

fn read_mcp_value(reader: &mut impl BufRead) -> io::Result<Option<serde_json::Value>> {
    let mut header = String::new();
    if reader.read_line(&mut header)? == 0 {
        return Ok(None);
    }
    if let Some(length) = header
        .to_ascii_lowercase()
        .strip_prefix("content-length:")
        .and_then(|value| value.trim().parse::<usize>().ok())
    {
        let mut rest = String::new();
        loop {
            rest.clear();
            if reader.read_line(&mut rest)? == 0 {
                return Ok(None);
            }
            if rest == "\r\n" || rest == "\n" {
                break;
            }
        }
        let mut body = vec![0; length];
        reader.read_exact(&mut body)?;
        return serde_json::from_slice(&body)
            .map(Some)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error));
    }
    let line = header.trim();
    if line.is_empty() {
        return Ok(None);
    }
    serde_json::from_str(line)
        .map(Some)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn write_mcp_value(writer: &mut impl Write, value: &serde_json::Value) -> io::Result<()> {
    let body = grok_acp_echo_mcp_stdio_frame(value)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    writer.write_all(&body)?;
    writer.flush()
}

fn record_method(method: &str) {
    let Ok(path) = env::var(ECHO_MCP_TRANSCRIPT_ENV) else {
        return;
    };
    let _ = append_echo_mcp_transcript(std::path::Path::new(&path), method);
}
