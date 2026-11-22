use super::session::cleanup::release_credential;
use super::validation::validate_catalogue;
use crate::driver::cleanup::{
    graceful_shutdown_deadline, join_session_pump, spawn_session_pump, stop_session_process,
};
use crate::driver::{deadline_elapsed, run_before_deadline, start_process_before_deadline};
use crate::failure::failure;
use crate::sidecar::catalogue::parse_catalogue;
use crate::sidecar::connection::{CommandResult, SidecarConnection};
use crate::sidecar::selection::{
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_WIRE_AXIS,
};
use crate::sidecar::wire::PiSdkSidecarCommand;
use crate::sidecar::{PI_SDK_SIDECAR_BEHAVIOR, PI_SDK_SIDECAR_SDK_PACKAGE, PI_SDK_SIDECAR_WIRE};
use serde_json::{Value, json};
use std::sync::Arc;
use swallowtail_core::{ModelCatalogEntry, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, ExecutableRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, ProcessRequest, RuntimeFailure, ScopeId,
};

const CATALOGUE_TIMEOUT: &str = "swallowtail.pi.sdk-sidecar.catalogue_timed_out";

impl ModelCatalogDriver for super::PiSdkSidecarDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate_catalogue(&plan, &services, &self.credential)?;
            if deadline_elapsed(request.deadline(), &services) {
                return Err(failure(
                    "swallowtail.pi.sdk-sidecar.catalogue_deadline_elapsed",
                    "Pi SDK sidecar model catalogue deadline elapsed before startup",
                ));
            }
            self.list_models_with_process(plan, request, services).await
        })
    }
}

impl super::PiSdkSidecarDriver {
    async fn list_models_with_process(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "pi-sdk-sidecar:catalogue:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.pi.sdk-sidecar.scope_invalid",
                "Pi SDK sidecar scope was invalid",
            )
        })?;
        let credential_service = services
            .credential()
            .cloned()
            .expect("validated sidecar credential service");
        let mut credential = Some(
            credential_service
                .acquire(
                    scope.clone(),
                    self.credential.clone(),
                    plan.endpoint_audience().clone(),
                )
                .await?,
        );
        if !matching_credential(
            credential
                .as_ref()
                .expect("sidecar credential was acquired"),
            &scope,
            &self.credential,
            plan.endpoint_audience(),
        ) {
            let _ = credential_service
                .release(credential.take().expect("sidecar credential was acquired"))
                .await;
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.credential_lease_rejected",
                "Pi SDK sidecar requires a matching delegated credential lease",
            ));
        }
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_environment([self.environment.clone()]);
        let process = match start_process_before_deadline(
            scope.clone(),
            process_request,
            request.deadline(),
            &services,
            CATALOGUE_TIMEOUT,
            "Pi SDK sidecar model catalogue timed out during process start",
        )
        .await
        {
            Ok(process) => process,
            Err(error) => {
                let _ = credential_service
                    .release(credential.take().expect("sidecar credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let connection = SidecarConnection::new(Arc::clone(&process), services.clone());
        let pump = Arc::clone(&connection);
        let pump_task = match spawn_session_pump(scope, &services, pump.pump()) {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                let _ = credential_service
                    .release(credential.take().expect("sidecar credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let command = connection.command(
            "catalogue-bootstrap-1".to_owned(),
            PiSdkSidecarCommand::Bootstrap,
            json!({"catalogueOnly": true}),
        );
        let response = run_before_deadline(
            command,
            request.deadline(),
            &services,
            CATALOGUE_TIMEOUT,
            "Pi SDK sidecar model catalogue discovery timed out",
        )
        .await;
        let timed_out = response
            .as_ref()
            .is_err_and(|error| error.diagnostic().code() == CATALOGUE_TIMEOUT);
        let graceful_stop = async {
            if timed_out || deadline_elapsed(request.deadline(), &services) {
                connection.begin_close().await;
            } else {
                connection
                    .close_session("catalogue-close-1".to_owned())
                    .await;
            }
        };
        let result = response.and_then(|response| parse_catalogue_response(&plan, response));
        let process_cleanup = stop_session_process(
            graceful_stop,
            join_session_pump(pump_task, || connection.cleanup_outcome()),
            async { connection.force_stop().await },
            graceful_shutdown_deadline(&services, None),
            &services,
        )
        .await;
        let credential_cleanup = release_credential(credential.take(), &services).await;
        let cleanup = CleanupOutcome::merge(process_cleanup, credential_cleanup);
        match (result, cleanup) {
            (Err(error), _) => Err(error),
            (Ok(models), CleanupOutcome::Clean) => Ok(models),
            (Ok(_), _) => Err(failure(
                "swallowtail.pi.sdk-sidecar.catalogue_cleanup_failed",
                "Pi SDK sidecar model catalogue cleanup failed",
            )),
        }
    }
}

fn parse_catalogue_response(
    plan: &PreflightPlan,
    response: CommandResult,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    if !response.success {
        return Err(failure(
            "swallowtail.pi.sdk-sidecar.catalogue_rejected",
            "Pi SDK sidecar rejected model catalogue discovery",
        ));
    }
    if !catalogue_identity_matches(plan, response.data.as_ref()) {
        return Err(failure(
            "swallowtail.pi.sdk-sidecar.bootstrap_mismatch",
            "Pi SDK sidecar catalogue identity did not match the preflight-bound runtime and wire",
        ));
    }
    parse_catalogue(response.data.as_ref())
}

fn catalogue_identity_matches(plan: &PreflightPlan, data: Option<&Value>) -> bool {
    let Some(data) = data else {
        return false;
    };
    let expected = |axis: &str| {
        plan.interface_versions()
            .find(|binding| binding.axis().as_str() == axis)
            .expect("validated sidecar plan binds every axis")
            .version()
            .as_str()
            .to_owned()
    };
    let sdk_version = expected(PI_SDK_SIDECAR_PACKAGE_AXIS);
    let node_version = expected(PI_SDK_SIDECAR_NODE_AXIS);
    let wire_version = expected(PI_SDK_SIDECAR_WIRE_AXIS);
    wire_version == PI_SDK_SIDECAR_WIRE
        && data.get("wire").and_then(Value::as_str) == Some(PI_SDK_SIDECAR_WIRE)
        && data.get("behavior").and_then(Value::as_str) == Some(PI_SDK_SIDECAR_BEHAVIOR)
        && data.get("sdkPackage").and_then(Value::as_str) == Some(PI_SDK_SIDECAR_SDK_PACKAGE)
        && data.get("sdkVersion").and_then(Value::as_str) == Some(sdk_version.as_str())
        && data.get("nodeVersion").and_then(Value::as_str) == Some(node_version.as_str())
        && data.get("sessionRef").is_none()
}

fn matching_credential(
    credential: &CredentialLease,
    scope: &ScopeId,
    reference: &swallowtail_core::CredentialRef,
    audience: &swallowtail_core::EndpointAudience,
) -> bool {
    matches!(credential, CredentialLease::Delegated(_))
        && credential.scope() == scope
        && credential.reference() == reference
        && credential.audience() == audience
}
