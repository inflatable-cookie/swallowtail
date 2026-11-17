//! Private original qualification consumer of the actual prepared/local-host route.
use futures_executor::block_on;
use futures_util::{FutureExt, StreamExt, select_biased};
use serde_json::{Value, json};
use std::{ffi::OsString, fs, path::PathBuf, time::Duration};
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkPackageNativePair, ClaudeAgentSdkPermissionMode,
    ClaudeAgentSdkSessionPreparation, ClaudeAgentSdkSessionProfile, ClaudeAgentSdkTool,
    prepare_claude_agent_sdk_session,
};
use swallowtail_core::{
    AccessProfileId, ConfiguredInstanceId, CredentialRef, EndpointAudience, ExecutionHostId,
    InstanceRevision, InstanceTargetRef, ModelId, ModelRouteId, ModelRouteRevision,
};
use swallowtail_host_local::{LocalExecutableLaunch, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    CallbackPayload, CallbackResponse, CallbackResult, CleanupOutcome, EnvironmentRef,
    ExecutableRef, OperationContent, RequestId, RuntimeEventKind, RuntimeTurnId,
    SessionCleanupRequest, SessionOptions, TerminalStatus, TurnRequest, WorkingResourceRef,
};

fn main() {
    let file = PathBuf::from(std::env::args_os().nth(1).expect("owned case config"));
    let config: Value =
        serde_json::from_slice(&fs::read(&file).expect("config bytes")).expect("JSON");
    assert!(
        std::env::var("GITHUB_ACTIONS").as_deref() == Ok("true") || config["ownedFake"] == true,
        "originals require a disposable runner"
    );
    let result = block_on(run(&config, &file));
    println!("{result}");
}

async fn run(config: &Value, config_file: &std::path::Path) -> Value {
    let text = |key: &str| config[key].as_str().expect("case string");
    let host = ExecutionHostId::new("sdk240.trusted.local").expect("host");
    let target = InstanceTargetRef::new("sdk240.trusted.sidecar").expect("target");
    let environment = EnvironmentRef::new("sdk240.trusted.environment").expect("environment");
    let resource = WorkingResourceRef::new("sdk240.trusted.workspace").expect("resource");
    let credential = CredentialRef::new("sdk240.trusted.delegated").expect("credential");
    let mut env: Vec<(OsString, OsString)> = config["environment"]
        .as_object()
        .expect("env")
        .iter()
        .map(|(k, v)| (k.into(), v.as_str().expect("env string").into()))
        .collect();
    env.extend([
        (
            "CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE".into(),
            text("sdk").into(),
        ),
        (
            "CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY".into(),
            text("native").into(),
        ),
        (
            "CLAUDE_AGENT_SDK_SIDECAR_MANIFEST".into(),
            text("manifest").into(),
        ),
    ]);
    let local = LocalProcessHost::builder(LocalProcessLimits::new(
        128, 262144, 1048576, 1048576, 1048576,
    ))
    .approve_executable_launch(
        ExecutableRef::from_instance_target(&target),
        LocalExecutableLaunch::new(text("node")).with_prefix_arguments([
            OsString::from(text("observer")),
            config_file.as_os_str().to_owned(),
            "--prepared".into(),
        ]),
    )
    .approve_environment(environment.clone(), env)
    .approve_working_resource(resource.clone(), text("workspace"))
    .approve_delegated_credential(
        credential.clone(),
        EndpointAudience::new("claude-agent-sdk").expect("audience"),
    )
    .build_services(host.clone());
    let services = local.services().clone();
    let pair = ClaudeAgentSdkPackageNativePair::MAINTAINED
        .iter()
        .copied()
        .find(|p| p.package_version() == text("sdkVersion"))
        .expect("exact maintained pair");
    let prepared = prepare_claude_agent_sdk_session(
        ClaudeAgentSdkSessionPreparation::new(
            ConfiguredInstanceId::new("sdk240.trusted").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host,
            target,
            environment,
            credential,
            AccessProfileId::new("sdk240.trusted").expect("access"),
            ModelRouteId::new("sdk240.trusted").expect("route"),
            ModelRouteRevision::new("1").expect("route revision"),
            ModelId::new("claude-sonnet-5-20250929").expect("model"),
            resource,
            RequestId::new("open").expect("request"),
            local.deadline_after(Duration::from_secs(25)),
        )
        .with_package_native_pair(pair)
        .with_session_profile(
            ClaudeAgentSdkSessionProfile::new(
                [ClaudeAgentSdkTool::Write],
                ClaudeAgentSdkPermissionMode::Default,
            )
            .expect("profile"),
        ),
        SessionOptions::default(),
    )
    .expect("prepare");
    let mut session = prepared
        .open_session(services.clone())
        .await
        .expect("actual prepared open");
    let case = text("case");
    let deadline = local.deadline_after(Duration::from_millis(if case == "deadline" {
        1000
    } else {
        35000
    }));
    let turn = session
        .start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("turn").expect("turn"),
                OperationContent::new("Use the owned fixture tool then report continued.")
                    .expect("prompt"),
            )
            .with_deadline(deadline),
            services.clone(),
        )
        .await
        .expect("actual prepared turn");
    let mut result = observe_turn(
        turn,
        config,
        case,
        deadline,
        local.deadline_after(Duration::from_secs(1)),
        &services,
    )
    .await;
    if config["lifecycle"] == true {
        fs::write(text("nextTurnMarker"), b"owned next turn").expect("owned marker");
        let reuse_deadline = local.deadline_after(Duration::from_secs(5));
        let mut refusals = Vec::new();
        loop {
            let next_deadline = local.deadline_after(Duration::from_secs(20));
            match session
                .start_turn(
                    TurnRequest::new(
                        RuntimeTurnId::new("next").expect("next id"),
                        OperationContent::new("Report continued for the owned next turn.")
                            .expect("next prompt"),
                    )
                    .with_deadline(next_deadline),
                    services.clone(),
                )
                .await
            {
                Ok(next) => {
                    result["nextTurn"] = observe_turn(
                        next,
                        config,
                        "next",
                        next_deadline,
                        local.deadline_after(Duration::from_secs(1)),
                        &services,
                    )
                    .await;
                    break;
                }
                Err(error) => {
                    assert_eq!(
                        error.diagnostic().code(),
                        "swallowtail.claude-agent.sdk.turn_active",
                        "only typed busy can be retried"
                    );
                    refusals.push(error.diagnostic().code().to_owned());
                    if config["expectReuseRefusal"] == true {
                        result["reuseBlocked"] = json!(true);
                        break;
                    }
                    assert!(
                        services.time().expect("clock").now() < reuse_deadline.instant(),
                        "terminal drain did not settle"
                    );
                    services
                        .time()
                        .expect("clock")
                        .wait_until(local.deadline_after(Duration::from_millis(20)))
                        .await;
                }
            }
        }
        result["reuseRefusals"] = json!(refusals);
    }
    let session_cleanup = session
        .close(
            SessionCleanupRequest::new(local.deadline_after(Duration::from_secs(5))),
            services,
        )
        .await;
    local.shutdown_task_reapers().expect("actual task joins");
    // Preserve the product's existing root-only degraded posture. This is a
    // positive host root wait, not attested descendant-tree emptiness.
    let session_class = match &session_cleanup {
        CleanupOutcome::Clean => "Clean",
        CleanupOutcome::Degraded(d)
            if d.code() == "swallowtail.claude-agent.sdk.close_root_only_degraded" =>
        {
            "RootCompleted"
        }
        CleanupOutcome::Degraded(_) => "Degraded",
        CleanupOutcome::Failed(_) => "Failed",
        CleanupOutcome::NotApplicable => "NotApplicable",
    };
    result["sessionCleanup"] = json!(session_class);
    result["sessionCleanupDiagnostic"] = json!(session_cleanup.diagnostic().map(|d| d.code()));
    result["sessionCleanupDetail"] = json!(format!("{session_cleanup:?}"));
    result["driverWaitObserved"] = json!(matches!(session_class, "Clean" | "RootCompleted"));
    result
}

async fn observe_turn(
    mut turn: Box<dyn swallowtail_runtime::TurnHandle>,
    config: &Value,
    case: &str,
    deadline: swallowtail_runtime::Deadline,
    cancel_at: swallowtail_runtime::Deadline,
    services: &swallowtail_runtime::HostServices,
) -> Value {
    let text = |key: &str| config[key].as_str().expect("case string");
    let mut events = turn.take_events().expect("events").fuse();
    let mut exchange = turn.take_callbacks().expect("callbacks");
    let responder = exchange.responder();
    let mut requests = exchange.take_requests().expect("request stream").fuse();
    let mut terminal = turn.take_terminal_outcome().expect("terminal").fuse();
    let clock = services.time().expect("host time");
    let mut cancel_timer = clock.wait_until(cancel_at).fuse();
    let mut recorded = Vec::new();
    let mut callbacks = Vec::new();
    let mut cancellation = None;
    let outcome = loop {
        select_biased! {
            request = requests.select_next_some() => {
                let request = request.expect("healthy callback");
                let before = std::path::Path::new(text("effect")).exists();
                assert!(!before, "permission before effect");
                let swallowtail_runtime::CallbackRequestKind::Extension(extension) = request.kind() else { panic!("typed tool extension") };
                assert_eq!(extension.namespace().as_str(), "claude-agent-sdk/can-use-tool");
                let payload: Value = serde_json::from_slice(extension.payload()).expect("callback JSON");
                assert_eq!(payload["toolName"], "Write");
                callbacks.push(json!({"event":"callback", "toolName":"Write", "effectBeforePermission":before}));
                let payload = if case == "allow" { br#"{"decision":"allow"}"#.to_vec() } else { br#"{"decision":"deny"}"#.to_vec() };
                responder.respond(CallbackResponse::for_request(&request,
                    CallbackResult::Success(CallbackPayload::new(payload,4096).expect("bounded callback"))))
                    .await.expect("real permission response");
            },
            event = events.select_next_some() => {
                let event = event.expect("healthy projection");
                let kind = match event.kind() {
                    RuntimeEventKind::Activity(activity) => format!("Activity:{:?}:{:?}:{:?}", activity.kind(), activity.phase(), activity.status()),
                    other => format!("{other:?}"),
                };
                recorded.push(json!({"kind":kind,"sequence":event.sequence(),"content":event.content().map(OperationContent::as_str)}));
            },
            result = terminal => break result,
            _ = cancel_timer => if case == "cancel" {
                cancellation = Some(format!("{:?}",turn.cancellation().request().await.expect("real cancellation")));
            },
        }
    };
    let terminal_status = match outcome.status() {
        TerminalStatus::Completed => "Completed",
        TerminalStatus::Cancelled => "Cancelled",
        TerminalStatus::TimedOut => "TimedOut",
        TerminalStatus::ProviderFailed(_) => "ProviderFailed",
        other => panic!("unexpected original outcome: {other:?}"),
    };
    let output = outcome
        .output()
        .map(OperationContent::as_str)
        .map(str::to_owned);
    let turn_cleanup = turn.close().await;
    json!({"terminalStatus":terminal_status,"output":output,"runtimeEvents":recorded,"callbacks":callbacks,
        "cancellation":cancellation,"deadlineTicks":deadline.instant().ticks(),
        "turnCleanup":format!("{turn_cleanup:?}")})
}
