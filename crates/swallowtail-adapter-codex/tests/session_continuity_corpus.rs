use semver::Version;

const CORPUS: &str = include_str!("fixtures/compatibility/app-server-session-continuity.json");

fn version(value: &serde_json::Value, field: &str) -> Version {
    Version::parse(value[field].as_str().expect("version field is text"))
        .expect("frozen version is semantic")
}

#[test]
fn continuity_segments_freeze_the_exact_maintained_window() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).expect("corpus parses");
    assert_eq!(corpus["qualified_range"]["baseline"], "0.80.0");
    assert_eq!(corpus["qualified_range"]["latest"], "0.145.0");
    assert_eq!(corpus["selected_method"], "thread/resume");

    let segments = corpus["segments"].as_array().expect("segments");
    assert_eq!(segments.len(), 6);
    for checkpoint in [
        "0.80.0", "0.81.0", "0.84.0", "0.99.0", "0.100.0", "0.107.0", "0.110.0", "0.128.0",
        "0.129.0", "0.130.0", "0.131.0", "0.145.0",
    ] {
        let checkpoint = Version::parse(checkpoint).expect("checkpoint");
        assert_eq!(
            segments
                .iter()
                .filter(|segment| {
                    version(segment, "minimum") <= checkpoint
                        && checkpoint <= version(segment, "maximum")
                })
                .count(),
            1,
            "{checkpoint} maps exactly once"
        );
    }
    for excluded in ["0.82.0", "0.83.0", "0.108.0", "0.109.0"] {
        let excluded = Version::parse(excluded).expect("excluded version");
        assert!(!segments.iter().any(|segment| {
            version(segment, "minimum") <= excluded && excluded <= version(segment, "maximum")
        }));
    }
}

#[test]
fn load_and_resume_keep_replay_semantics_separate() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).expect("corpus parses");
    let operations = &corpus["operations"];
    assert_eq!(
        operations["load"]["response_history"],
        "result.thread.turns"
    );
    assert_eq!(
        operations["load"]["replay_order"],
        "array_order_before_ready"
    );
    assert_eq!(operations["load"]["exclude_turns"], false);
    assert_eq!(operations["resume"]["through_0_128"]["replay_phase"], false);
    assert_eq!(
        operations["resume"]["through_0_128"]["response_turns"],
        "bounded_then_ignored"
    );
    assert_eq!(
        operations["resume"]["from_0_129"]["request"]["params"]["excludeTurns"],
        true
    );
    assert_eq!(
        operations["resume"]["from_0_129"]["experimental_api_required"],
        true
    );

    let turns = corpus["representative_load_response"]["result"]["thread"]["turns"]
        .as_array()
        .expect("ordered turns");
    assert_eq!(turns[0]["id"], "turn-1");
    assert_eq!(turns[1]["id"], "turn-2");
}

#[test]
fn continuity_failures_are_bounded_joined_and_safe() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).expect("corpus parses");
    assert!(
        corpus["limits"]["maximum_response_bytes"]
            .as_u64()
            .is_some_and(|n| n > 0)
    );
    assert!(
        corpus["limits"]["maximum_turns"]
            .as_u64()
            .is_some_and(|n| n > 0)
    );
    assert_eq!(
        corpus["failures"]["wrong_thread"],
        "fail_without_usable_handle"
    );
    assert_eq!(
        corpus["failures"]["malformed_history"],
        "fail_without_usable_handle"
    );
    assert_eq!(
        corpus["failures"]["disconnect"],
        "fail_without_usable_handle"
    );
    assert_eq!(corpus["cleanup"]["detached_tasks_allowed"], false);

    for forbidden in [
        "/Users/",
        "OPENAI_API_KEY",
        "Bearer ",
        "sk-",
        "thread-provider-private",
    ] {
        assert!(!CORPUS.contains(forbidden), "fixture leaked {forbidden}");
    }
}
