use super::session::cleanup::{merge_cleanup, release_credential};
use super::validation::validate_catalogue;
use crate::failure::failure;
use crate::sidecar::catalogue::parse_catalogue;
use crate::sidecar::connection::{CommandResult, SidecarConnection};
use crate::sidecar::selection::{
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_WIRE_AXIS,
};
use crate::sidecar::wire::PiSdkSidecarCommand;
use crate::sidecar::{PI_SDK_SIDECAR_BEHAVIOR, PI_SDK_SIDECAR_SDK_PACKAGE, PI_SDK_SIDECAR_WIRE};
use serde_json::{Value, json};
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{ModelCatalogEntry, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, ExecutableRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, ProcessHandle, ProcessRequest, RuntimeFailure, ScopeId,
};

impl ModelCatalogDriver for super::PiSdkSidecarDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate_catalogue(&plan, &services, &self.credential)?;
            if request.deadline().is_some_and(|deadline| {
                services
                    .time()
                    .expect("validated sidecar time service")
                    .now()
                    >= deadline.instant()
            }) {
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
        let process: Arc<dyn ProcessHandle> = match services
            .process()
            .expect("validated sidecar process service")
            .start(scope.clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => {
                let _ = credential_service
                    .release(credential.take().expect("sidecar credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let connection = SidecarConnection::new(Arc::clone(&process), services.clone());
        let pump = Arc::clone(&connection);
        let pump_task = match services
            .task()
            .expect("validated sidecar task service")
            .spawn(scope, Box::pin(async move { pump.pump().await }))
        {
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
        let response = match request.deadline() {
            Some(deadline) => {
                let wait = services
                    .time()
                    .cloned()
                    .expect("validated sidecar time service")
                    .wait_until(deadline);
                command_before_deadline(command, wait).await
            }
            None => command.await,
        };
        let result = response.and_then(|response| parse_catalogue_response(&plan, response));
        let _ = connection
            .command(
                "catalogue-close-1".to_owned(),
                PiSdkSidecarCommand::Close,
                json!({}),
            )
            .await;
        connection.begin_close().await;
        let process_cleanup = match pump_task.join().await {
            Ok(()) => connection.cleanup_outcome(),
            Err(_) => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.pi.sdk-sidecar.catalogue_cleanup_failed",
                "Pi SDK sidecar model catalogue protocol task did not join cleanly",
            )),
        };
        let credential_cleanup = release_credential(credential.take(), &services).await;
        let cleanup = merge_cleanup(process_cleanup, credential_cleanup);
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

async fn command_before_deadline<F>(
    command: F,
    mut deadline: BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
) -> Result<CommandResult, RuntimeFailure>
where
    F: Future<Output = Result<CommandResult, RuntimeFailure>>,
{
    let mut command = Box::pin(command);
    poll_fn(|context| {
        if let Poll::Ready(result) = command.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(Err(failure(
                "swallowtail.pi.sdk-sidecar.catalogue_timed_out",
                "Pi SDK sidecar model catalogue discovery timed out",
            )));
        }
        Poll::Pending
    })
    .await
}
