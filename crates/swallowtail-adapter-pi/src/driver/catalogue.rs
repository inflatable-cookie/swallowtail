use super::cleanup::{
    graceful_shutdown_deadline, join_session_pump, spawn_session_pump, stop_session_process,
};
use super::launch::catalogue_arguments;
use super::session::cleanup::release_credential;
use super::validation::validate_catalogue;
use super::{deadline_elapsed, run_before_deadline, start_process_before_deadline};
use crate::catalogue::parse_catalogue;
use crate::connection::PiConnection;
use crate::failure::failure;
use serde_json::json;
use std::sync::Arc;
use swallowtail_core::{ModelCatalogEntry, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, ExecutableRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, ProcessRequest, RuntimeFailure, ScopeId,
};

const CATALOGUE_TIMEOUT: &str = "swallowtail.pi.rpc.catalogue_timed_out";

impl ModelCatalogDriver for super::PiRpcDriver {
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
                    "swallowtail.pi.rpc.catalogue_deadline_elapsed",
                    "Pi RPC model catalogue deadline elapsed before startup",
                ));
            }
            self.list_models_with_process(plan, request, services).await
        })
    }
}

impl super::PiRpcDriver {
    async fn list_models_with_process(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "pi-rpc:catalogue:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.pi.rpc.scope_invalid",
                "Pi RPC scope was invalid",
            )
        })?;
        let credential_service = services
            .credential()
            .cloned()
            .expect("validated Pi credential service");
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
            credential.as_ref().expect("Pi credential was acquired"),
            &scope,
            &self.credential,
            plan.endpoint_audience(),
        ) {
            let _ = credential_service
                .release(credential.take().expect("Pi credential was acquired"))
                .await;
            return Err(failure(
                "swallowtail.pi.rpc.credential_lease_rejected",
                "Pi RPC requires a matching delegated credential lease",
            ));
        }
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(catalogue_arguments())
        .with_environment([self.environment.clone()]);
        let process = match start_process_before_deadline(
            scope.clone(),
            process_request,
            request.deadline(),
            &services,
            CATALOGUE_TIMEOUT,
            "Pi RPC model catalogue timed out during process start",
        )
        .await
        {
            Ok(process) => process,
            Err(error) => {
                let _ = credential_service
                    .release(credential.take().expect("Pi credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let connection = PiConnection::new(
            Arc::clone(&process),
            services.task().cloned().expect("validated Pi task service"),
            services.time().cloned().expect("validated Pi time service"),
            services.clone(),
        );
        let pump = Arc::clone(&connection);
        let pump_task = match spawn_session_pump(scope, &services, pump.pump()) {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                let _ = credential_service
                    .release(credential.take().expect("Pi credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let id = format!("catalogue:{}", request.request_id().as_str());
        let command = connection.command(
            id.clone(),
            "get_available_models",
            json!({"id": id, "type": "get_available_models"}),
        );
        let response = run_before_deadline(
            command,
            request.deadline(),
            &services,
            CATALOGUE_TIMEOUT,
            "Pi RPC model catalogue discovery timed out",
        )
        .await;
        let result = response.and_then(parse_catalogue);
        let process_cleanup = stop_session_process(
            connection.begin_close(),
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
                "swallowtail.pi.rpc.catalogue_cleanup_failed",
                "Pi RPC model catalogue cleanup failed",
            )),
        }
    }
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
