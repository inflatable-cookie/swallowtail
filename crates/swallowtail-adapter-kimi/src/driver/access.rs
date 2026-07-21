struct PendingAttachment {
    connection: Arc<AcpConnection>,
    pump_task: Option<Box<dyn JoinedTask>>,
    resource: Option<ResourceLease>,
    credential: Option<CredentialLease>,
    cwd: String,
}

impl KimiAcpDriver {
    async fn start_attachment(
        &self,
        plan: &PreflightPlan,
        request_id: &RequestId,
        working_resource: swallowtail_runtime::WorkingResourceRef,
        services: &HostServices,
    ) -> Result<PendingAttachment, RuntimeFailure> {
        services.require_execution_host(plan.execution_host_id())?;
        let scope = ScopeId::new(format!("kimi-acp:session:{}", request_id.as_str()))
            .map_err(|_| malformed())?;
        let credential_service = services
            .credential()
            .cloned()
            .expect("validated credential service");
        let credential_ref = plan.credential_reference().expect("validated credential").clone();
        let credential = credential_service
            .acquire(
                scope.clone(),
                credential_ref.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if credential.scope() != &scope
            || credential.reference() != &credential_ref
            || credential.audience() != plan.endpoint_audience()
            || !matches!(credential, CredentialLease::Delegated(_))
        {
            let _ = credential_service.release(credential).await;
            return Err(failure(
                "swallowtail.kimi.acp.credential_lease_rejected",
                "Kimi Code requires a matching delegated credential lease",
            ));
        }
        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated working-resource service");
        let resource = match resource_service
            .resolve(
                scope.clone(),
                working_resource.clone(),
                ResourceAccess::ReadWrite,
                ResourceRepresentation::Filesystem,
            )
            .await
        {
            Ok(resource) => resource,
            Err(error) => {
                let _ = credential_service.release(credential).await;
                return Err(error);
            }
        };
        let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite);
        if let Err(error) = validate_session_resource_lease(&policy, &working_resource, &resource) {
            let _ = resource_service.release(resource).await;
            let _ = credential_service.release(credential).await;
            return Err(error);
        }
        let cwd = resource
            .filesystem()
            .expect("validated filesystem lease")
            .as_driver_value()
            .to_owned();
        let process_service = services.process().cloned().expect("validated process service");
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(["acp".to_owned()])
        .with_environment([self.isolated_environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = match process_service.start(scope.clone(), process_request).await {
            Ok(process) => Arc::from(process),
            Err(error) => {
                let _ = resource_service.release(resource).await;
                let _ = credential_service.release(credential).await;
                return Err(error);
            }
        };
        let connection = AcpConnection::new(
            Arc::clone(&process),
            resource.clone(),
            services
                .working_resource_io()
                .cloned()
                .expect("validated resource I/O service"),
        );
        let pump = Arc::clone(&connection);
        let task = match services
            .task()
            .expect("validated task service")
            .spawn(scope, Box::pin(async move { pump.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                let _ = resource_service.release(resource).await;
                let _ = credential_service.release(credential).await;
                return Err(error);
            }
        };
        Ok(PendingAttachment {
            connection,
            pump_task: Some(task),
            resource: Some(resource),
            credential: Some(credential),
            cwd,
        })
    }
}

impl PendingAttachment {
    fn take_session(
        &mut self,
        request_id: RequestId,
        provider_ref: SessionRef,
        provider_id: String,
        binding: SessionResumeBinding,
        services: &HostServices,
    ) -> Result<KimiSessionHandle, RuntimeFailure> {
        let runtime_id = RuntimeSessionId::new(format!("kimi-acp:{}", request_id.as_str()))
            .map_err(|_| malformed())?;
        let active = Arc::new(Mutex::new(None));
        Ok(KimiSessionHandle {
            request_id,
            runtime_id,
            provider_ref,
            provider_id,
            binding,
            execution_host_id: services.execution_host_id().clone(),
            connection: Arc::clone(&self.connection),
            cancellation: SessionCancellation::new(Arc::clone(&self.connection)),
            pump_task: self.pump_task.take(),
            services: services.clone(),
            resource: self.resource.take(),
            credential: self.credential.take(),
            active,
        })
    }

    async fn abort(&mut self, services: &HostServices) -> CleanupOutcome {
        self.connection.begin_close().await;
        let task = match self.pump_task.take() {
            Some(task) => match task.join().await {
                Ok(()) => self.connection.cleanup_outcome(),
                Err(_) => cleanup_failure("task_join_failed", "Kimi ACP protocol task did not join"),
            },
            None => CleanupOutcome::NotApplicable,
        };
        let resource = release_resource(self.resource.take(), services).await;
        let credential = release_credential(self.credential.take(), services).await;
        merge_cleanup(merge_cleanup(task, resource), credential)
    }
}

async fn release_resource(lease: Option<ResourceLease>, services: &HostServices) -> CleanupOutcome {
    match (lease, services.working_resource()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "resource_release_failed",
            "Kimi ACP working-resource service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}

async fn release_credential(
    lease: Option<CredentialLease>,
    services: &HostServices,
) -> CleanupOutcome {
    match (lease, services.credential()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "credential_release_failed",
            "Kimi ACP credential service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}
