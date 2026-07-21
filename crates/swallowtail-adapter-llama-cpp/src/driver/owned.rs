use super::*;
use crate::protocol::OWNED_VERSION;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ExecutableRef, ModelArtifactLease, OwnedServingHandle, ProcessRequest, StartServingRequest,
    validate_owned_serving_start,
};

mod cleanup;
mod handle;
mod roles;
mod startup;

pub use roles::llama_cpp_owned_descriptor;

use cleanup::OwnedState;
use handle::LlamaCppOwnedHandle;
use startup::{
    ReadinessRequest, authorize_owned_endpoint, observe_startup_endpoint, verify_owned_readiness,
};

pub(super) const OWNED_DRIVER_ID: &str = "swallowtail.llama-cpp.owned-b10069-openai-chat";
const EXECUTABLE_ARGUMENTS: usize = 11;

#[derive(Clone)]
pub struct LlamaCppOwnedDriver {
    facade: LlamaCppAttachedDriver,
}

impl LlamaCppOwnedDriver {
    #[must_use]
    pub fn new() -> Self {
        Self {
            facade: LlamaCppAttachedDriver::for_facade(
                OWNED_DRIVER_ID,
                OWNED_VERSION,
                "llama-cpp-owned-b10069",
            ),
        }
    }

    fn validate_start(
        plan: &PreflightPlan,
        request: &StartServingRequest,
        services: &HostServices,
    ) -> Result<String, RuntimeFailure> {
        validate_owned_serving_start(plan, request, services)?;
        if plan.driver_identity().id().as_str() != OWNED_DRIVER_ID {
            return Err(failure(
                "swallowtail.llama_cpp.owned_driver_mismatch",
                "Owned llama.cpp preflight is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
        {
            return Err(failure(
                "swallowtail.llama_cpp.owned_access_rejected",
                "Owned llama.cpp requires local unauthenticated access",
            ));
        }
        if services.task().is_none()
            || services.blocking_work().is_none()
            || services.time().is_none()
            || services.process().is_none()
            || services.network().is_none()
            || services.model_artifact().is_none()
            || services.serving_endpoint().is_none()
        {
            return Err(failure(
                "swallowtail.llama_cpp.owned_service_missing",
                "Owned llama.cpp host services are unavailable",
            ));
        }
        if services.time().expect("validated time").now() >= request.deadline().instant() {
            return Err(failure(
                "swallowtail.llama_cpp.serving_readiness_timed_out",
                "Owned llama.cpp readiness deadline elapsed before launch",
            ));
        }
        plan.model_id()
            .map(|model| model.as_str().to_owned())
            .ok_or_else(|| {
                failure(
                    "swallowtail.llama_cpp.owned_route_missing",
                    "Owned llama.cpp requires one exact model route",
                )
            })
    }

    async fn start_owned(
        &self,
        plan: PreflightPlan,
        request: StartServingRequest,
        services: HostServices,
    ) -> Result<Box<dyn OwnedServingHandle>, RuntimeFailure> {
        let alias = Self::validate_start(&plan, &request, &services)?;
        let mut state = OwnedState::new(services.clone());
        let artifact = services
            .model_artifact()
            .expect("validated artifact service")
            .acquire(
                request.scope().clone(),
                plan.execution_host_id().clone(),
                request.artifact().clone(),
            )
            .await?;
        let arguments = launch_arguments(&artifact, &alias);
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments);
        state.artifact = Some(artifact);
        let process = match services
            .process()
            .expect("validated process service")
            .start(request.scope().clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => return state.fail(error).await,
        };
        state.process = Some(process);
        let startup = match observe_startup_endpoint(
            state.process.as_deref().expect("process is retained"),
            request.deadline(),
            &services,
        )
        .await
        {
            Ok(startup) => startup,
            Err(error) => return state.fail(error).await,
        };
        let endpoint = match services
            .serving_endpoint()
            .expect("validated endpoint service")
            .publish(
                request.scope().clone(),
                plan.execution_host_id().clone(),
                plan.endpoint_audience().clone(),
                startup.endpoint,
            )
            .await
        {
            Ok(endpoint) => endpoint,
            Err(error) => return state.fail(error).await,
        };
        state.endpoint = Some(endpoint);
        let authorized = match authorize_owned_endpoint(
            request.scope().clone(),
            state
                .endpoint
                .as_ref()
                .expect("endpoint is retained")
                .binding(),
            &services,
        )
        .await
        {
            Ok(endpoint) => endpoint,
            Err(error) => return state.fail(error).await,
        };
        if let Err(error) = verify_owned_readiness(
            &self.facade,
            ReadinessRequest {
                scope: request.scope().clone(),
                endpoint: authorized,
                alias: &alias,
                deadline: request.deadline(),
                services: &services,
                process: state.process.as_deref().expect("process is retained"),
                startup_output: startup.output,
            },
        )
        .await
        {
            return state.fail(error).await;
        }
        let binding = state
            .endpoint
            .as_ref()
            .expect("ready endpoint is retained")
            .binding()
            .clone();
        Ok(Box::new(LlamaCppOwnedHandle::new(
            request.serving_instance_id().clone(),
            plan.execution_host_id().clone(),
            binding,
            state,
        )))
    }
}

impl Default for LlamaCppOwnedDriver {
    fn default() -> Self {
        Self::new()
    }
}

fn launch_arguments(artifact: &ModelArtifactLease, alias: &str) -> Vec<String> {
    let arguments = vec![
        "--model".to_owned(),
        artifact.materialized().as_driver_value().to_owned(),
        "--alias".to_owned(),
        alias.to_owned(),
        "--host".to_owned(),
        "127.0.0.1".to_owned(),
        "--port".to_owned(),
        "0".to_owned(),
        "--offline".to_owned(),
        "--no-ui".to_owned(),
        "--no-agent".to_owned(),
    ];
    debug_assert_eq!(arguments.len(), EXECUTABLE_ARGUMENTS);
    arguments
}

fn cleanup_failure() -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.llama_cpp.serving_cleanup_failed",
        "Owned llama.cpp cleanup could not be joined",
    ))
}
