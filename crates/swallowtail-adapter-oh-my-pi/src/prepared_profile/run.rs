use super::OhMyPiPreparedRunFuture;
use super::input::OhMyPiRunProfileInput;
use super::plan::{
    OhMyPiPreparedEvidence, build_plan, instance_with_capabilities, run_requirements,
};
use crate::prepared::instance::{reasoning_capability, run_capabilities};
use crate::{OhMyPiPreparedIntegration, OhMyPiRpcDriver};
use swallowtail_core::{
    CapabilityRequirement, HarnessConfigurationPosture, HarnessIsolation, ModelRoute,
};
use swallowtail_runtime::{
    HostServices, OperationPolicy, PreparationFailure, ProviderRetentionPolicy,
    StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OhMyPiPreparedRun {
    evidence: OhMyPiPreparedEvidence,
    request: StructuredRunRequest,
}

impl OhMyPiPreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &OhMyPiPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> OhMyPiRpcDriver {
        self.evidence.low_level_driver()
    }

    pub fn start_run(&self, services: HostServices) -> OhMyPiPreparedRunFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        OhMyPiPreparedEvidence,
        swallowtail_core::PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OhMyPiPreparedIntegration {
    pub fn prepare_run(
        &self,
        input: OhMyPiRunProfileInput,
    ) -> Result<OhMyPiPreparedRun, PreparationFailure> {
        let (request_id, model, content, working_resource, deadline, attachments, reasoning_mode) =
            input.into_parts();
        let image_attachments = !attachments.is_empty();
        validate_attachments(&attachments)?;
        let activity_profile = super::activity_profile::activity_profile(self)?;
        if reasoning_mode
            .as_ref()
            .is_some_and(|mode| !crate::driver::validation::reasoning_mode_supported(mode))
        {
            return Err(super::plan::failure(
                "swallowtail.oh_my_pi.preparation.reasoning_mode_unsupported",
                "OhMyPi RPC reasoning mode is unsupported",
            ));
        }
        let mut capabilities = run_capabilities(image_attachments);
        if let Some(mode) = reasoning_mode.as_ref() {
            capabilities = swallowtail_core::CapabilityProfile::new(
                capabilities
                    .iter()
                    .map(|(capability, constraints)| {
                        CapabilityRequirement::new(capability, constraints.iter().cloned())
                    })
                    .chain([reasoning_capability(mode)]),
            );
        }
        let capabilities = super::activity_profile::with_activity(capabilities, &activity_profile);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, provider_id, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities.clone(),
        )
        .with_provider_id(provider_id);
        let requirements = run_requirements(
            self,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
            image_attachments,
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::Prohibited)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed);
        if let Some(mode) = reasoning_mode {
            policy = policy.with_reasoning_mode(mode);
        }
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_deadline(deadline)
            .with_attachments(attachments);
        Ok(OhMyPiPreparedRun {
            evidence: OhMyPiPreparedEvidence::from_prepared_with_activity(
                self,
                plan,
                activity_profile,
            )?,
            request,
        })
    }
}

fn validate_attachments(
    attachments: &[swallowtail_runtime::AttachmentDescriptor],
) -> Result<(), PreparationFailure> {
    if attachments.len() > 1
        || attachments.iter().any(|attachment| {
            attachment.media_type() != "image/png"
                || attachment
                    .known_length()
                    .is_some_and(|length| length > 1024 * 1024)
        })
    {
        return Err(super::plan::failure(
            "swallowtail.oh_my_pi.preparation.attachments_unsupported",
            "OhMyPi RPC supports one PNG attachment up to one MiB",
        ));
    }
    Ok(())
}
