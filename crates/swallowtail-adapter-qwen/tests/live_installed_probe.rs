use serde_json::{Value, json};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_adapter_qwen::{qwen_code_binding, qwen_headless_claim};

const MAXIMUM_OUTPUT_BYTES: usize = 2 * 1024 * 1024;
const MAXIMUM_RUNTIME: Duration = Duration::from_secs(45);

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_QWEN=1 and an installed Qwen Code CLI"]
fn installed_qwen_code_is_exactly_classified() {
    gate("SWALLOWTAIL_LIVE_QWEN");
    let output = run_bounded(["--version"], &[]);
    assert!(output.success, "Qwen version probe did not succeed");
    let version = std::str::from_utf8(&output.stdout)
        .expect("Qwen version output is UTF-8")
        .trim();
    let binding = qwen_code_binding(version).expect("Qwen emits one semantic version");
    assert!(
        qwen_headless_claim().supports(binding.version()),
        "installed Qwen is not inside the qualified range"
    );
}

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_QWEN_CATALOGUE=1 and configured Qwen harness access"]
fn configured_qwen_code_returns_a_bounded_model_catalogue() {
    gate("SWALLOWTAIL_LIVE_QWEN_CATALOGUE");
    let input = [
        json!({
            "type": "control_request",
            "request_id": "swallowtail-live-initialize",
            "request": {"subtype": "initialize"}
        }),
        json!({
            "type": "control_request",
            "request_id": "swallowtail-live-models",
            "request": {"subtype": "get_available_models"}
        }),
    ]
    .into_iter()
    .map(|value| serde_json::to_string(&value).expect("request serializes"))
    .collect::<Vec<_>>()
    .join("\n")
        + "\n";
    let output = run_bounded(
        [
            "--input-format",
            "stream-json",
            "--output-format",
            "stream-json",
            "--safe-mode",
            "--approval-mode",
            "default",
        ],
        input.as_bytes(),
    );
    assert!(output.success, "Qwen catalogue probe did not succeed");
    let responses = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice::<Value>(line).expect("catalogue line is JSON"))
        .collect::<Vec<_>>();
    let initialize = response(&responses, "swallowtail-live-initialize");
    assert_eq!(
        initialize
            .pointer("/capabilities/can_get_available_models")
            .and_then(Value::as_bool),
        Some(true)
    );
    let models = response(&responses, "swallowtail-live-models")["models"]
        .as_array()
        .expect("catalogue models are an array");
    assert!(!models.is_empty(), "Qwen returned no configured models");
    assert!(models.len() <= 512, "Qwen returned too many models");
    assert!(models.iter().all(|model| {
        model
            .get("id")
            .and_then(Value::as_str)
            .is_some_and(|id| !id.is_empty() && id.len() <= 512)
    }));
}

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_QWEN_PROMPT=1, SWALLOWTAIL_LIVE_QWEN_MODEL, and configured Qwen harness access"]
fn configured_qwen_code_completes_one_bounded_read_only_prompt() {
    gate("SWALLOWTAIL_LIVE_QWEN_PROMPT");
    let model = std::env::var("SWALLOWTAIL_LIVE_QWEN_MODEL")
        .expect("live Qwen prompt requires one explicit model id");
    assert!(
        !model.is_empty() && model.len() <= 512 && !model.chars().any(char::is_control),
        "live Qwen model id is invalid"
    );
    let output = run_bounded(
        [
            "--input-format",
            "text",
            "--output-format",
            "stream-json",
            "--include-partial-messages",
            "--safe-mode",
            "--approval-mode",
            "default",
            "--model",
            &model,
            "--core-tools",
            "read_file,grep_search,glob,list_directory,lsp",
            "--exclude-tools",
            "run_shell_command,monitor,edit,write_file,notebook_edit,agent,web_fetch,save_memory,skill,workflow,artifact,record_artifact,cron_create,cron_delete,create_sub_session,task_create,task_update,task_stop,team_create,team_delete,send_message,enter_worktree,exit_worktree",
            "--max-wall-time",
            "30s",
            "--max-tool-calls",
            "1",
            "--max-session-turns",
            "4",
        ],
        b"Reply exactly QWEN_LIVE_OK without using tools.",
    );
    assert!(output.success, "Qwen read-only prompt did not succeed");
    let records = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice::<Value>(line).expect("prompt line is JSON"))
        .collect::<Vec<_>>();
    assert!(records.iter().any(|record| {
        record.get("type").and_then(Value::as_str) == Some("system")
            && record.get("subtype").and_then(Value::as_str) == Some("session_start")
            && record.get("model").and_then(Value::as_str) == Some(model.as_str())
            && record
                .get("qwen_code_version")
                .and_then(Value::as_str)
                .is_some_and(|version| qwen_code_binding(version).is_some())
    }));
    assert!(records.iter().any(|record| {
        record.get("type").and_then(Value::as_str) == Some("result")
            && record.get("subtype").and_then(Value::as_str) == Some("success")
            && record.get("is_error").and_then(Value::as_bool) == Some(false)
    }));
}

fn response<'a>(records: &'a [Value], request_id: &str) -> &'a Value {
    records
        .iter()
        .find_map(|record| {
            let response = record.get("response")?;
            (response.get("request_id").and_then(Value::as_str) == Some(request_id))
                .then(|| response.get("response"))
                .flatten()
        })
        .expect("Qwen returned the requested control response")
}

fn gate(name: &str) {
    assert_eq!(
        std::env::var(name).as_deref(),
        Ok("1"),
        "live Qwen probe requires its explicit gate"
    );
}

struct BoundedOutput {
    success: bool,
    stdout: Vec<u8>,
}

fn run_bounded<const N: usize>(arguments: [&str; N], input: &[u8]) -> BoundedOutput {
    let mut child = Command::new("qwen")
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Qwen Code is installed");
    child
        .stdin
        .take()
        .expect("Qwen stdin is available")
        .write_all(input)
        .expect("Qwen stdin accepts bounded input");
    let stdout = bounded_reader(child.stdout.take().expect("Qwen stdout is available"));
    let stderr = bounded_reader(child.stderr.take().expect("Qwen stderr is available"));
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().expect("Qwen process remains observable") {
            break status;
        }
        if started.elapsed() >= MAXIMUM_RUNTIME {
            let _ = child.kill();
            let _ = child.wait();
            panic!("Qwen live probe exceeded its host deadline");
        }
        thread::sleep(Duration::from_millis(25));
    };
    let stdout = stdout.join().expect("Qwen stdout reader joins");
    let stderr = stderr.join().expect("Qwen stderr reader joins");
    assert!(
        stdout.len() <= MAXIMUM_OUTPUT_BYTES && stderr.len() <= MAXIMUM_OUTPUT_BYTES,
        "Qwen live output exceeded its bound"
    );
    BoundedOutput {
        success: status.success(),
        stdout,
    }
}

fn bounded_reader(reader: impl Read + Send + 'static) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut bytes = Vec::new();
        reader
            .take((MAXIMUM_OUTPUT_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .expect("Qwen output remains readable");
        bytes
    })
}
