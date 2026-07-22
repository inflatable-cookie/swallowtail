mod access;
mod lifecycle;
mod session;
mod turn;

use crate::failure::{failure, protocol};
use crate::selection::validate_alibaba_model_studio_plan;
use crate::transport::CurlTransport;
use swallowtail_core::{CredentialMechanism, PreflightPlan};

#[derive(Clone, Default)]
pub struct AlibabaModelStudioDriver {
    transport: CurlTransport,
}

impl AlibabaModelStudioDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn validate_plan(plan: &PreflightPlan) -> Result<(), swallowtail_runtime::RuntimeFailure> {
        validate_alibaba_model_studio_plan(plan).map_err(protocol)?;
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference().is_none()
        {
            return Err(failure(
                "swallowtail.alibaba_model_studio.access_binding_rejected",
                "Alibaba Model Studio requires its exact API-key access binding",
            ));
        }
        Ok(())
    }
}
