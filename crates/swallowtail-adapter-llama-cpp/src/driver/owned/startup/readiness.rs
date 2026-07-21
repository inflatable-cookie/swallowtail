use super::{extend_bounded, parse_listening_endpoint, timeout_failure};
use crate::driver::{LlamaCppAttachedDriver, complete_before_deadline, validate_evidence};
use crate::failure::failure;
use crate::protocol::{Readiness, Request, Response, parse_health, parse_models, parse_properties};
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::task::Poll;
use swallowtail_runtime::{
    Deadline, HostServices, ProcessHandle, ProcessOutputChunk, ProcessOutputStream, RuntimeFailure,
    ScopeId,
};

pub(crate) struct ReadinessRequest<'a> {
    pub scope: ScopeId,
    pub endpoint: String,
    pub alias: &'a str,
    pub deadline: Deadline,
    pub services: &'a HostServices,
    pub process: &'a dyn ProcessHandle,
    pub startup_output: Vec<u8>,
}

pub(crate) async fn verify_owned_readiness(
    facade: &LlamaCppAttachedDriver,
    request: ReadinessRequest<'_>,
) -> Result<(), RuntimeFailure> {
    let ReadinessRequest {
        scope,
        endpoint,
        alias,
        deadline,
        services,
        process,
        mut startup_output,
    } = request;
    let cancelled = Arc::new(AtomicBool::new(false));
    loop {
        let health = guarded_response(
            complete_before_deadline(
                facade.transport.request(
                    scope.clone(),
                    endpoint.clone(),
                    Request::health(),
                    services,
                    Arc::clone(&cancelled),
                ),
                Some(deadline),
                services,
                Arc::clone(&cancelled),
            ),
            process,
            &mut startup_output,
        )
        .await
        .map_err(readiness_failure)?;
        match parse_health(&health)? {
            Readiness::Ready => break,
            Readiness::Loading => {
                if services.time().expect("validated time").now() >= deadline.instant() {
                    return Err(timeout_failure());
                }
            }
        }
    }
    let properties = guarded_response(
        complete_before_deadline(
            facade.transport.request(
                scope.clone(),
                endpoint.clone(),
                Request::properties(),
                services,
                Arc::clone(&cancelled),
            ),
            Some(deadline),
            services,
            Arc::clone(&cancelled),
        ),
        process,
        &mut startup_output,
    )
    .await
    .map_err(readiness_failure)?;
    let evidence = parse_properties(&properties, facade.version).map_err(readiness_failure)?;
    validate_evidence(&evidence, Some(alias)).map_err(|_| {
        failure(
            "swallowtail.llama_cpp.serving_route_mismatch",
            "Owned llama.cpp deployment did not expose the configured route",
        )
    })?;
    let models = guarded_response(
        complete_before_deadline(
            facade.transport.request(
                scope,
                endpoint,
                Request::models(),
                services,
                Arc::clone(&cancelled),
            ),
            Some(deadline),
            services,
            cancelled,
        ),
        process,
        &mut startup_output,
    )
    .await
    .map_err(readiness_failure)
    .and_then(|response| parse_models(&response).map_err(readiness_failure))?;
    if models.len() != 1 || models[0].id().as_str() != alias {
        return Err(failure(
            "swallowtail.llama_cpp.serving_route_mismatch",
            "Owned llama.cpp catalogue did not expose the configured route",
        ));
    }
    Ok(())
}

async fn guarded_response<F>(
    response: F,
    process: &dyn ProcessHandle,
    startup_output: &mut Vec<u8>,
) -> Result<Response, RuntimeFailure>
where
    F: Future<Output = Result<Response, RuntimeFailure>>,
{
    let mut response = Box::pin(response);
    loop {
        let mut output = Box::pin(process.read_output());
        let signal = poll_fn(|context| {
            if let Poll::Ready(chunk) = output.as_mut().poll(context) {
                return Poll::Ready(GuardSignal::Output(chunk));
            }
            response.as_mut().poll(context).map(GuardSignal::Response)
        })
        .await;
        match signal {
            GuardSignal::Response(response) => return response,
            GuardSignal::Output(Err(error)) => return Err(error),
            GuardSignal::Output(Ok(None)) => {
                return Err(failure(
                    "swallowtail.llama_cpp.serving_process_exited",
                    "Owned llama.cpp exited before readiness completed",
                ));
            }
            GuardSignal::Output(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stderr =>
            {
                extend_bounded(startup_output, &chunk)?;
                parse_listening_endpoint(startup_output)?;
            }
            GuardSignal::Output(Ok(Some(_))) => {}
        }
    }
}

enum GuardSignal {
    Output(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Response(Result<Response, RuntimeFailure>),
}

fn readiness_failure(error: RuntimeFailure) -> RuntimeFailure {
    match error.diagnostic().code() {
        "swallowtail.llama_cpp.timed_out" | "swallowtail.llama_cpp.deadline_elapsed" => {
            timeout_failure()
        }
        "swallowtail.llama_cpp.version_mismatch" => failure(
            "swallowtail.llama_cpp.serving_build_mismatch",
            "Owned llama.cpp build did not match b10069",
        ),
        _ => error,
    }
}
