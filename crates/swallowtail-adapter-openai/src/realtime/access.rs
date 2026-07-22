use super::worker::{ConnectionWorker, WorkerHandle};
use crate::failure::failure;
use futures_channel::oneshot;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, EndpointRef, HostServices, NetworkGrant,
    RuntimeFailure, ScopeId,
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
        let endpoint = EndpointRef::from_instance_target(plan.instance_target_ref());
        let grant = services
            .network()
            .expect("validated network service")
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
                "swallowtail.openai.realtime_network_grant_mismatch",
                "OpenAI Realtime network grant did not match preflight",
            ));
        }
        let reference = plan.credential_reference().expect("validated").clone();
        let credential = services
            .credential()
            .expect("validated credential service")
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
            let _ = services
                .credential()
                .expect("validated credential service")
                .release(credential)
                .await;
            return Err(failure(
                "swallowtail.openai.realtime_credential_lease_rejected",
                "OpenAI Realtime credential lease did not match preflight",
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
    ) -> Result<(WorkerHandle, BoxFuture<'static, Result<(), RuntimeFailure>>), RuntimeFailure>
    {
        let endpoint = self.grant.authorized().as_driver_value().to_owned();
        let secret = match self.credential.as_ref() {
            Some(CredentialLease::Secret(secret)) => SecretCopy(secret.expose_secret().to_vec()),
            _ => return Err(unavailable()),
        };
        let (sender, receiver) = oneshot::channel();
        let connect_scope = scope.clone();
        let work = services
            .blocking_work()
            .expect("validated blocking service")
            .run(
                scope,
                Box::new(move || {
                    let result = ConnectionWorker::open(&endpoint, &secret.0);
                    let _ = sender.send(result);
                    Ok(())
                }),
            );
        work.await?;
        let worker = receiver.await.map_err(|_| unavailable())??;
        let blocking = services
            .blocking_work()
            .expect("validated blocking service")
            .clone();
        let (worker, handle) = worker;
        let work = blocking.run(connect_scope, Box::new(move || worker.run()));
        Ok((handle, work))
    }

    pub(super) async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        match self.credential.take() {
            Some(lease) => {
                services
                    .credential()
                    .expect("validated credential service")
                    .release(lease)
                    .await
            }
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

fn unavailable() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_connection_unavailable",
        "OpenAI Realtime connection could not be established",
    )
}
