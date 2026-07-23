use crate::failure::failure;
use crate::protocol::{FinalStreamUpdate, Usage};
use std::collections::BTreeMap;
use swallowtail_core::ProviderRequestRef;
use swallowtail_runtime::{
    DirectAttemptFinishObservation, DirectAttemptUsageObservation, DirectInferenceAttemptId,
    OperationContent, ProviderFinishReason, ProviderObservation, RuntimeEvent, RuntimeEventKind,
    RuntimeEventSender, RuntimeFailure, TokenUsage,
};

pub(super) fn emit_usage(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    attempt_id: &DirectInferenceAttemptId,
    usage: Usage,
) -> Result<(), RuntimeFailure> {
    if usage.prompt_tokens.saturating_add(usage.completion_tokens) != usage.total_tokens {
        return Err(failure(
            "swallowtail.deepseek.usage_invalid",
            "DeepSeek usage totals were inconsistent",
        ));
    }
    let usage = TokenUsage::new(Some(usage.prompt_tokens), Some(usage.completion_tokens))
        .with_cache_tokens(Some(usage.cache_hit_tokens), None)
        .with_cache_miss_input_tokens(Some(usage.cache_miss_tokens));
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::DirectAttemptUsage(
            DirectAttemptUsageObservation::new(attempt_id.clone(), usage),
        )),
    )
}

pub(super) fn emit_request(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    headers: &BTreeMap<String, String>,
) -> Result<(), RuntimeFailure> {
    let Some(value) = headers.get("x-request-id") else {
        return Ok(());
    };
    let reference = ProviderRequestRef::new(value.clone()).map_err(|_| {
        failure(
            "swallowtail.deepseek.request_id_invalid",
            "DeepSeek request correlation was invalid",
        )
    })?;
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(reference)),
    )
}

pub(super) fn emit_update(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    attempt_id: &DirectInferenceAttemptId,
    update: FinalStreamUpdate,
) -> Result<(), RuntimeFailure> {
    match update {
        FinalStreamUpdate::Output(delta) => {
            emit_content(events, sequence, RuntimeEventKind::OutputDelta, delta)
        }
        FinalStreamUpdate::Usage(usage) => emit_usage(events, sequence, attempt_id, usage),
        FinalStreamUpdate::Finished(reason) => {
            let reason = match reason.as_str() {
                "stop" => ProviderFinishReason::Stop,
                "length" => ProviderFinishReason::Length,
                "content_filter" => ProviderFinishReason::ContentFiltered,
                "insufficient_system_resource" => ProviderFinishReason::InsufficientResources,
                _ => {
                    return Err(failure(
                        "swallowtail.deepseek.finish_reason_invalid",
                        "DeepSeek finish reason was not qualified",
                    ));
                }
            };
            emit(
                events,
                sequence,
                RuntimeEventKind::ProviderObservation(ProviderObservation::DirectAttemptFinish(
                    DirectAttemptFinishObservation::new(attempt_id.clone(), reason),
                )),
            )
        }
    }
}

pub(super) fn emit_output(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    output: &str,
) -> Result<(), RuntimeFailure> {
    emit_content(
        events,
        sequence,
        RuntimeEventKind::OutputAvailable,
        output.to_owned(),
    )
}

pub(super) fn emit(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::new(*sequence, kind))?;
    *sequence += 1;
    Ok(())
}

fn emit_content(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    content: String,
) -> Result<(), RuntimeFailure> {
    let content = OperationContent::new(content).map_err(|_| {
        failure(
            "swallowtail.deepseek.output_invalid",
            "DeepSeek emitted empty output content",
        )
    })?;
    events.send(RuntimeEvent::with_content(*sequence, kind, content))?;
    *sequence += 1;
    Ok(())
}
