//! Sidecar launch: leases, host-owned process tree, and protocol pump.

use super::{ClaudeAgentSdkDriver, PendingSession, scope_invalid};
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use std::sync::Arc;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    CredentialLease, ExecutableRef, HostServices, ProcessHandle, ProcessRequest, ResourceAccess,
    ResourceRepresentation, RuntimeFailure, ScopeId, WorkingResourceRef,
    validate_session_resource_lease,
};

impl ClaudeAgentSdkDriver {
    /// Acquires the credential and resource leases, starts the sidecar inside
    /// the host's descendant-tree authority, and starts the protocol pump,
    /// releasing leases in order on any failure.
    pub(super) async fn spawn_session(
        &self,
        plan: &PreflightPlan,
        request_id: swallowtail_runtime::RequestId,
        working_resource: WorkingResourceRef,
        access_policy: &swallowtail_core::SessionAccessPolicy,
        services: HostServices,
    ) -> Result<PendingSession, RuntimeFailure> {
        let scope = ScopeId::new(format!("claude-agent-sdk:session:{}", request_id.as_str()))
            .map_err(|_| scope_invalid())?;
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
        // Only a delegated lease is admissible: this route never receives,
        // stores, or forwards a subscription credential value.
        if !matches!(credential.as_ref(), Some(CredentialLease::Delegated(_)))
            || credential.as_ref().is_some_and(|lease| {
                lease.scope() != &scope
                    || lease.reference() != &self.credential
                    || lease.audience() != plan.endpoint_audience()
            })
        {
            let _ = credential_service
                .release(credential.take().expect("sidecar credential was acquired"))
                .await;
            return Err(failure(
                "swallowtail.claude-agent.sdk.credential_lease_rejected",
                "Claude Agent SDK sidecar requires a matching delegated credential lease",
            ));
        }
        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated sidecar working-resource service");
        let mut resource = match resource_service
            .resolve(
                scope.clone(),
                working_resource.clone(),
                ResourceAccess::Read,
                ResourceRepresentation::Filesystem,
            )
            .await
        {
            Ok(resource) => Some(resource),
            Err(error) => {
                let _ = credential_service
                    .release(credential.take().expect("sidecar credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        if let Err(error) = validate_session_resource_lease(
            access_policy,
            &working_resource,
            resource.as_ref().expect("sidecar resource was resolved"),
        ) {
            let _ = resource_service
                .release(resource.take().expect("sidecar resource was resolved"))
                .await;
            let _ = credential_service
                .release(credential.take().expect("sidecar credential was acquired"))
                .await;
            return Err(error);
        }
        let leased_cwd = resource
            .as_ref()
            .expect("sidecar resource was resolved")
            .filesystem()
            .expect("validated sidecar filesystem lease exposes a root")
            .as_driver_value()
            .to_owned();
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = match services
            .process()
            .expect("validated sidecar process service")
            .start(scope.clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => {
                let _ = resource_service
                    .release(resource.take().expect("sidecar resource was resolved"))
                    .await;
                let _ = credential_service
                    .release(credential.take().expect("sidecar credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let connection = SdkConnection::new(Arc::clone(&process), services.clone());
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
                let _ = resource_service
                    .release(resource.take().expect("sidecar resource was resolved"))
                    .await;
                let _ = credential_service
                    .release(credential.take().expect("sidecar credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        Ok(PendingSession {
            request_id,
            connection,
            pump_task: Some(pump_task),
            services,
            resource,
            credential,
            leased_cwd,
        })
    }
}
