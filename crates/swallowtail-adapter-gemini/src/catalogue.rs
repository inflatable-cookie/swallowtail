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
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, IntegrationFamilyId, ModelCatalogEntry,
    ModelCatalogObservations, ModelId, ModelMetadata, ModelTokenLimits, OperationShape,
    PreflightPlan, ProviderId, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, RuntimeFailure, ScopeId,
};
use url::Url;

const DRIVER_ID: &str = "swallowtail.gemini.models";
const MAXIMUM_MODELS: usize = 4_096;
const MAXIMUM_PAGES: usize = 32;
const MAXIMUM_BODY_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_TEXT_BYTES: usize = 512;

#[derive(Clone, Default)]
pub struct GeminiModelsDriver;

impl GeminiModelsDriver {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[must_use]
pub fn gemini_models_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("gemini").expect("static family id is valid"),
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
    .with_interface_compatibility(crate::gemini_models_facade_claim())
}

impl ModelCatalogDriver for GeminiModelsDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate(&plan, &request, &services)?;
            let scope = ScopeId::new(format!("gemini-models:{}", request.request_id().as_str()))
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
                    "swallowtail.gemini.models.network_grant_mismatch",
                    "Gemini Models network grant does not match its immutable plan",
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
                            "swallowtail.gemini.models.credential_lease_rejected",
                            "Gemini Models requires a matching API-key secret lease",
                        ))
                    } else {
                        Err(cleanup_failure())
                    };
                }
            };
            let result = list_pages(
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

async fn list_pages(
    scope: ScopeId,
    endpoint: &str,
    mut secret: Vec<u8>,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let mut models = Vec::new();
    let mut identities = BTreeSet::new();
    let mut page_token = None;
    for page in 0..MAXIMUM_PAGES {
        ensure_before_deadline(request, services)?;
        let cancelled = Arc::new(AtomicBool::new(false));
        let response = complete_before_deadline(
            http_get(
                ScopeId::new(format!("{}:page-{page}", scope.as_str()))
                    .map_err(|_| protocol_failure())?,
                endpoint,
                page_token.as_deref(),
                secret.clone(),
                Arc::clone(&cancelled),
                services,
            ),
            request.deadline(),
            services,
            cancelled,
        )
        .await?;
        let (page_models, next) = parse_page(&response)?;
        for model in page_models {
            if !identities.insert(model.id().as_str().to_owned()) || models.len() >= MAXIMUM_MODELS
            {
                secret.fill(0);
                return Err(protocol_failure());
            }
            models.push(model);
        }
        match next {
            Some(next) => page_token = Some(next),
            None => {
                secret.fill(0);
                return Ok(models);
            }
        }
    }
    secret.fill(0);
    Err(protocol_failure())
}

fn parse_page(bytes: &[u8]) -> Result<(Vec<ModelCatalogEntry>, Option<String>), RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| protocol_failure())?;
    let models = value
        .get("models")
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if models.len() > 1_000 {
        return Err(protocol_failure());
    }
    let models = models
        .iter()
        .map(|model| {
            let id = bounded_text(model, "baseModelId")?;
            let mut metadata = match optional_bounded_text(model, "displayName")? {
                Some(display) => {
                    ModelMetadata::with_display_name(display).map_err(|_| protocol_failure())?
                }
                None => ModelMetadata::default(),
            };
            if let Some(description) = optional_bounded_text(model, "description")? {
                metadata = metadata
                    .with_description(description)
                    .map_err(|_| protocol_failure())?;
            }
            let input = optional_positive_u64(model, "inputTokenLimit")?;
            let output = optional_positive_u64(model, "outputTokenLimit")?;
            if input.is_some() || output.is_some() {
                metadata = metadata.with_token_limits(ModelTokenLimits::new(input, output));
            }
            if let Some(reasoning) = optional_bool(model, "thinking")? {
                metadata = metadata.with_catalog_observations(
                    ModelCatalogObservations::new(
                        IntegrationFamilyId::new("gemini").expect("static family id is valid"),
                    )
                    .with_reasoning_supported(reasoning),
                );
            }
            Ok(
                ModelCatalogEntry::new(ModelId::new(id).map_err(|_| protocol_failure())?, metadata)
                    .with_provider_id(
                        ProviderId::new("google").expect("static provider id is valid"),
                    ),
            )
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    let next = match value.get("nextPageToken") {
        None | Some(Value::Null) => None,
        Some(Value::String(next)) => {
            bounded(next)?;
            Some(next.clone())
        }
        Some(_) => return Err(protocol_failure()),
    };
    Ok((models, next))
}

async fn http_get(
    scope: ScopeId,
    endpoint: &str,
    page_token: Option<&str>,
    secret: Vec<u8>,
    cancelled: Arc<AtomicBool>,
    services: &HostServices,
) -> Result<Vec<u8>, RuntimeFailure> {
    let blocking = services
        .blocking_work()
        .cloned()
        .expect("validated blocking-work service");
    let endpoint = endpoint.to_owned();
    let page_token = page_token.map(str::to_owned);
    let (sender, receiver) = oneshot::channel();
    blocking
        .run(
            scope,
            Box::new(move || {
                let result = perform_get(&endpoint, page_token.as_deref(), secret, cancelled);
                let _ = sender.send(result);
                Ok(())
            }),
        )
        .await?;
    receiver.await.map_err(|_| {
        failure(
            "swallowtail.gemini.models.blocking_result_missing",
            "Gemini Models blocking HTTP work returned no result",
        )
    })?
}

fn perform_get(
    endpoint: &str,
    page_token: Option<&str>,
    mut secret: Vec<u8>,
    cancelled: Arc<AtomicBool>,
) -> Result<Vec<u8>, RuntimeFailure> {
    let mut url = Url::parse(endpoint).map_err(|_| protocol_failure())?;
    if url.scheme() != "https"
        || url.host_str() != Some("generativelanguage.googleapis.com")
        || !matches!(url.path(), "" | "/")
    {
        return Err(protocol_failure());
    }
    url.set_path("/v1beta/models");
    url.query_pairs_mut().append_pair("pageSize", "1000");
    if let Some(token) = page_token {
        url.query_pairs_mut().append_pair("pageToken", token);
    }
    let key = std::str::from_utf8(&secret).map_err(|_| protocol_failure())?;
    if key.is_empty() || key.bytes().any(|byte| matches!(byte, b'\r' | b'\n')) {
        secret.fill(0);
        return Err(protocol_failure());
    }
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
    let mut headers = List::new();
    headers
        .append(&format!("x-goog-api-key: {key}"))
        .map_err(|_| transport_failure())?;
    headers
        .append("accept: application/json")
        .map_err(|_| transport_failure())?;
    secret.fill(0);
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
            "swallowtail.gemini.models.provider_rejected",
            "Gemini rejected model catalogue discovery",
        ));
    }
    Ok(body)
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
            "swallowtail.gemini.models.deadline_elapsed",
            "Gemini Models deadline elapsed before dispatch",
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
            "swallowtail.gemini.models.timed_out",
            "Gemini Models operation timed out",
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
        || plan.endpoint_audience().as_str() != crate::GEMINI_MODELS_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            "swallowtail.gemini.models.preflight_rejected",
            "Gemini Models requires its exact Developer API-key plan",
        ));
    }
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.gemini.models.host_service_missing",
            "Gemini Models requires blocking-work, time, network, and credential services",
        ));
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
            "swallowtail.gemini.models.deadline_elapsed",
            "Gemini Models deadline elapsed before dispatch",
        ))
    } else {
        Ok(())
    }
}

fn bounded_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    let value = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(protocol_failure)?;
    bounded(value)?;
    Ok(value)
}

fn optional_bounded_text<'a>(
    value: &'a Value,
    field: &str,
) -> Result<Option<&'a str>, RuntimeFailure> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => {
            bounded(value)?;
            Ok(Some(value))
        }
        Some(_) => Err(protocol_failure()),
    }
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

fn optional_positive_u64(value: &Value, field: &str) -> Result<Option<u64>, RuntimeFailure> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_u64()
            .filter(|value| *value > 0)
            .map(Some)
            .ok_or_else(protocol_failure),
    }
}

fn optional_bool(value: &Value, field: &str) -> Result<Option<bool>, RuntimeFailure> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value.as_bool().map(Some).ok_or_else(protocol_failure),
    }
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.models.protocol_invalid",
        "Gemini returned an invalid bounded model catalogue",
    )
}

fn transport_failure() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.models.transport_failed",
        "Gemini Models HTTP transport failed",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.models.cleanup_failed",
        "Gemini Models credential cleanup failed",
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
    fn official_page_shape_preserves_identity_limits_reasoning_and_cursor() {
        let (models, next) = parse_page(br#"{"models":[{"name":"models/gemini-fixture-001","baseModelId":"gemini-fixture","version":"1.0","displayName":"Gemini Fixture","description":"Fixture model","inputTokenLimit":1000000,"outputTokenLimit":8192,"supportedGenerationMethods":["generateContent","bidiGenerateContent"],"thinking":true}],"nextPageToken":"page-2"}"#).expect("page parses");
        assert_eq!(models[0].id().as_str(), "gemini-fixture");
        assert_eq!(models[0].metadata().display_name(), Some("Gemini Fixture"));
        assert_eq!(
            models[0]
                .metadata()
                .catalog_observations()
                .and_then(|observations| observations.reasoning_supported()),
            Some(true)
        );
        assert_eq!(next.as_deref(), Some("page-2"));
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
            "swallowtail.gemini.models.timed_out"
        );
        assert!(cancelled.load(Ordering::SeqCst));
    }
}
