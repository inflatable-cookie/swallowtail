use super::{
    EFFECTIVE_DRIFT, EFFORT_REASONING, EFFORT_REASONING_SUCCESS, LEGACY_REASONING,
    LEGACY_REASONING_SUCCESS, MISSING_CONFIRMATION, PROVIDER_REJECTION,
};
use crate::support::{Direction, methods, parse_json, parse_transcript};
use serde_json::{Value, json};
use swallowtail_protocol_acp::ACP_PROTOCOL_VERSION;

#[test]
fn exact_release_sources_adapter_sdk_and_wire_are_frozen() {
    let legacy = parse_json(LEGACY_REASONING);
    let effort = parse_json(EFFORT_REASONING);

    assert_release(
        &legacy,
        "0.28.1",
        "0032545b65f95c139ecba5a48ba1b911844e1ffe",
        "efacf0452d46f5dbd67499eabc053869495d5213",
        "0.3.4",
        "kimi.acp.reasoning.legacy-select-v1",
    );
    assert_release(
        &effort,
        "0.29.0",
        "03c34eefa49513e6216390a9773326077a37f414",
        "8bf5bacba9e524c38fb808c0122070037ead25a8",
        "0.3.5",
        "kimi.acp.reasoning.declared-effort-v2",
    );
    for fixture in [&legacy, &effort] {
        assert_eq!(fixture["release"]["locked_acp_sdk"], "0.23.0");
        assert_eq!(fixture["release"]["locked_zod"], "4.3.6");
        assert_eq!(fixture["release"]["wire_version"], ACP_PROTOCOL_VERSION);
        for digest in fixture["source"]
            .as_object()
            .expect("source digest object")
            .values()
        {
            assert_eq!(
                digest.as_str().expect("digest is text").len(),
                64,
                "source evidence uses exact SHA-256"
            );
        }
    }
}

#[test]
fn compatibility_corpus_uses_two_singletons_and_no_inferred_gap() {
    let fixture = parse_json(EFFORT_REASONING);
    let compatibility = &fixture["compatibility"];
    let segments = compatibility["segments"]
        .as_array()
        .expect("compatibility segments");
    assert_eq!(segments.len(), 2);
    assert!(segments.iter().all(|segment| {
        segment["minimum"] == segment["maximum"]
            && segment["behavior_revision"]
                .as_str()
                .is_some_and(|revision| !revision.is_empty())
    }));
    assert_eq!(compatibility["newer_version_posture"], "allow_unverified");

    let cases = compatibility["cases"]
        .as_array()
        .expect("compatibility cases");
    for (version, assessment) in [
        ("0.28.1", "qualified"),
        ("0.29.0", "qualified"),
        ("0.28.0", "incompatible"),
        ("0.28.2", "incompatible"),
        ("0.29.0-rc.1", "incompatible"),
        ("not-a-version", "incompatible"),
        ("0.30.0", "unverified_newer"),
    ] {
        assert!(
            cases
                .iter()
                .any(|case| { case["version"] == version && case["assessment"] == assessment })
        );
    }
    let newer = cases
        .iter()
        .find(|case| case["version"] == "0.30.0")
        .expect("newer case exists");
    assert_eq!(
        newer["behavior_revision"],
        "kimi.acp.reasoning.declared-effort-v2"
    );
}

#[test]
fn private_option_shapes_cover_legacy_effort_boolean_and_always_thinking() {
    let legacy = parse_json(LEGACY_REASONING);
    let effort = parse_json(EFFORT_REASONING);

    assert_eq!(values(&legacy["option_shapes"]["legacy"]), ["off", "on"]);
    assert_eq!(values(&legacy["option_shapes"]["always_thinking"]), ["on"]);
    assert_eq!(
        values(&effort["option_shapes"]["declared_efforts"]),
        ["off", "low", "medium", "high"]
    );
    assert_eq!(
        values(&effort["option_shapes"]["boolean_fallback"]),
        ["off", "on"]
    );
    assert_eq!(
        values(&effort["option_shapes"]["always_thinking"]),
        ["low", "medium", "high"]
    );
    assert!(
        effort["option_shapes"]["missing"]
            .as_array()
            .expect("missing shape")
            .is_empty()
    );
    assert_eq!(
        effort["option_shapes"]["ambiguous"]
            .as_array()
            .expect("ambiguous shape")
            .len(),
        2
    );
}

#[test]
fn both_releases_use_one_correlated_selection_and_exact_confirmation() {
    for (transcript, requested) in [
        (LEGACY_REASONING_SUCCESS, "on"),
        (EFFORT_REASONING_SUCCESS, "high"),
    ] {
        let frames = parse_transcript(transcript).expect("reasoning transcript parses");
        assert_eq!(
            methods(&frames),
            ["session/new", "session/set_config_option", "session/update"]
        );
        let request = frames
            .iter()
            .find(|frame| frame.method() == Some("session/set_config_option"))
            .expect("selection request exists");
        assert_eq!(request.direction(), Direction::ClientToAgent);
        assert_eq!(request.message()["params"]["configId"], "thinking");
        assert_eq!(request.message()["params"]["value"], requested);

        let response = frames
            .iter()
            .find(|frame| frame.method().is_none() && frame.id() == request.id())
            .expect("correlated selection response exists");
        assert_eq!(
            thinking_option(&response.message()["result"]["configOptions"])["currentValue"],
            requested
        );
        let update = frames
            .iter()
            .find(|frame| frame.method() == Some("session/update"))
            .expect("effective update exists");
        assert_eq!(
            update.message()["params"]["update"]["sessionUpdate"],
            "config_option_update"
        );
        assert_eq!(
            thinking_option(&update.message()["params"]["update"]["configOptions"])["currentValue"],
            requested
        );
    }
}

#[test]
fn rejection_missing_confirmation_and_effective_drift_have_no_fallback() {
    let rejection =
        parse_transcript(PROVIDER_REJECTION).expect("provider rejection transcript parses");
    assert_eq!(methods(&rejection), ["session/set_config_option"]);
    assert_eq!(rejection[1].id(), rejection[0].id());
    assert_eq!(rejection[1].message()["error"]["code"], -32602);

    let missing =
        parse_transcript(MISSING_CONFIRMATION).expect("missing confirmation transcript parses");
    assert_eq!(methods(&missing), ["session/set_config_option"]);
    assert!(find_thinking_option(&missing[1].message()["result"]["configOptions"]).is_none());

    let drift = parse_transcript(EFFECTIVE_DRIFT).expect("effective drift transcript parses");
    assert_eq!(methods(&drift), ["session/set_config_option"]);
    assert_eq!(drift[0].message()["params"]["value"], "high");
    assert_eq!(
        thinking_option(&drift[1].message()["result"]["configOptions"])["currentValue"],
        "medium"
    );

    for fixture in [parse_json(LEGACY_REASONING), parse_json(EFFORT_REASONING)] {
        assert_eq!(fixture["rejections"]["fallback_allowed"], false);
        assert_eq!(
            fixture["rejections"]["unsupported_value"],
            "reject_before_provider_request"
        );
        assert_eq!(
            fixture["portable_mapping"]["load_or_resume"],
            "reject_before_process_or_wire"
        );
    }
    for transcript in [PROVIDER_REJECTION, MISSING_CONFIRMATION, EFFECTIVE_DRIFT] {
        assert_eq!(transcript.matches("session/set_config_option").count(), 1);
        assert!(!transcript.contains("token"));
        assert!(!transcript.contains("api-key"));
    }
}

fn assert_release(
    fixture: &Value,
    version: &str,
    tag_object: &str,
    source_commit: &str,
    adapter: &str,
    behavior: &str,
) {
    assert_eq!(fixture["release"]["repository"], "MoonshotAI/kimi-code");
    assert_eq!(fixture["release"]["version"], version);
    assert_eq!(fixture["release"]["annotated_tag_object"], tag_object);
    assert_eq!(fixture["release"]["source_commit"], source_commit);
    assert_eq!(fixture["release"]["acp_adapter_package"], adapter);
    assert_eq!(fixture["behavior_revision"], behavior);
}

fn values(option: &Value) -> Vec<&str> {
    option["options"]
        .as_array()
        .expect("select options")
        .iter()
        .map(|row| row["value"].as_str().expect("option value"))
        .collect()
}

fn thinking_option(options: &Value) -> &Value {
    find_thinking_option(options).expect("one thinking option exists")
}

fn find_thinking_option(options: &Value) -> Option<&Value> {
    options
        .as_array()?
        .iter()
        .find(|option| option["id"] == json!("thinking"))
}
