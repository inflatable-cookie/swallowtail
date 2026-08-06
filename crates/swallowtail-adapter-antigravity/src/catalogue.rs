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
const MAXIMUM_STDERR_BYTES: usize = 4 * 1024;
const MAXIMUM_LINE_BYTES: usize = 256;

/// Low-level driver for Antigravity's authenticated installed model catalogue.
pub struct AntigravityCatalogueDriver {
    environment: EnvironmentRef,
}

impl AntigravityCatalogueDriver {
    #[must_use]
    /// Creates a catalogue driver with one approved process environment.
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }

    #[must_use]
    /// Returns the approved process environment.
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }
}

impl ModelCatalogDriver for AntigravityCatalogueDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            validate(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "antigravity:catalogue:{}",
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
                        "swallowtail.antigravity.catalogue_stdin_close_failed",
                        "Antigravity catalogue process stdin could not be closed",
                    ),
                )
                .await;
            }
            let captured = match read_output(process.as_ref(), request.deadline(), &services).await
            {
                Ok(captured) => captured,
                Err(error) => return fail_with_cleanup(process.as_ref(), error).await,
            };
            let exit = process.wait().await.map_err(|_| cleanup_failure())?;
            if !exit.success() {
                let mut message = match exit.code() {
                    Some(code) => {
                        format!("Antigravity catalogue process exited with status {code}")
                    }
                    None => "Antigravity catalogue process did not exit successfully".to_owned(),
                };
                if let Some(stderr) =
                    crate::discovery::sanitized_stderr(&captured.stderr, captured.stderr_truncated)
                {
                    message.push_str("; stderr: ");
                    message.push_str(&stderr);
                }
                return Err(crate::failure::failure(
                    "swallowtail.antigravity.catalogue_exit_failed",
                    message,
                ));
            }
            parse_catalogue(&captured.stdout)
        })
    }
}

#[derive(Default)]
struct CapturedOutput {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    stderr_truncated: bool,
}

async fn read_output(
    process: &dyn ProcessHandle,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<CapturedOutput, RuntimeFailure> {
    let mut deadline = deadline.map(|deadline| {
        services
            .time()
            .cloned()
            .expect("validated time service is present")
            .wait_until(deadline)
    });
    let mut captured = CapturedOutput::default();
    loop {
        let output = next_output(process, &mut deadline).await?;
        let Some(output) = output else {
            return Ok(captured);
        };
        match output.stream() {
            ProcessOutputStream::Stdout => {
                if captured.stdout.len().saturating_add(output.bytes().len()) > MAXIMUM_OUTPUT_BYTES
                {
                    return Err(protocol_failure());
                }
                captured.stdout.extend_from_slice(output.bytes());
            }
            ProcessOutputStream::Stderr => {
                let remaining = MAXIMUM_STDERR_BYTES.saturating_sub(captured.stderr.len());
                let copied = remaining.min(output.bytes().len());
                captured.stderr.extend_from_slice(&output.bytes()[..copied]);
                captured.stderr_truncated |= copied < output.bytes().len();
            }
        }
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
                "swallowtail.antigravity.catalogue_timed_out",
                "Antigravity model catalogue discovery timed out",
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
    let mut models = Vec::new();
    let mut identities = BTreeSet::new();
    for line in text.lines() {
        if !valid_model_id(line)
            || !identities.insert(line.to_owned())
            || models.len() == MAXIMUM_MODELS
        {
            return Err(protocol_failure());
        }
        models.push(ModelCatalogEntry::new(
            ModelId::new(line).map_err(|_| protocol_failure())?,
            ModelMetadata::default(),
        ));
    }
    if models.is_empty() {
        return Err(protocol_failure());
    }
    Ok(models)
}

fn valid_model_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAXIMUM_LINE_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn validate(
    plan: &PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    crate::selection::validate_antigravity_catalogue_plan(plan)?;
    let requirements = plan.requirements();
    if plan.driver_identity().id().as_str() != crate::CATALOGUE_DRIVER_ID
        || requirements.driver_role() != DriverRole::ModelCatalog
        || requirements.execution_layer() != swallowtail_core::ExecutionLayer::HarnessInteraction
        || requirements.operation_shape() != swallowtail_core::OperationShape::StructuredRun
        || plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
        || requirements.harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.endpoint_audience().as_str() != crate::ANTIGRAVITY_PERSONAL_GOOGLE_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
        || !requirements
            .capabilities()
            .any(|required| required.capability() == Capability::ModelCatalog)
    {
        return Err(crate::failure::failure(
            "swallowtail.antigravity.catalogue_plan_mismatch",
            "Antigravity model catalogue request does not match its immutable plan",
        ));
    }
    if services.process().is_none() || services.time().is_none() {
        return Err(crate::failure::failure(
            "swallowtail.antigravity.catalogue_host_service_missing",
            "Antigravity model catalogue requires process and time services",
        ));
    }
    if request.deadline().is_some_and(|deadline| {
        services.time().expect("validated time service").now() >= deadline.instant()
    }) {
        return Err(crate::failure::failure(
            "swallowtail.antigravity.catalogue_deadline_elapsed",
            "Antigravity model catalogue deadline elapsed before startup",
        ));
    }
    Ok(())
}

fn protocol_failure() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.antigravity.catalogue_invalid",
        "Antigravity returned an invalid bounded model catalogue",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.antigravity.catalogue_cleanup_failed",
        "Antigravity model catalogue process did not join cleanly",
    )
}

#[cfg(test)]
mod tests {
    use super::parse_catalogue;

    const CORPUS: &str = include_str!("../tests/fixtures/antigravity-cli-1.1.9/models.txt");

    #[test]
    fn exact_plain_text_catalogue_preserves_only_model_identity() {
        let models = parse_catalogue(CORPUS.as_bytes()).expect("catalogue parses");
        assert_eq!(models.len(), 11);
        assert_eq!(models[0].id().as_str(), "gemini-3.6-flash-high");
        assert_eq!(models[8].id().as_str(), "claude-sonnet-4-6");
        assert_eq!(models[10].id().as_str(), "gpt-oss-120b-medium");
        assert!(models.iter().all(|model| model.provider_id().is_none()));
        assert!(
            models
                .iter()
                .all(|model| model.metadata().display_name().is_none())
        );
    }

    #[test]
    fn malformed_duplicate_and_unbounded_catalogues_fail_closed() {
        for corpus in [
            "",
            "gemini-3.6-flash-high\n\n",
            "gemini/3.6\n",
            "gemini 3.6\n",
            "gemini-3.6\ngemini-3.6\n",
            "\u{1b}[31mgemini-3.6\n",
        ] {
            assert!(parse_catalogue(corpus.as_bytes()).is_err(), "{corpus:?}");
        }
        let oversized = format!("{}\n", "x".repeat(257));
        assert!(parse_catalogue(oversized.as_bytes()).is_err());
    }
}
