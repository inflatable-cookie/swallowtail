use crate::exec_validation::validate;
use crate::selection::CodexExecBehavior;
use std::sync::Arc;
use std::sync::Mutex;
use swallowtail_core::{ModelId, PreflightPlan};
use swallowtail_runtime::{
    AttachmentFileLease, AttachmentService, CleanupOutcome, ExternalSearchPolicy, HostServices,
    RuntimeFailure, SchemaFileLease, SchemaService, ScopeId, StructuredRunRequest,
};

pub(crate) struct PreparedExecInput {
    arguments: Vec<String>,
    materializations: ExecMaterializations,
}

impl PreparedExecInput {
    pub(crate) fn into_parts(self) -> (Vec<String>, ExecMaterializations) {
        (self.arguments, self.materializations)
    }
}

pub(crate) struct ExecMaterializations {
    attachment_service: Option<Arc<dyn AttachmentService>>,
    attachments: Vec<AttachmentFileLease>,
    schema_service: Option<Arc<dyn SchemaService>>,
    schema: Option<SchemaFileLease>,
}

#[derive(Clone)]
pub(crate) struct SharedExecMaterializations(Arc<Mutex<Option<ExecMaterializations>>>);

impl SharedExecMaterializations {
    pub(crate) fn new(materializations: ExecMaterializations) -> Self {
        Self(Arc::new(Mutex::new(Some(materializations))))
    }

    pub(crate) async fn release(&self) -> CleanupOutcome {
        let materializations = self
            .0
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .take();
        match materializations {
            Some(materializations) => materializations.release().await,
            None => CleanupOutcome::NotApplicable,
        }
    }
}

impl ExecMaterializations {
    fn new(
        attachment_service: Option<Arc<dyn AttachmentService>>,
        schema_service: Option<Arc<dyn SchemaService>>,
    ) -> Self {
        Self {
            attachment_service,
            attachments: Vec::new(),
            schema_service,
            schema: None,
        }
    }

    pub(crate) async fn release(self) -> CleanupOutcome {
        let mut cleanup = CleanupOutcome::Clean;
        if let Some(service) = self.schema_service
            && let Some(lease) = self.schema
        {
            cleanup = merge_cleanup(cleanup, service.release_file(lease).await);
        }
        if let Some(service) = self.attachment_service {
            for lease in self.attachments.into_iter().rev() {
                cleanup = merge_cleanup(cleanup, service.release_file(lease).await);
            }
        }
        cleanup
    }
}

pub(crate) async fn prepare(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
    scope: &ScopeId,
    model: &ModelId,
    behavior: CodexExecBehavior,
) -> Result<PreparedExecInput, RuntimeFailure> {
    validate(plan, request, services, behavior)?;
    let attachment_service = services.attachment().cloned();
    let schema_service = services.schema().cloned();
    let mut materializations =
        ExecMaterializations::new(attachment_service.clone(), schema_service.clone());
    let mut arguments = base_arguments(model, request, behavior);

    if let Some(mode) = request.policy().reasoning_mode() {
        arguments.extend([
            "--config".to_owned(),
            config_string("model_reasoning_effort", mode.as_str()),
        ]);
    }

    if let Some(service) = attachment_service {
        for descriptor in request.attachments() {
            let lease = match service
                .materialize_file(scope.clone(), descriptor.clone())
                .await
            {
                Ok(lease) => lease,
                Err(error) => {
                    let _ = materializations.release().await;
                    return Err(error);
                }
            };
            arguments.extend([
                "--image".to_owned(),
                lease.file().as_driver_value().to_owned(),
            ]);
            materializations.attachments.push(lease);
        }
    }

    if let Some(output) = request.structured_output() {
        let service = schema_service.expect("validated schema service is present");
        let lease = match service
            .materialize_file(scope.clone(), output.document().clone())
            .await
        {
            Ok(lease) => lease,
            Err(error) => {
                let _ = materializations.release().await;
                return Err(error);
            }
        };
        arguments.extend([
            "--output-schema".to_owned(),
            lease.file().as_driver_value().to_owned(),
        ]);
        materializations.schema = Some(lease);
    }

    arguments.push("-".to_owned());
    Ok(PreparedExecInput {
        arguments,
        materializations,
    })
}

fn base_arguments(
    model: &ModelId,
    request: &StructuredRunRequest,
    behavior: CodexExecBehavior,
) -> Vec<String> {
    let mut arguments = vec!["exec".to_owned(), "--json".to_owned()];
    if matches!(
        behavior,
        CodexExecBehavior::EphemeralAmbient | CodexExecBehavior::EphemeralSuppressed
    ) {
        arguments.push("--ephemeral".to_owned());
    }
    arguments.extend(["--color".to_owned(), "never".to_owned()]);
    if behavior == CodexExecBehavior::EphemeralSuppressed {
        arguments.extend([
            "--ignore-user-config".to_owned(),
            "--ignore-rules".to_owned(),
        ]);
    }
    arguments.extend([
        "--skip-git-repo-check".to_owned(),
        "--sandbox".to_owned(),
        "read-only".to_owned(),
        "--model".to_owned(),
        model.as_str().to_owned(),
        "--config".to_owned(),
        config_string("approval_policy", "never"),
        "--config".to_owned(),
        config_string("shell_environment_policy.inherit", "none"),
        "--config".to_owned(),
        "hide_agent_reasoning=false".to_owned(),
        "--config".to_owned(),
        "show_raw_agent_reasoning=false".to_owned(),
        "--config".to_owned(),
        search_config(request.policy().external_search(), behavior),
    ]);
    arguments
}

fn search_config(policy: ExternalSearchPolicy, behavior: CodexExecBehavior) -> String {
    match behavior {
        CodexExecBehavior::RetainedBooleanSearch => format!(
            "features.web_search_request={}",
            policy == ExternalSearchPolicy::Enabled
        ),
        _ => config_string(
            "web_search",
            match policy {
                ExternalSearchPolicy::Disabled => "disabled",
                ExternalSearchPolicy::Enabled => "live",
            },
        ),
    }
}

fn config_string(key: &str, value: &str) -> String {
    let encoded = serde_json::to_string(value).expect("a string always serializes");
    format!("{key}={encoded}")
}

fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        _ => current,
    }
}
