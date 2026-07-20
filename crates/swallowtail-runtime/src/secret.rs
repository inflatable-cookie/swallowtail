use crate::CredentialRef;
use std::fmt;
use swallowtail_core::EndpointAudience;

pub struct SecretLease {
    bytes: Vec<u8>,
    audience: EndpointAudience,
    release: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl SecretLease {
    #[must_use]
    pub const fn new(bytes: Vec<u8>, audience: EndpointAudience) -> Self {
        Self {
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
    reference: CredentialRef,
}

impl DelegatedCredential {
    #[must_use]
    pub const fn new(reference: CredentialRef) -> Self {
        Self { reference }
    }

    #[must_use]
    pub const fn reference(&self) -> &CredentialRef {
        &self.reference
    }
}

pub enum CredentialLease {
    Secret(SecretLease),
    Delegated(DelegatedCredential),
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
        let lease = SecretLease::new(b"raw-secret".to_vec(), audience).with_release(move || {
            release_counter.fetch_add(1, Ordering::SeqCst);
        });

        assert!(!format!("{lease:?}").contains("raw-secret"));
        assert!(!lease.to_string().contains("raw-secret"));
        drop(lease);
        assert_eq!(releases.load(Ordering::SeqCst), 1);
    }
}
