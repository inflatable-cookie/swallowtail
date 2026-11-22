#[path = "prepared_profile_cases/mod.rs"]
mod prepared_profile_cases;
use crate::support;

use futures_executor::block_on;
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::{
    FakeProcessService, host_services, host_services_for, host_services_with,
    host_services_with_for, working_resource,
};
use swallowtail_adapter_codex::{
    CODEX_CLI_AXIS, CodexExecProfileInput, CodexModelSelection, CodexModelVerbosity,
    CodexPreparationInput, CodexPreparationProbe, CodexPreparedDriver, CodexPreparedSessionKind,
    CodexSessionProfileInput, codex_chatgpt_subscription_access_profile, prepare_codex,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ActivityContentStream, ActivityDisclosure,
    ActivityKindClass, ActivityLifecycleFidelity, ActivityUnknownEventPosture, Capability,
    CapabilityConstraint, ConfiguredInstanceId, CredentialState, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionHostId, ExternalNetworkPolicy, ExternalSearchPolicy, HarnessMode,
    HostServiceKind, InstanceRevision, InterfaceVersionAxis, ModelId, ModelRouteId,
    ModelRouteRevision, ObservableActivityAvailability, ReasoningMode, RuntimeReadiness,
    SubagentObservationFidelity, SupportAuthority,
};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, BoxFuture, CleanupOutcome, Deadline,
    DeadlineObservation, DiscoveryCancellation, EnvironmentRef, ExecutableRef, HarnessQuestionId,
    HarnessQuestionOptionId, HarnessUserInputAnswer, HarnessUserInputResponse,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparationStage,
    PreparedAccessEvidence, ProviderRetentionPolicy, RequestId, RuntimeTurnId, SchemaDocument,
    SessionOptions, StructuredOutputDescriptor, TimeService, ToolDeclaration, TurnRequest,
};
use swallowtail_testkit::RecordingHostServices;

const COMPLETED_JSONL: &str = concat!(
    "{\"type\":\"thread.started\",\"thread_id\":\"private-thread\"}\n",
    "{\"type\":\"turn.started\"}\n",
    "{\"type\":\"item.completed\",\"item\":{\"id\":\"message-1\",\"type\":\"agent_message\",\"text\":\"finished\"}}\n",
    "{\"type\":\"turn.completed\"}\n"
);

#[test]
fn prepared_catalogue_and_read_only_session_execute_through_bound_operations() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let catalogue = prepared_app
        .prepare_catalogue(RequestId::new("catalogue").unwrap(), None)
        .expect("catalogue prepares");
    assert_eq!(
        catalogue.plan().requirements().driver_role(),
        DriverRole::ModelCatalog
    );
    assert!(catalogue.plan().model_id().is_none());

    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let models = block_on(catalogue.list_models(support::host_services(process)))
        .expect("prepared catalogue executes");
    assert_eq!(models.len(), 2);
    assert!(state.waited());

    let options = SessionOptions::default()
        .with_reasoning_mode(ReasoningMode::new("low").unwrap())
        .with_tools([tool("lookup")]);
    let session = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("session").unwrap(),
            model(),
            working_resource(),
            None,
            options,
        ))
        .expect("read-only session prepares");
    assert_eq!(session.kind(), CodexPreparedSessionKind::ReadOnly);
    assert_eq!(
        session.request().access_policy(),
        &swallowtail_core::SessionAccessPolicy::read_only()
    );
    assert_eq!(
        session.request().harness_configuration_posture(),
        Some(swallowtail_core::HarnessConfigurationPosture::Ambient)
    );
    assert_eq!(session.request().options().tools().len(), 1);
    assert!(
        session
            .plan()
            .requirements()
            .host_services()
            .any(|service| service == HostServiceKind::Time)
    );

    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let services = support::host_services(process).with_time(Arc::new(PendingTime));
    let mut handle =
        block_on(session.open_session(services.clone())).expect("prepared session opens");
    let turn = block_on(
        handle.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("prepared-deadline-turn").unwrap(),
                OperationContent::new("bounded turn").unwrap(),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(200))),
            services.clone(),
        ),
    )
    .expect("prepared session starts a deadline-bound turn");
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(handle, services)),
        CleanupOutcome::Clean
    );
    assert!(state.methods().contains(&"thread/start".to_owned()));
    assert!(state.waited());
}

fn prepared(
    driver: CodexPreparedDriver,
    version: &str,
    recording: &RecordingHostServices,
    optional_services: bool,
) -> swallowtail_adapter_codex::CodexPreparedIntegration {
    prepared_on_host(
        driver,
        version,
        recording,
        optional_services,
        ExecutionHostId::new("host.local").unwrap(),
    )
}

fn prepared_on_host(
    driver: CodexPreparedDriver,
    version: &str,
    recording: &RecordingHostServices,
    optional_services: bool,
    host: ExecutionHostId,
) -> swallowtail_adapter_codex::CodexPreparedIntegration {
    let target = InstalledExecutableTarget::new(
        ExecutableRef::new("codex-executable").unwrap(),
        InterfaceVersionAxis::new(CODEX_CLI_AXIS).unwrap(),
    );
    let input = CodexPreparationInput::new(
        driver,
        ConfiguredInstanceId::new("codex.prepared").unwrap(),
        InstanceRevision::new("1").unwrap(),
        host.clone(),
        target,
        EnvironmentRef::new("saved-login").unwrap(),
        access_profile(),
        PreparedAccessEvidence::caller_asserted(access_status()),
    );
    let probe = CodexPreparationProbe::new(
        RequestId::new(format!("probe-{version}")).unwrap(),
        swallowtail_runtime::ScopeId::new(format!("probe-{version}")).unwrap(),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    );
    let (process, _) = FakeProcessService::completed(&format!("codex-cli {version}\n"));
    let mut services = host_services_for(host, process).with_time(Arc::new(PendingTime));
    if optional_services {
        let available = recording.services();
        services = services
            .with_network(available.network().unwrap().clone())
            .with_working_resource(available.working_resource().unwrap().clone())
            .with_attachment(available.attachment().unwrap().clone())
            .with_schema(available.schema().unwrap().clone());
    }
    block_on(prepare_codex(input, probe, services)).expect("installed Codex prepares")
}

fn model() -> CodexModelSelection {
    CodexModelSelection::new(
        ModelRouteId::new("codex-model").unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ModelId::new("gpt-5.4-mini").unwrap(),
    )
}

fn access_profile() -> AccessProfile {
    codex_chatgpt_subscription_access_profile(AccessProfileId::new("access.codex").unwrap())
}

fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("access.codex").unwrap(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

fn tool(name: &str) -> ToolDeclaration {
    ToolDeclaration::new(
        name,
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1024).unwrap(),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .unwrap()
}

struct PendingTime;

impl TimeService for PendingTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

use std::sync::Arc;

#[test]
fn idioms_session_option_binds_into_the_plan_and_folds_into_developer_instructions() {
    use swallowtail_idioms::{
        BoundedText, Idiom, IdiomConstraint, IdiomId, IdiomScope, MonotonicInstant as IdiomClock,
        Provenance, StaticRulesSource,
    };
    use swallowtail_runtime::IdiomSessionOption;
    use swallowtail_testkit::assert_idioms_route_opt_in_contract;

    assert_idioms_route_opt_in_contract();

    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.145.0", &recording, false);
    let options = SessionOptions::default()
        .with_developer_instructions(OperationContent::new("consumer text").unwrap())
        .with_idioms(IdiomSessionOption::new(IdiomScope::Project, 8).unwrap());
    let session = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("idioms-session").unwrap(),
            model(),
            working_resource(),
            None,
            options,
        ))
        .expect("read-only session with idioms prepares");
    assert!(
        session
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::IdiomsSessionOption),
        "the prepared plan must bind the idioms session option"
    );

    let rule = Idiom::new(
        IdiomId::new("named-exports").unwrap(),
        IdiomScope::Project,
        IdiomConstraint::text("use named exports").unwrap(),
        90,
        IdiomClock::from_ticks(0),
        Provenance::Static(BoundedText::new("team rules", 256).unwrap()),
    )
    .unwrap();
    let source = StaticRulesSource::new(vec![rule]);

    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services(process).with_idiom_source(Arc::new(source));
    let handle = block_on(session.open_session(services.clone())).expect("prepared session opens");
    let _ = block_on(support::close_session(handle, services));

    let start = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(|value| value.as_str()) == Some("thread/start")
        })
        .expect("thread/start is captured");
    let params = start.get("params").expect("params are captured");
    let instructions = params
        .get("developerInstructions")
        .and_then(|value| value.as_str())
        .expect("developer instructions are folded in");
    assert!(
        instructions.starts_with("consumer text"),
        "consumer instructions stay first"
    );
    assert!(
        instructions.contains("[idioms]")
            && instructions.contains("[project static] use named exports"),
        "the folded idioms block is appended under its labeled header"
    );
}

#[test]
fn bounded_workspace_session_and_import_accept_current_workspace_roots_revisions() {
    use swallowtail_adapter_codex::{
        CodexSessionCatalogueInput, codex_approval_request_extension,
        codex_user_input_request_extension,
    };
    use swallowtail_core::{
        ProviderSessionActivityState, ProviderSessionDisplayContent,
        ProviderSessionImportAvailability, SessionRef,
    };
    use swallowtail_runtime::{ProviderSessionCandidate, ProviderSessionCandidateId};

    let recording = RecordingHostServices::default();

    for version in ["0.156.0", "0.159.0", "0.164.0"] {
        let prepared_app = prepared(CodexPreparedDriver::AppServer, version, &recording, true);
        let session = prepared_app
            .prepare_bounded_workspace_session(CodexSessionProfileInput::new(
                RequestId::new(format!("bounded-session-{version}")).unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default(),
            ))
            .unwrap_or_else(|error| {
                panic!("bounded workspace session must prepare on {version}: {error:?}")
            });
        assert_eq!(session.kind(), CodexPreparedSessionKind::BoundedWorkspace);

        let catalogue = prepared_app
            .prepare_session_catalogue(CodexSessionCatalogueInput::new(
                RequestId::new(format!("bounded-catalogue-{version}")).unwrap(),
                swallowtail_runtime::ProviderSessionCatalogueId::new(format!(
                    "codex-bounded-catalogue-{version}"
                ))
                .unwrap(),
                working_resource(),
                swallowtail_core::ProviderSessionCatalogueBounds::new(
                    std::num::NonZeroU32::new(2).unwrap(),
                    std::num::NonZeroU32::new(4).unwrap(),
                    std::num::NonZeroU32::new(64).unwrap(),
                    std::num::NonZeroU32::new(128).unwrap(),
                    std::num::NonZeroU32::new(128).unwrap(),
                )
                .unwrap(),
            ))
            .expect("catalogue prepares");
        let candidate = ProviderSessionCandidate::new(
            catalogue.plan(),
            ProviderSessionCandidateId::new("candidate-1").unwrap(),
            SessionRef::new("thread-1").unwrap(),
            ProviderSessionDisplayContent::new(Some("title".to_owned()), None).unwrap(),
            Some(1_700_000_000_000),
            ProviderSessionActivityState::Inactive,
            ProviderSessionImportAvailability::Available,
        )
        .expect("candidate builds");

        let import = prepared_app
            .prepare_bounded_workspace_session_import(
                &catalogue,
                candidate,
                CodexSessionProfileInput::new(
                    RequestId::new(format!("bounded-import-{version}")).unwrap(),
                    model(),
                    working_resource(),
                    None,
                    SessionOptions::default(),
                )
                .with_user_input_exchange(),
            )
            .unwrap_or_else(|error| {
                panic!("bounded workspace import must prepare on {version}: {error:?}")
            });
        let namespaces: Vec<_> = import
            .plan()
            .preflight()
            .requirements()
            .extension_namespaces()
            .cloned()
            .collect();
        assert!(
            namespaces.contains(&codex_approval_request_extension()),
            "bounded workspace import must populate approval request extension namespace on {version}"
        );
        assert!(
            namespaces.contains(&codex_user_input_request_extension()),
            "bounded workspace import with user input exchange must populate user input extension namespace on {version}"
        );
    }

    // 0.130.0 precedes workspace-roots support and must remain rejected.
    let pre_workspace = prepared(CodexPreparedDriver::AppServer, "0.130.0", &recording, true);
    let session_err = pre_workspace
        .prepare_bounded_workspace_session(CodexSessionProfileInput::new(
            RequestId::new("pre-workspace-session").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect_err("0.130.0 must reject bounded workspace session");
    assert_eq!(
        session_err.diagnostic().safe().code(),
        "swallowtail.codex.preparation.workspace_version_unsupported"
    );
}
