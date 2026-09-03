//! Sidecar launch: leases, host-owned process tree, and protocol pump.
//!
//! Every acquisition is recorded in the open guard the instant it succeeds, so
//! a caller deadline that drops this future mid-flight still leaves the guard
//! holding what was acquired. Failure paths return the error and let the guard
//! release; they never release behind its back, because a partially acquired
//! open and a fully acquired one must clean up the same way.

use super::{ClaudeAgentSdkDriver, PendingSession, scope_invalid};
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use crate::sdk::guardian::OpenGuard;
use std::sync::Arc;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    ExecutableRef, HostServices, ProcessHandle, ProcessRequest, ResourceAccess,
    ResourceRepresentation, RuntimeFailure, ScopeId, WorkingResourceRef,
    validate_session_resource_lease,
};

impl ClaudeAgentSdkDriver {
    /// Acquires the credential and resource leases, starts the sidecar inside
    /// the host's descendant-tree authority, and starts the protocol pump.
    pub(super) async fn spawn_session(
        &self,
        plan: &PreflightPlan,
        request_id: swallowtail_runtime::RequestId,
        working_resource: WorkingResourceRef,
        access_policy: &swallowtail_core::SessionAccessPolicy,
        services: HostServices,
        guard: &OpenGuard,
    ) -> Result<PendingSession, RuntimeFailure> {
        let scope = ScopeId::new(format!("claude-agent-sdk:session:{}", request_id.as_str()))
            .map_err(|_| scope_invalid())?;
        let credential_service = services
            .credential()
            .cloned()
            .expect("validated sidecar credential service");
        let credential = credential_service
            .acquire(
                scope.clone(),
                self.credential.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        guard.ledger().record_credential(credential);

        // Only a delegated lease is admissible: this route never receives,
        // stores, or forwards a subscription credential value.
        if !guard
            .ledger()
            .credential_matches(&scope, &self.credential, plan.endpoint_audience())
        {
            return Err(failure(
                "swallowtail.claude-agent.sdk.credential_lease_rejected",
                "Claude Agent SDK sidecar requires a matching delegated credential lease",
            ));
        }

        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated sidecar working-resource service");
        let resource = resource_service
            .resolve(
                scope.clone(),
                working_resource.clone(),
                ResourceAccess::Read,
                ResourceRepresentation::Filesystem,
            )
            .await?;
        let leased_cwd = resource
            .filesystem()
            .expect("validated sidecar filesystem lease exposes a root")
            .as_driver_value()
            .to_owned();
        validate_session_resource_lease(access_policy, &working_resource, &resource)?;
        guard.ledger().record_resource(resource);

        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = Arc::from(
            services
                .process()
                .expect("validated sidecar process service")
                .start(scope.clone(), process_request)
                .await?,
        );
        guard.ledger().record_process(Arc::clone(&process));

        let connection = SdkConnection::new(Arc::clone(&process), services.clone());
        let pump = Arc::clone(&connection);
        let pump_task = services
            .task()
            .expect("validated sidecar task service")
            .spawn(scope, Box::pin(async move { pump.pump().await }))?;
        guard.ledger().record_pump(pump_task);

        Ok(PendingSession {
            request_id,
            connection,
            services,
            leased_cwd,
        })
    }
}
