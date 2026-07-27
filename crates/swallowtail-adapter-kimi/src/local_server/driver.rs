mod control;
mod execution;
mod response;

use crate::failure::unsupported;
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, BoxFuture, DeleteProviderSessionRequest, HostServices,
    ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, RestoreProviderSessionRequest, RuntimeFailure,
    validate_provider_session_management_request,
};

use super::transport::CurlTransport;

#[derive(Clone, Default)]
pub struct KimiLocalServerDriver {
    pub(super) transport: CurlTransport,
    pub(super) session_configuration: Option<super::KimiLocalServerSessionConfiguration>,
}

impl KimiLocalServerDriver {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_session_configuration(
        configuration: super::KimiLocalServerSessionConfiguration,
    ) -> Self {
        Self {
            transport: CurlTransport,
            session_configuration: Some(configuration),
        }
    }
}

impl ProviderSessionManagementDriver for KimiLocalServerDriver {
    fn archive_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: ArchiveProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            self.execute(
                plan,
                request.agreement(),
                request.cancellation(),
                request.request_id(),
                services,
            )
            .await
        })
    }

    fn restore_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: RestoreProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            self.execute(
                plan,
                request.agreement(),
                request.cancellation(),
                request.request_id(),
                services,
            )
            .await
        })
    }

    fn delete_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: DeleteProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("Kimi local-server session deletion")) })
    }
}
