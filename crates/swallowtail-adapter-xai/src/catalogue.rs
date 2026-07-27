use crate::failure::failure;
use curl::easy::{Easy, List, WriteError};
use futures_channel::oneshot;
use serde_json::Value;
use std::collections::BTreeSet;
use std::future::Future;
use std::future::poll_fn;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use std::time::Duration;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CatalogObservation, CredentialMechanism,
    DriverDescriptor, DriverRole, ExecutionLayer, HostServiceKind, IntegrationFamilyId,
    ModelCatalogEntry, ModelCatalogObservations, ModelId, ModelMetadata, ModelModality,
    OperationShape, PreflightPlan, ProviderCatalogValue, ProviderId, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};
use url::Url;

const DRIVER_ID: &str = "swallowtail.xai.models";
const MAXIMUM_MODELS: usize = 2_048;
const MAXIMUM_BODY_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_TEXT_BYTES: usize = 512;

#[derive(Clone, Default)]
pub struct XaiModelsDriver;

impl XaiModelsDriver {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[must_use]
pub fn xai_models_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("xai").expect("static family id is valid"),
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
    .with_interface_compatibility(crate::xai_models_facade_claim())
}

impl ModelCatalogDriver for XaiModelsDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate(&plan, &request, &services)?;
            let scope = ScopeId::new(format!("xai-models:{}", request.request_id().as_str()))
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
                    "swallowtail.xai.models.network_grant_mismatch",
                    "xAI Models network grant does not match its immutable plan",
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
                            "swallowtail.xai.models.credential_lease_rejected",
                            "xAI Models requires a matching API-key secret lease",
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
                        http_get(
                            scope,
                            grant.authorized().as_driver_value(),
                            secret,
                            Arc::clone(&cancelled),
                            &services,
                        ),
                        request.deadline(),
                        &services,
                        cancelled,
                    )
                    .await
                    .and_then(|body| parse_response(&body))
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
            "swallowtail.xai.models.deadline_elapsed",
            "xAI Models deadline elapsed before dispatch",
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
            "swallowtail.xai.models.timed_out",
            "xAI Models operation timed out",
        ))
    } else {
        result
    }
}

fn parse_response(bytes: &[u8]) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| protocol_failure())?;
    let models = value
        .get("models")
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if models.len() > MAXIMUM_MODELS {
        return Err(protocol_failure());
    }
    let source = IntegrationFamilyId::new("xai").expect("static family id is valid");
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
            let mut observations = ModelCatalogObservations::new(source.clone());
            observations = observations
                .with_input_modalities(parse_modalities(model, "input_modalities", &source)?)
                .with_output_modalities(parse_modalities(model, "output_modalities", &source)?);
            Ok(ModelCatalogEntry::new(
                ModelId::new(id).map_err(|_| protocol_failure())?,
                ModelMetadata::default().with_catalog_observations(observations),
            )
            .with_provider_id(ProviderId::new("xai").expect("static provider identity is valid")))
        })
        .collect()
}

fn parse_modalities(
    value: &Value,
    field: &str,
    source: &IntegrationFamilyId,
) -> Result<Vec<CatalogObservation<ModelModality>>, RuntimeFailure> {
    let values = value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if values.len() > 16 {
        return Err(protocol_failure());
    }
    values
        .iter()
        .map(|value| {
            let value = value.as_str().ok_or_else(protocol_failure)?;
            bounded(value)?;
            match value {
                "text" => Ok(CatalogObservation::Known(ModelModality::Text)),
                "image" => Ok(CatalogObservation::Known(ModelModality::Image)),
                "embedding" => Ok(CatalogObservation::Known(ModelModality::Embedding)),
                other => ProviderCatalogValue::new(source.clone(), other)
                    .map(CatalogObservation::ProviderDefined)
                    .map_err(|_| protocol_failure()),
            }
        })
        .collect()
}

async fn http_get(
    scope: ScopeId,
    endpoint: &str,
    secret: Vec<u8>,
    cancelled: Arc<AtomicBool>,
    services: &HostServices,
) -> Result<Vec<u8>, RuntimeFailure> {
    let blocking = services
        .blocking_work()
        .cloned()
        .expect("validated blocking-work service");
    let endpoint = endpoint.to_owned();
    let (sender, receiver) = oneshot::channel();
    blocking
        .run(
            scope,
            Box::new(move || {
                let result = perform_get(&endpoint, secret, cancelled);
                let _ = sender.send(result);
                Ok(())
            }),
        )
        .await?;
    receiver.await.map_err(|_| {
        failure(
            "swallowtail.xai.models.blocking_result_missing",
            "xAI Models blocking HTTP work returned no result",
        )
    })?
}

fn perform_get(
    endpoint: &str,
    mut secret: Vec<u8>,
    cancelled: Arc<AtomicBool>,
) -> Result<Vec<u8>, RuntimeFailure> {
    let mut url = Url::parse(endpoint).map_err(|_| protocol_failure())?;
    if url.scheme() != "https"
        || url.host_str() != Some("api.x.ai")
        || !matches!(url.path(), "" | "/")
    {
        return Err(protocol_failure());
    }
    url.set_path("/v1/language-models");
    let key = std::str::from_utf8(&secret).map_err(|_| protocol_failure())?;
    if key.is_empty() || key.bytes().any(|byte| matches!(byte, b'\r' | b'\n')) {
        secret.fill(0);
        return Err(protocol_failure());
    }
    let mut headers = List::new();
    headers
        .append(&format!("authorization: Bearer {key}"))
        .map_err(|_| transport_failure())?;
    headers
        .append("accept: application/json")
        .map_err(|_| transport_failure())?;
    secret.fill(0);
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(|_| transport_failure())?;
    easy.follow_location(false)
        .map_err(|_| transport_failure())?;
    easy.proxy("").map_err(|_| transport_failure())?;
    easy.timeout(Duration::from_secs(10))
        .map_err(|_| transport_failure())?;
    easy.progress(true).map_err(|_| transport_failure())?;
    let progress = Arc::clone(&cancelled);
    easy.progress_function(move |_, _, _, _| !progress.load(Ordering::SeqCst))
        .map_err(|_| transport_failure())?;
    easy.http_headers(headers)
        .map_err(|_| transport_failure())?;
    let mut body = Vec::new();
    let overflow = Arc::new(AtomicBool::new(false));
    {
        let callback_overflow = Arc::clone(&overflow);
        let mut transfer = easy.transfer();
        transfer
            .write_function(|chunk| {
                if body.len().saturating_add(chunk.len()) > MAXIMUM_BODY_BYTES {
                    callback_overflow.store(true, Ordering::SeqCst);
                    return Err(WriteError::Pause);
                }
                body.extend_from_slice(chunk);
                Ok(chunk.len())
            })
            .map_err(|_| transport_failure())?;
        transfer.perform().map_err(|_| transport_failure())?;
    }
    if overflow.load(Ordering::SeqCst) {
        return Err(protocol_failure());
    }
    let status = easy.response_code().map_err(|_| transport_failure())?;
    if !(200..300).contains(&status) {
        return Err(failure(
            "swallowtail.xai.models.provider_rejected",
            "xAI rejected model catalogue discovery",
        ));
    }
    Ok(body)
}

fn validate(
    plan: &swallowtail_core::PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    if plan.driver_identity().id().as_str() != DRIVER_ID
        || plan.credential_mechanism() != &CredentialMechanism::ApiKey
        || plan.credential_reference().is_none()
        || plan.endpoint_audience().as_str() != crate::XAI_MODELS_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            "swallowtail.xai.models.preflight_rejected",
            "xAI Models requires its exact public API-key plan",
        ));
    }
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.xai.models.host_service_missing",
            "xAI Models requires blocking-work, time, network, and credential services",
        ));
    }
    ensure_before_deadline(request, services)
}

fn bounded_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    let value = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(protocol_failure)?;
    bounded(value)?;
    Ok(value)
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
            "swallowtail.xai.models.deadline_elapsed",
            "xAI Models deadline elapsed before dispatch",
        ))
    } else {
        Ok(())
    }
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.models.protocol_invalid",
        "xAI returned an invalid bounded language-model catalogue",
    )
}

fn transport_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.models.transport_failed",
        "xAI Models HTTP transport failed",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.models.cleanup_failed",
        "xAI Models credential cleanup failed",
    )
}

#[cfg(test)]
mod tests {
    use super::{complete_before_deadline, parse_response};
    use futures_executor::block_on;
    use std::future::poll_fn;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::task::Poll;
    use swallowtail_core::{CatalogObservation, ModelModality};
    use swallowtail_runtime::{Deadline, MonotonicInstant, RuntimeFailure};
    use swallowtail_testkit::RecordingHostServices;

    #[test]
    fn official_language_model_shape_preserves_identity_and_modalities() {
        let models = parse_response(br#"{"models":[{"id":"latest","fingerprint":"fp_fixture","created":1776556800,"object":"model","owned_by":"xai","version":"1.0","input_modalities":["text","image"],"output_modalities":["text"],"aliases":["grok-latest"]}]}"#)
            .expect("catalogue parses");
        assert_eq!(models[0].id().as_str(), "latest");
        let observations = models[0]
            .metadata()
            .catalog_observations()
            .expect("modalities are retained");
        assert!(
            observations
                .input_modalities()
                .expect("input modalities are retained")
                .contains(&CatalogObservation::Known(ModelModality::Image))
        );
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
            "swallowtail.xai.models.timed_out"
        );
        assert!(cancelled.load(Ordering::SeqCst));
    }
}
