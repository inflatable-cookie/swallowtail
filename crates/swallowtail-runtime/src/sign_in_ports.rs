//! Optional Contract 010 interactive sign-in host ports.
//!
//! These kinds do not collapse into Credential, Process, or Network. Registering
//! a port does not start sign-in. Ports never return secret bytes.

use crate::{ApprovedUrlRef, BoxFuture, CleanupOutcome, CredentialRef, RuntimeFailure, ScopeId};
use std::fmt;
use swallowtail_core::EndpointAudience;

use crate::identity::{DeviceAuthorizationId, LoopbackCallbackId};

/// User-visible device-code prompt displayed by the host.
///
/// The host shows this text. Formatting stays opaque so portable records and
/// diagnostics cannot scrape a login code.
#[derive(Clone, Eq, PartialEq)]
pub struct DeviceCodePrompt {
    user_code: String,
    verification_url: Option<ApprovedUrlRef>,
}

impl DeviceCodePrompt {
    /// Creates a nonempty device-code prompt.
    pub fn new(user_code: impl Into<String>) -> Result<Self, crate::InputValueRequired> {
        crate::input::required_text("device user code", user_code).map(|user_code| Self {
            user_code,
            verification_url: None,
        })
    }

    #[must_use]
    /// Attaches an optional host-approved verification URL.
    pub fn with_verification_url(mut self, url: ApprovedUrlRef) -> Self {
        self.verification_url = Some(url);
        self
    }

    #[must_use]
    /// Returns the user-visible device code for host display only.
    pub fn user_code(&self) -> &str {
        &self.user_code
    }

    #[must_use]
    /// Returns the optional host-approved verification URL.
    pub const fn verification_url(&self) -> Option<&ApprovedUrlRef> {
        self.verification_url.as_ref()
    }
}

impl fmt::Debug for DeviceCodePrompt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeviceCodePrompt")
            .field("user_code", &"<opaque>")
            .field("verification_url", &self.verification_url)
            .finish()
    }
}

/// Operation-scoped loopback bind for one sign-in callback.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoopbackCallbackLease {
    scope: ScopeId,
    callback_id: LoopbackCallbackId,
}

impl LoopbackCallbackLease {
    #[must_use]
    /// Creates a lease for one scope and callback identity.
    pub const fn new(scope: ScopeId, callback_id: LoopbackCallbackId) -> Self {
        Self { scope, callback_id }
    }

    #[must_use]
    /// Returns the sign-in scope that owns this callback.
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    #[must_use]
    /// Returns the opaque callback identity. It is not an authorization code.
    pub const fn callback_id(&self) -> &LoopbackCallbackId {
        &self.callback_id
    }
}

/// Proof that a loopback callback arrived, without secret bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoopbackCallbackReceipt {
    callback_id: LoopbackCallbackId,
}

impl LoopbackCallbackReceipt {
    #[must_use]
    /// Creates a receipt for one opaque callback identity.
    pub const fn new(callback_id: LoopbackCallbackId) -> Self {
        Self { callback_id }
    }

    #[must_use]
    /// Returns the opaque callback identity. It is not an authorization code.
    pub const fn callback_id(&self) -> &LoopbackCallbackId {
        &self.callback_id
    }
}

/// Proof that device authorization finished, without secret bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceAuthorizationReceipt {
    authorization_id: DeviceAuthorizationId,
}

impl DeviceAuthorizationReceipt {
    #[must_use]
    /// Creates a receipt for one opaque device-authorization identity.
    pub const fn new(authorization_id: DeviceAuthorizationId) -> Self {
        Self { authorization_id }
    }

    #[must_use]
    /// Returns the opaque authorization identity. It is not a token.
    pub const fn authorization_id(&self) -> &DeviceAuthorizationId {
        &self.authorization_id
    }
}

/// Host boundary for opening one host-approved URL.
///
/// The host places the browser. This port does not return tokens or start a
/// sign-in loop by being registered.
pub trait UrlOpenService: Send + Sync {
    /// Opens `url` for `scope`. The URL stays host-approved and opaque.
    fn open(
        &self,
        scope: ScopeId,
        url: ApprovedUrlRef,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>>;
}

/// Host boundary for one sign-in loopback callback.
///
/// The host binds the listener. Receipts and materialization expose no secret
/// bytes. Spawning a login helper stays [`crate::ProcessService`].
pub trait LoopbackCallbackService: Send + Sync {
    /// Binds a loopback callback for one sign-in operation.
    fn bind(
        &self,
        scope: ScopeId,
    ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>>;

    /// Returns a receipt when the callback has arrived, without secret bytes.
    fn poll(
        &self,
        lease: &LoopbackCallbackLease,
    ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>>;

    /// Materializes a credential reference from a callback for one audience.
    fn materialize_credential(
        &self,
        receipt: &LoopbackCallbackReceipt,
        audience: &EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure>;

    /// Releases a previously bound loopback callback.
    fn release(&self, lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome>;
}

/// Host boundary for displaying one device code.
///
/// The host shows the code. This port does not return tokens or start a
/// sign-in loop by being registered.
pub trait DeviceCodeDisplayService: Send + Sync {
    /// Displays `prompt` for `scope`.
    fn display(
        &self,
        scope: ScopeId,
        prompt: DeviceCodePrompt,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>>;

    /// Returns a receipt when device authorization has finished, without tokens.
    fn poll_authorization(
        &self,
        scope: &ScopeId,
    ) -> BoxFuture<'static, Result<Option<DeviceAuthorizationReceipt>, RuntimeFailure>>;

    /// Materializes a credential reference from a device receipt for one audience.
    fn materialize_credential(
        &self,
        receipt: &DeviceAuthorizationReceipt,
        audience: &EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure>;
}

#[cfg(test)]
mod tests {
    use super::{DeviceAuthorizationReceipt, DeviceCodePrompt, LoopbackCallbackReceipt};
    use crate::{ApprovedUrlRef, DeviceAuthorizationId, LoopbackCallbackId};

    #[test]
    fn port_records_redact_displayable_and_host_values() {
        let url = ApprovedUrlRef::new("https://login.example.test/authorize?secret=token-bytes")
            .expect("url is valid");
        let prompt = DeviceCodePrompt::new("WDJB-MJHT")
            .expect("user code is valid")
            .with_verification_url(url.clone());
        let loopback = LoopbackCallbackReceipt::new(
            LoopbackCallbackId::new("callback-1").expect("callback id is valid"),
        );
        let device = DeviceAuthorizationReceipt::new(
            DeviceAuthorizationId::new("device-1").expect("authorization id is valid"),
        );

        assert!(!format!("{url:?}").contains("token-bytes"));
        assert!(!format!("{prompt:?}").contains("WDJB-MJHT"));
        assert!(!format!("{loopback:?}").contains("callback-1"));
        assert!(!format!("{device:?}").contains("device-1"));
        assert_eq!(prompt.user_code(), "WDJB-MJHT");
    }
}
