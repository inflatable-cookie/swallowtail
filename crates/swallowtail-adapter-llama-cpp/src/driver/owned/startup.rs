use super::failure;
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_runtime::{
    Deadline, HostServices, ObservedServingEndpoint, ProcessHandle, ProcessOutputChunk,
    ProcessOutputStream, RuntimeFailure, ScopeId, ServingEndpointBinding,
};

mod parser;
mod readiness;
use parser::{extend_bounded, parse_listening_endpoint};
pub(super) use readiness::{ReadinessRequest, verify_owned_readiness};

pub(super) async fn observe_startup_endpoint(
    process: &dyn ProcessHandle,
    deadline: Deadline,
    services: &HostServices,
) -> Result<StartupObservation, RuntimeFailure> {
    let mut output = Vec::new();
    loop {
        let chunk = read_before_deadline(process, deadline, services).await?;
        let Some(chunk) = chunk else {
            return Err(failure(
                "swallowtail.llama_cpp.serving_process_exited",
                "Owned llama.cpp exited before reporting readiness",
            ));
        };
        if chunk.stream() != ProcessOutputStream::Stderr {
            continue;
        }
        extend_bounded(&mut output, &chunk)?;
        if let Some(endpoint) = parse_listening_endpoint(&output)? {
            let endpoint = ObservedServingEndpoint::new(endpoint).map_err(|_| {
                failure(
                    "swallowtail.llama_cpp.serving_endpoint_invalid",
                    "Owned llama.cpp reported an invalid endpoint",
                )
            })?;
            return Ok(StartupObservation { endpoint, output });
        }
    }
}

pub(super) struct StartupObservation {
    pub endpoint: ObservedServingEndpoint,
    pub output: Vec<u8>,
}

pub(super) async fn authorize_owned_endpoint(
    scope: ScopeId,
    binding: &ServingEndpointBinding,
    services: &HostServices,
) -> Result<String, RuntimeFailure> {
    let grant = services
        .network()
        .expect("validated network service")
        .authorize(
            scope.clone(),
            binding.endpoint().clone(),
            binding.audience().clone(),
        )
        .await?;
    if grant.scope() != &scope
        || grant.endpoint() != binding.endpoint()
        || grant.audience() != binding.audience()
    {
        return Err(failure(
            "swallowtail.llama_cpp.network_grant_mismatch",
            "Owned llama.cpp network grant did not match its endpoint binding",
        ));
    }
    Ok(grant.authorized().as_driver_value().to_owned())
}

async fn read_before_deadline(
    process: &dyn ProcessHandle,
    deadline: Deadline,
    services: &HostServices,
) -> Result<Option<ProcessOutputChunk>, RuntimeFailure> {
    let read = process.read_output();
    let wait = services
        .time()
        .expect("validated time service")
        .wait_until(deadline);
    let mut read = Box::pin(read);
    let mut wait = Box::pin(wait);
    poll_fn(|context| {
        if let Poll::Ready(result) = read.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if wait.as_mut().poll(context).is_ready() {
            return Poll::Ready(Err(timeout_failure()));
        }
        Poll::Pending
    })
    .await
}

pub(super) fn timeout_failure() -> RuntimeFailure {
    failure(
        "swallowtail.llama_cpp.serving_readiness_timed_out",
        "Owned llama.cpp readiness deadline elapsed",
    )
}
