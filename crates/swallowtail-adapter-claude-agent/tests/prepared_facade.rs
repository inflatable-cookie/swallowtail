#![allow(dead_code)]

#[path = "prepared_facade/session_management.rs"]
mod session_management;
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, ClaudeAgentModelSelection, ClaudeAgentPreparationInput,
    ClaudeAgentPreparationProbe, ClaudeAgentRunProfileInput, ClaudeAgentSessionManagementInput,
    ClaudeAgentSessionProfileInput, prepare_claude_agent,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, CapabilityConstraint,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceRevision,
    InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, OwnedRemoteResourceKind,
    ProviderSessionAffectedScope, ProviderSessionDeletionStrength, ProviderSessionEffectTruth,
    ResourceAccess, RuntimeReadiness, SessionAccessPolicy, SupportAuthority,
};
use swallowtail_runtime::{
    CallbackPayload, CallbackResponse, CallbackResult, CleanupOutcome, Deadline,
    DiscoveryCancellation, EnvironmentRef, ExecutableRef, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, PreparedAccessEvidence, ProviderRetentionPolicy,
    RemoteResourceDeletionOutcome, RequestId, ScopeId, SessionOptions, TerminalStatus,
    WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn prepared_sessions_bind_version_access_model_and_ambient_read_policy() {
    for host_value in ["fixture.prepared.local", "fixture.prepared.remote"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
        let prepared = block_on(prepare_claude_agent(
            preparation_input(host_id.clone()),
            probe(),
            preparation_host.services(host_id.clone()),
        ))
        .expect("Claude Agent prepares");
        let profile = prepared
            .prepare_session(ClaudeAgentSessionProfileInput::new(
                RequestId::new("claude-agent-prepared-open").expect("valid request"),
                ClaudeAgentModelSelection::new(
                    ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ModelId::new("claude-sonnet-4-6").expect("valid model"),
                ),
                WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
                SessionOptions::default().with_reasoning_mode(
                    swallowtail_core::ReasoningMode::new("high").expect("valid reasoning mode"),
                ),
            ))
            .expect("session profile prepares");

        assert_eq!(
            profile
                .evidence()
                .observation()
                .version()
                .version()
                .as_str(),
            "0.61.0"
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.request().access_policy(),
            &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        );
        assert_eq!(
            profile.plan().model_id().map(ModelId::as_str),
            Some("claude-sonnet-4-6")
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );

        let operation_host = FixtureHost::new(Scenario::Success, "0.61.0");
        let session = block_on(profile.open_session(operation_host.services(host_id.clone())))
            .expect("prepared session opens");
        let binding = session
            .management_binding()
            .expect("prepared session returns lifecycle binding")
            .clone();
        assert!(binding.supports(Capability::ProviderNativeSessionClose));
        assert!(binding.supports(Capability::ProviderSessionDelete));
        assert!(session.resume_binding().is_some());
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        let writes = operation_host.writes();
        let config = writes
            .iter()
            .filter(|message| {
                message.get("method").and_then(serde_json::Value::as_str)
                    == Some("session/set_config_option")
            })
            .collect::<Vec<_>>();
        assert_eq!(config.len(), 2);
        assert_eq!(config[0]["params"]["configId"], "model");
        assert_eq!(config[0]["params"]["value"], "claude-sonnet-4-6");
        assert_eq!(config[1]["params"]["configId"], "effort");
        assert_eq!(config[1]["params"]["value"], "high");
        assert!(writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/close")
        }));
        assert_eq!(operation_host.credential_acquires(), 1);
        assert_eq!(operation_host.credential_releases(), 1);
        assert_eq!(operation_host.resource_releases(), 1);

        let delete = prepared
            .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
                RequestId::new(format!("claude-agent-delete-{host_value}"))
                    .expect("valid delete request"),
                binding,
            ))
            .expect("qualified Claude Agent delete prepares");
        assert_prepared_operation_evidence_matches_plan(
            delete.evidence().operation(),
            delete.plan().preflight(),
        );
        let delete_host = FixtureHost::new(Scenario::Success, "0.61.0");
        let outcome = block_on(delete.execute(delete_host.services(host_id)))
            .expect("prepared Claude Agent delete executes");
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );
        assert_eq!(
            outcome.effect().confirmed_deletion_strength(),
            Some(ProviderSessionDeletionStrength::ProviderDataDeleted)
        );
        assert_eq!(
            outcome.effect().affected_scope(),
            Some(ProviderSessionAffectedScope::ProviderDefinedDescendants)
        );
        let writes = delete_host.writes();
        assert!(writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/delete")
        }));
        assert!(!writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/new")
        }));
        assert_eq!(delete_host.credential_acquires(), 1);
        assert_eq!(delete_host.credential_releases(), 1);
        assert_eq!(delete_host.resource_releases(), 1);
    }
}

#[test]
fn prepared_session_load_and_resume_preserve_replay_and_attachment_truth() {
    let host_id = ExecutionHostId::new("fixture.prepared.continuity").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("Claude Agent prepares");
    let profile = prepared
        .prepare_session(ClaudeAgentSessionProfileInput::new(
            RequestId::new("claude-agent-continuity-open").expect("valid request"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ModelId::new("claude-sonnet-4-6").expect("valid model"),
            ),
            WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
            SessionOptions::default(),
        ))
        .expect("session profile prepares");

    let open_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let opened = block_on(profile.open_session(open_host.services(host_id.clone())))
        .expect("prepared session opens");
    let binding = opened
        .resume_binding()
        .expect("prepared session returns resume binding")
        .clone();
    assert_eq!(block_on(opened.close()), CleanupOutcome::Clean);

    let load_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let loaded = block_on(
        profile
            .load_session(
                RequestId::new("claude-agent-continuity-load").expect("valid request"),
                binding.clone(),
                load_host.services(host_id.clone()),
            )
            .expect("prepared load operation derives"),
    )
    .expect("prepared session loads");
    assert_eq!(
        loaded
            .replay()
            .map(swallowtail_runtime::SessionReplayItem::sequence)
            .collect::<Vec<_>>(),
        [0, 1]
    );
    let (_, loaded_handle) = loaded.into_parts();
    assert_eq!(
        loaded_handle
            .management_binding()
            .expect("loaded session returns management authority")
            .origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Loaded
    );
    assert_eq!(block_on(loaded_handle.close()), CleanupOutcome::Clean);

    let resume_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let resumed = block_on(
        profile
            .resume_session(
                RequestId::new("claude-agent-continuity-resume").expect("valid request"),
                binding,
                resume_host.services(host_id),
            )
            .expect("prepared resume operation derives"),
    )
    .expect("prepared session resumes");
    assert_eq!(
        resumed
            .management_binding()
            .expect("resumed session returns management authority")
            .origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Resumed
    );
    assert_eq!(block_on(resumed.close()), CleanupOutcome::Clean);
}

#[test]
fn unsupported_options_fail_before_session_process_effects() {
    let host_id = ExecutionHostId::new("fixture.prepared.options").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id),
    ))
    .expect("Claude Agent prepares");
    let result = prepared.prepare_session(ClaudeAgentSessionProfileInput::new(
        RequestId::new("claude-agent-options").expect("valid request"),
        ClaudeAgentModelSelection::new(
            ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
            ModelRouteRevision::new("1").expect("valid route revision"),
            ModelId::new("claude-sonnet-4-6").expect("valid model"),
        ),
        WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
        SessionOptions::default().with_developer_instructions(
            OperationContent::new("unsupported developer instruction").expect("valid content"),
        ),
    ));
    assert!(result.is_err());

    let result = prepared.prepare_session(ClaudeAgentSessionProfileInput::new(
        RequestId::new("claude-agent-unsupported-reasoning").expect("valid request"),
        ClaudeAgentModelSelection::new(
            ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
            ModelRouteRevision::new("1").expect("valid route revision"),
            ModelId::new("claude-sonnet-4-6").expect("valid model"),
        ),
        WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
        SessionOptions::default().with_reasoning_mode(
            swallowtail_core::ReasoningMode::new("ultra").expect("valid reasoning mode"),
        ),
    ));
    assert!(result.is_err());
}

#[test]
fn prepared_structured_run_binds_one_prompt_and_durable_retention_on_both_hosts() {
    for host_value in ["fixture.run.local", "fixture.run.remote-authoritative"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
        let prepared = block_on(prepare_claude_agent(
            preparation_input(host_id.clone()),
            probe(),
            preparation_host.services(host_id.clone()),
        ))
        .expect("Claude Agent prepares");
        let profile = prepared
            .prepare_run(
                ClaudeAgentRunProfileInput::new(
                    RequestId::new(format!("claude-agent-run-{host_value}"))
                        .expect("valid request"),
                    ClaudeAgentModelSelection::new(
                        ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                        ModelRouteRevision::new("1").expect("valid route revision"),
                        ModelId::new("claude-sonnet-4-6").expect("valid model"),
                    ),
                    OperationContent::new("one private prepared prompt").expect("valid prompt"),
                    WorkingResourceRef::new("claude-agent.prepared.workspace")
                        .expect("valid resource"),
                    None,
                )
                .with_reasoning_mode(
                    swallowtail_core::ReasoningMode::new("high").expect("valid reasoning mode"),
                ),
            )
            .expect("structured run prepares");
        assert_eq!(
            profile.plan().requirements().driver_role(),
            swallowtail_core::DriverRole::StructuredRun
        );
        assert!(
            profile
                .plan()
                .requirements()
                .capabilities()
                .any(|required| {
                    required.capability() == Capability::WorkingResource
                        && required.constraints().any(|constraint| {
                            constraint
                                == &CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite)
                        })
                })
        );
        assert_eq!(
            profile.request().policy().provider_retention(),
            ProviderRetentionPolicy::DurableAllowed
        );
        assert_eq!(
            profile
                .request()
                .policy()
                .reasoning_mode()
                .map(swallowtail_core::ReasoningMode::as_str),
            Some("high")
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );

        let operation_host = FixtureHost::new(Scenario::Success, "0.61.0");
        let mut run = block_on(profile.start_run(operation_host.services(host_id)))
            .expect("structured run starts");
        assert!(run.provider_run_ref().is_none());
        assert!(run.take_callbacks().is_none());
        let mut events = run.take_events().expect("events");
        let terminal = run.take_terminal_outcome().expect("terminal");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("event succeeds");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        let writes = operation_host.writes();
        let config = writes
            .iter()
            .filter(|message| {
                message.get("method").and_then(serde_json::Value::as_str)
                    == Some("session/set_config_option")
            })
            .collect::<Vec<_>>();
        assert_eq!(config.len(), 2);
        assert_eq!(config[0]["params"]["configId"], "model");
        assert_eq!(config[1]["params"]["configId"], "effort");
        assert_eq!(config[1]["params"]["value"], "high");
        assert_eq!(
            writes
                .iter()
                .filter(|message| {
                    message.get("method").and_then(serde_json::Value::as_str)
                        == Some("session/prompt")
                })
                .count(),
            1
        );
        assert!(writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/close")
        }));
        assert!(!writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/delete")
        }));
        assert_eq!(operation_host.credential_releases(), 1);
        assert_eq!(operation_host.resource_releases(), 1);
    }
}

#[test]
fn prepared_structured_run_can_opt_into_operation_owned_session_cleanup() {
    for scenario in [
        Scenario::Success,
        Scenario::Cancellation,
        Scenario::RunDeleteDisconnect,
    ] {
        let host_id = ExecutionHostId::new("fixture.run.owned-cleanup").expect("valid host");
        let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
        let prepared = block_on(prepare_claude_agent(
            preparation_input(host_id.clone()),
            probe(),
            preparation_host.services(host_id.clone()),
        ))
        .expect("Claude Agent prepares");
        let profile = prepared
            .prepare_run(
                ClaudeAgentRunProfileInput::new(
                    RequestId::new("claude-agent-owned-cleanup").expect("valid request"),
                    ClaudeAgentModelSelection::new(
                        ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                        ModelRouteRevision::new("1").expect("valid route revision"),
                        ModelId::new("claude-sonnet-4-6").expect("valid model"),
                    ),
                    OperationContent::new("one temporary prompt").expect("valid prompt"),
                    WorkingResourceRef::new("claude-agent.prepared.workspace")
                        .expect("valid resource"),
                    None,
                )
                .with_owned_session_cleanup(),
            )
            .expect("temporary structured run prepares");
        assert_eq!(
            profile.request().policy().provider_retention(),
            ProviderRetentionPolicy::TemporaryAllowed
        );
        assert!(
            profile
                .plan()
                .requirements()
                .capabilities()
                .any(|required| {
                    required.capability() == Capability::OwnedRemoteResourceDeletion
                        && required
                            .constraints()
                            .eq([&CapabilityConstraint::OwnedRemoteResource(
                                OwnedRemoteResourceKind::Session,
                            )])
                })
        );

        let operation_host = FixtureHost::new(scenario, "0.61.0");
        let mut run = block_on(profile.start_run(operation_host.services(host_id.clone())))
            .expect("temporary structured run starts");
        if scenario == Scenario::Cancellation {
            block_on(run.cancellation().request()).expect("run cancellation is accepted");
        }
        let mut events = run.take_events().expect("events");
        let terminal = run.take_terminal_outcome().expect("terminal");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("event succeeds");
            }
            terminal.await
        });
        assert_eq!(
            outcome.status(),
            if scenario == Scenario::Cancellation {
                &TerminalStatus::Cancelled
            } else {
                &TerminalStatus::Completed
            }
        );
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
            Some(if scenario == Scenario::RunDeleteDisconnect {
                RemoteResourceDeletionOutcome::Unconfirmed
            } else {
                RemoteResourceDeletionOutcome::Confirmed
            })
        );
        assert_eq!(
            matches!(outcome.cleanup(), CleanupOutcome::Degraded(_)),
            scenario == Scenario::RunDeleteDisconnect
        );
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        let writes = operation_host.writes();
        let close = writes
            .iter()
            .position(|message| message["method"] == "session/close")
            .expect("native close dispatched");
        let delete = writes
            .iter()
            .position(|message| message["method"] == "session/delete")
            .expect("owned delete dispatched");
        assert!(close < delete);
        assert_eq!(operation_host.credential_releases(), 1);
        assert_eq!(operation_host.resource_releases(), 1);
    }
}

#[test]
fn prepared_structured_run_opt_in_exposes_one_shot_permission_exchange() {
    let host_id = ExecutionHostId::new("fixture.run.consumer-mediated").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("Claude Agent prepares");
    let profile = prepared
        .prepare_run(
            ClaudeAgentRunProfileInput::new(
                RequestId::new("claude-agent-consumer-mediated").expect("valid request"),
                ClaudeAgentModelSelection::new(
                    ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ModelId::new("claude-sonnet-4-6").expect("valid model"),
                ),
                OperationContent::new("request one provider permission").expect("valid prompt"),
                WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
                Some(Deadline::at(MonotonicInstant::from_ticks(u64::MAX))),
            )
            .with_consumer_mediated_permissions(),
        )
        .expect("consumer-mediated run prepares");
    assert_eq!(
        profile
            .plan()
            .requirements()
            .extension_namespaces()
            .map(swallowtail_core::ExtensionNamespace::as_str)
            .collect::<Vec<_>>(),
        vec!["acp/session/request-permission"]
    );

    let operation_host = FixtureHost::new(Scenario::Permission, "0.61.0");
    let mut run = block_on(profile.start_run(operation_host.services(host_id)))
        .expect("structured run starts");
    let mut callbacks = run.take_callbacks().expect("permission callbacks exist");
    let mut requests = callbacks
        .take_requests()
        .expect("callback request stream exists");
    let callback = block_on(requests.next())
        .expect("permission callback arrives")
        .expect("permission callback is valid");
    let callback_id = callback.callback_id().clone();
    let turn_id = callback.turn_id().expect("callback retains turn").clone();
    assert_eq!(
        callback.deadline(),
        Some(Deadline::at(MonotonicInstant::from_ticks(u64::MAX)))
    );
    let swallowtail_runtime::CallbackRequestKind::Extension(extension) = callback.kind() else {
        panic!("permission is a provider extension");
    };
    assert_eq!(
        extension.namespace().as_str(),
        "acp/session/request-permission"
    );
    let payload: serde_json::Value =
        serde_json::from_slice(extension.payload()).expect("permission payload is JSON");
    assert_eq!(payload["toolCall"]["toolCallId"], "shell-1");
    assert_eq!(payload["options"].as_array().expect("options").len(), 2);

    let responder = callbacks.responder();
    assert!(
        block_on(
            responder.respond(CallbackResponse::new(
                callback_id.clone(),
                swallowtail_runtime::RuntimeTurnId::new("wrong-turn").expect("valid turn"),
                CallbackResult::Success(
                    CallbackPayload::new(br#"{"optionId":"allow-once"}"#, 256)
                        .expect("selection is bounded"),
                ),
            ))
        )
        .is_err()
    );
    assert!(
        block_on(
            responder.respond(CallbackResponse::new(
                callback_id.clone(),
                turn_id.clone(),
                CallbackResult::Success(
                    CallbackPayload::new(br#"{"optionId":"allow-always"}"#, 256)
                        .expect("selection is bounded"),
                ),
            ))
        )
        .is_err()
    );
    let selection = CallbackResponse::new(
        callback_id.clone(),
        turn_id,
        CallbackResult::Success(
            CallbackPayload::new(br#"{"optionId":"allow-once"}"#, 256)
                .expect("selection is bounded"),
        ),
    );
    block_on(responder.respond(selection.clone())).expect("permission selection is transported");
    assert!(block_on(responder.respond(selection)).is_err());

    let mut events = run.take_events().expect("events");
    let terminal = run.take_terminal_outcome().expect("terminal");
    let (observed, outcome) = block_on(async {
        let mut observed = Vec::new();
        while let Some(event) = events.next().await {
            observed.push(event.expect("event succeeds"));
        }
        (observed, terminal.await)
    });
    assert!(observed.iter().any(|event| {
        matches!(
            event.kind(),
            swallowtail_runtime::RuntimeEventKind::CallbackRequested(id)
                if id == &callback_id
        )
    }));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let writes = operation_host.writes();
    assert!(writes.iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_u64) == Some(900)
            && message["result"]["outcome"]["optionId"] == "allow-once"
    }));
    assert!(!writes.iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/cancel")
    }));
}

#[test]
fn local_subscription_facade_inherits_harness_auth_without_a_credential_lease() {
    let host_id = ExecutionHostId::new("fixture.run.local-subscription").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        local_preparation_input(host_id.clone()),
        probe(),
        preparation_host.services_without_credential(host_id.clone()),
    ))
    .expect("locally authenticated Claude Agent prepares");
    let profile = prepared
        .prepare_run(ClaudeAgentRunProfileInput::new(
            RequestId::new("claude-agent-local-subscription-run").expect("valid request"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ModelId::new("claude-sonnet-4-6").expect("valid model"),
            ),
            OperationContent::new("use the local Claude subscription").expect("valid prompt"),
            WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
            None,
        ))
        .expect("local subscription run prepares");

    assert_eq!(
        profile.plan().credential_mechanism(),
        &CredentialMechanism::LocalUnauthenticated
    );
    assert!(profile.plan().credential_reference().is_none());
    assert!(
        !profile
            .plan()
            .requirements()
            .host_services()
            .any(|service| service == HostServiceKind::Credential)
    );

    let operation_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let mut run =
        block_on(profile.start_run(operation_host.services_without_credential(host_id.clone())))
            .expect("local subscription run starts");
    let mut events = run.take_events().expect("events");
    let terminal = run.take_terminal_outcome().expect("terminal");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(operation_host.credential_acquires(), 0);
    assert_eq!(operation_host.credential_releases(), 0);
    assert_eq!(operation_host.observed_process().environment_count, 1);

    let session_profile = prepared
        .prepare_session(ClaudeAgentSessionProfileInput::new(
            RequestId::new("claude-agent-local-subscription-session").expect("valid request"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ModelId::new("claude-sonnet-4-6").expect("valid model"),
            ),
            WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
            SessionOptions::default(),
        ))
        .expect("local subscription session prepares");
    let session_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let session = block_on(
        session_profile.open_session(session_host.services_without_credential(host_id.clone())),
    )
    .expect("local subscription session opens");
    let binding = session
        .management_binding()
        .expect("session returns a management binding")
        .clone();
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(session_host.credential_acquires(), 0);
    assert_eq!(session_host.credential_releases(), 0);

    let delete = prepared
        .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
            RequestId::new("claude-agent-local-subscription-delete").expect("valid request"),
            binding,
        ))
        .expect("local subscription delete prepares");
    let delete_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let outcome = block_on(delete.execute(delete_host.services_without_credential(host_id)))
        .expect("local subscription delete executes");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::Applied
    );
    assert_eq!(delete_host.credential_acquires(), 0);
    assert_eq!(delete_host.credential_releases(), 0);
}

fn preparation_input(host: ExecutionHostId) -> ClaudeAgentPreparationInput {
    ClaudeAgentPreparationInput::new(
        ConfiguredInstanceId::new("claude-agent.prepared").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("claude-agent.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("claude-agent.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            AccessProfileId::new("claude-agent.prepared.access").expect("valid access"),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            EndpointAudience::new("api.anthropic.com").expect("valid audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        )
        .with_credential_reference(
            CredentialRef::new("claude-agent.prepared.credential").expect("valid credential"),
        ),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
}

fn local_preparation_input(host: ExecutionHostId) -> ClaudeAgentPreparationInput {
    let access_id =
        AccessProfileId::new("claude-agent.prepared.local-access").expect("valid access");
    ClaudeAgentPreparationInput::new(
        ConfiguredInstanceId::new("claude-agent.prepared.local").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("claude-agent.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("claude-agent.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::LocalUnauthenticated,
            EntitlementMetering::SubscriptionAllowance,
            EndpointAudience::new("api.anthropic.com").expect("valid audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access_id,
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        )),
    )
}

fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("claude-agent.prepared.access").expect("valid access"),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn probe() -> ClaudeAgentPreparationProbe {
    ClaudeAgentPreparationProbe::new(
        RequestId::new("claude-agent-prepared-probe").expect("valid request"),
        ScopeId::new("claude-agent-prepared-probe").expect("valid scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}
