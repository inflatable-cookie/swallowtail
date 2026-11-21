//! Library-owned interactive sign-in loop for Contract 057.
//!
//! Swallowtail owns start, poll, complete, cancel, and timeout. Host ports open
//! a URL, bind a loopback callback, display a device code, or spawn an approved
//! login helper through process authority. [`swallowtail_core::SignInAction`]
//! remains an advertisement. ACP authenticate and Contract 017 delegated login
//! are not this loop.

use super::{ConnectionLifecycleStore, ConnectionLifecycleStoreFailure};
use crate::{
    ApprovedUrlRef, BoxFuture, CredentialRef, Deadline, DeviceCodePrompt, HostServices,
    ProcessHandle, ProcessRequest, RuntimeFailure, ScopeId,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, PoisonError};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{
    AccessProfileId, AddableRouteId, AdmittedInstanceRecord, ConfiguredInstanceId,
    CredentialFieldDescriptor, CredentialFieldId, CredentialMechanism, EndpointAudience,
    EntitlementMetering, HostServiceKind, IntegrationFamilyId, SafeDiagnostic,
};

/// Kind of library-owned sign-in loop.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignInKind {
    /// Interactive OAuth through URL-open and loopback-callback ports.
    InteractiveOauth,
    /// Device OAuth through the device-code display port.
    DeviceOauth,
    /// Delegated CLI login through process authority.
    DelegatedCliLogin,
    /// API-key collection through credential-field descriptors.
    ApiKeyCollection,
}

/// Observed status of one sign-in session.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignInStatus {
    /// Ports or process work have started. No credential yet.
    Started,
    /// The loop is waiting for a host callback, device finish, process, or field.
    InProgress,
    /// Host proof is present and complete may run.
    ReadyToComplete,
    /// Complete materialized a credential reference.
    Completed,
    /// The caller cancelled the loop.
    Cancelled,
    /// The host monotonic clock passed the supplied deadline.
    TimedOut,
}

/// Stable reason a sign-in loop failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignInFailureKind {
    /// A required host port is missing.
    MissingHostPort,
    /// Kind and credential mechanism do not match.
    MechanismMismatch,
    /// Complete would change endpoint audience.
    AudienceMismatch,
    /// Complete would change billing authority.
    BillingMismatch,
    /// Complete would change the bound account.
    AccountMismatch,
    /// The session is not in a state that allows this step.
    InvalidState,
    /// An advertised credential field is missing or unknown.
    UnknownCredentialField,
    /// Required API-key fields have not been submitted.
    Incomplete,
    /// The admitted instance is absent from the store.
    InstanceAbsent,
    /// The store rejected the credential reference write.
    Store,
    /// A host port returned a runtime failure.
    HostPort,
}

/// Rejection raised by the library-owned sign-in loop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignInFailure {
    kind: SignInFailureKind,
    diagnostic: SafeDiagnostic,
}

impl SignInFailure {
    fn new(kind: SignInFailureKind, code: &'static str, message: &'static str) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    fn missing_port(kind: HostServiceKind) -> Self {
        let message = match kind {
            HostServiceKind::UrlOpen => "Sign-in loop requires a URL-open host port",
            HostServiceKind::LoopbackCallback => {
                "Sign-in loop requires a loopback-callback host port"
            }
            HostServiceKind::DeviceCodeDisplay => {
                "Sign-in loop requires a device-code display host port"
            }
            HostServiceKind::Process => "Delegated CLI login requires process host authority",
            HostServiceKind::Time => "Sign-in timeout requires a host time service",
            _ => "Sign-in loop requires a host port",
        };
        Self {
            kind: SignInFailureKind::MissingHostPort,
            diagnostic: SafeDiagnostic::new("swallowtail.sign_in.missing_host_port", message),
        }
    }

    fn from_port(failure: RuntimeFailure) -> Self {
        Self {
            kind: SignInFailureKind::HostPort,
            diagnostic: failure.diagnostic().clone(),
        }
    }

    fn from_store(failure: ConnectionLifecycleStoreFailure) -> Self {
        Self {
            kind: SignInFailureKind::Store,
            diagnostic: failure.diagnostic().clone(),
        }
    }

    #[must_use]
    /// Returns the stable failure classification.
    pub const fn kind(&self) -> SignInFailureKind {
        self.kind
    }

    #[must_use]
    /// Returns the redacted sign-in diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for SignInFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for SignInFailure {}

/// Method-specific host work for one sign-in start.
#[derive(Clone, Debug)]
pub enum SignInMethod {
    /// Open a host-approved authorize URL and wait for a loopback callback.
    InteractiveOauth {
        /// Host-approved authorize URL. Never a token.
        authorize_url: ApprovedUrlRef,
        /// Store field that receives the materialized credential reference.
        credential_field: CredentialFieldId,
    },
    /// Display a device code and wait for host authorization.
    DeviceOauth {
        /// User-visible device prompt displayed by the host.
        prompt: DeviceCodePrompt,
        /// Store field that receives the materialized credential reference.
        credential_field: CredentialFieldId,
    },
    /// Spawn an approved login helper. Process authority stays on Process.
    DelegatedCliLogin {
        /// Approved helper launch. The host still owns the executable.
        process: ProcessRequest,
        /// Credential reference the helper is expected to populate.
        credential: CredentialRef,
        /// Store field that receives the credential reference.
        credential_field: CredentialFieldId,
    },
    /// Collect API-key field descriptors into opaque credential references.
    ApiKeyCollection {
        /// Advertised fields. Descriptors never carry secret bytes.
        fields: Vec<CredentialFieldDescriptor>,
    },
}

/// Frozen authority for one sign-in loop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignInAuthorityBinding {
    mechanism: CredentialMechanism,
    audience: EndpointAudience,
    billing: EntitlementMetering,
    account: Option<AccessProfileId>,
    existing_credential: Option<CredentialRef>,
}

impl SignInAuthorityBinding {
    #[must_use]
    /// Creates a binding that complete must not change.
    pub fn new(
        mechanism: CredentialMechanism,
        audience: EndpointAudience,
        billing: EntitlementMetering,
    ) -> Self {
        Self {
            mechanism,
            audience,
            billing,
            account: None,
            existing_credential: None,
        }
    }

    #[must_use]
    /// Binds an existing account identity. Complete cannot change it.
    pub fn with_account(mut self, account: AccessProfileId) -> Self {
        self.account = Some(account);
        self
    }

    #[must_use]
    /// Binds an existing credential reference. Complete cannot replace it.
    pub fn with_existing_credential(mut self, credential: CredentialRef) -> Self {
        self.existing_credential = Some(credential);
        self
    }

    #[must_use]
    /// Returns the frozen credential mechanism.
    pub const fn mechanism(&self) -> &CredentialMechanism {
        &self.mechanism
    }

    #[must_use]
    /// Returns the frozen endpoint audience.
    pub const fn audience(&self) -> &EndpointAudience {
        &self.audience
    }

    #[must_use]
    /// Returns the frozen billing authority.
    pub const fn billing(&self) -> &EntitlementMetering {
        &self.billing
    }

    #[must_use]
    /// Returns the frozen account identity, when one was bound.
    pub const fn account(&self) -> Option<&AccessProfileId> {
        self.account.as_ref()
    }

    #[must_use]
    /// Returns the existing credential that complete must not replace.
    pub const fn existing_credential(&self) -> Option<&CredentialRef> {
        self.existing_credential.as_ref()
    }
}

/// Host-owned input for starting one sign-in loop.
#[derive(Clone, Debug)]
pub struct SignInStartRequest {
    scope: ScopeId,
    instance_id: ConfiguredInstanceId,
    family: IntegrationFamilyId,
    route_id: AddableRouteId,
    binding: SignInAuthorityBinding,
    method: SignInMethod,
    deadline: Option<Deadline>,
}

impl SignInStartRequest {
    /// Creates a sign-in start for one admitted instance, route, and method.
    #[must_use]
    pub fn new(
        scope: ScopeId,
        instance_id: ConfiguredInstanceId,
        family: IntegrationFamilyId,
        route_id: AddableRouteId,
        binding: SignInAuthorityBinding,
        method: SignInMethod,
    ) -> Self {
        Self {
            scope,
            instance_id,
            family,
            route_id,
            binding,
            method,
            deadline: None,
        }
    }

    #[must_use]
    /// Sets an explicit monotonic deadline. Timeout is observed through Time.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

/// Materialized result of a successful sign-in complete.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignInOutcome {
    instance_id: ConfiguredInstanceId,
    route_id: AddableRouteId,
    audience: EndpointAudience,
    mechanism: CredentialMechanism,
    billing: EntitlementMetering,
    credential_refs: BTreeMap<CredentialFieldId, CredentialRef>,
}

impl SignInOutcome {
    #[must_use]
    /// Returns the instance the loop started for.
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    /// Returns the route the loop started for.
    pub const fn route_id(&self) -> &AddableRouteId {
        &self.route_id
    }

    #[must_use]
    /// Returns the frozen endpoint audience.
    pub const fn audience(&self) -> &EndpointAudience {
        &self.audience
    }

    #[must_use]
    /// Returns the frozen credential mechanism.
    pub const fn mechanism(&self) -> &CredentialMechanism {
        &self.mechanism
    }

    #[must_use]
    /// Returns the frozen billing authority.
    pub const fn billing(&self) -> &EntitlementMetering {
        &self.billing
    }

    /// Iterates materialized credential references in stable field-id order.
    pub fn credential_refs(
        &self,
    ) -> impl ExactSizeIterator<Item = (&CredentialFieldId, &CredentialRef)> {
        self.credential_refs.iter()
    }
}

enum PendingSignInPoll {
    Loopback(BoxFuture<'static, Result<Option<crate::LoopbackCallbackReceipt>, RuntimeFailure>>),
    Device(BoxFuture<'static, Result<Option<crate::DeviceAuthorizationReceipt>, RuntimeFailure>>),
    Delegated(BoxFuture<'static, Result<crate::ProcessExit, RuntimeFailure>>),
}

/// In-flight library-owned sign-in session.
pub struct SignInSession {
    scope: ScopeId,
    instance_id: ConfiguredInstanceId,
    family: IntegrationFamilyId,
    route_id: AddableRouteId,
    binding: SignInAuthorityBinding,
    kind: SignInKind,
    deadline: Option<Deadline>,
    status: SignInStatus,
    loopback: Option<crate::LoopbackCallbackLease>,
    loopback_receipt: Option<crate::LoopbackCallbackReceipt>,
    device_receipt: Option<crate::DeviceAuthorizationReceipt>,
    process: Option<Arc<dyn ProcessHandle>>,
    delegated_credential: Option<CredentialRef>,
    credential_field: Option<CredentialFieldId>,
    api_key_fields: BTreeMap<CredentialFieldId, CredentialFieldDescriptor>,
    submitted_refs: BTreeMap<CredentialFieldId, CredentialRef>,
    pending_poll: Mutex<Option<PendingSignInPoll>>,
    pending_loopback_release: Mutex<Option<BoxFuture<'static, crate::CleanupOutcome>>>,
}

impl fmt::Debug for SignInSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SignInSession")
            .field("kind", &self.kind)
            .field("status", &self.status)
            .field("route_id", &self.route_id)
            .finish_non_exhaustive()
    }
}

impl SignInSession {
    #[must_use]
    /// Returns the current loop status.
    pub const fn status(&self) -> SignInStatus {
        self.status
    }

    #[must_use]
    /// Returns the loop kind.
    pub const fn kind(&self) -> SignInKind {
        self.kind
    }

    #[must_use]
    /// Returns the frozen authority binding.
    pub const fn binding(&self) -> &SignInAuthorityBinding {
        &self.binding
    }
}

/// Starts one sign-in loop through the required host ports.
///
/// Registration of a port is not enough. This call performs host work for the
/// selected method. [`swallowtail_core::SignInAction`] is not consulted.
///
/// Host start futures that yield `Poll::Pending` fail closed with
/// [`SignInFailureKind::HostPort`] after cleaning up any partial lease; use
/// [`start_sign_in_async`] when host ports are asynchronous.
pub fn start_sign_in(
    services: &HostServices,
    request: SignInStartRequest,
) -> Result<SignInSession, SignInFailure> {
    let (mut session, method) = prepare_session(services, request)?;
    match method {
        SignInMethod::InteractiveOauth {
            authorize_url,
            credential_field,
        } => {
            session.credential_field = Some(credential_field);
            start_interactive(services, &mut session, authorize_url)?;
        }
        SignInMethod::DeviceOauth {
            prompt,
            credential_field,
        } => {
            session.credential_field = Some(credential_field);
            start_device(services, &mut session, prompt)?;
        }
        SignInMethod::DelegatedCliLogin {
            process,
            credential,
            credential_field,
        } => {
            session.credential_field = Some(credential_field);
            session.delegated_credential = Some(credential);
            start_delegated(services, &mut session, process)?;
        }
        SignInMethod::ApiKeyCollection { fields } => {
            start_api_key(&mut session, fields)?;
        }
    }
    Ok(session)
}

/// Starts one sign-in loop asynchronously through the required host ports.
///
/// Registration of a port is not enough. This call awaits host work for the
/// selected method and releases any acquired loopback lease if opening the
/// authorize URL fails. [`swallowtail_core::SignInAction`] is not consulted.
pub async fn start_sign_in_async(
    services: &HostServices,
    request: SignInStartRequest,
) -> Result<SignInSession, SignInFailure> {
    let (mut session, method) = prepare_session(services, request)?;
    match method {
        SignInMethod::InteractiveOauth {
            authorize_url,
            credential_field,
        } => {
            session.credential_field = Some(credential_field);
            start_interactive_async(services, &mut session, authorize_url).await?;
        }
        SignInMethod::DeviceOauth {
            prompt,
            credential_field,
        } => {
            session.credential_field = Some(credential_field);
            start_device_async(services, &mut session, prompt).await?;
        }
        SignInMethod::DelegatedCliLogin {
            process,
            credential,
            credential_field,
        } => {
            session.credential_field = Some(credential_field);
            session.delegated_credential = Some(credential);
            start_delegated_async(services, &mut session, process).await?;
        }
        SignInMethod::ApiKeyCollection { fields } => {
            start_api_key(&mut session, fields)?;
        }
    }
    Ok(session)
}

fn prepare_session(
    services: &HostServices,
    request: SignInStartRequest,
) -> Result<(SignInSession, SignInMethod), SignInFailure> {
    let kind = match (&request.method, &request.binding.mechanism) {
        (SignInMethod::InteractiveOauth { .. }, CredentialMechanism::InteractiveOauth) => {
            SignInKind::InteractiveOauth
        }
        (SignInMethod::DeviceOauth { .. }, CredentialMechanism::DeviceOauth) => {
            SignInKind::DeviceOauth
        }
        (SignInMethod::DelegatedCliLogin { .. }, CredentialMechanism::GatewayHelper) => {
            SignInKind::DelegatedCliLogin
        }
        (SignInMethod::ApiKeyCollection { .. }, CredentialMechanism::ApiKey) => {
            SignInKind::ApiKeyCollection
        }
        _ => {
            return Err(SignInFailure::new(
                SignInFailureKind::MechanismMismatch,
                "swallowtail.sign_in.mechanism_mismatch",
                "Sign-in method does not match the frozen credential mechanism",
            ));
        }
    };
    if request.deadline.is_some() && services.time().is_none() {
        return Err(SignInFailure::missing_port(HostServiceKind::Time));
    }

    let session = SignInSession {
        scope: request.scope,
        instance_id: request.instance_id,
        family: request.family,
        route_id: request.route_id,
        binding: request.binding,
        kind,
        deadline: request.deadline,
        status: SignInStatus::Started,
        loopback: None,
        loopback_receipt: None,
        device_receipt: None,
        process: None,
        delegated_credential: None,
        credential_field: None,
        api_key_fields: BTreeMap::new(),
        submitted_refs: BTreeMap::new(),
        pending_poll: Mutex::new(None),
        pending_loopback_release: Mutex::new(None),
    };
    Ok((session, request.method))
}

fn start_api_key(
    session: &mut SignInSession,
    fields: Vec<CredentialFieldDescriptor>,
) -> Result<(), SignInFailure> {
    if fields.is_empty() {
        return Err(SignInFailure::new(
            SignInFailureKind::UnknownCredentialField,
            "swallowtail.sign_in.unknown_credential_field",
            "API-key collection requires advertised credential-field descriptors",
        ));
    }
    session.api_key_fields = fields
        .into_iter()
        .map(|field| (field.id().clone(), field))
        .collect();
    session.status = SignInStatus::InProgress;
    Ok(())
}

/// Observes timeout, host callback, device finish, or helper exit.
///
/// Pending host port futures advance one poll without panicking and leave the
/// session in [`SignInStatus::InProgress`]. Timeout releases host leases and
/// stops/reaps any child process; if cleanup returns `Poll::Pending`, this
/// call fails closed with [`SignInFailureKind::HostPort`] without transitioning
/// to [`SignInStatus::TimedOut`] so callers can finish cleanup with
/// [`poll_sign_in_async`] or [`cancel_sign_in_async`].
pub fn poll_sign_in(
    session: &mut SignInSession,
    services: &HostServices,
) -> Result<SignInStatus, SignInFailure> {
    reject_terminal(session)?;
    if timed_out(session, services)? {
        cleanup_session_now(session, services)?;
        session.status = SignInStatus::TimedOut;
        return Ok(session.status);
    }
    match session.kind {
        SignInKind::InteractiveOauth => poll_interactive(session, services)?,
        SignInKind::DeviceOauth => poll_device(session, services)?,
        SignInKind::DelegatedCliLogin => poll_delegated(session)?,
        SignInKind::ApiKeyCollection => poll_api_key(session),
    }
    Ok(session.status)
}

/// Asynchronously observes timeout, host callback, device finish, or helper exit.
///
/// When a deadline is set on the session, awaiting a host callback, device
/// authorization, or delegated login helper races against
/// [`crate::TimeService::wait_until`] and cleans up host leases and child
/// processes if the deadline elapses while waiting.
pub async fn poll_sign_in_async(
    session: &mut SignInSession,
    services: &HostServices,
) -> Result<SignInStatus, SignInFailure> {
    reject_terminal(session)?;
    if timed_out(session, services)? {
        cleanup_session_async(session, services).await;
        session.status = SignInStatus::TimedOut;
        return Ok(session.status);
    }
    match session.kind {
        SignInKind::InteractiveOauth => poll_interactive_async(session, services).await?,
        SignInKind::DeviceOauth => poll_device_async(session, services).await?,
        SignInKind::DelegatedCliLogin => poll_delegated_async(session, services).await?,
        SignInKind::ApiKeyCollection => poll_api_key(session),
    }
    Ok(session.status)
}

fn poll_api_key(session: &mut SignInSession) {
    if session.submitted_refs.len() == session.api_key_fields.len() {
        session.status = SignInStatus::ReadyToComplete;
    } else {
        session.status = SignInStatus::InProgress;
    }
}

/// Submits one host-stored API-key field as an opaque credential reference.
///
/// The host already holds the secret bytes. This call never accepts secret
/// bytes and does not acquire a Contract 014 lease.
pub fn submit_sign_in_credential_field(
    session: &mut SignInSession,
    field: CredentialFieldId,
    reference: CredentialRef,
) -> Result<SignInStatus, SignInFailure> {
    reject_terminal(session)?;
    if session.kind != SignInKind::ApiKeyCollection {
        return Err(SignInFailure::new(
            SignInFailureKind::InvalidState,
            "swallowtail.sign_in.invalid_state",
            "Credential-field submit is only valid for API-key collection",
        ));
    }
    if !session.api_key_fields.contains_key(&field) {
        return Err(SignInFailure::new(
            SignInFailureKind::UnknownCredentialField,
            "swallowtail.sign_in.unknown_credential_field",
            "Submitted credential field is not advertised on this sign-in loop",
        ));
    }
    session.submitted_refs.insert(field, reference);
    if session.submitted_refs.len() == session.api_key_fields.len() {
        session.status = SignInStatus::ReadyToComplete;
    } else {
        session.status = SignInStatus::InProgress;
    }
    Ok(session.status)
}

/// Completes a ready sign-in loop and materializes opaque credential references.
///
/// When `store` is present, references are written onto the admitted instance.
/// Contract 014 still owns acquire, audience binding, redaction, and release.
/// Releases any held host leases and stops/reaps any child process on both
/// success and error paths; if cleanup returns `Poll::Pending`, fails closed
/// with [`SignInFailureKind::HostPort`] without mutating the store. Use
/// [`complete_sign_in_async`] when host cleanup ports are asynchronous.
pub fn complete_sign_in(
    mut session: SignInSession,
    services: &HostServices,
    store: Option<&dyn ConnectionLifecycleStore>,
) -> Result<SignInOutcome, SignInFailure> {
    let refs = validate_complete(&session, services);
    cleanup_session_now(&mut session, services)?;
    let refs = refs?;
    if let Some(store) = store {
        persist_refs(
            store,
            &session.instance_id,
            &session.family,
            &session.route_id,
            &refs,
        )?;
    }
    session.status = SignInStatus::Completed;
    Ok(build_outcome(session, refs))
}

/// Asynchronously completes a ready sign-in loop and materializes opaque credential references.
///
/// Releases any held host leases and stops/reaps any child process on both
/// success and error paths.
pub async fn complete_sign_in_async(
    mut session: SignInSession,
    services: &HostServices,
    store: Option<&dyn ConnectionLifecycleStore>,
) -> Result<SignInOutcome, SignInFailure> {
    let refs = validate_complete(&session, services);
    cleanup_session_async(&mut session, services).await;
    let refs = refs?;
    if let Some(store) = store {
        persist_refs(
            store,
            &session.instance_id,
            &session.family,
            &session.route_id,
            &refs,
        )?;
    }
    session.status = SignInStatus::Completed;
    Ok(build_outcome(session, refs))
}

fn validate_complete(
    session: &SignInSession,
    services: &HostServices,
) -> Result<BTreeMap<CredentialFieldId, CredentialRef>, SignInFailure> {
    if session.status == SignInStatus::TimedOut {
        return Err(SignInFailure::new(
            SignInFailureKind::InvalidState,
            "swallowtail.sign_in.timed_out",
            "Sign-in loop timed out",
        ));
    }
    if session.status == SignInStatus::Cancelled {
        return Err(SignInFailure::new(
            SignInFailureKind::InvalidState,
            "swallowtail.sign_in.cancelled",
            "Sign-in loop was cancelled",
        ));
    }
    if session.status != SignInStatus::ReadyToComplete {
        return Err(SignInFailure::new(
            SignInFailureKind::Incomplete,
            "swallowtail.sign_in.incomplete",
            "Sign-in loop is not ready to complete",
        ));
    }

    let refs = match session.kind {
        SignInKind::InteractiveOauth => materialize_interactive(session, services)?,
        SignInKind::DeviceOauth => materialize_device(session, services)?,
        SignInKind::DelegatedCliLogin => materialize_delegated(session)?,
        SignInKind::ApiKeyCollection => session.submitted_refs.clone(),
    };
    reject_account_change(&session.binding, &refs)?;

    Ok(refs)
}

fn build_outcome(
    session: SignInSession,
    refs: BTreeMap<CredentialFieldId, CredentialRef>,
) -> SignInOutcome {
    SignInOutcome {
        instance_id: session.instance_id,
        route_id: session.route_id,
        audience: session.binding.audience,
        mechanism: session.binding.mechanism,
        billing: session.binding.billing,
        credential_refs: refs,
    }
}

/// Cancels an in-flight sign-in loop and releases host port leases.
///
/// Fails closed with [`SignInFailureKind::HostPort`] if cleanup futures return
/// `Poll::Pending`; use [`cancel_sign_in_async`] when host ports are asynchronous.
pub fn cancel_sign_in(
    mut session: SignInSession,
    services: &HostServices,
) -> Result<SignInStatus, SignInFailure> {
    reject_terminal(&session)?;
    cleanup_session_now(&mut session, services)?;
    Ok(SignInStatus::Cancelled)
}

/// Asynchronously cancels an in-flight sign-in loop, releases host port leases,
/// and stops and reaps any child process.
pub async fn cancel_sign_in_async(
    mut session: SignInSession,
    services: &HostServices,
) -> Result<SignInStatus, SignInFailure> {
    reject_terminal(&session)?;
    cleanup_session_async(&mut session, services).await;
    Ok(SignInStatus::Cancelled)
}

fn start_interactive(
    services: &HostServices,
    session: &mut SignInSession,
    url: ApprovedUrlRef,
) -> Result<(), SignInFailure> {
    let (loopback, url_open) = interactive_ports(services)?;
    let lease = poll_ready_or_pending_error(
        loopback.bind(session.scope.clone()),
        "Sign-in loopback bind did not resolve immediately; use start_sign_in_async",
    )?;
    let opened = poll_ready_or_pending_error(
        url_open.open(session.scope.clone(), url),
        "Sign-in URL open did not resolve immediately; use start_sign_in_async",
    );
    if let Err(failure) = opened {
        if release_loopback_lease_now(services, lease) {
            return Err(SignInFailure::new(
                SignInFailureKind::HostPort,
                "swallowtail.sign_in.host_port_pending",
                "Sign-in loopback release did not resolve immediately; use start_sign_in_async",
            ));
        }
        return Err(failure);
    }
    session.loopback = Some(lease);
    session.status = SignInStatus::Started;
    Ok(())
}

async fn start_interactive_async(
    services: &HostServices,
    session: &mut SignInSession,
    url: ApprovedUrlRef,
) -> Result<(), SignInFailure> {
    let (loopback, url_open) = interactive_ports(services)?;
    let lease = loopback
        .bind(session.scope.clone())
        .await
        .map_err(SignInFailure::from_port)?;
    if let Err(failure) = url_open.open(session.scope.clone(), url).await {
        release_loopback_async(services, Some(lease)).await;
        return Err(SignInFailure::from_port(failure));
    }
    session.loopback = Some(lease);
    session.status = SignInStatus::Started;
    Ok(())
}

fn interactive_ports(
    services: &HostServices,
) -> Result<
    (
        &dyn crate::LoopbackCallbackService,
        &dyn crate::UrlOpenService,
    ),
    SignInFailure,
> {
    let loopback = services
        .loopback_callback()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::LoopbackCallback))?;
    let url_open = services
        .url_open()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::UrlOpen))?;
    Ok((loopback.as_ref(), url_open.as_ref()))
}

fn start_device(
    services: &HostServices,
    session: &mut SignInSession,
    prompt: DeviceCodePrompt,
) -> Result<(), SignInFailure> {
    let display = services
        .device_code_display()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::DeviceCodeDisplay))?;
    poll_ready_or_pending_error(
        display.display(session.scope.clone(), prompt),
        "Sign-in device display did not resolve immediately; use start_sign_in_async",
    )?;
    session.status = SignInStatus::Started;
    Ok(())
}

async fn start_device_async(
    services: &HostServices,
    session: &mut SignInSession,
    prompt: DeviceCodePrompt,
) -> Result<(), SignInFailure> {
    let display = services
        .device_code_display()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::DeviceCodeDisplay))?;
    display
        .display(session.scope.clone(), prompt)
        .await
        .map_err(SignInFailure::from_port)?;
    session.status = SignInStatus::Started;
    Ok(())
}

fn start_delegated(
    services: &HostServices,
    session: &mut SignInSession,
    request: ProcessRequest,
) -> Result<(), SignInFailure> {
    let process = services
        .process()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::Process))?;
    let handle = poll_ready_or_pending_error(
        process.start(session.scope.clone(), request),
        "Sign-in process start did not resolve immediately; use start_sign_in_async",
    )?;
    session.process = Some(Arc::from(handle));
    session.status = SignInStatus::Started;
    Ok(())
}

async fn start_delegated_async(
    services: &HostServices,
    session: &mut SignInSession,
    request: ProcessRequest,
) -> Result<(), SignInFailure> {
    let process = services
        .process()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::Process))?;
    let handle = process
        .start(session.scope.clone(), request)
        .await
        .map_err(SignInFailure::from_port)?;
    session.process = Some(Arc::from(handle));
    session.status = SignInStatus::Started;
    Ok(())
}

fn poll_interactive(
    session: &mut SignInSession,
    services: &HostServices,
) -> Result<(), SignInFailure> {
    let mut future = match take_pending_poll(session) {
        Some(PendingSignInPoll::Loopback(future)) => future,
        _ => {
            let lease = session.loopback.as_ref().ok_or_else(|| {
                SignInFailure::new(
                    SignInFailureKind::InvalidState,
                    "swallowtail.sign_in.invalid_state",
                    "Interactive sign-in is missing its loopback lease",
                )
            })?;
            let loopback = services
                .loopback_callback()
                .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::LoopbackCallback))?;
            loopback.poll(lease)
        }
    };
    match poll_box_future_once(&mut future) {
        Poll::Pending => {
            set_pending_poll(session, PendingSignInPoll::Loopback(future));
            session.status = SignInStatus::InProgress;
        }
        Poll::Ready(Ok(Some(receipt))) => {
            session.loopback_receipt = Some(receipt);
            session.status = SignInStatus::ReadyToComplete;
        }
        Poll::Ready(Ok(None)) => {
            session.status = SignInStatus::InProgress;
        }
        Poll::Ready(Err(failure)) => return Err(SignInFailure::from_port(failure)),
    }
    Ok(())
}

async fn poll_interactive_async(
    session: &mut SignInSession,
    services: &HostServices,
) -> Result<(), SignInFailure> {
    let future = match take_pending_poll(session) {
        Some(PendingSignInPoll::Loopback(future)) => future,
        _ => {
            let lease = session.loopback.as_ref().ok_or_else(|| {
                SignInFailure::new(
                    SignInFailureKind::InvalidState,
                    "swallowtail.sign_in.invalid_state",
                    "Interactive sign-in is missing its loopback lease",
                )
            })?;
            let loopback = services
                .loopback_callback()
                .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::LoopbackCallback))?;
            loopback.poll(lease)
        }
    };
    match await_with_deadline(future, session.deadline, services).await? {
        DeadlineRace::TimedOut => {
            cleanup_session_async(session, services).await;
            session.status = SignInStatus::TimedOut;
        }
        DeadlineRace::Completed(result) => match result.map_err(SignInFailure::from_port)? {
            Some(receipt) => {
                session.loopback_receipt = Some(receipt);
                session.status = SignInStatus::ReadyToComplete;
            }
            None => {
                session.status = SignInStatus::InProgress;
            }
        },
    }
    Ok(())
}

fn poll_device(session: &mut SignInSession, services: &HostServices) -> Result<(), SignInFailure> {
    let mut future = match take_pending_poll(session) {
        Some(PendingSignInPoll::Device(future)) => future,
        _ => {
            let display = services
                .device_code_display()
                .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::DeviceCodeDisplay))?;
            display.poll_authorization(&session.scope)
        }
    };
    match poll_box_future_once(&mut future) {
        Poll::Pending => {
            set_pending_poll(session, PendingSignInPoll::Device(future));
            session.status = SignInStatus::InProgress;
        }
        Poll::Ready(Ok(Some(receipt))) => {
            session.device_receipt = Some(receipt);
            session.status = SignInStatus::ReadyToComplete;
        }
        Poll::Ready(Ok(None)) => {
            session.status = SignInStatus::InProgress;
        }
        Poll::Ready(Err(failure)) => return Err(SignInFailure::from_port(failure)),
    }
    Ok(())
}

async fn poll_device_async(
    session: &mut SignInSession,
    services: &HostServices,
) -> Result<(), SignInFailure> {
    let future = match take_pending_poll(session) {
        Some(PendingSignInPoll::Device(future)) => future,
        _ => {
            let display = services
                .device_code_display()
                .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::DeviceCodeDisplay))?;
            display.poll_authorization(&session.scope)
        }
    };
    match await_with_deadline(future, session.deadline, services).await? {
        DeadlineRace::TimedOut => {
            cleanup_session_async(session, services).await;
            session.status = SignInStatus::TimedOut;
        }
        DeadlineRace::Completed(result) => match result.map_err(SignInFailure::from_port)? {
            Some(receipt) => {
                session.device_receipt = Some(receipt);
                session.status = SignInStatus::ReadyToComplete;
            }
            None => {
                session.status = SignInStatus::InProgress;
            }
        },
    }
    Ok(())
}

fn poll_delegated(session: &mut SignInSession) -> Result<(), SignInFailure> {
    let mut wait = match take_pending_poll(session) {
        Some(PendingSignInPoll::Delegated(wait)) => wait,
        _ => {
            let handle = session.process.as_ref().ok_or_else(|| {
                SignInFailure::new(
                    SignInFailureKind::InvalidState,
                    "swallowtail.sign_in.invalid_state",
                    "Delegated CLI login is missing its process handle",
                )
            })?;
            // Own the handle inside the retained future so the pending wait
            // survives across polls instead of being dropped after one poll.
            let handle = Arc::clone(handle);
            Box::pin(async move { handle.wait().await })
        }
    };
    match poll_box_future_once(&mut wait) {
        Poll::Pending => {
            set_pending_poll(session, PendingSignInPoll::Delegated(wait));
            session.status = SignInStatus::InProgress;
            Ok(())
        }
        Poll::Ready(result) => finish_delegated_wait(session, result),
    }
}

async fn poll_delegated_async(
    session: &mut SignInSession,
    services: &HostServices,
) -> Result<(), SignInFailure> {
    let wait_outcome = {
        let Some(handle) = session.process.as_ref() else {
            return Err(SignInFailure::new(
                SignInFailureKind::InvalidState,
                "swallowtail.sign_in.invalid_state",
                "Delegated CLI login is missing its process handle",
            ));
        };
        await_with_deadline(handle.wait(), session.deadline, services).await?
    };
    match wait_outcome {
        DeadlineRace::Completed(result) => finish_delegated_wait(session, result),
        DeadlineRace::TimedOut => {
            cleanup_session_async(session, services).await;
            session.status = SignInStatus::TimedOut;
            Ok(())
        }
    }
}

fn finish_delegated_wait(
    session: &mut SignInSession,
    result: Result<crate::ProcessExit, RuntimeFailure>,
) -> Result<(), SignInFailure> {
    session.process = None;
    match result {
        Ok(exit) if exit.success() => {
            session.status = SignInStatus::ReadyToComplete;
            Ok(())
        }
        Ok(_) => Err(SignInFailure::new(
            SignInFailureKind::HostPort,
            "swallowtail.sign_in.login_helper_failed",
            "Approved login helper exited unsuccessfully",
        )),
        Err(failure) => Err(SignInFailure::from_port(failure)),
    }
}

fn materialize_interactive(
    session: &SignInSession,
    services: &HostServices,
) -> Result<BTreeMap<CredentialFieldId, CredentialRef>, SignInFailure> {
    let receipt = session.loopback_receipt.as_ref().ok_or_else(|| {
        SignInFailure::new(
            SignInFailureKind::Incomplete,
            "swallowtail.sign_in.incomplete",
            "Interactive sign-in has no loopback receipt",
        )
    })?;
    let field = session.credential_field.clone().ok_or_else(|| {
        SignInFailure::new(
            SignInFailureKind::UnknownCredentialField,
            "swallowtail.sign_in.unknown_credential_field",
            "Interactive sign-in is missing its credential field",
        )
    })?;
    let loopback = services
        .loopback_callback()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::LoopbackCallback))?;
    let reference = loopback
        .materialize_credential(receipt, session.binding.audience())
        .map_err(SignInFailure::from_port)?;
    Ok(BTreeMap::from([(field, reference)]))
}

fn materialize_device(
    session: &SignInSession,
    services: &HostServices,
) -> Result<BTreeMap<CredentialFieldId, CredentialRef>, SignInFailure> {
    let receipt = session.device_receipt.as_ref().ok_or_else(|| {
        SignInFailure::new(
            SignInFailureKind::Incomplete,
            "swallowtail.sign_in.incomplete",
            "Device sign-in has no authorization receipt",
        )
    })?;
    let field = session.credential_field.clone().ok_or_else(|| {
        SignInFailure::new(
            SignInFailureKind::UnknownCredentialField,
            "swallowtail.sign_in.unknown_credential_field",
            "Device sign-in is missing its credential field",
        )
    })?;
    let display = services
        .device_code_display()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::DeviceCodeDisplay))?;
    let reference = display
        .materialize_credential(receipt, session.binding.audience())
        .map_err(SignInFailure::from_port)?;
    Ok(BTreeMap::from([(field, reference)]))
}

fn materialize_delegated(
    session: &SignInSession,
) -> Result<BTreeMap<CredentialFieldId, CredentialRef>, SignInFailure> {
    let field = session.credential_field.clone().ok_or_else(|| {
        SignInFailure::new(
            SignInFailureKind::UnknownCredentialField,
            "swallowtail.sign_in.unknown_credential_field",
            "Delegated CLI login is missing its credential field",
        )
    })?;
    let reference = session.delegated_credential.clone().ok_or_else(|| {
        SignInFailure::new(
            SignInFailureKind::Incomplete,
            "swallowtail.sign_in.incomplete",
            "Delegated CLI login is missing its credential reference",
        )
    })?;
    Ok(BTreeMap::from([(field, reference)]))
}

fn reject_account_change(
    binding: &SignInAuthorityBinding,
    refs: &BTreeMap<CredentialFieldId, CredentialRef>,
) -> Result<(), SignInFailure> {
    let Some(existing) = binding.existing_credential.as_ref() else {
        return Ok(());
    };
    if refs.values().any(|reference| reference != existing) {
        return Err(SignInFailure::new(
            SignInFailureKind::AccountMismatch,
            "swallowtail.sign_in.account_mismatch",
            "Sign-in complete would change the bound account",
        ));
    }
    Ok(())
}

fn persist_refs(
    store: &dyn ConnectionLifecycleStore,
    instance_id: &ConfiguredInstanceId,
    family: &IntegrationFamilyId,
    route_id: &AddableRouteId,
    refs: &BTreeMap<CredentialFieldId, CredentialRef>,
) -> Result<AdmittedInstanceRecord, SignInFailure> {
    let mut record = store
        .get_instance(instance_id)
        .map_err(SignInFailure::from_store)?
        .ok_or_else(|| {
            SignInFailure::new(
                SignInFailureKind::InstanceAbsent,
                "swallowtail.sign_in.instance_absent",
                "Sign-in complete requires an admitted instance in the store",
            )
        })?;
    if record.family() != family || record.route_id() != route_id {
        return Err(SignInFailure::new(
            SignInFailureKind::InvalidState,
            "swallowtail.sign_in.route_mismatch",
            "Sign-in complete must use the same route that started the loop",
        ));
    }
    let mut merged: BTreeMap<_, _> = record
        .credential_refs()
        .map(|(id, reference)| (id.clone(), reference.clone()))
        .collect();
    merged.extend(
        refs.iter()
            .map(|(id, reference)| (id.clone(), reference.clone())),
    );
    record = record.with_credential_refs(merged);
    store
        .put_instance(record.clone())
        .map_err(SignInFailure::from_store)?;
    Ok(record)
}

fn timed_out(session: &SignInSession, services: &HostServices) -> Result<bool, SignInFailure> {
    let Some(deadline) = session.deadline else {
        return Ok(false);
    };
    let time = services
        .time()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::Time))?;
    Ok(time.now() >= deadline.instant())
}

fn reject_terminal(session: &SignInSession) -> Result<(), SignInFailure> {
    if matches!(
        session.status,
        SignInStatus::Completed | SignInStatus::Cancelled | SignInStatus::TimedOut
    ) {
        return Err(SignInFailure::new(
            SignInFailureKind::InvalidState,
            "swallowtail.sign_in.invalid_state",
            "Sign-in loop is already terminal",
        ));
    }
    Ok(())
}

fn take_pending_poll(session: &SignInSession) -> Option<PendingSignInPoll> {
    session
        .pending_poll
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .take()
}

fn set_pending_poll(session: &SignInSession, pending: PendingSignInPoll) {
    *session
        .pending_poll
        .lock()
        .unwrap_or_else(PoisonError::into_inner) = Some(pending);
}

fn take_pending_loopback_release(
    session: &SignInSession,
) -> Option<BoxFuture<'static, crate::CleanupOutcome>> {
    session
        .pending_loopback_release
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .take()
}

fn set_pending_loopback_release(
    session: &SignInSession,
    pending: BoxFuture<'static, crate::CleanupOutcome>,
) {
    *session
        .pending_loopback_release
        .lock()
        .unwrap_or_else(PoisonError::into_inner) = Some(pending);
}

fn cleanup_session_now(
    session: &mut SignInSession,
    services: &HostServices,
) -> Result<(), SignInFailure> {
    let _ = take_pending_poll(session);
    let mut pending_cleanup = release_loopback_now(session, services);
    if let Some(process) = session.process.take() {
        let stop_pending = poll_now_or_pending(process.request_stop()).is_none();
        let force_pending = poll_now_or_pending(process.force_stop()).is_none();
        let wait_pending = poll_now_or_pending(process.wait()).is_none();
        if stop_pending || force_pending || wait_pending {
            session.process = Some(process);
            pending_cleanup = true;
        }
    }
    if pending_cleanup {
        return Err(SignInFailure::new(
            SignInFailureKind::HostPort,
            "swallowtail.sign_in.host_port_pending",
            "Sign-in cleanup did not resolve immediately; use *_sign_in_async",
        ));
    }
    Ok(())
}

async fn cleanup_session_async(session: &mut SignInSession, services: &HostServices) {
    let _ = take_pending_poll(session);
    if let Some(future) = take_pending_loopback_release(session) {
        let _ = future.await;
    }
    release_loopback_async(services, session.loopback.take()).await;
    if let Some(process) = session.process.take() {
        let _ = process.request_stop().await;
        let _ = process.force_stop().await;
        let _ = process.wait().await;
    }
}

fn release_loopback_now(session: &mut SignInSession, services: &HostServices) -> bool {
    let mut future = if let Some(future) = take_pending_loopback_release(session) {
        future
    } else if let Some(lease) = session.loopback.take() {
        let Some(loopback) = services.loopback_callback() else {
            return false;
        };
        loopback.release(lease)
    } else {
        return false;
    };
    match poll_box_future_once(&mut future) {
        Poll::Ready(_) => false,
        Poll::Pending => {
            set_pending_loopback_release(session, future);
            true
        }
    }
}

fn release_loopback_lease_now(
    services: &HostServices,
    lease: crate::LoopbackCallbackLease,
) -> bool {
    let Some(loopback) = services.loopback_callback() else {
        return false;
    };
    poll_now_or_pending(loopback.release(lease)).is_none()
}

async fn release_loopback_async(
    services: &HostServices,
    lease: Option<crate::LoopbackCallbackLease>,
) {
    let Some(lease) = lease else {
        return;
    };
    let Some(loopback) = services.loopback_callback() else {
        return;
    };
    let _ = loopback.release(lease).await;
}

enum DeadlineRace<T> {
    Completed(T),
    TimedOut,
}

async fn await_with_deadline<T>(
    future: impl Future<Output = T>,
    deadline: Option<Deadline>,
    services: &HostServices,
) -> Result<DeadlineRace<T>, SignInFailure> {
    let Some(deadline) = deadline else {
        return Ok(DeadlineRace::Completed(future.await));
    };
    let time = services
        .time()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::Time))?;
    if time.now() >= deadline.instant() {
        return Ok(DeadlineRace::TimedOut);
    }
    let mut future = std::pin::pin!(future);
    let mut timer = None;
    let outcome = std::future::poll_fn(|context| {
        if let Poll::Ready(value) = future.as_mut().poll(context) {
            return Poll::Ready(DeadlineRace::Completed(value));
        }
        if time.now() >= deadline.instant() {
            return Poll::Ready(DeadlineRace::TimedOut);
        }
        let timer = timer.get_or_insert_with(|| time.wait_until(deadline));
        if timer.as_mut().poll(context).is_ready() {
            Poll::Ready(DeadlineRace::TimedOut)
        } else {
            Poll::Pending
        }
    })
    .await;
    Ok(outcome)
}

fn poll_ready_or_pending_error<T>(
    future: impl Future<Output = Result<T, RuntimeFailure>>,
    pending_message: &'static str,
) -> Result<T, SignInFailure> {
    match poll_now_or_pending(future) {
        Some(Ok(value)) => Ok(value),
        Some(Err(failure)) => Err(SignInFailure::from_port(failure)),
        None => Err(SignInFailure::new(
            SignInFailureKind::HostPort,
            "swallowtail.sign_in.host_port_pending",
            pending_message,
        )),
    }
}

fn poll_box_future_once<T>(future: &mut BoxFuture<'static, T>) -> Poll<T> {
    let mut context = Context::from_waker(Waker::noop());
    future.as_mut().poll(&mut context)
}

fn poll_now_or_pending<T>(future: impl Future<Output = T>) -> Option<T> {
    let mut future = std::pin::pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    match Pin::as_mut(&mut future).poll(&mut context) {
        Poll::Ready(value) => Some(value),
        Poll::Pending => None,
    }
}
