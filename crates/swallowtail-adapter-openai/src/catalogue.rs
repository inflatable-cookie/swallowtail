use crate::failure::failure;
use crate::protocol::Request;
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

const DRIVER_ID: &str = "swallowtail.openai.models";
const MAXIMUM_MODELS: usize = 2_048;
const MAXIMUM_TEXT_BYTES: usize = 256;

#[derive(Clone, Default)]
pub struct OpenAiModelsDriver {
    transport: CurlTransport,
}

impl OpenAiModelsDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[must_use]
pub fn openai_models_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new(crate::INTEGRATION_FAMILY).expect("static family id is valid"),
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
    .with_interface_compatibility(crate::openai_models_facade_claim())
}

impl ModelCatalogDriver for OpenAiModelsDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate(&plan, &request, &services)?;
            let scope = ScopeId::new(format!("openai-models:{}", request.request_id().as_str()))
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
                    "swallowtail.openai.models.network_grant_mismatch",
                    "OpenAI Models network grant does not match its immutable plan",
                ));
            }
            ensure_before_deadline(&request, &services)?;
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
                            "swallowtail.openai.models.credential_lease_rejected",
                            "OpenAI Models requires a matching API-key secret lease",
                        ))
                    } else {
                        Err(cleanup_failure())
                    };
                }
            };
            let result = match ensure_before_deadline(&request, &services) {
                Ok(()) => {
                    let cancelled = Arc::new(AtomicBool::new(false));
                    complete_before_deadline(
                        self.transport.request(
                            scope,
                            grant.authorized().as_driver_value().to_owned(),
                            secret,
                            Request::models(),
                            &services,
                            Arc::clone(&cancelled),
                        ),
                        request.deadline(),
                        &services,
                        cancelled,
                    )
                    .await
                    .and_then(parse_response)
                }
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
            "swallowtail.openai.models.deadline_elapsed",
            "OpenAI Models deadline elapsed before dispatch",
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
            "swallowtail.openai.models.timed_out",
            "OpenAI Models operation timed out",
        ))
    } else {
        result
    }
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
        || plan.endpoint_audience().as_str() != crate::OPENAI_MODELS_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            "swallowtail.openai.models.preflight_rejected",
            "OpenAI Models requires its exact public API-key plan",
        ));
    }
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.openai.models.host_service_missing",
            "OpenAI Models requires blocking-work, time, network, and credential services",
        ));
    }
    ensure_before_deadline(request, services)
}

fn parse_response(
    response: crate::protocol::Response,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    if !(200..300).contains(&response.status) {
        return Err(failure(
            "swallowtail.openai.models.provider_rejected",
            "OpenAI rejected model catalogue discovery",
        ));
    }
    let value: Value = serde_json::from_slice(&response.body).map_err(|_| protocol_failure())?;
    if value.get("object").and_then(Value::as_str) != Some("list") {
        return Err(protocol_failure());
    }
    let models = value
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if models.len() > MAXIMUM_MODELS {
        return Err(protocol_failure());
    }
    let mut identities = BTreeSet::new();
    models
        .iter()
        .map(|model| {
            if model.get("object").and_then(Value::as_str) != Some("model") {
                return Err(protocol_failure());
            }
            let id = bounded_text(model, "id")?;
            if !identities.insert(id.to_owned()) {
                return Err(protocol_failure());
            }
            Ok(ModelCatalogEntry::new(
                ModelId::new(id).map_err(|_| protocol_failure())?,
                ModelMetadata::default(),
            )
            .with_provider_id(
                ProviderId::new("openai").expect("static provider identity is valid"),
            ))
        })
        .collect()
}

fn bounded_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    let value = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(protocol_failure)?;
    if value.is_empty()
        || value.len() > MAXIMUM_TEXT_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        Err(protocol_failure())
    } else {
        Ok(value)
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
            "swallowtail.openai.models.deadline_elapsed",
            "OpenAI Models deadline elapsed before dispatch",
        ))
    } else {
        Ok(())
    }
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.openai.models.protocol_invalid",
        "OpenAI returned an invalid bounded model catalogue",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    failure(
        "swallowtail.openai.models.cleanup_failed",
        "OpenAI Models credential cleanup failed",
    )
}

#[cfg(test)]
mod tests {
    use super::{complete_before_deadline, parse_response};
    use futures_executor::block_on;
    use std::collections::BTreeMap;
    use std::future::poll_fn;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::task::Poll;
    use swallowtail_runtime::{Deadline, MonotonicInstant, RuntimeFailure};
    use swallowtail_testkit::RecordingHostServices;

    #[test]
    fn official_list_shape_projects_only_stable_model_identity() {
        let models = parse_response(crate::protocol::Response {
            status: 200,
            headers: BTreeMap::new(),
            body: br#"{"object":"list","data":[{"id":"model-id-0","object":"model","created":1686935002,"owned_by":"organization-owner"},{"id":"model-id-1","object":"model","created":1686935002,"owned_by":"openai"}]}"#.to_vec(),
        })
        .expect("catalogue parses");
        assert_eq!(
            models
                .iter()
                .map(|model| model.id().as_str())
                .collect::<Vec<_>>(),
            ["model-id-0", "model-id-1"]
        );
        assert!(format!("{models:?}").contains("model-id-0"));
        assert!(!format!("{models:?}").contains("organization-owner"));
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
            "swallowtail.openai.models.timed_out"
        );
        assert!(cancelled.load(Ordering::SeqCst));
    }
}
