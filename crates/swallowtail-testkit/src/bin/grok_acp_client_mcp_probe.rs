//! Desktop/fixture runner for the Grok ACP client-MCP probe.

#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use swallowtail_testkit::{
    ClientMcpVerdict, grok_acp_client_mcp_fixture_probe, isolated_grok_home,
    open_desktop_live_grok_acp_peer, run_grok_acp_client_mcp_probe_with_transcript,
};

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let parsed = parse_args(&args)?;
    let capsule = match parsed.mode {
        Mode::Fixture(verdict) => grok_acp_client_mcp_fixture_probe(&parsed.version, verdict)
            .map_err(|error| error.to_string())?,
        Mode::Live {
            grok_executable,
            echo_mcp,
        } => {
            let grok_home = isolated_grok_home().map_err(|error| error.to_string())?;
            let cwd = grok_home.display().to_string();
            let transcript_path = grok_home.join("echo-mcp-transcript.ndjson");
            let echo_command = echo_mcp.display().to_string();
            let mut peer = open_desktop_live_grok_acp_peer(&grok_executable, &echo_mcp)
                .map_err(|error| error.to_string())?;
            run_grok_acp_client_mcp_probe_with_transcript(
                &mut peer,
                &parsed.version,
                &echo_command,
                &cwd,
                Some(&transcript_path),
            )
            .map_err(|error| error.to_string())?
        }
    };
    let body = serde_json::to_string_pretty(&capsule.to_json()).expect("capsule serializes");
    if let Some(output_path) = parsed.output {
        fs::write(&output_path, body.as_bytes()).map_err(|error| error.to_string())?;
    } else {
        println!("{body}");
    }
    Ok(())
}

struct Parsed {
    mode: Mode,
    version: String,
    output: Option<PathBuf>,
}

enum Mode {
    Fixture(ClientMcpVerdict),
    Live {
        grok_executable: PathBuf,
        echo_mcp: PathBuf,
    },
}

fn parse_args(args: &[String]) -> Result<Parsed, String> {
    let mut version = None;
    let mut output = None;
    let mut fixture = None;
    let mut live = false;
    let mut grok_executable = None;
    let mut echo_mcp = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--version" => {
                version = Some(require_value(args, &mut index, "--version")?);
            }
            "--output" => {
                output = Some(PathBuf::from(require_value(args, &mut index, "--output")?));
            }
            "--fixture" => {
                let spelling = require_value(args, &mut index, "--fixture")?;
                fixture = Some(
                    ClientMcpVerdict::from_capsule(&spelling).map_err(|error| error.to_string())?,
                );
            }
            "--live" => {
                live = true;
                index += 1;
            }
            "--grok-executable" => {
                grok_executable = Some(PathBuf::from(require_value(
                    args,
                    &mut index,
                    "--grok-executable",
                )?));
            }
            "--echo-mcp" => {
                echo_mcp = Some(PathBuf::from(require_value(
                    args,
                    &mut index,
                    "--echo-mcp",
                )?));
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    let version = version.ok_or("missing --version")?;
    let mode = if live {
        Mode::Live {
            grok_executable: grok_executable.ok_or("missing --grok-executable")?,
            echo_mcp: echo_mcp.ok_or("missing --echo-mcp")?,
        }
    } else {
        Mode::Fixture(fixture.ok_or("missing --fixture or --live")?)
    };
    Ok(Parsed {
        mode,
        version,
        output,
    })
}

fn require_value(args: &[String], index: &mut usize, flag: &str) -> Result<String, String> {
    let value = args
        .get(*index + 1)
        .cloned()
        .ok_or_else(|| format!("missing value after {flag}"))?;
    *index += 2;
    Ok(value)
}
