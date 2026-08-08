use super::{cleanup_or, control_failure, from_runtime};
use std::sync::Arc;
use swallowtail_core::{ResourceAccess, ResourceRepresentation};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, ProviderSessionCataloguePlan, ProviderSessionHistoryPlan,
    ProviderSessionImportPlan, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage, ProviderSessionReconciliationPlan, ResourceLease,
    ScopeId, WorkingResourceService,
};

pub(super) struct ScopedResource {
    service: Arc<dyn WorkingResourceService>,
    lease: ResourceLease,
    root: String,
}

impl ScopedResource {
    pub(super) async fn resolve(
        plan: &ProviderSessionCataloguePlan,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, ProviderSessionOperationFailure> {
        Self::resolve_reference(
            plan.agreement().scope().working_resource_ref().clone(),
            scope,
            services,
        )
        .await
    }

    pub(super) async fn resolve_import(
        plan: &ProviderSessionImportPlan,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, ProviderSessionOperationFailure> {
        Self::resolve_reference(plan.agreement().working_resource().clone(), scope, services).await
    }

    pub(super) async fn resolve_reconciliation(
        plan: &ProviderSessionReconciliationPlan,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, ProviderSessionOperationFailure> {
        Self::resolve_binding_resource(
            plan.agreement().binding().working_resource().cloned(),
            "Codex thread reconciliation requires a working resource",
            scope,
            services,
        )
        .await
    }

    pub(super) async fn resolve_history(
        plan: &ProviderSessionHistoryPlan,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, ProviderSessionOperationFailure> {
        Self::resolve_binding_resource(
            plan.agreement().binding().working_resource().cloned(),
            "Codex thread history requires a working resource",
            scope,
            services,
        )
        .await
    }

    async fn resolve_binding_resource(
        working_resource: Option<swallowtail_runtime::WorkingResourceRef>,
        missing_message: &'static str,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, ProviderSessionOperationFailure> {
        let working_resource = working_resource.ok_or_else(|| {
            control_failure(
                ProviderSessionOperationFailureStage::BeforeDispatch,
                "swallowtail.codex.thread_catalogue.working_resource_required",
                missing_message,
            )
        })?;
        Self::resolve_reference(working_resource, scope, services).await
    }

    async fn resolve_reference(
        reference: swallowtail_runtime::WorkingResourceRef,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, ProviderSessionOperationFailure> {
        let service = services.working_resource().cloned().ok_or_else(|| {
            control_failure(
                ProviderSessionOperationFailureStage::BeforeDispatch,
                "swallowtail.codex.thread_catalogue.working_resource_service_missing",
                "Codex thread catalogue requires a working-resource service",
            )
        })?;
        let lease = service
            .resolve(
                scope,
                reference.clone(),
                ResourceAccess::Read,
                ResourceRepresentation::Filesystem,
            )
            .await
            .map_err(|error| {
                from_runtime(ProviderSessionOperationFailureStage::BeforeDispatch, error)
            })?;
        let valid = lease.reference() == &reference
            && lease.access() == ResourceAccess::Read
            && lease.representation() == ResourceRepresentation::Filesystem;
        let root = lease
            .filesystem()
            .map(|root| root.as_driver_value().to_owned());
        if !valid || root.is_none() {
            let cleanup = service.release(lease).await;
            return Err(cleanup_or(
                cleanup,
                control_failure(
                    ProviderSessionOperationFailureStage::BeforeDispatch,
                    "swallowtail.codex.thread_catalogue.working_resource_mismatch",
                    "Codex thread catalogue working-resource lease does not match its plan",
                ),
            ));
        }
        Ok(Self {
            service,
            lease,
            root: root.expect("validated filesystem lease has a root"),
        })
    }

    pub(super) fn reference(&self) -> &swallowtail_runtime::WorkingResourceRef {
        self.lease.reference()
    }

    pub(super) fn root(&self) -> &str {
        &self.root
    }

    pub(super) async fn release(self) -> CleanupOutcome {
        self.service.release(self.lease).await
    }
}
