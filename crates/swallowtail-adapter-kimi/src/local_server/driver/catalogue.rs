use super::KimiLocalServerDriver;
use crate::failure::failure;
use crate::local_server::catalogue::decode_catalogue;
use crate::local_server::transport::Request;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::{CredentialMechanism, ExtensionNamespace, ModelCatalogEntry, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};

const DRIVER_ID: &str = "swallowtail.kimi.local-server";

impl ModelCatalogDriver for KimiLocalServerDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "kimi-local-server-catalogue-{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.kimi.local_server.catalogue_scope_invalid",
                    "Kimi local-server catalogue scope is invalid",
                )
            })?;
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
                    "swallowtail.kimi.local_server.catalogue_network_grant_mismatch",
                    "Kimi local-server catalogue network grant does not match its plan",
                ));
            }
            ensure_before_deadline(&request, &services)?;

            let credential_service = services
                .credential()
                .cloned()
                .expect("validated credential service");
            let reference = plan
                .credential_reference()
                .expect("validated credential reference")
                .clone();
            let mut lease = Some(
                credential_service
                    .acquire(scope.clone(), reference.clone(), audience.clone())
                    .await?,
            );
            let bearer = match lease.as_ref().expect("credential was acquired") {
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
                            "swallowtail.kimi.local_server.catalogue_credential_lease_rejected",
                            "Kimi local-server catalogue requires a matching secret bearer lease",
                        ))
                    } else {
                        Err(cleanup_failure())
                    };
                }
            };
            let result = match ensure_before_deadline(&request, &services) {
                Ok(()) => self
                    .transport
                    .request(
                        scope,
                        grant.authorized().as_driver_value().to_owned(),
                        Request::get("/api/v1/models"),
                        Some(bearer),
                        &services,
                        Arc::new(AtomicBool::new(false)),
                    )
                    .await
                    .and_then(|response| decode_catalogue(response.status, &response.body)),
                Err(error) => Err(error),
            };
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

fn validate(
    plan: &PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    if plan.driver_identity().id().as_str() != DRIVER_ID {
        return Err(failure(
            "swallowtail.kimi.local_server.catalogue_plan_driver_mismatch",
            "Kimi local-server catalogue plan is bound to a different driver",
        ));
    }
    let credential = CredentialMechanism::ProviderSpecific(
        ExtensionNamespace::new("kimi-code/local-server-bearer")
            .expect("static credential namespace is valid"),
    );
    if plan.credential_mechanism() != &credential || plan.credential_reference().is_none() {
        return Err(failure(
            "swallowtail.kimi.local_server.catalogue_access_rejected",
            "Kimi local-server catalogue requires its opaque local bearer profile",
        ));
    }
    for (present, code, message) in [
        (
            services.blocking_work().is_some(),
            "swallowtail.kimi.local_server.catalogue_blocking_service_missing",
            "Kimi local-server catalogue requires a blocking-work service",
        ),
        (
            services.time().is_some(),
            "swallowtail.kimi.local_server.catalogue_time_service_missing",
            "Kimi local-server catalogue requires a time service",
        ),
        (
            services.network().is_some(),
            "swallowtail.kimi.local_server.catalogue_network_service_missing",
            "Kimi local-server catalogue requires a network-policy service",
        ),
        (
            services.credential().is_some(),
            "swallowtail.kimi.local_server.catalogue_credential_service_missing",
            "Kimi local-server catalogue requires a credential service",
        ),
    ] {
        if !present {
            return Err(failure(code, message));
        }
    }
    ensure_before_deadline(request, services)
}

fn ensure_before_deadline(
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if request.deadline().is_some_and(|deadline| {
        services.time().expect("validated time service").now() >= deadline.instant()
    }) {
        Err(failure(
            "swallowtail.kimi.local_server.catalogue_deadline_elapsed",
            "Kimi local-server catalogue deadline elapsed before dispatch",
        ))
    } else {
        Ok(())
    }
}

fn cleanup_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.catalogue_cleanup_failed",
        "Kimi local-server catalogue credential cleanup failed",
    )
}
