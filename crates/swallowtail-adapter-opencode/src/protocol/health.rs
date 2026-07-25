use super::{Response, parse_json, require_success};
use crate::failure::failure;
use crate::selection::{OpenCodePlanVersion, opencode_http_claim, opencode_server_binding};
use serde::Deserialize;
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersionBinding};
use swallowtail_runtime::RuntimeFailure;

#[derive(Deserialize)]
struct Health {
    healthy: bool,
    version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OpenCodeServerObservation {
    binding: InterfaceVersionBinding,
    assessment: InterfaceCompatibilityAssessment,
}

impl OpenCodeServerObservation {
    pub(crate) const fn binding(&self) -> &InterfaceVersionBinding {
        &self.binding
    }

    pub(crate) const fn assessment(&self) -> &InterfaceCompatibilityAssessment {
        &self.assessment
    }
}

pub(crate) fn observe_health(
    response: &Response,
) -> Result<OpenCodeServerObservation, RuntimeFailure> {
    require_success(response, "health request")?;
    let health: Health = parse_json(&response.body, "health response")?;
    if !health.healthy {
        return Err(failure(
            "swallowtail.opencode.unhealthy",
            "OpenCode reported an unhealthy server",
        ));
    }
    let binding = opencode_server_binding(&health.version).ok_or_else(|| {
        failure(
            "swallowtail.opencode.version_invalid",
            "OpenCode reported an invalid server version",
        )
    })?;
    let assessment = opencode_http_claim().assess(binding.version());
    if !assessment.is_permitted() {
        return Err(failure(
            "swallowtail.opencode.version_unsupported",
            "OpenCode server version is incompatible with this driver",
        ));
    }
    Ok(OpenCodeServerObservation {
        binding,
        assessment,
    })
}

pub(crate) fn require_health_matches(
    response: &Response,
    expected: &OpenCodePlanVersion,
) -> Result<(), RuntimeFailure> {
    let observation = observe_health(response)?;
    if observation.binding() != expected.binding()
        || observation.assessment() != expected.assessment()
    {
        return Err(failure(
            "swallowtail.opencode.version_mismatch",
            "OpenCode server version does not match the exact preflight plan",
        ));
    }
    Ok(())
}
