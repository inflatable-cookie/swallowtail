//! Library-owned interactive sign-in loop for Contract 057.
//!
//! Swallowtail owns start, poll, complete, cancel, and timeout. Host ports open
//! a URL, bind a loopback callback, display a device code, or spawn an approved
//! login helper through process authority. [`swallowtail_core::SignInAction`]
//! remains an advertisement. ACP authenticate and Contract 017 delegated login
//! are not this loop.

use super::{ConnectionLifecycleStore, ConnectionLifecycleStoreFailure};
use crate::{
    ApprovedUrlRef, CredentialRef, Deadline, DeviceCodePrompt, HostServices, ProcessHandle,
    ProcessRequest, RuntimeFailure, ScopeId,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
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
    process: Option<Box<dyn ProcessHandle>>,
    delegated_credential: Option<CredentialRef>,
    credential_field: Option<CredentialFieldId>,
    api_key_fields: BTreeMap<CredentialFieldId, CredentialFieldDescriptor>,
    submitted_refs: BTreeMap<CredentialFieldId, CredentialRef>,
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
pub fn start_sign_in(
    services: &HostServices,
    request: SignInStartRequest,
) -> Result<SignInSession, SignInFailure> {
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

    let mut session = SignInSession {
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
    };

    match request.method {
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
        }
    }
    Ok(session)
}

/// Observes timeout, host callback, device finish, or helper exit.
pub fn poll_sign_in(
    session: &mut SignInSession,
    services: &HostServices,
) -> Result<SignInStatus, SignInFailure> {
    reject_terminal(session)?;
    if timed_out(session, services)? {
        session.status = SignInStatus::TimedOut;
        return Ok(session.status);
    }
    match session.kind {
        SignInKind::InteractiveOauth => poll_interactive(session, services)?,
        SignInKind::DeviceOauth => poll_device(session, services)?,
        SignInKind::DelegatedCliLogin => poll_delegated(session)?,
        SignInKind::ApiKeyCollection => {
            if session.submitted_refs.len() == session.api_key_fields.len() {
                session.status = SignInStatus::ReadyToComplete;
            } else {
                session.status = SignInStatus::InProgress;
            }
        }
    }
    Ok(session.status)
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
pub fn complete_sign_in(
    mut session: SignInSession,
    services: &HostServices,
    store: Option<&dyn ConnectionLifecycleStore>,
) -> Result<SignInOutcome, SignInFailure> {
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
        SignInKind::InteractiveOauth => materialize_interactive(&session, services)?,
        SignInKind::DeviceOauth => materialize_device(&session, services)?,
        SignInKind::DelegatedCliLogin => materialize_delegated(&session)?,
        SignInKind::ApiKeyCollection => session.submitted_refs.clone(),
    };
    reject_account_change(&session.binding, &refs)?;

    if let Some(store) = store {
        persist_refs(
            store,
            &session.instance_id,
            &session.family,
            &session.route_id,
            &refs,
        )?;
    }

    release_loopback(services, session.loopback.take());
    session.status = SignInStatus::Completed;
    Ok(SignInOutcome {
        instance_id: session.instance_id,
        route_id: session.route_id,
        audience: session.binding.audience,
        mechanism: session.binding.mechanism,
        billing: session.binding.billing,
        credential_refs: refs,
    })
}

/// Cancels an in-flight sign-in loop and releases host port leases.
pub fn cancel_sign_in(
    mut session: SignInSession,
    services: &HostServices,
) -> Result<SignInStatus, SignInFailure> {
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
    release_loopback(services, session.loopback.take());
    if let Some(process) = session.process.take() {
        let _ = poll_now(process.request_stop());
        let _ = poll_now(process.force_stop());
    }
    Ok(SignInStatus::Cancelled)
}

fn start_interactive(
    services: &HostServices,
    session: &mut SignInSession,
    url: ApprovedUrlRef,
) -> Result<(), SignInFailure> {
    let loopback = services
        .loopback_callback()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::LoopbackCallback))?;
    let url_open = services
        .url_open()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::UrlOpen))?;
    let lease = poll_now(loopback.bind(session.scope.clone())).map_err(SignInFailure::from_port)?;
    poll_now(url_open.open(session.scope.clone(), url)).map_err(SignInFailure::from_port)?;
    session.loopback = Some(lease);
    session.status = SignInStatus::Started;
    Ok(())
}

fn start_device(
    services: &HostServices,
    session: &mut SignInSession,
    prompt: DeviceCodePrompt,
) -> Result<(), SignInFailure> {
    let display = services
        .device_code_display()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::DeviceCodeDisplay))?;
    poll_now(display.display(session.scope.clone(), prompt)).map_err(SignInFailure::from_port)?;
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
    let handle = poll_now(process.start(session.scope.clone(), request))
        .map_err(SignInFailure::from_port)?;
    session.process = Some(handle);
    session.status = SignInStatus::Started;
    Ok(())
}

fn poll_interactive(
    session: &mut SignInSession,
    services: &HostServices,
) -> Result<(), SignInFailure> {
    let Some(lease) = session.loopback.as_ref() else {
        return Err(SignInFailure::new(
            SignInFailureKind::InvalidState,
            "swallowtail.sign_in.invalid_state",
            "Interactive sign-in is missing its loopback lease",
        ));
    };
    let loopback = services
        .loopback_callback()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::LoopbackCallback))?;
    if let Some(receipt) = poll_now(loopback.poll(lease)).map_err(SignInFailure::from_port)? {
        session.loopback_receipt = Some(receipt);
        session.status = SignInStatus::ReadyToComplete;
    } else {
        session.status = SignInStatus::InProgress;
    }
    Ok(())
}

fn poll_device(session: &mut SignInSession, services: &HostServices) -> Result<(), SignInFailure> {
    let display = services
        .device_code_display()
        .ok_or_else(|| SignInFailure::missing_port(HostServiceKind::DeviceCodeDisplay))?;
    if let Some(receipt) =
        poll_now(display.poll_authorization(&session.scope)).map_err(SignInFailure::from_port)?
    {
        session.device_receipt = Some(receipt);
        session.status = SignInStatus::ReadyToComplete;
    } else {
        session.status = SignInStatus::InProgress;
    }
    Ok(())
}

fn poll_delegated(session: &mut SignInSession) -> Result<(), SignInFailure> {
    let Some(handle) = session.process.as_ref() else {
        return Err(SignInFailure::new(
            SignInFailureKind::InvalidState,
            "swallowtail.sign_in.invalid_state",
            "Delegated CLI login is missing its process handle",
        ));
    };
    match poll_now_or_pending(handle.wait()) {
        None => session.status = SignInStatus::InProgress,
        Some(Ok(exit)) if exit.success() => session.status = SignInStatus::ReadyToComplete,
        Some(Ok(_)) => {
            return Err(SignInFailure::new(
                SignInFailureKind::HostPort,
                "swallowtail.sign_in.login_helper_failed",
                "Approved login helper exited unsuccessfully",
            ));
        }
        Some(Err(failure)) => return Err(SignInFailure::from_port(failure)),
    }
    Ok(())
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

fn release_loopback(services: &HostServices, lease: Option<crate::LoopbackCallbackLease>) {
    let Some(lease) = lease else {
        return;
    };
    let Some(loopback) = services.loopback_callback() else {
        return;
    };
    let _ = poll_now(loopback.release(lease));
}

fn poll_now<T>(future: impl Future<Output = T>) -> T {
    match poll_now_or_pending(future) {
        Some(value) => value,
        None => panic!("sign-in host port future must be immediately ready"),
    }
}

fn poll_now_or_pending<T>(future: impl Future<Output = T>) -> Option<T> {
    let mut future = std::pin::pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    match Pin::as_mut(&mut future).poll(&mut context) {
        Poll::Ready(value) => Some(value),
        Poll::Pending => None,
    }
}
