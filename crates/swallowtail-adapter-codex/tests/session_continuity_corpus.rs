use semver::Version;

use crate::support;

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

#[test]
fn all_six_segments_expose_bounded_load_and_range_aware_resume() {
    use futures_executor::block_on;
    use support::app_server::{AppServerMode, ScriptedAppServer};
    use support::{
        app_server_plan_for_version, app_server_session_agreement, host_services,
        session_resume_binding, working_resource,
    };
    use swallowtail_adapter_codex::CodexAppServerDriver;
    use swallowtail_core::{
        ConfiguredInstanceId, DriverRole, ExecutionHostId, InstanceTargetRef, SessionAccessPolicy,
    };
    use swallowtail_runtime::{
        CleanupOutcome, EnvironmentRef, InteractiveSessionDriver, LoadSessionRequest, RequestId,
        ResumeSessionRequest,
    };

    for version in [
        "0.80.0", "0.84.0", "0.100.0", "0.110.0", "0.129.0", "0.131.0",
    ] {
        let plan = app_server_plan_for_version(
            DriverRole::InteractiveSession,
            ExecutionHostId::new("host.local").unwrap(),
            ConfiguredInstanceId::new(format!("codex.continuity.{version}")).unwrap(),
            InstanceTargetRef::new("codex-app-server-executable").unwrap(),
            version,
            [],
            [],
        );
        let binding = session_resume_binding(&plan, "thread-provider-existing");
        let driver = CodexAppServerDriver::new(EnvironmentRef::new("codex.fixture").unwrap());
        let (process, _) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
        let loaded = block_on(driver.load_session(
            plan.clone(),
            LoadSessionRequest::new(
                RequestId::new(format!("load-{version}")).unwrap(),
                binding.clone(),
                working_resource(),
                None,
                app_server_session_agreement(SessionAccessPolicy::read_only()),
            ),
            host_services(process),
        ))
        .expect("session loads");
        assert_eq!(
            loaded
                .replay()
                .filter_map(|item| item.content().map(|content| content.as_str()))
                .collect::<Vec<_>>(),
            ["Earlier question.", "Earlier answer."]
        );
        let (_, handle) = loaded.into_parts();
        assert_eq!(handle.resume_binding(), Some(&binding));
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

        let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
        let resumed = block_on(driver.resume_session(
            plan,
            ResumeSessionRequest::new(
                RequestId::new(format!("resume-{version}")).unwrap(),
                binding,
                working_resource(),
                None,
                app_server_session_agreement(SessionAccessPolicy::read_only()),
            ),
            host_services(process),
        ))
        .expect("session resumes");
        let request = state
            .messages()
            .into_iter()
            .find(|message| message["method"] == "thread/resume")
            .expect("resume request");
        assert_eq!(
            request["params"]
                .get("excludeTurns")
                .and_then(serde_json::Value::as_bool),
            (Version::parse(version).unwrap() >= Version::new(0, 129, 0)).then_some(true)
        );
        assert_eq!(block_on(resumed.close()), CleanupOutcome::Clean);
    }
}
