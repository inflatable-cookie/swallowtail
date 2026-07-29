use super::activity_profile::exec_activity_profile;
use super::input::{CodexExecProfileInput, CodexExecProfileParts};
use super::plan::{
    CodexPreparedEvidence, build_plan, descriptor, failure, instance_with_capabilities,
    model_route, require_driver, requirements,
};
use crate::{CodexExecDriver, CodexPreparedDriver, CodexPreparedIntegration};
use std::collections::BTreeSet;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, DriverRole,
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServiceKind, InstalledExecutableCompatibility,
    OperationShape, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, ProviderRetentionPolicy,
    RunHandle, RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

const JSON_SCHEMA_MEDIA_TYPE: &str = "application/schema+json";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexPreparedExec {
    evidence: CodexPreparedEvidence,
    request: StructuredRunRequest,
}

impl CodexPreparedExec {
    #[must_use]
    pub const fn evidence(&self) -> &CodexPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> CodexExecDriver {
        CodexExecDriver::new(self.evidence.environment().clone())
    }

    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(self) -> (CodexPreparedEvidence, PreflightPlan, StructuredRunRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl CodexPreparedIntegration {
    pub fn prepare_structured_exec(
        &self,
        input: CodexExecProfileInput,
    ) -> Result<CodexPreparedExec, PreparationFailure> {
        require_driver(self, CodexPreparedDriver::StructuredExec)?;
        let parts = input.into_parts();
        let (mut capability_requirements, host_services) = exec_requirements(&parts)?;
        let activity_profile = exec_activity_profile(self)?;
        capability_requirements.extend([
            CapabilityRequirement::new(Capability::StreamingEvents, []),
            activity_profile
                .capability_requirement()
                .expect("available Codex exec activity has a capability requirement"),
        ]);
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(
            self,
            parts.model.route_id().clone(),
            parts.model.route_revision().clone(),
            parts.model.model_id().clone(),
            capabilities,
        );
        let posture = self
            .instance()
            .harness_configuration_posture()
            .expect("prepared Codex exec binds configuration posture");
        let requirements = requirements(
            self,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            host_services,
            capability_requirements,
        )
        .with_harness_configuration_posture(posture)
        .require_model_route();
        let descriptor = descriptor(self);
        let plan = build_plan(self, &descriptor, &instance, Some(&route), &requirements)?;
        let mut policy = OperationPolicy::new(parts.external_network, parts.external_search)
            .map_err(|error| {
                swallowtail_runtime::PreparationFailure::new(
                    swallowtail_runtime::PreparationStage::Preflight,
                    swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
                )
            })?
            .with_provider_retention(retention(self))
            .with_harness_configuration_posture(posture);
        if let Some(mode) = parts.reasoning_mode {
            policy = policy.with_reasoning_mode(mode);
        }
        let mut request = StructuredRunRequest::new(parts.request_id, parts.content, policy)
            .with_working_resource(parts.working_resource)
            .with_attachments(parts.attachments)
            .with_tools(parts.tools);
        if let Some(deadline) = parts.deadline {
            request = request.with_deadline(deadline);
        }
        if let Some(output) = parts.structured_output {
            request = request.with_structured_output(output);
        }
        Ok(CodexPreparedExec {
            evidence: CodexPreparedEvidence::from_prepared_with_activity_profile(
                self,
                plan,
                activity_profile,
            )?,
            request,
        })
    }
}

fn exec_requirements(
    parts: &CodexExecProfileParts,
) -> Result<(Vec<CapabilityRequirement>, Vec<HostServiceKind>), PreparationFailure> {
    if !parts.tools.is_empty() {
        return Err(failure(
            "swallowtail.codex.preparation.exec_tools_unsupported",
            "Codex structured exec does not support declared tools",
        ));
    }
    match (parts.external_network, parts.external_search) {
        (ExternalNetworkPolicy::Denied, ExternalSearchPolicy::Disabled)
        | (ExternalNetworkPolicy::HostApproved, ExternalSearchPolicy::Enabled) => {}
        _ => {
            return Err(failure(
                "swallowtail.codex.preparation.exec_network_unsupported",
                "Codex structured exec requires offline mode or host-approved external search",
            ));
        }
    }
    let mut capabilities = vec![CapabilityRequirement::new(Capability::StructuredRun, [])];
    let mut host_services = vec![HostServiceKind::Task, HostServiceKind::Process];
    if parts.deadline.is_some() {
        host_services.push(HostServiceKind::Time);
    }
    if let Some(mode) = &parts.reasoning_mode {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::reasoning_mode(mode.clone())],
        ));
    }
    if !parts.attachments.is_empty() {
        if parts.attachments.len() > 1
            || parts
                .attachments
                .iter()
                .any(|attachment| !attachment.media_type().starts_with("image/"))
        {
            return Err(failure(
                "swallowtail.codex.preparation.attachments_unsupported",
                "Codex structured exec supports at most one image attachment",
            ));
        }
        let mut constraints = BTreeSet::from([CapabilityConstraint::AttachmentMaximumCount(1)]);
        for attachment in &parts.attachments {
            constraints.insert(
                CapabilityConstraint::attachment_media_type(attachment.media_type())
                    .expect("attachment media type is non-empty"),
            );
            if let Some(length) = attachment.known_length() {
                constraints.insert(CapabilityConstraint::AttachmentMaximumBytes(length));
            }
        }
        capabilities.push(CapabilityRequirement::new(
            Capability::Attachments,
            constraints,
        ));
        host_services.push(HostServiceKind::Attachment);
    }
    if let Some(output) = &parts.structured_output {
        if output.media_type() != JSON_SCHEMA_MEDIA_TYPE {
            return Err(failure(
                "swallowtail.codex.preparation.structured_output_unsupported",
                "Codex structured exec requires JSON Schema structured output",
            ));
        }
        capabilities.push(CapabilityRequirement::new(
            Capability::StructuredOutput,
            [CapabilityConstraint::schema_dialect(output.dialect())
                .expect("structured output dialect is non-empty")],
        ));
        host_services.push(HostServiceKind::Schema);
    }
    if parts.external_search == ExternalSearchPolicy::Enabled {
        capabilities.push(CapabilityRequirement::new(Capability::ExternalSearch, []));
        host_services.push(HostServiceKind::Network);
    }
    Ok((capabilities, host_services))
}

fn retention(prepared: &CodexPreparedIntegration) -> ProviderRetentionPolicy {
    let behavior = match prepared.observation().compatibility() {
        InstalledExecutableCompatibility::Qualified(assessment) => {
            assessment.behavior_revision().as_str()
        }
        InstalledExecutableCompatibility::UnverifiedNewer(assessment) => {
            assessment.behavior_revision().as_str()
        }
        InstalledExecutableCompatibility::Incompatible => {
            unreachable!("incompatible executable cannot be prepared")
        }
    };
    if behavior.contains(".retained-") {
        ProviderRetentionPolicy::DurableAllowed
    } else {
        ProviderRetentionPolicy::Prohibited
    }
}
