use crate::validation::failure;
use serde_json::{Value, json};
use std::collections::{BTreeSet, VecDeque};
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{
    Capability, CredentialMechanism, DriverRole, HarnessConfigurationPosture, HarnessIsolation,
    InstanceOwnership, ModelCatalogEntry, ModelId, ModelMetadata, ModelTokenLimits, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, ExecutableRef, HostServices, ModelCatalogDriver, ModelCatalogRequest, ProcessHandle,
    ProcessInputChunk, ProcessOutputStream, ProcessRequest, RuntimeFailure, ScopeId,
};

const MAXIMUM_MODELS: usize = 512;
const MAXIMUM_OUTPUT_BYTES: usize = 2 * 1024 * 1024;
const MAXIMUM_TEXT_BYTES: usize = 512;

impl ModelCatalogDriver for crate::QwenHeadlessDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate_catalogue(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "qwen-headless:catalogue:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| protocol_failure())?;
            let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
                plan.instance_target_ref(),
            ))
            .with_arguments(catalogue_arguments())
            .with_environment([self.environment().clone()]);
            let process: Arc<dyn ProcessHandle> = Arc::from(
                services
                    .process()
                    .expect("validated process service")
                    .start(scope, process_request)
                    .await?,
            );
            let result = exchange(process.as_ref(), &request, &services).await;
            let cleanup = force_cleanup(process.as_ref()).await;
            match (result, cleanup) {
                (Ok(models), Ok(())) => Ok(models),
                (Err(error), _) => Err(error),
                (Ok(_), Err(error)) => Err(error),
            }
        })
    }
}

async fn exchange(
    process: &dyn ProcessHandle,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let mut reader = BoundedLines::default();
    let mut deadline = request.deadline().map(|deadline| {
        services
            .time()
            .cloned()
            .expect("validated time service")
            .wait_until(deadline)
    });
    write_request(
        process,
        json!({
            "type": "control_request",
            "request_id": "swallowtail-initialize",
            "request": {"subtype": "initialize"}
        }),
    )
    .await?;
    let initialize = reader
        .response(process, "swallowtail-initialize", &mut deadline)
        .await?;
    if initialize
        .pointer("/capabilities/can_get_available_models")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return Err(failure(
            "swallowtail.qwen.catalogue_unavailable",
            "Qwen Code did not advertise model catalogue control support",
        ));
    }
    write_request(
        process,
        json!({
            "type": "control_request",
            "request_id": "swallowtail-models",
            "request": {"subtype": "get_available_models"}
        }),
    )
    .await?;
    process.close_stdin().await.map_err(|_| {
        failure(
            "swallowtail.qwen.catalogue_stdin_close_failed",
            "Qwen Code catalogue stdin could not be closed",
        )
    })?;
    let models = reader
        .response(process, "swallowtail-models", &mut deadline)
        .await?;
    parse_qwen_catalogue(&models)
}

async fn write_request(process: &dyn ProcessHandle, value: Value) -> Result<(), RuntimeFailure> {
    let mut bytes = serde_json::to_vec(&value).map_err(|_| protocol_failure())?;
    bytes.push(b'\n');
    process.write_stdin(ProcessInputChunk::new(bytes)).await
}

#[derive(Default)]
struct BoundedLines {
    bytes: Vec<u8>,
    lines: VecDeque<Value>,
}

impl BoundedLines {
    async fn response(
        &mut self,
        process: &dyn ProcessHandle,
        request_id: &str,
        deadline: &mut Option<BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>,
    ) -> Result<Value, RuntimeFailure> {
        loop {
            while let Some(value) = self.lines.pop_front() {
                if value.get("type").and_then(Value::as_str) != Some("control_response") {
                    continue;
                }
                let response = value.get("response").ok_or_else(protocol_failure)?;
                if response.get("request_id").and_then(Value::as_str) != Some(request_id) {
                    continue;
                }
                if response.get("subtype").and_then(Value::as_str) != Some("success") {
                    return Err(failure(
                        "swallowtail.qwen.catalogue_rejected",
                        "Qwen Code rejected model catalogue discovery",
                    ));
                }
                let payload = response.get("response").ok_or_else(protocol_failure)?;
                if payload.get("subtype").and_then(Value::as_str)
                    == Some(if request_id == "swallowtail-initialize" {
                        "initialize"
                    } else {
                        "get_available_models"
                    })
                {
                    return Ok(payload.clone());
                }
                return Err(protocol_failure());
            }
            let output = next_output(process, deadline).await?;
            let Some(output) = output else {
                return Err(failure(
                    "swallowtail.qwen.catalogue_response_missing",
                    "Qwen Code ended before returning model catalogue evidence",
                ));
            };
            if output.stream() != ProcessOutputStream::Stdout {
                continue;
            }
            self.push(output.bytes())?;
        }
    }

    fn push(&mut self, chunk: &[u8]) -> Result<(), RuntimeFailure> {
        if self.bytes.len().saturating_add(chunk.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(protocol_failure());
        }
        self.bytes.extend_from_slice(chunk);
        while let Some(index) = self.bytes.iter().position(|byte| *byte == b'\n') {
            let mut line = self.bytes.drain(..=index).collect::<Vec<_>>();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            if line.is_empty() {
                continue;
            }
            self.lines
                .push_back(serde_json::from_slice(&line).map_err(|_| protocol_failure())?);
        }
        Ok(())
    }
}

async fn next_output(
    process: &dyn ProcessHandle,
    deadline: &mut Option<BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>,
) -> Result<Option<swallowtail_runtime::ProcessOutputChunk>, RuntimeFailure> {
    let mut read = process.read_output();
    poll_fn(|context| {
        if let Some(wait) = deadline.as_mut()
            && wait.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(Err(failure(
                "swallowtail.qwen.catalogue_timed_out",
                "Qwen Code model catalogue discovery timed out",
            )));
        }
        read.as_mut().poll(context)
    })
    .await
}

pub(crate) fn parse_qwen_catalogue(
    value: &Value,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let models = value
        .get("models")
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if models.len() > MAXIMUM_MODELS {
        return Err(protocol_failure());
    }
    let mut identities = BTreeSet::new();
    models
        .iter()
        .map(|model| {
            let id = bounded_text(model, "id")?;
            if !identities.insert(id.to_owned()) {
                return Err(protocol_failure());
            }
            let mut metadata = match optional_bounded_text(model, "label")? {
                Some(label) => {
                    ModelMetadata::with_display_name(label).map_err(|_| protocol_failure())?
                }
                None => ModelMetadata::default(),
            };
            if let Some(input) = optional_positive_u64(model, "contextWindowSize")? {
                metadata = metadata.with_token_limits(ModelTokenLimits::new(Some(input), None));
            }
            Ok(ModelCatalogEntry::new(
                ModelId::new(id).map_err(|_| protocol_failure())?,
                metadata,
            ))
        })
        .collect()
}

fn validate_catalogue(
    plan: &PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    crate::selection::validate_qwen_plan_version(plan)?;
    if plan.driver_identity().id().as_str() != crate::DRIVER_ID
        || plan.requirements().driver_role() != DriverRole::ModelCatalog
        || plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || !matches!(
            plan.credential_mechanism(),
            CredentialMechanism::ProviderSpecific(namespace)
                if namespace.as_str() == "qwen-code/delegated-harness-auth"
        )
        || !plan
            .requirements()
            .capabilities()
            .any(|required| required.capability() == Capability::ModelCatalog)
    {
        return Err(failure(
            "swallowtail.qwen.catalogue_plan_mismatch",
            "Qwen Code model catalogue request does not match its immutable plan",
        ));
    }
    if services.process().is_none() || services.time().is_none() {
        return Err(failure(
            "swallowtail.qwen.catalogue_host_service_missing",
            "Qwen Code model catalogue requires process and time services",
        ));
    }
    if request.deadline().is_some_and(|deadline| {
        services.time().expect("validated time service").now() >= deadline.instant()
    }) {
        return Err(failure(
            "swallowtail.qwen.catalogue_deadline_elapsed",
            "Qwen Code model catalogue deadline elapsed before startup",
        ));
    }
    Ok(())
}

fn catalogue_arguments() -> Vec<String> {
    [
        "--input-format",
        "stream-json",
        "--output-format",
        "stream-json",
        "--safe-mode",
        "--approval-mode",
        "default",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

async fn force_cleanup(process: &dyn ProcessHandle) -> Result<(), RuntimeFailure> {
    let force = process.force_stop().await;
    let wait = process.wait().await;
    if force.is_err() || wait.is_err() {
        Err(failure(
            "swallowtail.qwen.catalogue_cleanup_failed",
            "Qwen Code model catalogue process cleanup failed",
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

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.qwen.catalogue_invalid",
        "Qwen Code returned an invalid bounded model catalogue",
    )
}

#[cfg(test)]
mod tests {
    use super::parse_qwen_catalogue;
    use serde_json::json;

    #[test]
    fn exact_control_response_projects_models_without_raw_capability_payloads() {
        let models = parse_qwen_catalogue(&json!({
            "subtype": "get_available_models",
            "models": [{
                "id": "qwen-fixture",
                "label": "Qwen Fixture",
                "capabilities": {"private_provider_detail": "ignored"},
                "contextWindowSize": 131072
            }]
        }))
        .expect("catalogue parses");
        assert_eq!(models[0].id().as_str(), "qwen-fixture");
        assert_eq!(models[0].metadata().display_name(), Some("Qwen Fixture"));
        assert!(!format!("{models:?}").contains("private_provider_detail"));
    }
}
