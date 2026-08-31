use super::mode;
use crate::claude_code_support::{
    FakeProcessService, PendingTimeService, host_services, preparation_input, preparation_probe,
    response_preparation_input, response_preparation_probe,
};
use futures_executor::block_on;
use std::num::NonZeroU32;
use std::sync::Arc;
use swallowtail_adapter_claude_agent::{
    ClaudeCodeMaximumTurns, ClaudeCodeModelSelection, ClaudeCodePreparedIntegration,
    ClaudeCodePreparedRun, ClaudeCodeResponseModelSelection, ClaudeCodeResponsePreparedIntegration,
    ClaudeCodeResponsePreparedRun, ClaudeCodeResponseProfileInput, ClaudeCodeRunProfileInput,
    prepare_claude_code_headless, prepare_claude_code_response_only,
};
use swallowtail_core::{ExecutionHostId, ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    Deadline, MonotonicInstant, OperationContent, RequestId, WorkingResourceRef,
};

pub(crate) fn code_run(
    reasoning: Option<&str>,
    maximum_turns: Option<u32>,
) -> ClaudeCodePreparedRun {
    let prepared = code_prepared();
    let mut input = ClaudeCodeRunProfileInput::new(
        RequestId::new("projection-code-run").expect("request is valid"),
        ClaudeCodeModelSelection::new(
            ModelRouteId::new("projection.code.route").expect("route is valid"),
            ModelRouteRevision::new("1").expect("revision is valid"),
            ModelId::new("claude-opus-5").expect("model is valid"),
        ),
        OperationContent::new("projection fixture").expect("content is valid"),
        WorkingResourceRef::new("projection.code.workspace").expect("resource is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    );
    if let Some(reasoning) = reasoning {
        input = input.with_reasoning_mode(mode(reasoning));
    }
    if let Some(maximum) = maximum_turns {
        input = input.with_maximum_turns(ClaudeCodeMaximumTurns::new(
            NonZeroU32::new(maximum).expect("maximum is positive"),
        ));
    }
    prepared.prepare_run(input).expect("code run prepares")
}

pub(crate) fn response_run(reasoning: Option<&str>) -> ClaudeCodeResponsePreparedRun {
    let prepared = response_prepared();
    let mut input = ClaudeCodeResponseProfileInput::new(
        RequestId::new("projection-response-run").expect("request is valid"),
        ClaudeCodeResponseModelSelection::new(
            ModelRouteId::new("projection.response.route").expect("route is valid"),
            ModelRouteRevision::new("1").expect("revision is valid"),
            ModelId::new("claude-sonnet-5").expect("model is valid"),
        ),
        OperationContent::new("projection fixture").expect("content is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    );
    if let Some(reasoning) = reasoning {
        input = input.with_reasoning_mode(mode(reasoning));
    }
    prepared.prepare_run(input).expect("response run prepares")
}

fn code_prepared() -> ClaudeCodePreparedIntegration {
    let host = ExecutionHostId::new("fixture.projection.code").expect("host is valid");
    let (process, _) = FakeProcessService::completed("2.1.220 (Claude Code)\n");
    let (services, _) = host_services(host.clone(), process, Arc::new(PendingTimeService));
    block_on(prepare_claude_code_headless(
        preparation_input(host),
        preparation_probe(),
        services,
    ))
    .expect("Claude Code prepares")
}

fn response_prepared() -> ClaudeCodeResponsePreparedIntegration {
    let host = ExecutionHostId::new("fixture.projection.response").expect("host is valid");
    let (process, _) = FakeProcessService::completed("2.1.228 (Claude Code)\n");
    let (services, _) = host_services(host.clone(), process, Arc::new(PendingTimeService));
    block_on(prepare_claude_code_response_only(
        response_preparation_input(host),
        response_preparation_probe(),
        services,
    ))
    .expect("Claude Code response-only prepares")
}
