impl ModelCatalogDriver for AlibabaDeployableModelsDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "alibaba-deployable-models:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| protocol_failure())?;
            let endpoint_ref = EndpointRef::from_instance_target(plan.instance_target_ref());
            let audience = plan.endpoint_audience().clone();
            let grant = services
                .network()
                .expect("validated network service")
                .authorize(scope.clone(), endpoint_ref.clone(), audience.clone())
                .await?;
            if grant.scope() != &scope
                || grant.endpoint() != &endpoint_ref
                || grant.audience() != &audience
            {
                return Err(failure(
                    "swallowtail.alibaba_model_studio.models.network_grant_mismatch",
                    "Alibaba deployable-model grant does not match its immutable plan",
                ));
            }
            let credential_service = services
                .credential()
                .cloned()
                .expect("validated credential service");
            let reference = plan
                .credential_reference()
                .expect("validated credential")
                .clone();
            let mut lease = Some(
                credential_service
                    .acquire(scope.clone(), reference.clone(), audience.clone())
                    .await?,
            );
            let secret = match lease.as_ref().expect("credential was acquired") {
                CredentialLease::Secret(secret)
                    if secret.scope() == &scope
                        && secret.reference() == &reference
                        && secret.audience() == &audience =>
                {
                    secret.expose_secret().to_vec()
                }
                CredentialLease::Secret(_) | CredentialLease::Delegated(_) => {
                    let cleanup = credential_service
                        .release(lease.take().expect("credential was acquired"))
                        .await;
                    return if matches!(
                        cleanup,
                        CleanupOutcome::Clean | CleanupOutcome::NotApplicable
                    ) {
                        Err(failure(
                            "swallowtail.alibaba_model_studio.models.credential_lease_rejected",
                            "Alibaba deployable models require a matching API-key secret lease",
                        ))
                    } else {
                        Err(cleanup_failure())
                    };
                }
            };
            let result = list_sources(
                &self.transport,
                scope,
                grant.authorized().as_driver_value(),
                secret,
                &request,
                &services,
            )
            .await;
            let cleanup = credential_service
                .release(lease.take().expect("credential was acquired"))
                .await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Ok(models),
                (Err(error), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Err(error),
                _ => Err(cleanup_failure()),
            }
        })
    }
}
