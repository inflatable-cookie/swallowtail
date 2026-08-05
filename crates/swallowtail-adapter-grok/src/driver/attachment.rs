struct PendingAttachment {
    connection: Arc<AcpConnection>,
    pump_task: Option<Box<dyn JoinedTask>>,
    resource: Option<ResourceLease>,
    credential: Option<CredentialLease>,
    cwd: String,
}

struct GrokSessionInput {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    provider_ref: SessionRef,
    provider_id: String,
    binding: SessionResumeBinding,
    model_options: NegotiatedSessionModelOptions,
}

impl GrokAcpDriver {
    async fn start_attachment(
        &self,
        plan: &PreflightPlan,
        request_id: &RequestId,
        working_resource: &swallowtail_runtime::WorkingResourceRef,
        access_policy: &SessionAccessPolicy,
        services: &HostServices,
    ) -> Result<PendingAttachment, RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "grok-acp:session:{}",
            request_id.as_str()
        ))
        .map_err(|_| malformed())?;
        let credential_service = services
            .credential()
            .cloned()
            .expect("validated credential service");
        let credential_ref = plan
            .credential_reference()
            .expect("validated credential")
            .clone();
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
                "swallowtail.grok.acp.credential_lease_rejected",
                "Grok Build requires a matching delegated credential lease",
            ));
        }
        let working_resource = working_resource.clone();
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
        if let Err(error) =
            validate_session_resource_lease(access_policy, &working_resource, &resource)
        {
            let _ = resource_service.release(resource).await;
            let _ = credential_service.release(credential).await;
            return Err(error);
        }
        let cwd = resource
            .filesystem()
            .expect("validated filesystem lease")
            .as_driver_value()
            .to_owned();
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments([
            "--no-auto-update".to_owned(),
            "agent".to_owned(),
            "stdio".to_owned(),
        ])
        .with_environment([self.ambient_environment().clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = match services
            .process()
            .expect("validated process service")
            .start(scope.clone(), process_request)
            .await
        {
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
        let pump_task = match services
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
            pump_task: Some(pump_task),
            resource: Some(resource),
            credential: Some(credential),
            cwd,
        })
    }
}

impl PendingAttachment {
    fn into_session(
        mut self,
        input: GrokSessionInput,
        services: &HostServices,
    ) -> GrokSessionHandle {
        GrokSessionHandle {
            request_id: input.request_id,
            runtime_id: input.runtime_id,
            provider_ref: input.provider_ref,
            provider_id: input.provider_id,
            binding: input.binding,
            model_options: input.model_options,
            execution_host_id: services.execution_host_id().clone(),
            connection: Arc::clone(&self.connection),
            cancellation: SessionCancellation::new(Arc::clone(&self.connection)),
            pump_task: self.pump_task.take(),
            services: services.clone(),
            resource: self.resource.take(),
            credential: self.credential.take(),
            active: Arc::new(Mutex::new(None)),
        }
    }

    async fn abort(&mut self, services: &HostServices) -> CleanupOutcome {
        self.connection.begin_close().await;
        let task = match self.pump_task.take() {
            Some(task) => match task.join().await {
                Ok(()) => self.connection.cleanup_outcome(),
                Err(_) => cleanup_failure(
                    "swallowtail.grok.acp.task_join_failed",
                    "Grok Build ACP protocol task did not join",
                ),
            },
            None => CleanupOutcome::NotApplicable,
        };
        let resource = release_resource(self.resource.take(), services).await;
        let credential = release_credential(self.credential.take(), services).await;
        merge_cleanup(merge_cleanup(task, resource), credential)
    }
}
