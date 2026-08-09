use super::common::{host_id, model, prepare};
use super::support;
use futures_executor::block_on;
use swallowtail_adapter_command_code::CommandCodeSessionProfileInput;
use swallowtail_core::{Capability, DriverRole, SessionProviderStatePolicy};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, OperationContent, RequestId, RuntimeTurnId,
    TerminalStatus, TurnRequest, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

const INTERACTIVE_FIRST: &str =
    include_str!("../fixtures/command-code-1.15.1/interactive-first-turn.jsonl");
const INTERACTIVE_RESUME: &str =
    include_str!("../fixtures/command-code-1.15.1/interactive-resume-turn.jsonl");
const SESSION_ID: &str = "00000000-0000-4000-8000-000000000101";

#[test]
fn descriptor_advertises_interactive_session_beside_structured_run() {
    let descriptor = swallowtail_adapter_command_code::command_code_headless_descriptor();
    assert!(descriptor.supports_role(DriverRole::StructuredRun));
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));
    assert!(!descriptor.supports_role(DriverRole::ProviderSessionCatalogue));
}

#[test]
fn prepared_session_uses_exact_resume_on_later_turns_without_ambient_selectors() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    let session = prepared
        .prepare_session(CommandCodeSessionProfileInput::new(
            RequestId::new("command-code.fixture.session").expect("request"),
            model(),
            WorkingResourceRef::new("command-code.fixture.workspace").expect("resource"),
        ))
        .expect("session prepares");
    assert_prepared_operation_evidence_matches_plan(session.evidence(), session.plan());
    assert!(
        session
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::InteractiveSession)
    );
    assert!(
        session
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::ProviderDurableRetention)
    );
    assert_eq!(
        session.request().provider_state_policy(),
        Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
    );
    assert_eq!(
        session
            .prepare_working_state_restoration(
                RuntimeTurnId::new("lost-command-code-turn").expect("turn")
            )
            .method(),
        swallowtail_runtime::WorkingStateRestorationMethod::FreshSessionReplacement
    );

    let host = support::FixtureHost::scripted([INTERACTIVE_FIRST, INTERACTIVE_RESUME]);
    let services = host.services(host_id);
    let mut handle = block_on(session.open_session(services.clone())).expect("session opens");
    assert!(handle.provider_session_ref().is_none());
    assert!(handle.resume_binding().is_none());

    for (index, prompt) in ["first prompt", "second prompt"].into_iter().enumerate() {
        let mut turn = block_on(
            handle.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("command-code-turn-{}", index + 1)).expect("turn"),
                    OperationContent::new(prompt).expect("prompt"),
                )
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
                services.clone(),
            ),
        )
        .expect("turn starts");
        let terminal = block_on(turn.take_terminal_outcome().expect("terminal"));
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }

    let observed = host.observations();
    assert_eq!(observed.len(), 2);
    let first = &observed[0].arguments;
    assert!(!first.iter().any(|argument| argument == "--no-session"));
    assert!(!first.iter().any(|argument| argument == "--resume"));
    assert!(!first.iter().any(|argument| argument == "--continue"));
    assert!(!first.iter().any(|argument| argument == "--fork-session"));
    let second = &observed[1].arguments;
    assert_eq!(
        second
            .windows(2)
            .find(|pair| pair[0] == "--resume")
            .map(|pair| pair.to_vec()),
        Some(vec!["--resume".to_owned(), SESSION_ID.to_owned()])
    );
    assert!(!second.iter().any(|argument| argument == "--no-session"));
    assert!(!second.iter().any(|argument| argument == "--continue"));
    assert!(!second.iter().any(|argument| argument == "--fork-session"));
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn structured_run_still_binds_no_session_and_prohibited_retention() {
    let prepared = prepare(host_id());
    let run = prepared
        .prepare_run(super::common::run_input(model(), "structured-retained"))
        .expect("run prepares");
    assert!(
        run.request().policy().provider_retention()
            == swallowtail_runtime::ProviderRetentionPolicy::Prohibited
    );
    let host = support::FixtureHost::scripted([super::common::NO_TOOL_SUCCESS]);
    let mut handle =
        block_on(run.start_run(host.services(host_id()))).expect("structured run starts");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(
        host.observations()[0]
            .arguments
            .iter()
            .any(|argument| argument == "--no-session")
    );
}
