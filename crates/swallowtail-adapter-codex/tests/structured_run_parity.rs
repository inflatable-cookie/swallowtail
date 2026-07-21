mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::{
    FakeProcessService, app_server_plan, host_services, host_services_with, plan, plan_with,
    working_resource,
};
use swallowtail_adapter_codex::{CodexAppServerDriver, CodexExecDriver};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, DriverRole, HostServiceKind,
    ReasoningMode,
};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, CancellationAcknowledgement,
    CleanupOutcome, Deadline, EnvironmentRef, ExternalNetworkPolicy, ExternalSearchPolicy,
    ModelCatalogDriver, ModelCatalogRequest, MonotonicInstant, OperationContent, OperationPolicy,
    RequestId, RuntimeEventKind, SchemaDocument, StructuredOutputDescriptor, StructuredRunDriver,
    StructuredRunRequest, TerminalStatus,
};
use swallowtail_testkit::{RecordedHostCall, RecordingHostServices};

const STRUCTURED_JSONL: &str = concat!(
    "{\"type\":\"thread.started\",\"thread_id\":\"private-thread\"}\n",
    "{\"type\":\"turn.started\"}\n",
    "{\"type\":\"item.started\",\"item\":{\"type\":\"web_search\"}}\n",
    "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"{\\\"label\\\":\\\"example\\\"}\"}}\n",
    "{\"type\":\"turn.completed\"}\n"
);

#[test]
fn public_catalog_and_structured_run_cover_the_complete_transport_seam() {
    let environment = environment();
    let (catalog_process, catalog_state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let models = block_on(CodexAppServerDriver::new(environment.clone()).list_models(
        app_server_plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("parity-catalog").expect("request id is valid")),
        host_services(catalog_process),
    ))
    .expect("model catalog succeeds");
    let selected = models
        .iter()
        .find(|model| model.id().as_str() == "gpt-5.4-mini")
        .expect("selected model is catalogued");
    let low = ReasoningMode::new("low").expect("reasoning mode is valid");
    assert!(selected.metadata().is_default());
    assert_eq!(
        selected.metadata().description(),
        Some("Fast structured work")
    );
    assert!(
        selected
            .metadata()
            .reasoning()
            .expect("reasoning evidence is present")
            .supports(&low)
    );
    assert!(catalog_state.waited());

    let capabilities = structured_capabilities(low.clone());
    let optional_services = [
        HostServiceKind::Attachment,
        HostServiceKind::Schema,
        HostServiceKind::Network,
    ];
    let recording = RecordingHostServices::default();
    let (process, process_state) = FakeProcessService::completed(STRUCTURED_JSONL);
    let request = StructuredRunRequest::new(
        RequestId::new("parity-run").expect("request id is valid"),
        OperationContent::new("Return one structured result").expect("content is valid"),
        OperationPolicy::new(
            ExternalNetworkPolicy::HostApproved,
            ExternalSearchPolicy::Enabled,
        )
        .expect("search policy is valid")
        .with_reasoning_mode(low),
    )
    .with_working_resource(working_resource())
    .with_attachments([image("parity-image")])
    .with_structured_output(schema());
    let mut handle = block_on(CodexExecDriver::new(environment).start_run(
        plan_with(capabilities, optional_services),
        request,
        host_services_with(process, &recording, optional_services),
    ))
    .expect("structured run starts");
    let events = block_on(
        handle
            .take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    );
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert!(events.iter().all(Result::is_ok));
    let events = events.into_iter().map(Result::unwrap).collect::<Vec<_>>();
    assert_eq!(
        events.first().map(|event| event.kind()),
        Some(&RuntimeEventKind::Started)
    );
    assert!(
        events
            .iter()
            .any(|event| event.kind() == &RuntimeEventKind::Progress)
    );
    assert!(
        events
            .iter()
            .any(|event| event.kind() == &RuntimeEventKind::OutputAvailable)
    );
    assert!(
        events
            .windows(2)
            .all(|pair| pair[0].sequence() < pair[1].sequence())
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(OperationContent::as_str),
        Some("{\"label\":\"example\"}")
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(process_state.waited());
    let arguments = process_state.request().arguments;
    assert!(has_pair(&arguments, "--model", "gpt-5.4-mini"));
    assert!(has_pair(
        &arguments,
        "--config",
        "model_reasoning_effort=\"low\""
    ));
    assert!(has_pair(&arguments, "--config", "web_search=\"live\""));
    assert!(has_pair(
        &arguments,
        "--image",
        "/private/recording/attachment.png"
    ));
    assert!(has_pair(
        &arguments,
        "--output-schema",
        "/private/recording/schema.json"
    ));
    assert!(
        arguments
            .iter()
            .any(|argument| argument == "--skip-git-repo-check")
    );
    assert_eq!(recording.count(RecordedHostCall::AttachmentFileRelease), 1);
    assert_eq!(recording.count(RecordedHostCall::SchemaFileRelease), 1);
    assert!(!format!("{terminal:?}").contains("example"));
}

#[test]
fn public_deadline_path_times_out_joins_and_releases_materialization() {
    let recording = RecordingHostServices::default();
    let (process, state) = FakeProcessService::held_open();
    let attachment_capability = CapabilityRequirement::new(
        Capability::Attachments,
        [
            CapabilityConstraint::attachment_media_type("image/png").expect("media type is valid"),
            CapabilityConstraint::AttachmentMaximumBytes(2048),
            CapabilityConstraint::AttachmentMaximumCount(1),
        ],
    );
    let optional_services = [HostServiceKind::Time, HostServiceKind::Attachment];
    let request = basic_request("parity-timeout")
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(20)))
        .with_attachments([image("parity-timeout-image")]);
    let mut handle = block_on(CodexExecDriver::new(environment()).start_run(
        plan_with([attachment_capability], optional_services),
        request,
        host_services_with(process, &recording, optional_services),
    ))
    .expect("deadline-bound run starts");
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
    assert_eq!(recording.count(RecordedHostCall::AttachmentFileRelease), 1);
}

#[test]
fn public_cancellation_path_cancels_joins_and_reports_distinctly() {
    let (process, state) = FakeProcessService::held_open();
    let mut handle = block_on(CodexExecDriver::new(environment()).start_run(
        plan(),
        basic_request("parity-cancel"),
        host_services(process),
    ))
    .expect("run starts");
    assert_eq!(
        block_on(handle.cancellation().request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
}

fn structured_capabilities(reasoning: ReasoningMode) -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(
            Capability::Attachments,
            [
                CapabilityConstraint::attachment_media_type("image/png")
                    .expect("media type is valid"),
                CapabilityConstraint::AttachmentMaximumBytes(2048),
                CapabilityConstraint::AttachmentMaximumCount(1),
            ],
        ),
        CapabilityRequirement::new(
            Capability::StructuredOutput,
            [CapabilityConstraint::schema_dialect("json-schema-2020-12")
                .expect("schema dialect is valid")],
        ),
        CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::reasoning_mode(reasoning)],
        ),
        CapabilityRequirement::new(Capability::ExternalSearch, []),
    ]
}

fn image(reference: &str) -> AttachmentDescriptor {
    AttachmentDescriptor::new(
        AttachmentRef::new(reference).expect("attachment reference is valid"),
        "image/png",
        AttachmentRole::Input,
    )
    .expect("attachment is valid")
    .with_known_length(1024)
}

fn schema() -> StructuredOutputDescriptor {
    StructuredOutputDescriptor::new(
        SchemaDocument::inline(br#"{"type":"object","required":["label"]}"#.to_vec(), 2048)
            .expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("structured output is valid")
}

fn basic_request(id: &str) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("Return one result").expect("content is valid"),
        OperationPolicy::offline(),
    )
    .with_working_resource(working_resource())
}

fn environment() -> EnvironmentRef {
    EnvironmentRef::new("codex-saved-login").expect("environment is valid")
}

fn has_pair(arguments: &[String], key: &str, value: &str) -> bool {
    arguments
        .windows(2)
        .any(|pair| pair[0] == key && pair[1] == value)
}
