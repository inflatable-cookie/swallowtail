pub(crate) enum ClinePlanAcknowledgement {
    NotRequested,
    Effective(String),
}

pub(crate) enum ClineModelObservation {
    Absent,
    Exact(swallowtail_runtime::NegotiatedSessionModelOptions),
    Invalid(RuntimeFailure),
}

impl ClineModelObservation {
    pub(crate) const fn exact(
        &self,
    ) -> Option<&swallowtail_runtime::NegotiatedSessionModelOptions> {
        match self {
            Self::Exact(options) => Some(options),
            Self::Absent | Self::Invalid(_) => None,
        }
    }
}

pub(crate) struct ClineOpenObservation {
    pub(crate) plan_acknowledgement: ClinePlanAcknowledgement,
    pub(crate) model: ClineModelObservation,
}

pub(crate) struct ClineOpenRejection {
    failure: RuntimeFailure,
    rejected_plan: Option<String>,
}

impl ClineOpenRejection {
    pub(crate) const fn runtime(failure: RuntimeFailure) -> Self {
        Self {
            failure,
            rejected_plan: None,
        }
    }

    pub(crate) const fn rejected_plan(failure: RuntimeFailure, value: String) -> Self {
        Self {
            failure,
            rejected_plan: Some(value),
        }
    }

    pub(crate) fn rejected_plan_value(&self) -> Option<&str> {
        self.rejected_plan.as_deref()
    }

    pub(crate) fn into_failure(self) -> RuntimeFailure {
        self.failure
    }
}

impl From<RuntimeFailure> for ClineOpenRejection {
    fn from(failure: RuntimeFailure) -> Self {
        Self::runtime(failure)
    }
}

impl ClineAcpDriver {
    pub(crate) async fn open_session_lifecycle(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> Result<
        (Box<dyn InteractiveSessionHandle>, ClineOpenObservation),
        ClineOpenRejection,
    > {
        let selected = self.validate_plan(&plan)?;
        services.require_execution_host(plan.execution_host_id())?;
        validate_open(&plan, &request, &services)?;
        let scope = ScopeId::new(format!(
            "cline-acp:session:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| malformed())?;
        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated working-resource service");
        let resource_access = session_resource_access(&plan)?;
        let resource = resource_service
            .resolve(
                scope.clone(),
                request
                    .working_resource()
                    .expect("validated resource")
                    .clone(),
                resource_access,
                ResourceRepresentation::Filesystem,
            )
            .await?;
        if let Err(error) = validate_session_resource_lease(
            request.access_policy(),
            request.working_resource().expect("validated resource"),
            &resource,
        ) {
            let _ = resource_service.release(resource).await;
            return Err(error.into());
        }
        let result = self
            .start_session(&plan, &request, &services, scope, resource, selected)
            .await;
        match result {
            Ok((session, observation)) => Ok((
                Box::new(session) as Box<dyn InteractiveSessionHandle>,
                observation,
            )),
            Err(pair) => {
                let (error, resource) = *pair;
                let _ = resource_service.release(resource).await;
                Err(error)
            }
        }
    }
}
