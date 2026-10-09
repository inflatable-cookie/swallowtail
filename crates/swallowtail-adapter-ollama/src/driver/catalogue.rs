impl OllamaNativeAttachedDriver {
    async fn observe_catalogue(
        &self,
        scope: ScopeId,
        endpoint: &str,
        plan: &PreflightPlan,
        deadline: Option<Deadline>,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<ObservedCatalogue, RuntimeFailure> {
        let expected = plan
            .requirements()
            .attached_runtime()
            .expect("validated attached requirements");
        let version_response = complete_before_deadline(
            self.transport.request(
                scope.clone(),
                endpoint.to_owned(),
                Request::version(),
                services,
                Arc::clone(&cancelled),
            ),
            deadline,
            services,
            Arc::clone(&cancelled),
        )
        .await?;
        let version = parse_version(&version_response)?;
        if &version != expected.runtime_version() {
            return Err(failure(
                "swallowtail.ollama.version_drift",
                "Ollama runtime version changed after preflight",
            ));
        }
        let observed_at = services.time().expect("validated time").catalog_now()?;
        let binding = ObservationBinding {
            instance_id: plan.instance_id().clone(),
            execution_host_id: plan.execution_host_id().clone(),
            runtime_version: version,
            observed_at,
        };
        let installed_response = complete_before_deadline(
            self.transport.request(
                scope.clone(),
                endpoint.to_owned(),
                Request::installed_models(),
                services,
                Arc::clone(&cancelled),
            ),
            deadline,
            services,
            Arc::clone(&cancelled),
        )
        .await?;
        let installed_parse = parse_inventory_rows(
            &installed_response,
            AttachedModelObservationScope::InstalledInventory,
            &binding,
        )?;
        let selected = bind_selected_inventory(
            &installed_parse,
            expected.model_tag(),
            expected.manifest_digest(),
        )?;
        let selected_runner = selected.runner;
        let installed = installed_parse
            .mapped
            .into_iter()
            .map(|row| row.observation)
            .collect::<Vec<_>>();
        let running_response = complete_before_deadline(
            self.transport.request(
                scope.clone(),
                endpoint.to_owned(),
                Request::running_models(),
                services,
                Arc::clone(&cancelled),
            ),
            deadline,
            services,
            Arc::clone(&cancelled),
        )
        .await?;
        let running = parse_inventory_rows(
            &running_response,
            AttachedModelObservationScope::RunningInventory,
            &binding,
        )?
        .mapped
        .into_iter()
        .map(|row| row.observation)
        .collect::<Vec<_>>();
        if running.iter().any(|candidate| {
            !installed.iter().any(|installed| {
                installed.model_tag() == candidate.model_tag()
                    && installed.manifest_digest() == candidate.manifest_digest()
            })
        }) {
            return Err(failure(
                "swallowtail.ollama.inventory_drift",
                "Ollama running inventory did not match installed inventory",
            ));
        }
        let detail_response = complete_before_deadline(
            self.transport.request(
                scope,
                endpoint.to_owned(),
                Request::show_with_runner(expected.model_tag().as_str(), selected_runner)?,
                services,
                Arc::clone(&cancelled),
            ),
            deadline,
            services,
            cancelled,
        )
        .await?;
        let detail = parse_model_detail(
            &detail_response,
            &binding,
            expected.model_tag().clone(),
            expected.manifest_digest().clone(),
        )?;
        Ok(ObservedCatalogue {
            installed,
            running,
            detail: detail.observation().clone(),
            selected_runner,
        })
    }
}

struct ObservedCatalogue {
    installed: Vec<AttachedModelObservation>,
    running: Vec<AttachedModelObservation>,
    detail: AttachedModelObservation,
    selected_runner: Option<OllamaNativeRunner>,
}

impl ObservedCatalogue {
    fn into_entries(self) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
        let mut observations: BTreeMap<String, Vec<AttachedModelObservation>> = BTreeMap::new();
        for observation in self.installed.into_iter().chain(self.running) {
            observations
                .entry(observation.model_tag().as_str().to_owned())
                .or_default()
                .push(observation);
        }
        observations
            .entry(self.detail.model_tag().as_str().to_owned())
            .or_default()
            .push(self.detail);
        observations
            .into_iter()
            .map(|(tag, observations)| {
                let model = ModelId::new(tag).map_err(|_| {
                    failure(
                        "swallowtail.ollama.model_identity_invalid",
                        "Ollama model tag could not form a catalogue identity",
                    )
                })?;
                Ok(ModelCatalogEntry::new(
                    model,
                    ModelMetadata::default().with_attached_model_observations(observations),
                ))
            })
            .collect()
    }
}

impl ModelCatalogDriver for OllamaNativeAttachedDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, false)?;
            let scope = operation_scope("catalog", request.request_id().as_str())?;
            let endpoint = authorize_endpoint(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            self.observe_catalogue(
                scope,
                &endpoint,
                &plan,
                request.deadline(),
                &services,
                cancelled,
            )
            .await?
            .into_entries()
        })
    }
}
