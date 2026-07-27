#[path = "driver/handle.rs"]
mod handle;
#[path = "driver/validation.rs"]
mod validation;

use self::handle::build_handle;
use self::validation::*;
use super::access::SessionAccess;
use crate::failure::failure;
use crate::local_server::transport::{Request, session_path};
use std::sync::Arc;
use swallowtail_core::{ConfiguredInstance, PreflightPlan, ProviderSessionBindingOrigin};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparedAccessEvidence, ResumeSessionRequest, RuntimeFailure,
};

impl super::super::KimiLocalServerDriver {
    pub(super) fn configuration(
        &self,
    ) -> Result<&super::KimiLocalServerSessionConfiguration, RuntimeFailure> {
        self.session_configuration.as_ref().ok_or_else(|| {
            failure(
                "swallowtail.kimi.local_server.session_configuration_missing",
                "Kimi local-server interactive driver requires exact session configuration",
            )
        })
    }

    pub(super) async fn open_bound_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
        management_instance: ConfiguredInstance,
        access_evidence: PreparedAccessEvidence,
    ) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
        self.open_inner(
            plan,
            request,
            services,
            Some((management_instance, access_evidence)),
        )
        .await
    }

    pub(super) async fn resume_bound_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
        management_instance: ConfiguredInstance,
        access_evidence: PreparedAccessEvidence,
    ) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
        self.resume_inner(
            plan,
            request,
            services,
            Some((management_instance, access_evidence)),
        )
        .await
    }

    async fn open_inner(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
        management: Option<(ConfiguredInstance, PreparedAccessEvidence)>,
    ) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
        validate_open(self, &plan, &request, &services)?;
        let scope = scope("session", request.request_id())?;
        let mut access = SessionAccess::acquire(
            &plan,
            scope.clone(),
            &services,
            request.working_resource().expect("validated resource"),
            request.access_policy(),
        )
        .await?;
        let body = serde_json::to_vec(&serde_json::json!({"metadata":{"cwd":access.directory}}))
            .map_err(|_| protocol_failure())?;
        let response = before_deadline(
            self.transport.request(
                scope,
                access.endpoint.clone(),
                Request::post_json("/api/v1/sessions", body),
                Some(access.secret.copy()),
                &services,
                Arc::new(std::sync::atomic::AtomicBool::new(false)),
            ),
            request.deadline(),
            &services,
        )
        .await;
        let record = match response.and_then(require_interactive_session) {
            Ok(record) if !record.archived && !record.busy => record,
            Ok(_) => {
                let _ = access.release(&services).await;
                return Err(protocol_failure());
            }
            Err(error) => {
                let _ = access.release(&services).await;
                return Err(error);
            }
        };
        build_handle(
            self,
            &plan,
            request.request_id().clone(),
            request
                .working_resource()
                .expect("validated resource")
                .clone(),
            request.options().clone(),
            request.access_policy().clone(),
            record,
            access,
            services,
            management,
            ProviderSessionBindingOrigin::Created,
        )
    }

    async fn resume_inner(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
        management: Option<(ConfiguredInstance, PreparedAccessEvidence)>,
    ) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
        validate_resume(self, &plan, &request, &services)?;
        let scope = scope("resume", request.request_id())?;
        let mut access = SessionAccess::acquire(
            &plan,
            scope.clone(),
            &services,
            request.working_resource(),
            request.access_policy(),
        )
        .await?;
        let path = session_path(request.provider_session_ref().as_provider_value())?;
        let response = before_deadline(
            self.transport.request(
                scope,
                access.endpoint.clone(),
                Request::get(path),
                Some(access.secret.copy()),
                &services,
                Arc::new(std::sync::atomic::AtomicBool::new(false)),
            ),
            request.deadline(),
            &services,
        )
        .await;
        let record = match response.and_then(require_interactive_session) {
            Ok(record)
                if record.id == request.provider_session_ref().as_provider_value()
                    && record.working_directory == access.directory
                    && !record.archived
                    && !record.busy =>
            {
                record
            }
            Ok(_) => {
                let _ = access.release(&services).await;
                return Err(binding_failure());
            }
            Err(error) => {
                let _ = access.release(&services).await;
                return Err(error);
            }
        };
        build_handle(
            self,
            &plan,
            request.request_id().clone(),
            request.working_resource().clone(),
            request.options().clone(),
            request.access_policy().clone(),
            record,
            access,
            services,
            management,
            ProviderSessionBindingOrigin::Resumed,
        )
    }
}

impl InteractiveSessionDriver for super::super::KimiLocalServerDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move { self.open_inner(plan, request, services, None).await })
    }

    fn resume_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move { self.resume_inner(plan, request, services, None).await })
    }
}
