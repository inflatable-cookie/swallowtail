use crate::host::LocalProcessHost;
use crate::hosted::LocalCredentialApproval;
use crate::output::failure;
use std::collections::HashMap;
use std::sync::Mutex;
use swallowtail_core::EndpointAudience;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, CredentialLease, CredentialRef, CredentialService,
    DelegatedCredential, RuntimeFailure, ScopeId, SecretLease,
};

#[derive(Default)]
pub(crate) struct LocalCredentialLeaseState {
    issued: Mutex<HashMap<(ScopeId, CredentialRef, EndpointAudience), usize>>,
}

impl LocalCredentialLeaseState {
    fn issue(&self, lease: &CredentialLease) {
        let mut issued = self
            .issued
            .lock()
            .expect("local credential lease lock poisoned");
        *issued
            .entry((
                lease.scope().clone(),
                lease.reference().clone(),
                lease.audience().clone(),
            ))
            .or_default() += 1;
    }

    fn release(&self, lease: &CredentialLease) -> bool {
        let mut issued = self
            .issued
            .lock()
            .expect("local credential lease lock poisoned");
        let key = (
            lease.scope().clone(),
            lease.reference().clone(),
            lease.audience().clone(),
        );
        let Some(count) = issued.get_mut(&key) else {
            return false;
        };
        *count -= 1;
        if *count == 0 {
            issued.remove(&key);
        }
        true
    }
}

impl CredentialService for LocalProcessHost {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        let result = self
            .approvals
            .credentials
            .get(&reference)
            .ok_or_else(|| {
                failure(
                    "swallowtail.local_credential.credential_not_approved",
                    "Local credential reference is not approved",
                )
            })
            .and_then(|approved| {
                if approved.audience() != &audience {
                    return Err(failure(
                        "swallowtail.local_credential.audience_mismatch",
                        "Local credential is approved for a different audience",
                    ));
                }
                let lease = match approved {
                    LocalCredentialApproval::Secret { bytes, .. } => CredentialLease::Secret(
                        SecretLease::new(scope, reference, bytes.clone(), audience),
                    ),
                    LocalCredentialApproval::Delegated { .. } => CredentialLease::Delegated(
                        DelegatedCredential::new(scope, reference, audience),
                    ),
                };
                self.credential_leases.issue(&lease);
                Ok(lease)
            });
        Box::pin(async move { result })
    }

    fn release(&self, lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        let approved = self.approvals.credentials.get(lease.reference());
        let outcome = match approved {
            Some(approved)
                if approved.audience() == lease.audience()
                    && self.credential_leases.release(&lease) =>
            {
                CleanupOutcome::Clean
            }
            _ => CleanupOutcome::Failed(
                failure(
                    "swallowtail.local_credential.lease_not_owned",
                    "Credential lease is not owned by this local host",
                )
                .diagnostic()
                .clone(),
            ),
        };
        drop(lease);
        Box::pin(async move { outcome })
    }
}
