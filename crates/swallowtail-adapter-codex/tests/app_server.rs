use crate::support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::{Arc, Mutex};
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::{
    app_server_plan, app_server_plan_with, host_services, host_services_with,
    session_resume_binding, working_resource,
};
use swallowtail_adapter_codex::CodexAppServerDriver;
use swallowtail_core::Diagnostic;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, DriverRole, HarnessMode,
    HostServiceKind, ReasoningMode,
};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityKind, CallbackPayload, CallbackRequestKind, CallbackResponse,
    CallbackResult, CancellationAcknowledgement, CleanupOutcome, Deadline, DeadlineObservation,
    DebugObservation, DebugObservationKind, DiagnosticObserver, EnvironmentRef,
    InteractiveSessionDriver, ModelCatalogDriver, ModelCatalogRequest, MonotonicInstant,
    OperationContent, RequestId, RuntimeEventKind, RuntimeTurnId, SchemaDocument,
    SessionAccessPolicy, SessionOptions, SessionPlanAgreement, SessionResumeBinding,
    StructuredOutputDescriptor, TerminalStatus, TimeService, ToolDeclaration, TurnRequest,
    WorkingResourceRef,
};
use swallowtail_testkit::RecordingHostServices;

fn driver() -> CodexAppServerDriver {
    CodexAppServerDriver::new(
        EnvironmentRef::new("codex-saved-login").expect("environment is valid"),
    )
}

fn read_only_open_request(
    request_id: RequestId,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
) -> swallowtail_runtime::OpenSessionRequest {
    swallowtail_runtime::OpenSessionRequest::new(
        request_id,
        working_resource,
        deadline,
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::read_only(),
            Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
            Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
        ),
    )
}

fn read_only_resume_request(
    request_id: RequestId,
    binding: SessionResumeBinding,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
) -> swallowtail_runtime::ResumeSessionRequest {
    swallowtail_runtime::ResumeSessionRequest::new(
        request_id,
        binding,
        working_resource,
        deadline,
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::read_only(),
            Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
            Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
        ),
    )
}

fn tool_declaration(name: &str) -> ToolDeclaration {
    ToolDeclaration::new(
        name,
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"operation":{"type":"string"}}}"#.to_vec(),
            1024,
        )
        .expect("tool schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool declaration is valid")
    .with_description(OperationContent::new("Operate on tasks").expect("description is valid"))
}

fn tool_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::ToolCalls,
        [
            CapabilityConstraint::ToolMaximumCount(4),
            CapabilityConstraint::ToolMaximumSchemaBytes(4096),
            CapabilityConstraint::tool_schema_dialect("json-schema-2020-12")
                .expect("dialect is valid"),
        ],
    )
}

fn reasoning_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::ReasoningSelection,
        [CapabilityConstraint::reasoning_mode(
            ReasoningMode::new("low").expect("reasoning mode is valid"),
        )],
    )
}

fn harness_mode_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::HarnessModeSelection,
        [CapabilityConstraint::harness_mode(HarnessMode::Plan)],
    )
}

fn session_options(tool_name: &str) -> SessionOptions {
    SessionOptions::default()
        .with_developer_instructions(
            OperationContent::new("private session instructions").expect("instructions are valid"),
        )
        .with_reasoning_mode(ReasoningMode::new("low").expect("reasoning mode is valid"))
        .with_tools([tool_declaration(tool_name)])
}

#[derive(Clone)]
struct ControllableTime {
    shared: Arc<Mutex<ControllableTimeState>>,
}

struct ControllableTimeState {
    now: u64,
    fire_through: Option<u64>,
    waiters: Vec<std::task::Waker>,
}

impl ControllableTime {
    fn new(now: u64) -> Self {
        Self {
            shared: Arc::new(Mutex::new(ControllableTimeState {
                now,
                fire_through: None,
                waiters: Vec::new(),
            })),
        }
    }

    fn advance_to(&self, ticks: u64) {
        let waiters = {
            let mut state = self.shared.lock().expect("controllable time lock");
            state.now = ticks;
            state.fire_through = Some(ticks);
            std::mem::take(&mut state.waiters)
        };
        for waiter in waiters {
            waiter.wake();
        }
    }
}

impl TimeService for ControllableTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(self.shared.lock().expect("controllable time lock").now)
    }

    fn wait_until(
        &self,
        deadline: Deadline,
    ) -> swallowtail_runtime::BoxFuture<'static, DeadlineObservation> {
        let shared = Arc::clone(&self.shared);
        Box::pin(std::future::poll_fn(move |context| {
            let mut state = shared.lock().expect("controllable time lock");
            if state
                .fire_through
                .is_some_and(|maximum| deadline.instant().ticks() <= maximum)
            {
                std::task::Poll::Ready(DeadlineObservation::new(
                    deadline,
                    MonotonicInstant::from_ticks(state.now),
                ))
            } else {
                state.waiters.push(context.waker().clone());
                std::task::Poll::Pending
            }
        }))
    }
}

include!("app_server/catalogue_and_session.rs");
include!("app_server/callbacks_and_resume.rs");
include!("app_server/failure_boundaries.rs");
