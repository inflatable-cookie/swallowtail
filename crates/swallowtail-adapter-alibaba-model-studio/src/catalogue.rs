use crate::failure::failure;
use crate::protocol::WireRequest;
use crate::transport::CurlTransport;
use serde_json::Value;
use std::collections::BTreeSet;
use std::future::Future;
use std::future::poll_fn;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, IntegrationFamilyId, ModelCatalogEntry, ModelId,
    ModelMetadata, OperationShape, PreflightPlan, ProviderId, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};

const DRIVER_ID: &str = "swallowtail.alibaba-model-studio.deployable-models";
const MAXIMUM_MODELS: usize = 4_096;
const MAXIMUM_PAGES_PER_SOURCE: u32 = 32;
const MAXIMUM_TEXT_BYTES: usize = 512;

#[derive(Clone, Default)]
pub struct AlibabaDeployableModelsDriver {
    transport: CurlTransport,
}

impl AlibabaDeployableModelsDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[must_use]
pub fn alibaba_deployable_models_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("alibaba-model-studio").expect("static family id is valid"),
        TransportFamilyId::new("https-json-models").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::ModelCatalog])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
        ],
    )
    .with_interface_compatibility(crate::alibaba_deployable_models_facade_claim())
}

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

async fn list_sources(
    transport: &CurlTransport,
    scope: ScopeId,
    endpoint: &str,
    mut secret: Vec<u8>,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let mut models = Vec::new();
    let mut identities = BTreeSet::new();
    for source in ["base", "custom"] {
        let mut page = 1_u32;
        loop {
            ensure_before_deadline(request, services)?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let response = complete_before_deadline(
                transport.request(
                    ScopeId::new(format!("{}:{source}:{page}", scope.as_str()))
                        .map_err(|_| protocol_failure())?,
                    endpoint.to_owned(),
                    secret.clone(),
                    WireRequest::deployable_models(page, source),
                    services,
                    Arc::clone(&cancelled),
                ),
                request.deadline(),
                services,
                cancelled,
            )
            .await?;
            let parsed = parse_page(&response.body, page)?;
            for model in parsed.models {
                if !identities.insert(model.id().as_str().to_owned())
                    || models.len() >= MAXIMUM_MODELS
                {
                    secret.fill(0);
                    return Err(protocol_failure());
                }
                models.push(model);
            }
            if parsed.last_page {
                break;
            }
            page = page.checked_add(1).ok_or_else(protocol_failure)?;
            if page > MAXIMUM_PAGES_PER_SOURCE {
                secret.fill(0);
                return Err(protocol_failure());
            }
        }
    }
    secret.fill(0);
    Ok(models)
}

async fn complete_before_deadline<T, F>(
    work: F,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<T, RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let Some(deadline) = deadline else {
        return work.await;
    };
    let time = services.time().expect("validated time service");
    if time.now() >= deadline.instant() {
        return Err(failure(
            "swallowtail.alibaba_model_studio.models.deadline_elapsed",
            "Alibaba deployable-model deadline elapsed before dispatch",
        ));
    }
    let mut work = Box::pin(work);
    let mut wait = time.wait_until(deadline);
    let mut timed_out = false;
    let result = poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if !timed_out && wait.as_mut().poll(context).is_ready() {
            timed_out = true;
            cancelled.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    if timed_out {
        Err(failure(
            "swallowtail.alibaba_model_studio.models.timed_out",
            "Alibaba deployable-model operation timed out",
        ))
    } else {
        result
    }
}

struct ParsedPage {
    models: Vec<ModelCatalogEntry>,
    last_page: bool,
}

fn parse_page(bytes: &[u8], expected_page: u32) -> Result<ParsedPage, RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| protocol_failure())?;
    let output = value.get("output").ok_or_else(protocol_failure)?;
    let page = output
        .get("page_no")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(protocol_failure)?;
    let page_size = output
        .get("page_size")
        .and_then(Value::as_u64)
        .ok_or_else(protocol_failure)?;
    let total = output
        .get("total")
        .and_then(Value::as_u64)
        .ok_or_else(protocol_failure)?;
    if page != expected_page || page_size == 0 || page_size > 100 {
        return Err(protocol_failure());
    }
    let models = output
        .get("models")
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if models.len() > 100 {
        return Err(protocol_failure());
    }
    let models = models
        .iter()
        .map(|model| {
            let name = model
                .get("model_name")
                .and_then(Value::as_str)
                .ok_or_else(protocol_failure)?;
            bounded(name)?;
            Ok(ModelCatalogEntry::new(
                ModelId::new(name).map_err(|_| protocol_failure())?,
                ModelMetadata::default(),
            )
            .with_provider_id(
                ProviderId::new("alibaba-cloud").expect("static provider identity is valid"),
            ))
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    let seen = u64::from(page.saturating_sub(1))
        .saturating_mul(page_size)
        .saturating_add(models.len() as u64);
    Ok(ParsedPage {
        models,
        last_page: seen >= total,
    })
}

fn validate(
    plan: &PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    if plan.driver_identity().id().as_str() != DRIVER_ID
        || plan.credential_mechanism() != &CredentialMechanism::ApiKey
        || plan.credential_reference().is_none()
        || plan.endpoint_audience().as_str() != crate::ALIBABA_DEPLOYABLE_MODELS_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.models.preflight_rejected",
            "Alibaba deployable models require their exact international API-key plan",
        ));
    }
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.models.host_service_missing",
            "Alibaba deployable models require blocking-work, time, network, and credential services",
        ));
    }
    ensure_before_deadline(request, services)
}

fn bounded(value: &str) -> Result<(), RuntimeFailure> {
    if value.is_empty()
        || value.len() > MAXIMUM_TEXT_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        Err(protocol_failure())
    } else {
        Ok(())
    }
}

fn ensure_before_deadline(
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if request.deadline().is_some_and(|deadline| {
        services.time().expect("validated time service").now() >= deadline.instant()
    }) {
        Err(failure(
            "swallowtail.alibaba_model_studio.models.deadline_elapsed",
            "Alibaba deployable-model deadline elapsed before dispatch",
        ))
    } else {
        Ok(())
    }
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.models.protocol_invalid",
        "Alibaba returned an invalid bounded deployable-model catalogue",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.models.cleanup_failed",
        "Alibaba deployable-model credential cleanup failed",
    )
}

#[cfg(test)]
mod tests {
    use super::{complete_before_deadline, parse_page};
    use futures_executor::block_on;
    use std::future::poll_fn;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::task::Poll;
    use swallowtail_runtime::{Deadline, MonotonicInstant, RuntimeFailure};
    use swallowtail_testkit::RecordingHostServices;

    #[test]
    fn official_deployable_model_shape_preserves_identity_and_pagination() {
        let page = parse_page(
            br#"{"request_id":"fixture","output":{"page_no":1,"page_size":100,"total":1,"models":[{"model_name":"qwen3-8b","plans":[{"plan":"mu"},{"plan":"lora"}]}]}}"#,
            1,
        )
        .expect("page parses");
        assert_eq!(page.models[0].id().as_str(), "qwen3-8b");
        assert!(page.last_page);
    }

    #[test]
    fn in_flight_deadline_requests_transport_stop_before_returning() {
        let host = RecordingHostServices::default();
        let cancelled = Arc::new(AtomicBool::new(false));
        let work_cancelled = Arc::clone(&cancelled);
        let error = block_on(complete_before_deadline(
            poll_fn(move |_| {
                if work_cancelled.load(Ordering::SeqCst) {
                    Poll::Ready(Ok::<(), RuntimeFailure>(()))
                } else {
                    Poll::Pending
                }
            }),
            Some(Deadline::at(MonotonicInstant::from_ticks(18))),
            host.services(),
            Arc::clone(&cancelled),
        ))
        .expect_err("deadline wins");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.alibaba_model_studio.models.timed_out"
        );
        assert!(cancelled.load(Ordering::SeqCst));
    }
}
