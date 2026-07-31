use std::collections::BTreeSet;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{
    Capability, CredentialMechanism, DriverRole, HarnessConfigurationPosture, HarnessIsolation,
    InstanceOwnership, ModelCatalogEntry, ModelId, ModelMetadata, PreflightPlan, SupportAuthority,
};
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, ProcessHandle, ProcessOutputStream, ProcessRequest, RuntimeFailure,
    ScopeId,
};

const MAXIMUM_MODELS: usize = 512;
const MAXIMUM_OUTPUT_BYTES: usize = 512 * 1024;
const MAXIMUM_LINE_BYTES: usize = 1_024;

pub struct CursorCatalogueDriver {
    environment: EnvironmentRef,
}

impl CursorCatalogueDriver {
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }
}

impl ModelCatalogDriver for CursorCatalogueDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "cursor-agent:catalogue:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| protocol_failure())?;
            let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
                plan.instance_target_ref(),
            ))
            .with_arguments(["models".to_owned()])
            .with_environment([self.environment.clone()]);
            let process: Arc<dyn ProcessHandle> = Arc::from(
                services
                    .process()
                    .expect("validated process service is present")
                    .start(scope, process_request)
                    .await?,
            );
            if process.close_stdin().await.is_err() {
                return fail_with_cleanup(
                    process.as_ref(),
                    crate::failure::failure(
                        "swallowtail.cursor.catalogue_stdin_close_failed",
                        "Cursor catalogue process stdin could not be closed",
                    ),
                )
                .await;
            }
            let bytes = match read_stdout(process.as_ref(), request.deadline(), &services).await {
                Ok(bytes) => bytes,
                Err(error) => return fail_with_cleanup(process.as_ref(), error).await,
            };
            let exit = process.wait().await.map_err(|_| cleanup_failure())?;
            if !exit.success() {
                return Err(crate::failure::failure(
                    "swallowtail.cursor.catalogue_exit_failed",
                    match exit.code() {
                        Some(code) => {
                            format!("Cursor catalogue process exited with status {code}")
                        }
                        None => "Cursor catalogue process did not exit successfully".to_owned(),
                    },
                ));
            }
            parse_catalogue(&bytes)
        })
    }
}

async fn read_stdout(
    process: &dyn ProcessHandle,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<Vec<u8>, RuntimeFailure> {
    let mut deadline = deadline.map(|deadline| {
        services
            .time()
            .cloned()
            .expect("validated time service is present")
            .wait_until(deadline)
    });
    let mut stdout = Vec::new();
    loop {
        let output = next_output(process, &mut deadline).await?;
        let Some(output) = output else {
            return Ok(stdout);
        };
        if output.stream() != ProcessOutputStream::Stdout {
            continue;
        }
        if stdout.len().saturating_add(output.bytes().len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(protocol_failure());
        }
        stdout.extend_from_slice(output.bytes());
    }
}

async fn next_output(
    process: &dyn ProcessHandle,
    deadline: &mut Option<BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>,
) -> Result<Option<swallowtail_runtime::ProcessOutputChunk>, RuntimeFailure> {
    let mut output = process.read_output();
    poll_fn(|context| {
        if let Some(wait) = deadline.as_mut()
            && wait.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(Err(crate::failure::failure(
                "swallowtail.cursor.catalogue_timed_out",
                "Cursor model catalogue discovery timed out",
            )));
        }
        output.as_mut().poll(context)
    })
    .await
}

async fn fail_with_cleanup<T>(
    process: &dyn ProcessHandle,
    error: RuntimeFailure,
) -> Result<T, RuntimeFailure> {
    let graceful = process.request_stop().await;
    let forced = process.force_stop().await;
    let waited = process.wait().await;
    if graceful.is_err() || forced.is_err() || waited.is_err() {
        Err(cleanup_failure())
    } else {
        Err(error)
    }
}

pub(crate) fn parse_catalogue(bytes: &[u8]) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    if bytes.is_empty() || bytes.len() > MAXIMUM_OUTPUT_BYTES {
        return Err(protocol_failure());
    }
    let text = std::str::from_utf8(bytes).map_err(|_| protocol_failure())?;
    let mut lines = text.lines();
    if lines.next() != Some("Available models") || lines.next() != Some("") {
        return Err(protocol_failure());
    }
    let mut models = Vec::new();
    let mut identities = BTreeSet::new();
    let mut saw_tip = false;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.len() > MAXIMUM_LINE_BYTES || line.chars().any(char::is_control) {
            return Err(protocol_failure());
        }
        if line.starts_with("Tip:") {
            saw_tip = true;
            continue;
        }
        if saw_tip {
            return Err(protocol_failure());
        }
        let (id, display) = line.split_once(" - ").ok_or_else(protocol_failure)?;
        if !valid_model_id(id) || display.is_empty() || display.trim() != display {
            return Err(protocol_failure());
        }
        if !identities.insert(id.to_owned()) || models.len() == MAXIMUM_MODELS {
            return Err(protocol_failure());
        }
        let (display, is_default) = strip_default_marker(display);
        let metadata = ModelMetadata::with_display_name(display)
            .map_err(|_| protocol_failure())?
            .with_default(is_default);
        models.push(ModelCatalogEntry::new(
            ModelId::new(id).map_err(|_| protocol_failure())?,
            metadata,
        ));
    }
    if models.is_empty() {
        return Err(protocol_failure());
    }
    Ok(models)
}

fn valid_model_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn strip_default_marker(value: &str) -> (&str, bool) {
    value
        .strip_suffix(" (current, default)")
        .map_or((value, false), |value| (value, true))
}

fn validate(
    plan: &PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    crate::selection::validate_cursor_catalogue_plan(plan)?;
    let requirements = plan.requirements();
    if plan.driver_identity().id().as_str() != crate::CATALOGUE_DRIVER_ID
        || requirements.driver_role() != DriverRole::ModelCatalog
        || requirements.execution_layer() != swallowtail_core::ExecutionLayer::HarnessInteraction
        || requirements.operation_shape() != swallowtail_core::OperationShape::StructuredRun
        || plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
        || requirements.harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.endpoint_audience().as_str() != crate::CURSOR_SUBSCRIPTION_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
        || !requirements
            .capabilities()
            .any(|required| required.capability() == Capability::ModelCatalog)
    {
        return Err(crate::failure::failure(
            "swallowtail.cursor.catalogue_plan_mismatch",
            "Cursor model catalogue request does not match its immutable plan",
        ));
    }
    if services.process().is_none() || services.time().is_none() {
        return Err(crate::failure::failure(
            "swallowtail.cursor.catalogue_host_service_missing",
            "Cursor model catalogue requires process and time services",
        ));
    }
    if request.deadline().is_some_and(|deadline| {
        services.time().expect("validated time service").now() >= deadline.instant()
    }) {
        return Err(crate::failure::failure(
            "swallowtail.cursor.catalogue_deadline_elapsed",
            "Cursor model catalogue deadline elapsed before startup",
        ));
    }
    Ok(())
}

fn protocol_failure() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.cursor.catalogue_invalid",
        "Cursor returned an invalid bounded model catalogue",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.cursor.catalogue_cleanup_failed",
        "Cursor model catalogue process did not join cleanly",
    )
}

#[cfg(test)]
mod tests {
    use super::parse_catalogue;

    const CORPUS: &str = "Available models\n\nauto - Auto (current, default)\ngpt-5.3-codex-high - Codex 5.3 High\nclaude-fable-5-high - Fable 5 1M (NO ZDR)\nglm-5.2-max - GLM 5.2 Max\n\nTip: use --model <id> to switch.\n";

    #[test]
    fn auth_aware_plain_text_catalogue_preserves_ids_and_safe_presentation() {
        let models = parse_catalogue(CORPUS.as_bytes()).expect("catalogue parses");
        assert_eq!(models.len(), 4);
        assert_eq!(models[0].id().as_str(), "auto");
        assert_eq!(models[0].metadata().display_name(), Some("Auto"));
        assert!(models[0].metadata().is_default());
        assert_eq!(models[1].id().as_str(), "gpt-5.3-codex-high");
        assert_eq!(
            models[2].metadata().display_name(),
            Some("Fable 5 1M (NO ZDR)")
        );
        assert!(models.iter().all(|model| model.provider_id().is_none()));
    }

    #[test]
    fn malformed_duplicate_and_unbounded_catalogues_fail_closed() {
        for corpus in [
            "",
            "Models\n\nauto - Auto\n",
            "Available models\nauto - Auto\n",
            "Available models\n\nauto Auto\n",
            "Available models\n\nauto - Auto\nauto - Duplicate\n",
            "Available models\n\nTip: done\nauto - Auto\n",
            "Available models\n\nbad/id - Bad\n",
        ] {
            assert!(parse_catalogue(corpus.as_bytes()).is_err(), "{corpus:?}");
        }
        let oversized_tip = format!(
            "Available models\n\nauto - Auto\nTip: {}\n",
            "x".repeat(1_025)
        );
        assert!(parse_catalogue(oversized_tip.as_bytes()).is_err());
    }
}
