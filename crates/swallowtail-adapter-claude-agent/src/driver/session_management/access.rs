use super::super::access::{release_credential, release_resource};
use super::super::session::{cleanup_failure, merge_cleanup};
use super::*;
use crate::failure::{failure, malformed};
use swallowtail_core::ResourceRepresentation;
use swallowtail_runtime::{ProviderSessionManagementAgreement, ScopeId};

pub(super) struct PendingManagement {
    pub(super) connection: Arc<AcpConnection>,
    pump_task: Option<Box<dyn JoinedTask>>,
    resource: Option<ResourceLease>,
    credential: Option<CredentialLease>,
}

pub(super) async fn open_management_connection(
    driver: &ClaudeAgentAcpDriver,
    plan: &PreflightPlan,
    agreement: &ProviderSessionManagementAgreement,
    request_id: &RequestId,
    services: &HostServices,
) -> Result<PendingManagement, RuntimeFailure> {
    let scope = ScopeId::new(format!(
        "claude-agent-acp:management:{}",
        request_id.as_str()
    ))
    .map_err(|_| malformed())?;
    let credential_service = services
        .credential()
        .cloned()
        .expect("validated credential service");
    let mut credential = Some(
        credential_service
            .acquire(
                scope.clone(),
                driver.credential.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?,
    );
    if !matches!(credential.as_ref(), Some(CredentialLease::Secret(_)))
        || credential.as_ref().is_some_and(|lease| {
            lease.scope() != &scope
                || lease.reference() != &driver.credential
                || lease.audience() != plan.endpoint_audience()
        })
    {
        let _ = credential_service
            .release(credential.take().expect("credential was acquired"))
            .await;
        return Err(failure(
            "swallowtail.claude_agent.lifecycle.credential_lease_rejected",
            "Claude Agent deletion requires a matching API-key secret lease",
        ));
    }

    let working_resource = match agreement.binding().working_resource().cloned() {
        Some(resource) => resource,
        None => {
            let _ = credential_service
                .release(credential.take().expect("credential was acquired"))
                .await;
            return Err(failure(
                "swallowtail.claude_agent.lifecycle.resource_missing",
                "Claude Agent deletion requires its bound working resource",
            ));
        }
    };
    let resource_service = services
        .working_resource()
        .cloned()
        .expect("validated resource service");
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
                .release(credential.take().expect("credential was acquired"))
                .await;
            return Err(error);
        }
    };
    let lease = resource.as_ref().expect("resource was acquired");
    if lease.scope() != &scope
        || lease.reference() != &working_resource
        || lease.access() != ResourceAccess::Read
        || lease.representation() != ResourceRepresentation::Filesystem
        || lease.filesystem().is_none()
    {
        let _ = resource_service
            .release(resource.take().expect("resource was acquired"))
            .await;
        let _ = credential_service
            .release(credential.take().expect("credential was acquired"))
            .await;
        return Err(failure(
            "swallowtail.claude_agent.lifecycle.resource_lease_rejected",
            "Claude Agent deletion requires its exact read-only filesystem lease",
        ));
    }

    let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
        plan.instance_target_ref(),
    ))
    .with_environment([driver.environment.clone()])
    .with_working_resource(working_resource);
    let process: Arc<dyn ProcessHandle> = match services
        .process()
        .expect("validated process service")
        .start(scope.clone(), process_request)
        .await
    {
        Ok(process) => Arc::from(process),
        Err(error) => {
            let _ = release_resource(resource.take(), services).await;
            let _ = release_credential(credential.take(), services).await;
            return Err(error);
        }
    };
    let connection = AcpConnection::new(
        Arc::clone(&process),
        resource.as_ref().expect("resource remains held").clone(),
        services
            .working_resource_io()
            .cloned()
            .expect("validated resource I/O service"),
    );
    let pump = Arc::clone(&connection);
    let pump_task = match services
        .task()
        .expect("validated task service")
        .spawn(scope, Box::pin(async move { pump.pump().await }))
    {
        Ok(task) => task,
        Err(error) => {
            let _ = process.force_stop().await;
            let _ = process.wait().await;
            let _ = release_resource(resource.take(), services).await;
            let _ = release_credential(credential.take(), services).await;
            return Err(error);
        }
    };
    Ok(PendingManagement {
        connection,
        pump_task: Some(pump_task),
        resource,
        credential,
    })
}

impl PendingManagement {
    pub(super) async fn close(&mut self, services: &HostServices) -> CleanupOutcome {
        self.connection.begin_close().await;
        let task = match self.pump_task.take() {
            Some(task) => match task.join().await {
                Ok(()) => self.connection.cleanup_outcome(),
                Err(_) => cleanup_failure(
                    "task_join_failed",
                    "Claude Agent lifecycle protocol task did not join cleanly",
                ),
            },
            None => CleanupOutcome::NotApplicable,
        };
        let resource = release_resource(self.resource.take(), services).await;
        let credential = release_credential(self.credential.take(), services).await;
        merge_cleanup(merge_cleanup(task, resource), credential)
    }
}
