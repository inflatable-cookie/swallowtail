use crate::failure::failure;
use crate::transport::Connection;
use futures_channel::oneshot;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, EndpointRef, HostServices, NetworkGrant, RuntimeFailure,
    ScopeId,
};

pub(super) struct AccessLeases {
    grant: NetworkGrant,
    credential: Option<CredentialLease>,
}

impl AccessLeases {
    pub(super) async fn acquire(
        plan: &PreflightPlan,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        let network = services.network().ok_or_else(|| missing("network"))?;
        let credentials = services.credential().ok_or_else(|| missing("credential"))?;
        let endpoint = EndpointRef::from_instance_target(plan.instance_target_ref());
        let grant = network
            .authorize(
                scope.clone(),
                endpoint.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if grant.scope() != &scope
            || grant.endpoint() != &endpoint
            || grant.audience() != plan.endpoint_audience()
        {
            return Err(failure(
                "swallowtail.xai.network_grant_mismatch",
                "xAI WebSocket network grant did not match preflight",
            ));
        }
        let reference = plan.credential_reference().expect("validated").clone();
        let credential = credentials
            .acquire(
                scope.clone(),
                reference.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if credential.scope() != &scope
            || credential.reference() != &reference
            || credential.audience() != plan.endpoint_audience()
            || !matches!(credential, CredentialLease::Secret(_))
        {
            let _ = credentials.release(credential).await;
            return Err(failure(
                "swallowtail.xai.credential_lease_rejected",
                "xAI WebSocket credential lease did not match preflight",
            ));
        }
        Ok(Self {
            grant,
            credential: Some(credential),
        })
    }

    pub(super) async fn connect(
        &mut self,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Connection, RuntimeFailure> {
        let endpoint = self.grant.authorized().as_driver_value().to_owned();
        let secret = match self.credential.as_ref() {
            Some(CredentialLease::Secret(secret)) => SecretCopy(secret.expose_secret().to_vec()),
            _ => return Err(missing("secret credential")),
        };
        let blocking = services
            .blocking_work()
            .cloned()
            .ok_or_else(|| missing("blocking work"))?;
        let (sender, receiver) = oneshot::channel();
        let work = blocking.run(
            scope,
            Box::new(move || {
                let result = Connection::open(&endpoint, &secret.0);
                let _ = sender.send(result);
                Ok(())
            }),
        );
        work.await?;
        receiver.await.map_err(|_| {
            failure(
                "swallowtail.xai.connection_result_missing",
                "xAI WebSocket connection work returned no result",
            )
        })?
    }

    pub(super) async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        match self.credential.take() {
            Some(lease) => match services.credential() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.xai.credential_release_failed",
                    "xAI credential service disappeared during cleanup",
                )),
            },
            None => CleanupOutcome::NotApplicable,
        }
    }
}

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

fn missing(service: &'static str) -> RuntimeFailure {
    failure(
        "swallowtail.xai.host_service_missing",
        match service {
            "network" => "xAI WebSocket requires a network policy service",
            "credential" => "xAI WebSocket requires a credential service",
            "blocking work" => "xAI WebSocket requires a blocking-work service",
            _ => "xAI WebSocket secret credential was unavailable",
        },
    )
}
