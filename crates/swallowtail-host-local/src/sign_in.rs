//! Host-local test doubles for Contract 010 interactive sign-in ports.
//!
//! These doubles never open a browser, bind a real loopback, or return secret
//! bytes. Registering them does not start a sign-in loop.

use std::sync::{Arc, Mutex};
use swallowtail_core::EndpointAudience;
use swallowtail_runtime::{
    ApprovedUrlRef, BoxFuture, CleanupOutcome, CredentialRef, DeviceAuthorizationId,
    DeviceAuthorizationReceipt, DeviceCodeDisplayService, DeviceCodePrompt, LoopbackCallbackId,
    LoopbackCallbackLease, LoopbackCallbackReceipt, LoopbackCallbackService, RuntimeFailure,
    ScopeId, UrlOpenService,
};

/// Recorded interaction against [`LocalSignInPorts`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LocalSignInCall {
    /// A host-approved URL was opened.
    UrlOpen,
    /// A loopback callback was bound.
    LoopbackBind,
    /// Loopback arrival was polled.
    LoopbackPoll,
    /// A credential reference was materialized from a loopback receipt.
    LoopbackMaterialize,
    /// A loopback lease was released.
    LoopbackRelease,
    /// A device code was displayed.
    DeviceDisplay,
    /// Device authorization was polled.
    DevicePoll,
    /// A credential reference was materialized from a device receipt.
    DeviceMaterialize,
}

#[derive(Default)]
struct LocalSignInState {
    calls: Mutex<Vec<LocalSignInCall>>,
    loopback_ready: Mutex<bool>,
    device_ready: Mutex<bool>,
    loopback_credential: Mutex<Option<(EndpointAudience, CredentialRef)>>,
    device_credential: Mutex<Option<(EndpointAudience, CredentialRef)>>,
}

/// Recording sign-in ports for tests. They do not embed a browser or keychain.
#[derive(Clone, Default)]
pub struct LocalSignInPorts {
    state: Arc<LocalSignInState>,
}

impl LocalSignInPorts {
    /// Creates idle sign-in ports. Registration does not start sign-in.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns recorded port calls in order.
    #[must_use]
    pub fn calls(&self) -> Vec<LocalSignInCall> {
        self.state
            .calls
            .lock()
            .expect("sign-in port lock poisoned")
            .clone()
    }

    /// Counts one recorded port interaction.
    #[must_use]
    pub fn count(&self, call: LocalSignInCall) -> usize {
        self.calls().iter().filter(|seen| **seen == call).count()
    }

    /// Delivers a loopback callback that later materializes `credential`.
    pub fn deliver_loopback(&self, audience: EndpointAudience, credential: CredentialRef) {
        *self
            .state
            .loopback_credential
            .lock()
            .expect("sign-in port lock poisoned") = Some((audience, credential));
        *self
            .state
            .loopback_ready
            .lock()
            .expect("sign-in port lock poisoned") = true;
    }

    /// Delivers device authorization that later materializes `credential`.
    pub fn deliver_device(&self, audience: EndpointAudience, credential: CredentialRef) {
        *self
            .state
            .device_credential
            .lock()
            .expect("sign-in port lock poisoned") = Some((audience, credential));
        *self
            .state
            .device_ready
            .lock()
            .expect("sign-in port lock poisoned") = true;
    }

    fn record(&self, call: LocalSignInCall) {
        self.state
            .calls
            .lock()
            .expect("sign-in port lock poisoned")
            .push(call);
    }
}

impl UrlOpenService for LocalSignInPorts {
    fn open(
        &self,
        _scope: ScopeId,
        _url: ApprovedUrlRef,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        self.record(LocalSignInCall::UrlOpen);
        Box::pin(async { Ok(()) })
    }
}

impl LoopbackCallbackService for LocalSignInPorts {
    fn bind(
        &self,
        scope: ScopeId,
    ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>> {
        self.record(LocalSignInCall::LoopbackBind);
        Box::pin(async move {
            Ok(LoopbackCallbackLease::new(
                scope,
                LoopbackCallbackId::new("local.sign-in.loopback")
                    .expect("loopback callback id is valid"),
            ))
        })
    }

    fn poll(
        &self,
        lease: &LoopbackCallbackLease,
    ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>> {
        self.record(LocalSignInCall::LoopbackPoll);
        let ready = *self
            .state
            .loopback_ready
            .lock()
            .expect("sign-in port lock poisoned");
        let receipt = ready.then(|| LoopbackCallbackReceipt::new(lease.callback_id().clone()));
        Box::pin(async move { Ok(receipt) })
    }

    fn materialize_credential(
        &self,
        _receipt: &LoopbackCallbackReceipt,
        audience: &EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure> {
        self.record(LocalSignInCall::LoopbackMaterialize);
        let stored = self
            .state
            .loopback_credential
            .lock()
            .expect("sign-in port lock poisoned")
            .clone();
        match stored {
            Some((bound, credential)) if &bound == audience => Ok(credential),
            Some(_) => Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.local_sign_in.audience_mismatch",
                "Loopback credential is bound to a different audience",
            ))),
            None => Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.local_sign_in.loopback_not_ready",
                "Loopback callback has no materialized credential",
            ))),
        }
    }

    fn release(&self, _lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome> {
        self.record(LocalSignInCall::LoopbackRelease);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

impl DeviceCodeDisplayService for LocalSignInPorts {
    fn display(
        &self,
        _scope: ScopeId,
        _prompt: DeviceCodePrompt,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        self.record(LocalSignInCall::DeviceDisplay);
        Box::pin(async { Ok(()) })
    }

    fn poll_authorization(
        &self,
        _scope: &ScopeId,
    ) -> BoxFuture<'static, Result<Option<DeviceAuthorizationReceipt>, RuntimeFailure>> {
        self.record(LocalSignInCall::DevicePoll);
        let ready = *self
            .state
            .device_ready
            .lock()
            .expect("sign-in port lock poisoned");
        let receipt = ready.then(|| {
            DeviceAuthorizationReceipt::new(
                DeviceAuthorizationId::new("local.sign-in.device")
                    .expect("device authorization id is valid"),
            )
        });
        Box::pin(async move { Ok(receipt) })
    }

    fn materialize_credential(
        &self,
        _receipt: &DeviceAuthorizationReceipt,
        audience: &EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure> {
        self.record(LocalSignInCall::DeviceMaterialize);
        let stored = self
            .state
            .device_credential
            .lock()
            .expect("sign-in port lock poisoned")
            .clone();
        match stored {
            Some((bound, credential)) if &bound == audience => Ok(credential),
            Some(_) => Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.local_sign_in.audience_mismatch",
                "Device credential is bound to a different audience",
            ))),
            None => Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.local_sign_in.device_not_ready",
                "Device authorization has no materialized credential",
            ))),
        }
    }
}
