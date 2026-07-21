use crate::{CredentialRef, ScopeId};
use std::fmt;
use swallowtail_core::EndpointAudience;

pub struct SecretLease {
    scope: ScopeId,
    reference: CredentialRef,
    bytes: Vec<u8>,
    audience: EndpointAudience,
    release: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl SecretLease {
    #[must_use]
    pub const fn new(
        scope: ScopeId,
        reference: CredentialRef,
        bytes: Vec<u8>,
        audience: EndpointAudience,
    ) -> Self {
        Self {
            scope,
            reference,
            bytes,
            audience,
            release: None,
        }
    }

    #[must_use]
    pub fn with_release(mut self, release: impl FnOnce() + Send + 'static) -> Self {
        self.release = Some(Box::new(release));
        self
    }

    /// Exposes secret bytes only inside the authorized operation scope.
    #[must_use]
    pub fn expose_secret(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    #[must_use]
    pub const fn reference(&self) -> &CredentialRef {
        &self.reference
    }

    #[must_use]
    pub const fn audience(&self) -> &EndpointAudience {
        &self.audience
    }
}

impl Drop for SecretLease {
    fn drop(&mut self) {
        self.bytes.fill(0);
        if let Some(release) = self.release.take() {
            release();
        }
    }
}

impl fmt::Debug for SecretLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SecretLease")
            .field("scope", &self.scope)
            .field("reference", &self.reference)
            .field("bytes", &"<redacted>")
            .field("audience", &self.audience)
            .finish()
    }
}

impl fmt::Display for SecretLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted secret lease>")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegatedCredential {
    scope: ScopeId,
    reference: CredentialRef,
    audience: EndpointAudience,
}

impl DelegatedCredential {
    #[must_use]
    pub const fn new(scope: ScopeId, reference: CredentialRef, audience: EndpointAudience) -> Self {
        Self {
            scope,
            reference,
            audience,
        }
    }

    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    #[must_use]
    pub const fn reference(&self) -> &CredentialRef {
        &self.reference
    }

    #[must_use]
    pub const fn audience(&self) -> &EndpointAudience {
        &self.audience
    }
}

pub enum CredentialLease {
    Secret(SecretLease),
    Delegated(DelegatedCredential),
}

impl CredentialLease {
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        match self {
            Self::Secret(lease) => lease.scope(),
            Self::Delegated(credential) => credential.scope(),
        }
    }

    #[must_use]
    pub const fn reference(&self) -> &CredentialRef {
        match self {
            Self::Secret(lease) => lease.reference(),
            Self::Delegated(credential) => credential.reference(),
        }
    }

    #[must_use]
    pub const fn audience(&self) -> &EndpointAudience {
        match self {
            Self::Secret(lease) => lease.audience(),
            Self::Delegated(credential) => credential.audience(),
        }
    }
}

impl fmt::Debug for CredentialLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Secret(lease) => formatter.debug_tuple("Secret").field(lease).finish(),
            Self::Delegated(credential) => formatter
                .debug_tuple("Delegated")
                .field(credential)
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SecretLease;
    use crate::{CredentialRef, ScopeId};
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use swallowtail_core::EndpointAudience;

    #[test]
    fn secret_lease_redacts_and_releases() {
        let releases = Arc::new(AtomicUsize::new(0));
        let release_counter = Arc::clone(&releases);
        let audience = EndpointAudience::new("fixture-api").expect("audience is valid");
        let lease = SecretLease::new(
            ScopeId::new("scope-1").expect("scope is valid"),
            CredentialRef::new("credential-1").expect("credential is valid"),
            b"raw-secret".to_vec(),
            audience,
        )
        .with_release(move || {
            release_counter.fetch_add(1, Ordering::SeqCst);
        });

        assert!(!format!("{lease:?}").contains("raw-secret"));
        assert!(!lease.to_string().contains("raw-secret"));
        drop(lease);
        assert_eq!(releases.load(Ordering::SeqCst), 1);
    }
}
