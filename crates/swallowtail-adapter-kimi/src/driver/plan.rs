mod reasoning;

pub struct KimiAcpDriver {
    isolated_environment: EnvironmentRef,
    credential: CredentialRef,
}

impl KimiAcpDriver {
    #[must_use]
    pub const fn new(isolated_environment: EnvironmentRef, credential: CredentialRef) -> Self {
        Self {
            isolated_environment,
            credential,
        }
    }

    fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<crate::selection::KimiPlanSelection, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.kimi.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::InteractiveOauth
            || plan.credential_reference() != Some(&self.credential)
        {
            return Err(failure(
                "swallowtail.kimi.acp.access_profile_rejected",
                "Kimi Code ACP requires its delegated membership OAuth profile",
            ));
        }
        crate::selection::select_kimi_plan(plan)
    }
}
