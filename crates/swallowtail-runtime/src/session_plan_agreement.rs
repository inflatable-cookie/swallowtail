use crate::{PreparationFailure, PreparationStage, RuntimeFailure, SessionAccessPolicy};
use swallowtail_core::{
    Diagnostic, HarnessConfigurationPosture, PreflightPlan, SafeDiagnostic,
    SessionProviderStatePolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionPlanAgreement {
    access_policy: SessionAccessPolicy,
    provider_state_policy: Option<SessionProviderStatePolicy>,
    harness_configuration_posture: Option<HarnessConfigurationPosture>,
}

impl SessionPlanAgreement {
    #[must_use]
    pub const fn explicit(
        access_policy: SessionAccessPolicy,
        provider_state_policy: Option<SessionProviderStatePolicy>,
        harness_configuration_posture: Option<HarnessConfigurationPosture>,
    ) -> Self {
        Self {
            access_policy,
            provider_state_policy,
            harness_configuration_posture,
        }
    }

    pub fn from_plan(plan: &PreflightPlan) -> Result<Self, PreparationFailure> {
        let access_policy = plan
            .requirements()
            .session_access_policy()
            .cloned()
            .ok_or_else(|| {
                PreparationFailure::new(
                    PreparationStage::Preflight,
                    Diagnostic::new(SafeDiagnostic::new(
                        "swallowtail.session_request.plan_access_missing",
                        "Interactive-session plan is missing its access policy",
                    )),
                )
            })?;

        Ok(Self::explicit(
            access_policy,
            plan.requirements().session_provider_state_policy(),
            plan.requirements().harness_configuration_posture(),
        ))
    }

    #[must_use]
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        &self.access_policy
    }

    #[must_use]
    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy
    }

    #[must_use]
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture
    }
}

pub fn validate_session_plan_agreement(
    plan: &PreflightPlan,
    agreement: &SessionPlanAgreement,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().session_access_policy() != Some(agreement.access_policy()) {
        return Err(failure(
            "swallowtail.session_access.plan_mismatch",
            "Session access policy does not match its immutable preflight plan",
        ));
    }
    if plan.requirements().session_provider_state_policy() != agreement.provider_state_policy() {
        return Err(failure(
            "swallowtail.session_provider_state.plan_mismatch",
            "Session provider-state policy does not match its immutable preflight plan",
        ));
    }
    if plan.requirements().harness_configuration_posture()
        != agreement.harness_configuration_posture()
    {
        return Err(failure(
            "swallowtail.harness_configuration.plan_mismatch",
            "Harness configuration does not match its immutable preflight plan",
        ));
    }
    Ok(())
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
