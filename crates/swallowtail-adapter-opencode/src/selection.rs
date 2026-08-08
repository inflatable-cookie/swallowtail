use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
    InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

/// Semantic-version axis reported by an attached OpenCode server.
pub const OPENCODE_SERVER_AXIS: &str = "opencode.server";
/// Oldest OpenCode server release qualified by the HTTP facade.
pub const OPENCODE_BASELINE_VERSION: &str = "1.14.48";
/// Newest OpenCode server release behaviorally qualified by the HTTP facade.
pub const OPENCODE_LATEST_QUALIFIED_VERSION: &str = "1.18.10";
const MAX_SERVER_VERSION_BYTES: usize = 64;

/// Parses one exact OpenCode server semantic-version binding.
#[must_use]
pub fn opencode_server_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.is_empty()
        || value.len() > MAX_SERVER_VERSION_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
        || semver::Version::parse(value).is_err()
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

/// Returns the qualified OpenCode HTTP/SSE compatibility window.
#[must_use]
pub fn opencode_http_claim() -> InterfaceCompatibilityClaim {
    let segments = [
        (
            OPENCODE_BASELINE_VERSION,
            OPENCODE_BASELINE_VERSION,
            "surface-01",
        ),
        ("1.14.49", "1.14.49", "surface-02"),
        ("1.14.50", "1.14.50", "surface-03"),
        ("1.14.51", "1.14.51", "surface-04"),
        ("1.15.0", "1.15.4", "surface-05"),
        ("1.15.5", "1.15.5", "surface-06"),
        ("1.15.6", "1.15.6", "surface-07"),
        ("1.15.7", "1.15.7", "surface-08"),
        ("1.15.9", "1.15.12", "surface-08"),
        ("1.15.13", "1.15.13", "surface-09"),
        ("1.16.0", "1.16.0", "surface-10"),
        ("1.16.2", "1.16.2", "surface-11"),
        ("1.17.0", "1.17.3", "surface-12"),
        ("1.17.4", "1.17.4", "surface-13"),
        ("1.17.5", "1.17.6", "surface-14"),
        ("1.17.7", "1.17.9", "surface-15"),
        ("1.17.10", "1.17.10", "surface-16"),
        ("1.17.11", "1.17.11", "surface-17"),
        ("1.17.12", "1.17.20", "surface-18"),
        ("1.18.0", OPENCODE_LATEST_QUALIFIED_VERSION, "surface-18"),
    ]
    .into_iter()
    .map(|(minimum, maximum, surface)| segment(minimum, maximum, surface));

    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("opencode.http.server-window-1")
            .expect("static claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        segments,
        [],
    )
    .expect("static OpenCode claim is valid")
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OpenCodePlanVersion {
    binding: InterfaceVersionBinding,
    assessment: InterfaceCompatibilityAssessment,
}

impl OpenCodePlanVersion {
    pub(crate) const fn binding(&self) -> &InterfaceVersionBinding {
        &self.binding
    }

    pub(crate) const fn assessment(&self) -> &InterfaceCompatibilityAssessment {
        &self.assessment
    }
}

pub(crate) fn classify_plan(plan: &PreflightPlan) -> Result<OpenCodePlanVersion, RuntimeFailure> {
    let claim = opencode_http_claim();
    let mut plan_bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = plan_bindings.next().ok_or_else(|| {
        crate::failure::failure(
            "swallowtail.opencode.version_missing",
            "OpenCode plan is missing its exact server version",
        )
    })?;
    if plan_bindings.next().is_some() {
        return Err(crate::failure::failure(
            "swallowtail.opencode.version_ambiguous",
            "OpenCode plan contains more than one server version",
        ));
    }

    let mut required_bindings = plan
        .requirements()
        .interface_versions()
        .filter(|required| required.axis() == claim.axis());
    let required = required_bindings.next().ok_or_else(|| {
        crate::failure::failure(
            "swallowtail.opencode.version_missing",
            "OpenCode requirements are missing their exact server version",
        )
    })?;
    if required_bindings.next().is_some() || required != binding {
        return Err(crate::failure::failure(
            "swallowtail.opencode.version_ambiguous",
            "OpenCode plan and requirements do not select one server version",
        ));
    }

    let assessment = claim.assess(binding.version());
    if !assessment.is_permitted() {
        return Err(crate::failure::failure(
            "swallowtail.opencode.version_incompatible",
            "OpenCode server version is incompatible with this driver",
        ));
    }
    match assessment
        .behavior_revision()
        .expect("permitted assessment has behavior")
        .as_str()
    {
        "opencode.http-sse.surface-01"
        | "opencode.http-sse.surface-02"
        | "opencode.http-sse.surface-03"
        | "opencode.http-sse.surface-04"
        | "opencode.http-sse.surface-05"
        | "opencode.http-sse.surface-06"
        | "opencode.http-sse.surface-07"
        | "opencode.http-sse.surface-08"
        | "opencode.http-sse.surface-09"
        | "opencode.http-sse.surface-10"
        | "opencode.http-sse.surface-11"
        | "opencode.http-sse.surface-12"
        | "opencode.http-sse.surface-13"
        | "opencode.http-sse.surface-14"
        | "opencode.http-sse.surface-15"
        | "opencode.http-sse.surface-16"
        | "opencode.http-sse.surface-17"
        | "opencode.http-sse.surface-18" => Ok(OpenCodePlanVersion {
            binding: binding.clone(),
            assessment,
        }),
        _ => Err(crate::failure::failure(
            "swallowtail.opencode.behavior_incompatible",
            "OpenCode server behavior is not mapped by this driver",
        )),
    }
}

fn segment(minimum: &str, maximum: &str, surface: &str) -> InterfaceVersionSegment {
    InterfaceVersionSegment::new(
        version(minimum).expect("static OpenCode version is valid"),
        version(maximum).expect("static OpenCode version is valid"),
        InterfaceBehaviorRevision::new(format!("opencode.http-sse.{surface}"))
            .expect("static behavior revision is valid"),
        InterfaceSupportStatus::Maintained,
    )
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(OPENCODE_SERVER_AXIS).expect("static version axis is valid")
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

#[cfg(test)]
mod tests;
