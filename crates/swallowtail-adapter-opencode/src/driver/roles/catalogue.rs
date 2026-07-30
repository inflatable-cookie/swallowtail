impl ModelCatalogDriver for OpenCodeHttpDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            let version = Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, false)?;
            let scope = scope("catalog", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services, None).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let result = async {
                let health = complete_before_deadline(
                    self.transport.request(
                        scope.clone(),
                        access.endpoint.clone(),
                        Request::get("/global/health"),
                        &services,
                        Arc::clone(&cancelled),
                    ),
                    request.deadline(),
                    &services,
                    Arc::clone(&cancelled),
                    "swallowtail.opencode.catalog_timed_out",
                    "OpenCode model discovery timed out",
                )
                .await?;
                require_health_matches(&health, &version)?;
                let response = complete_before_deadline(
                    self.transport.request(
                        scope,
                        access.endpoint.clone(),
                        Request::get("/provider"),
                        &services,
                        Arc::clone(&cancelled),
                    ),
                    request.deadline(),
                    &services,
                    cancelled,
                    "swallowtail.opencode.catalog_timed_out",
                    "OpenCode model discovery timed out",
                )
                .await?;
                parse_catalog(&response)
            }
            .await;
            let cleanup = access.release(&services).await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Ok(models),
                (Err(error), _) => Err(error),
                (Ok(_), _) => Err(failure(
                    "swallowtail.opencode.catalog_cleanup_failed",
                    "OpenCode catalogue credential cleanup failed",
                )),
            }
        })
    }
}

