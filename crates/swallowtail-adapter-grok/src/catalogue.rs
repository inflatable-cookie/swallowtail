use std::collections::BTreeSet;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{
    Capability, CredentialMechanism, DriverRole, HarnessConfigurationPosture, HarnessIsolation,
    InstanceOwnership, ModelCatalogEntry, ModelId, ModelMetadata, ModelTokenLimits, PreflightPlan,
    ReasoningMetadata, ReasoningMode, SupportAuthority,
};
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ModelCatalogDriver,
    ModelCatalogRequest, ProcessHandle, ProcessOutputStream, ProcessRequest, RuntimeFailure,
    ScopeId,
};

const MAXIMUM_MODELS: usize = 64;
const MAXIMUM_OUTPUT_BYTES: usize = 64 * 1024;
const MAXIMUM_STDERR_BYTES: usize = 64 * 1024;
const MAXIMUM_LINE_BYTES: usize = 1_024;
const MAXIMUM_ID_BYTES: usize = 256;
const MAXIMUM_PRESENTATION_BYTES: usize = 1_024;

/// Frozen `default_models.json` carved from exact installed Grok `1.0.25`.
///
/// Research 305 freezes the provenance: two byte-identical copies at binary
/// offsets 111710804 and 113348525, SHA-256
/// `c7b26d2f4a4fc6f479b4ffa6c880eaec5eca1721701faa6d970d1cb4a51d5b7a`.
/// The live `models` text output stays authoritative for membership, order,
/// and default; this document only supplies supplemental metadata by exact id
/// equality.
const FROZEN_DEFAULT_MODELS: &str = include_str!("catalogue/default_models_1_0_25.json");

/// Redacted evidence for one bounded catalogue process.
///
/// Exact-`1.0.25` stdout and stderr are captured before parsing, but the raw
/// bytes stay host-private: this record retains only their byte counts and
/// SHA-256 digests, which are safe to keep and log.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokCatalogueOutputEvidence {
    stdout_bytes: usize,
    stderr_bytes: usize,
    stdout_sha256: String,
    stderr_sha256: String,
}

impl GrokCatalogueOutputEvidence {
    /// Returns the bounded byte count observed on stdout before parsing.
    #[must_use]
    pub const fn stdout_bytes(&self) -> usize {
        self.stdout_bytes
    }

    /// Returns the total byte count drained from stderr.
    #[must_use]
    pub const fn stderr_bytes(&self) -> usize {
        self.stderr_bytes
    }

    /// Returns the SHA-256 of the captured stdout bytes.
    #[must_use]
    pub fn stdout_sha256(&self) -> &str {
        &self.stdout_sha256
    }

    /// Returns the SHA-256 of the bounded stderr capture.
    #[must_use]
    pub fn stderr_sha256(&self) -> &str {
        &self.stderr_sha256
    }
}

/// One accepted catalogue listing plus its redacted process evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokCatalogueListing {
    entries: Vec<ModelCatalogEntry>,
    evidence: GrokCatalogueOutputEvidence,
}

impl GrokCatalogueListing {
    /// Returns the ordered model entries.
    #[must_use]
    pub fn entries(&self) -> &[ModelCatalogEntry] {
        &self.entries
    }

    /// Returns the redacted process evidence captured before parsing.
    #[must_use]
    pub const fn evidence(&self) -> &GrokCatalogueOutputEvidence {
        &self.evidence
    }

    /// Consumes the listing and returns the ordered entries.
    #[must_use]
    pub fn into_entries(self) -> Vec<ModelCatalogEntry> {
        self.entries
    }
}

/// Low-level driver for the installed Grok Build model catalogue command.
pub struct GrokCatalogueDriver {
    environment: EnvironmentRef,
}

impl GrokCatalogueDriver {
    /// Creates a catalogue driver bound to the prepared ambient environment.
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }
}

impl ModelCatalogDriver for GrokCatalogueDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            self.run(&plan, &request, &services)
                .await
                .map(GrokCatalogueListing::into_entries)
        })
    }
}

impl GrokCatalogueDriver {
    /// Runs one bounded provider-suppressed catalogue process and returns the
    /// parsed entries with the redacted output evidence captured before
    /// parsing.
    pub fn list_models_recorded(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<GrokCatalogueListing, RuntimeFailure>> {
        Box::pin(async move { self.run(&plan, &request, &services).await })
    }

    async fn run(
        &self,
        plan: &PreflightPlan,
        request: &ModelCatalogRequest,
        services: &HostServices,
    ) -> Result<GrokCatalogueListing, RuntimeFailure> {
        validate(plan, request, services)?;
        let scope = ScopeId::new(format!(
            "grok-build:catalogue:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| protocol_failure())?;
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(["--no-auto-update".to_owned(), "models".to_owned()])
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
                    "swallowtail.grok.catalogue_stdin_close_failed",
                    "Grok Build catalogue process stdin could not be closed",
                ),
            )
            .await;
        }
        let capture = match read_capture(process.as_ref(), request.deadline(), services).await {
            Ok(capture) => capture,
            Err(error) => return fail_with_cleanup(process.as_ref(), error).await,
        };
        let exit = process.wait().await.map_err(|_| cleanup_failure())?;
        if !exit.success() {
            // Raw stderr is never quoted: it may carry host paths or account
            // chrome that must not enter diagnostics.
            let message = match exit.code() {
                Some(code) => format!("Grok Build catalogue process exited with status {code}"),
                None => "Grok Build catalogue process did not exit successfully".to_owned(),
            };
            return Err(swallowtail_runtime::RuntimeFailure::new(
                swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.grok.catalogue_exit_failed",
                    message,
                ),
            ));
        }
        let evidence = capture.evidence();
        let entries = parse_catalogue(&capture.stdout)
            .map_err(|error| annotate_with_evidence(error, &evidence))?;
        Ok(GrokCatalogueListing { entries, evidence })
    }
}

struct CapturedOutput {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    stderr_bytes: usize,
}

impl CapturedOutput {
    fn evidence(&self) -> GrokCatalogueOutputEvidence {
        GrokCatalogueOutputEvidence {
            stdout_bytes: self.stdout.len(),
            stderr_bytes: self.stderr_bytes,
            stdout_sha256: sha256_hex(&self.stdout),
            stderr_sha256: sha256_hex(&self.stderr),
        }
    }
}

async fn read_capture(
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
    let mut capture = CapturedOutput {
        stdout: Vec::new(),
        stderr: Vec::new(),
        stderr_bytes: 0,
    };
    loop {
        let output = next_output(process, &mut deadline).await?;
        let Some(output) = output else {
            return Ok(capture);
        };
        match output.stream() {
            ProcessOutputStream::Stdout => {
                if capture.stdout.len().saturating_add(output.bytes().len()) > MAXIMUM_OUTPUT_BYTES
                {
                    return Err(protocol_failure());
                }
                capture.stdout.extend_from_slice(output.bytes());
            }
            ProcessOutputStream::Stderr => {
                capture.stderr_bytes = capture.stderr_bytes.saturating_add(output.bytes().len());
                let remaining = MAXIMUM_STDERR_BYTES.saturating_sub(capture.stderr.len());
                if remaining > 0 {
                    let take = remaining.min(output.bytes().len());
                    capture.stderr.extend_from_slice(&output.bytes()[..take]);
                }
            }
        }
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest as _, Sha256};
    use std::fmt::Write as _;
    let digest = Sha256::digest(bytes);
    let mut hex = String::with_capacity(64);
    for byte in digest {
        let _ = write!(hex, "{byte:02x}");
    }
    hex
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
                "swallowtail.grok.catalogue_timed_out",
                "Grok Build model catalogue discovery timed out",
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
    let lines: Vec<&str> = text.lines().collect();
    let default = parse_default_header(&lines)?;
    let rows = parse_rows(&lines, &default)?;
    let known = FrozenModels::parse()?;
    let mut models = Vec::with_capacity(rows.len());
    for row in &rows {
        models.push(project_row(row, &known)?);
    }
    if models.is_empty() {
        return Err(protocol_failure());
    }
    Ok(models)
}

struct DefaultHeader {
    header_index: usize,
    model_id: String,
}

fn parse_default_header(lines: &[&str]) -> Result<DefaultHeader, RuntimeFailure> {
    let mut found: Option<DefaultHeader> = None;
    for (index, line) in lines.iter().enumerate() {
        let Some(id) = line.strip_prefix("Default model: ") else {
            continue;
        };
        if !valid_model_id(id) || found.is_some() {
            return Err(protocol_failure());
        }
        found = Some(DefaultHeader {
            header_index: index,
            model_id: id.to_owned(),
        });
    }
    found.ok_or_else(protocol_failure)
}

struct ModelRow {
    id: String,
    marked_default: bool,
}

fn parse_rows(lines: &[&str], header: &DefaultHeader) -> Result<Vec<ModelRow>, RuntimeFailure> {
    let Some(available) = lines
        .iter()
        .enumerate()
        .skip(header.header_index + 1)
        .find(|(_, line)| **line == "Available models:")
        .map(|(index, _)| index)
    else {
        return Err(protocol_failure());
    };
    if lines[..header.header_index].contains(&"Available models:") {
        return Err(protocol_failure());
    }
    let mut rows = Vec::new();
    let mut identities = BTreeSet::new();
    let mut marked: Option<String> = None;
    for line in &lines[available + 1..] {
        if line.is_empty() {
            continue;
        }
        if line.len() > MAXIMUM_LINE_BYTES || line.chars().any(char::is_control) {
            return Err(protocol_failure());
        }
        let row = line.trim_start_matches([' ', '\t']);
        // Exact `1.0.25` renders each row with a bullet: `* ` marks the source
        // default and `- ` marks every other model. Research 306 froze the
        // shipped `xai-grok-pager/src/models.rs` format pieces (`"  - "`,
        // `"  * "`, `" (default)\n"`).
        let (row, bullet_default) = match row.strip_prefix("* ") {
            Some(rest) => (rest, true),
            None => match row.strip_prefix("- ") {
                Some(rest) => (rest, false),
                None => return Err(protocol_failure()),
            },
        };
        let (id, suffix_default) = match row.strip_suffix(" (default)") {
            Some(id) => (id, true),
            None => (row, false),
        };
        if suffix_default && !bullet_default {
            return Err(ambiguous_default());
        }
        let marked_default = bullet_default;
        if !valid_model_id(id) || !identities.insert(id.to_owned()) || rows.len() == MAXIMUM_MODELS
        {
            return Err(protocol_failure());
        }
        if marked_default {
            if marked.is_some() {
                return Err(ambiguous_default());
            }
            marked = Some(id.to_owned());
        }
        rows.push(ModelRow {
            id: id.to_owned(),
            marked_default,
        });
    }
    if marked.as_deref() != Some(header.model_id.as_str()) {
        return Err(ambiguous_default());
    }
    Ok(rows)
}

fn project_row(row: &ModelRow, known: &FrozenModels) -> Result<ModelCatalogEntry, RuntimeFailure> {
    let mut metadata = match known
        .entry(row.id.as_str())
        .and_then(|entry| entry.name.as_deref())
    {
        Some(name) => ModelMetadata::with_display_name(name).map_err(|_| protocol_failure())?,
        None => ModelMetadata::default(),
    };
    if let Some(description) = known
        .entry(row.id.as_str())
        .and_then(|entry| entry.description.as_deref())
    {
        metadata = metadata
            .with_description(description)
            .map_err(|_| protocol_failure())?;
    }
    if let Some(context_window) = known
        .entry(row.id.as_str())
        .and_then(|entry| entry.context_window)
    {
        metadata = metadata.with_token_limits(ModelTokenLimits::new(Some(context_window), None));
    }
    if let Some(reasoning) = known.reasoning(row.id.as_str())? {
        metadata = metadata.with_reasoning(reasoning);
    }
    metadata = metadata.with_default(row.marked_default);
    Ok(ModelCatalogEntry::new(
        ModelId::new(row.id.as_str()).map_err(|_| protocol_failure())?,
        metadata,
    ))
}

/// Supplemental metadata from the frozen exact-`1.0.25` default-model document.
struct FrozenModels {
    entries: Vec<FrozenEntry>,
}

struct FrozenEntry {
    id: String,
    name: Option<String>,
    description: Option<String>,
    context_window: Option<u64>,
    reasoning_modes: Vec<String>,
    reasoning_default: Option<String>,
}

impl FrozenModels {
    fn parse() -> Result<Self, RuntimeFailure> {
        let document: serde_json::Value =
            serde_json::from_str(FROZEN_DEFAULT_MODELS).map_err(|_| protocol_failure())?;
        let default = document
            .get("default")
            .and_then(serde_json::Value::as_str)
            .filter(|default| valid_model_id(default))
            .ok_or_else(protocol_failure)?;
        let models = document
            .get("models")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(protocol_failure)?;
        if models.is_empty() || models.len() > MAXIMUM_MODELS {
            return Err(protocol_failure());
        }
        let mut entries = Vec::with_capacity(models.len());
        let mut identities = BTreeSet::new();
        let mut default_seen = false;
        for model in models {
            let entry = FrozenEntry::parse(model)?;
            if !identities.insert(entry.id.clone()) {
                return Err(protocol_failure());
            }
            default_seen |= entry.id == default;
            entries.push(entry);
        }
        if !default_seen {
            return Err(protocol_failure());
        }
        Ok(Self { entries })
    }

    fn entry(&self, id: &str) -> Option<&FrozenEntry> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    fn reasoning(&self, id: &str) -> Result<Option<ReasoningMetadata>, RuntimeFailure> {
        let Some(entry) = self.entry(id) else {
            return Ok(None);
        };
        if entry.reasoning_modes.is_empty() {
            return Ok(None);
        }
        let mut modes = Vec::with_capacity(entry.reasoning_modes.len());
        for mode in &entry.reasoning_modes {
            modes.push(ReasoningMode::new(mode).map_err(|_| protocol_failure())?);
        }
        let default = match entry.reasoning_default.as_deref() {
            Some(name) => {
                let mode = ReasoningMode::new(name).map_err(|_| protocol_failure())?;
                if !modes.contains(&mode) {
                    return Err(protocol_failure());
                }
                Some(mode)
            }
            None => None,
        };
        Ok(Some(ReasoningMetadata::new(modes, default)))
    }
}

impl FrozenEntry {
    fn parse(model: &serde_json::Value) -> Result<Self, RuntimeFailure> {
        let id = model
            .get("id")
            .and_then(serde_json::Value::as_str)
            .filter(|id| valid_model_id(id))
            .ok_or_else(protocol_failure)?;
        let name = optional_presentation(model, "name")?;
        let description = optional_presentation(model, "description")?;
        let context_window = match model.get("context_window") {
            Some(value) => Some(
                value
                    .as_u64()
                    .filter(|window| *window > 0)
                    .ok_or_else(protocol_failure)?,
            ),
            None => None,
        };
        let (reasoning_modes, reasoning_default) = parse_reasoning(model)?;
        Ok(Self {
            id: id.to_owned(),
            name,
            description,
            context_window,
            reasoning_modes,
            reasoning_default,
        })
    }
}

fn optional_presentation(
    model: &serde_json::Value,
    field: &str,
) -> Result<Option<String>, RuntimeFailure> {
    match model.get(field) {
        Some(value) => {
            let text = value.as_str().ok_or_else(protocol_failure)?;
            if text.is_empty()
                || text.len() > MAXIMUM_PRESENTATION_BYTES
                || text.chars().any(char::is_control)
            {
                return Err(protocol_failure());
            }
            Ok(Some(text.to_owned()))
        }
        None => Ok(None),
    }
}

fn parse_reasoning(
    model: &serde_json::Value,
) -> Result<(Vec<String>, Option<String>), RuntimeFailure> {
    let supported = model
        .get("supports_reasoning_effort")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let Some(efforts) = model
        .get("reasoning_efforts")
        .and_then(serde_json::Value::as_array)
    else {
        return Ok((Vec::new(), None));
    };
    if !supported || efforts.is_empty() || efforts.len() > MAXIMUM_MODELS {
        return Ok((Vec::new(), None));
    }
    let mut modes = Vec::with_capacity(efforts.len());
    let mut flagged: Option<String> = None;
    for effort in efforts {
        let value = effort
            .get("value")
            .and_then(serde_json::Value::as_str)
            .filter(|value| valid_reasoning_name(value))
            .ok_or_else(protocol_failure)?;
        if effort
            .get("default")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            if flagged.is_some() {
                return Err(protocol_failure());
            }
            flagged = Some(value.to_owned());
        }
        modes.push(value.to_owned());
    }
    let fallback = model
        .get("reasoning_effort")
        .and_then(serde_json::Value::as_str)
        .filter(|fallback| modes.iter().any(|mode| mode == fallback));
    Ok((modes, flagged.or_else(|| fallback.map(str::to_owned))))
}

fn valid_model_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAXIMUM_ID_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn valid_reasoning_name(value: &str) -> bool {
    !value.is_empty() && value.len() <= MAXIMUM_ID_BYTES && !value.chars().any(char::is_control)
}

fn validate(
    plan: &PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    crate::selection::validate_grok_catalogue_plan(plan)?;
    let requirements = plan.requirements();
    if plan.driver_identity().id().as_str() != crate::GROK_BUILD_CATALOGUE_DRIVER_ID
        || requirements.driver_role() != DriverRole::ModelCatalog
        || requirements.execution_layer() != swallowtail_core::ExecutionLayer::HarnessInteraction
        || requirements.operation_shape() != swallowtail_core::OperationShape::StructuredRun
        || plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
        || requirements.harness_configuration_posture()
            != Some(HarnessConfigurationPosture::Ambient)
        || requirements.harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || plan.credential_mechanism() != &CredentialMechanism::InteractiveOauth
        || plan.endpoint_audience().as_str() != crate::GROK_BUILD_SUBSCRIPTION_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
        || !requirements
            .capabilities()
            .any(|required| required.capability() == Capability::ModelCatalog)
    {
        return Err(crate::failure::failure(
            "swallowtail.grok.catalogue_plan_mismatch",
            "Grok Build model catalogue request does not match its immutable plan",
        ));
    }
    if services.process().is_none() || services.time().is_none() {
        return Err(crate::failure::failure(
            "swallowtail.grok.catalogue_host_service_missing",
            "Grok Build model catalogue requires process and time services",
        ));
    }
    if request.deadline().is_some_and(|deadline| {
        services.time().expect("validated time service").now() >= deadline.instant()
    }) {
        return Err(crate::failure::failure(
            "swallowtail.grok.catalogue_deadline_elapsed",
            "Grok Build model catalogue deadline elapsed before startup",
        ));
    }
    Ok(())
}

/// Keeps the exact parse diagnostic code and appends the redacted capture
/// counts and digests, so a caller can persist evidence for a failed listing.
fn annotate_with_evidence(
    error: RuntimeFailure,
    evidence: &GrokCatalogueOutputEvidence,
) -> RuntimeFailure {
    let diagnostic = error.diagnostic();
    swallowtail_runtime::RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        diagnostic.code(),
        format!(
            "{} (stdout {} bytes sha256={}, stderr {} bytes sha256={})",
            diagnostic.message(),
            evidence.stdout_bytes,
            evidence.stdout_sha256,
            evidence.stderr_bytes,
            evidence.stderr_sha256,
        ),
    ))
}

fn protocol_failure() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.grok.catalogue_invalid",
        "Grok Build returned an invalid bounded model catalogue",
    )
}

fn ambiguous_default() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.grok.catalogue_default_ambiguous",
        "Grok Build catalogue default is missing, repeated, or disagrees with its header",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.grok.catalogue_cleanup_failed",
        "Grok Build model catalogue process did not join cleanly",
    )
}

#[cfg(test)]
mod tests {
    use super::parse_catalogue;

    const ACCEPTED: &str = "You are logged in with Example Account.\nDefault model: grok-4.6\nAvailable models:\n  * grok-4.6 (default)\n  - grok-4.5\n";

    #[test]
    fn accepted_document_preserves_order_default_and_aligned_metadata() {
        let models = parse_catalogue(ACCEPTED.as_bytes()).expect("catalogue parses");
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].id().as_str(), "grok-4.6");
        assert!(models[0].metadata().is_default());
        assert_eq!(models[0].metadata().display_name(), Some("Grok 4.6"));
        assert_eq!(
            models[0].metadata().description(),
            Some("SpaceXAI's latest frontier model")
        );
        assert_eq!(
            models[0]
                .metadata()
                .token_limits()
                .expect("context window maps")
                .maximum_input_tokens(),
            Some(500_000)
        );
        let reasoning = models[0].metadata().reasoning().expect("reasoning maps");
        assert_eq!(reasoning.supported_modes().len(), 4);
        assert_eq!(
            reasoning
                .default_mode()
                .map(swallowtail_core::ReasoningMode::as_str),
            Some("high")
        );
        assert_eq!(models[1].id().as_str(), "grok-4.5");
        assert!(!models[1].metadata().is_default());
        assert_eq!(models[1].metadata().display_name(), Some("Grok 4.5"));
        assert_eq!(models[1].metadata().description(), None);
        assert!(models.iter().all(|model| model.provider_id().is_none()));
    }

    #[test]
    fn malformed_duplicate_and_ambiguous_catalogues_fail_closed() {
        for corpus in [
            "",
            "Available models:\n  * grok-4.6 (default)\n",
            "Default model: grok-4.6\n  * grok-4.6 (default)\n",
            "Default model: grok-4.6\nAvailable models:\n  - grok-4.5\n",
            "Default model: grok-4.6\nAvailable models:\n  - grok-4.6\n",
            "Default model: grok-4.6\nAvailable models:\n  * grok-4.6 (default)\n  * grok-4.5 (default)\n",
            "Default model: grok-4.5\nAvailable models:\n  * grok-4.6 (default)\n  - grok-4.5\n",
            "Default model: grok-4.6\nAvailable models:\n  * grok-4.6 (default)\n  * grok-4.6 (default)\n",
            "Default model: \nAvailable models:\n  * grok-4.6 (default)\n",
            "Default model: grok-4.6\nAvailable models:\n  * grok/4.6 (default)\n",
            "Default model: grok-4.6\nAvailable models:\n",
            "Default model: grok-4.6\nDefault model: grok-4.5\nAvailable models:\n  * grok-4.6 (default)\n",
            // The frozen grammar requires the shipped bullet prefixes: a bare
            // two-space row is not the exact-1.0.25 format.
            "Default model: grok-4.6\nAvailable models:\n  grok-4.6 (default)\n  grok-4.5\n",
            // A `-` row cannot also carry the `(default)` marker.
            "Default model: grok-4.6\nAvailable models:\n  - grok-4.6 (default)\n",
        ] {
            assert!(parse_catalogue(corpus.as_bytes()).is_err(), "{corpus:?}");
        }
        assert!(parse_catalogue(b"\xff\xfe invalid utf-8").is_err());
    }
}
