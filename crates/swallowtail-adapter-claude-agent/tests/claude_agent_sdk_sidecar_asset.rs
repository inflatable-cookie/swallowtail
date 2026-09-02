//! Sidecar-level falsification of the shipped Claude Agent SDK asset.
//!
//! This runs the real `claude-agent-sdk-sidecar.mjs` under Node against a fake
//! SDK module and a fake native child. It is provider-free by construction:
//! nothing is installed, the official package is never present, no credential
//! exists, and no provider session is opened. What it proves is exactly the
//! part a Rust-side fake cannot: how the asset drives the SDK's own option
//! surface and `canUseTool` contract.

mod sidecar_asset_support;

use serde_json::json;
use sidecar_asset_support::SidecarProcess;

#[test]
fn every_allowed_invocation_crosses_the_callback_with_its_input_intact() {
    let mut sidecar = SidecarProcess::start();
    let open = sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    assert_eq!(open["success"], true, "open response: {open}");
    assert_eq!(open["data"]["model"], "m-1");

    sidecar.command("query-1", "query", json!({"text": "read it"}));

    // The allowed tool reaches the host as a bounded callback carrying the
    // tool name and nothing else: no input, no path, no provider payload.
    let request = sidecar.next_callback();
    assert_eq!(request["callback"], "can_use_tool");
    assert_eq!(request["toolName"], "Read");
    assert_eq!(
        request.as_object().expect("callback is an object").len(),
        4,
        "callback carries only type, id, callback, and toolName: {request}"
    );
    assert!(request.get("input").is_none());

    sidecar.respond_callback(request["id"].as_str().expect("callback id"), "allow");
    let observations = sidecar.admissions(&["Read"]);

    // An allowed decision returns the provider's own input unchanged. An empty
    // `updatedInput` would silently destroy the path the tool needs.
    let allowed = &observations["Read"];
    assert_eq!(allowed["behavior"], "allow");
    assert_eq!(
        allowed["updatedInput"],
        json!({"file_path": "/fixture/read-me.txt"}),
        "allowed input must survive the round trip"
    );
}

#[test]
fn a_tool_outside_the_read_only_set_is_denied_without_asking_the_host() {
    let mut sidecar = SidecarProcess::start();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    sidecar.command("query-1", "query", json!({"text": "run it"}));

    let request = sidecar.next_callback();
    assert_eq!(
        request["toolName"], "Read",
        "only allowed tools are offered"
    );
    sidecar.respond_callback(request["id"].as_str().expect("callback id"), "allow");

    let observations = sidecar.admissions(&["Read", "Bash"]);
    let denied = &observations["Bash"];
    assert_eq!(denied["behavior"], "deny");
    assert!(
        denied.get("updatedInput").is_none(),
        "a denial never returns tool input"
    );
    // The unadmitted tool produced no consumer round trip at all.
    assert_eq!(
        sidecar.callback_tool_names(),
        vec!["Read".to_owned()],
        "an unadmitted tool must never reach the host"
    );
}

#[test]
fn the_asset_restricts_availability_without_auto_allowing_anything() {
    let mut sidecar = SidecarProcess::start();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );
    let options = sidecar.observed_options();

    // `tools` restricts what exists; `allowedTools` would auto-allow without
    // prompting and must never be set by this route.
    assert_eq!(options["tools"], json!(["Read", "Glob", "Grep"]));
    assert!(
        options.get("allowedTools").is_none(),
        "allowedTools bypasses per-use admission: {options}"
    );
    assert_eq!(options["model"], "m-1");
    assert_eq!(options["settingSources"], json!([]));
    assert_eq!(options["skills"], json!([]));
    assert_eq!(options["persistSession"], json!(false));
    assert_eq!(options["env"], json!({}));
    for forbidden in ["apiKeyHelper", "awsAuthRefresh", "gcpAuthRefresh"] {
        assert!(
            options.get(forbidden).is_none(),
            "{forbidden} must be unset"
        );
    }
    assert_eq!(options["executable"], "node");
}

#[test]
fn close_joins_the_retained_native_child_before_reporting() {
    let mut sidecar = SidecarProcess::start();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );

    // The fake native child exits on its own well inside the declared bound.
    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 2000}));
    assert_eq!(close["success"], true, "close response: {close}");
    assert_eq!(close["data"]["closeState"], "graceful");
    assert_eq!(close["data"]["nativeExitObserved"], true);
    assert_eq!(close["data"]["joinBoundMs"], 2000);
}

#[test]
fn an_unjoinable_native_child_is_reported_unconfirmed_not_graceful() {
    let mut sidecar = SidecarProcess::start_with_surviving_native_child();
    sidecar.command(
        "open-1",
        "open",
        json!({"cwd": sidecar.cwd(), "model": "m-1"}),
    );

    // The child outlives the declared bound, so the sidecar reports what it
    // actually observed: nothing.
    let close = sidecar.command("close-1", "close", json!({"joinBoundMs": 300}));
    assert_eq!(close["data"]["closeState"], "unconfirmed");
    assert_eq!(close["data"]["nativeExitObserved"], false);
}
