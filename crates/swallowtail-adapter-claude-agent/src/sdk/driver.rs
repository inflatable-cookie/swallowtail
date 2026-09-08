//! Fresh-session driver for the source-tagged Claude Agent SDK Node sidecar.
//!
//! The driver owns no launch argv: the application-approved launch recipe
//! (host-local interpreted Node plus the sidecar entry point) comes from the
//! bound instance target, and the application-provisioned SDK module, native
//! binary, and shipped manifest arrive through the approved environment. The
//! sidecar starts inside the host's descendant-tree authority, so the native
//! binary and everything it spawns stay enrolled in one host-owned tree.

pub use self::session::ClaudeAgentSdkSessionHandle;
use self::validation::{validate_open, validate_resume};
use crate::sdk::bounded::HostBound;
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::{failure, unsupported};
use crate::sdk::guardian::OpenGuard;
use crate::sdk::prepared::ClaudeAgentSdkSessionListing;
use std::sync::{Arc, Mutex};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    LoadSessionRequest, LoadedSession, OpenSessionRequest, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, SessionResumeBinding,
};

use crate::sdk::open_receipt::{
    ClaudeAgentSdkFailedOpenCleanup, ClaudeAgentSdkOpenRejection, OpenFailure, open_rejection,
};

mod descriptor;
mod handle;
mod launch;
pub(in crate::sdk) mod registered;
mod session;
mod startup;
mod validation;

pub(crate) use startup::SessionListing;

pub(super) const SDK_DRIVER_ID: &str = "swallowtail.claude-agent.sdk";

/// Low-level driver for fresh Claude Agent SDK sidecar sessions.
pub struct ClaudeAgentSdkDriver {
    environment: EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
    profile: crate::sdk::profile::ClaudeAgentSdkSessionProfile,
    mcp_servers: Vec<crate::sdk::mcp::ClaudeAgentSdkMcpServer>,
    registered: Option<crate::sdk::registered_tool::ClaudeAgentSdkRegisteredToolBinding>,
    selected_skill: Option<Box<swallowtail_runtime::ResolvedSkillBundle>>,
}

impl ClaudeAgentSdkDriver {
    /// Binds the host-private environment and delegated subscription
    /// credential reference. No credential value is ever held here.
    #[must_use]
    pub const fn new(
        environment: EnvironmentRef,
        credential: swallowtail_core::CredentialRef,
    ) -> Self {
        Self {
            environment,
            credential,
            profile: crate::sdk::profile::ClaudeAgentSdkSessionProfile::read_only(),
            mcp_servers: Vec::new(),
            registered: None,
            selected_skill: None,
        }
    }

    /// Binds the admitted tool set and opening permission mode.
    ///
    /// Omitting this keeps the unchanged read-only profile. The profile must
    /// agree with the plan the caller opens against: a write profile against a
    /// read-only plan is refused before any lease, process, or provider
    /// contact exists.
    #[must_use]
    pub const fn with_session_profile(
        mut self,
        profile: crate::sdk::profile::ClaudeAgentSdkSessionProfile,
    ) -> Self {
        self.profile = profile;
        self
    }

    /// Binds declared stdio MCP servers for this driver.
    ///
    /// Omitting this keeps the empty set, so the default open never sends
    /// `mcpServers` and never queries `mcpServerStatus`.
    #[must_use]
    pub fn with_mcp_servers(
        mut self,
        servers: Vec<crate::sdk::mcp::ClaudeAgentSdkMcpServer>,
    ) -> Self {
        self.mcp_servers = servers;
        self
    }

    /// Binds one qualified registered selection to new sidecar sessions.
    ///
    /// Absence preserves every previous open, including the empty `mcpServers`
    /// omission. Resume and listing refuse a bound selection: the courier is
    /// minted at fresh open and is not redeclared onto a retained thread.
    #[must_use]
    pub fn with_registered_tools(
        mut self,
        binding: crate::sdk::registered_tool::ClaudeAgentSdkRegisteredToolBinding,
    ) -> Self {
        self.registered = Some(binding);
        self
    }

    /// Binds one immutable resolved selected-skill bundle to fresh sidecar
    /// sessions.
    ///
    /// The bundle crosses the private wire under its own labelled input. It
    /// is not merged into session instructions or any per-turn user text.
    /// Resumed sessions and listings refuse a bound bundle because they do
    /// not redeclare session-start inputs.
    #[must_use]
    pub fn with_selected_skill_bundle(
        mut self,
        bundle: swallowtail_runtime::ResolvedSkillBundle,
    ) -> Self {
        self.selected_skill = Some(Box::new(bundle));
        self
    }

    /// Binds the profile and its immutable resolved selected-skill bundle.
    #[must_use]
    pub fn with_selected_skill_binding(
        mut self,
        binding: crate::sdk::selected_skill::ClaudeAgentSdkSelectedSkillBinding,
    ) -> Self {
        self.profile = binding.session_profile();
        self.selected_skill = Some(Box::new(binding.bundle().clone()));
        self
    }

    /// Returns the admitted tool set and opening permission mode.
    #[must_use]
    pub const fn session_profile(&self) -> crate::sdk::profile::ClaudeAgentSdkSessionProfile {
        self.profile
    }
}

/// The reap authority one open takes before it acquires anything.
///
/// All of it is granted up front: the open guardian's, the pump's, and the
/// close guardian's. A host that cannot commit every lane refuses the whole
/// operation before a credential, a resource, a process, a task, or any
/// provider contact exists.
pub(super) struct Reservations {
    pub(super) pump: Box<dyn swallowtail_runtime::TaskReapReservation>,
    pub(super) pump_scope: swallowtail_runtime::ScopeId,
    /// The enclosing cleanup guardian, already started before any effect.
    pub(super) close_guardian: crate::sdk::guardian::SessionGuardian,
}

/// Everything one launch needs from the admitted open request.
pub(super) struct SessionLaunch<'a> {
    pub(super) request_id: swallowtail_runtime::RequestId,
    pub(super) working_resource: swallowtail_runtime::WorkingResourceRef,
    pub(super) access_policy: &'a swallowtail_core::SessionAccessPolicy,
    pub(super) reservations: Reservations,
}

/// One spawned sidecar attachment before readiness.
///
/// The leases, process, and pump task are not held here: the open guard holds
/// them until the success path claims them, so a dropped open future cannot
/// strand a partial acquisition.
pub(super) struct PendingSession {
    pub(super) request_id: swallowtail_runtime::RequestId,
    pub(super) connection: Arc<SdkConnection>,
    pub(super) services: HostServices,
    pub(super) leased_cwd: String,
    pub(super) working_resource: swallowtail_runtime::WorkingResourceRef,
    pub(super) access_policy: swallowtail_core::SessionAccessPolicy,
    /// The enclosing cleanup guardian, started before the first acquisition so
    /// activating it at close cannot fail while the session holds live state.
    pub(super) close_guardian: Option<crate::sdk::guardian::SessionGuardian>,
    pub(super) registered: Option<registered::ClaudeAgentSdkRegisteredToolSession>,
}

impl PendingSession {
    pub(in crate::sdk::driver) fn into_handle(
        mut self,
        plan: &PreflightPlan,
        readiness: startup::SessionReadiness,
        acquired: crate::sdk::guardian::Acquisitions,
        resume_binding: Option<SessionResumeBinding>,
    ) -> ClaudeAgentSdkSessionHandle {
        let runtime_id =
            RuntimeSessionId::new(format!("claude-agent-sdk:{}", self.request_id.as_str()))
                .expect("validated request id produces a valid sidecar runtime session id");
        let active: crate::sdk::driver::session::ActiveSlot = Arc::new(Mutex::new(None));
        let permission_mode = readiness.permission_mode();
        ClaudeAgentSdkSessionHandle {
            request_id: self.request_id,
            runtime_id,
            execution_host_id: plan.execution_host_id().clone(),
            connection: Arc::clone(&self.connection),
            cancellation: handle::SessionCancellation::new(Arc::clone(&active)),
            pump_task: acquired.pump,
            close_guardian: self.close_guardian,
            services: self.services,
            resource: acquired.resource,
            credential: acquired.credential,
            plan: plan.clone(),
            working_resource: self.working_resource,
            access_policy: self.access_policy,
            provider_session_ref: resume_binding
                .as_ref()
                .map(|binding| binding.provider_session_ref().clone()),
            resume_binding,
            readiness,
            active,
            permission_mode,
            permission_mode_changes: 0,
            model_changes: 0,
            first_turn_rejection: None,
            registered: self.registered.take(),
        }
    }
}

impl InteractiveSessionDriver for ClaudeAgentSdkDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            self.open_route_session(plan, request, services)
                .await
                .map(|handle| Box::new(handle) as Box<dyn InteractiveSessionHandle>)
        })
    }

    fn load_session(
        &self,
        _plan: PreflightPlan,
        _request: LoadSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
        Box::pin(async { Err(crate::sdk::failure::unsupported("session load")) })
    }

    fn resume_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            self.resume_route_session(plan, request, services, None)
                .await
                .map(|handle| Box::new(handle) as Box<dyn InteractiveSessionHandle>)
        })
    }
}

impl ClaudeAgentSdkDriver {
    /// Opens one fresh session and returns the route-local handle, which adds
    /// mid-session permission-mode control to the shared session surface.
    pub fn open_route_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ClaudeAgentSdkSessionHandle, RuntimeFailure>> {
        Box::pin(async move {
            self.open_route_session_with_receipt(plan, request, services)
                .await
                .map_err(ClaudeAgentSdkOpenRejection::into_failure)
        })
    }

    /// Opens the same fresh session and reports one structured failed-open
    /// receipt when the open fails. The carried failure is byte-identical to
    /// what [`Self::open_route_session`] returns; only the error shape
    /// differs.
    pub(crate) fn open_route_session_with_receipt(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ClaudeAgentSdkSessionHandle, ClaudeAgentSdkOpenRejection>> {
        self.open_with_start(plan, request, services, SessionStart::Fresh)
    }

    /// Resumes the exact provider session in a Contract 017 binding without
    /// replaying its transcript.
    pub fn resume_route_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
        resume_session_at: Option<String>,
    ) -> BoxFuture<'_, Result<ClaudeAgentSdkSessionHandle, RuntimeFailure>> {
        Box::pin(async move {
            let working_resource = request.working_resource().clone();
            let open = OpenSessionRequest::from_plan(
                &plan,
                request.request_id().clone(),
                working_resource,
                request.deadline(),
            )
            .map_err(|_| {
                failure(
                    "swallowtail.claude-agent.sdk.resume_binding_mismatch",
                    "Claude Agent SDK resume request could not be reconstructed from its bound plan",
                )
            })?
            .with_options(request.options().clone());
            self.open_with_start(
                plan,
                open,
                services,
                SessionStart::Resume {
                    binding: Box::new(request.resume_binding().clone()),
                    resume_session_at,
                },
            )
            .await
            .map_err(ClaudeAgentSdkOpenRejection::into_failure)
        })
    }

    /// Lists provider-owned session metadata for the exact leased working
    /// resource. This operation never opens a provider session and never
    /// treats a listing record as resume authority.
    pub fn list_sessions(
        &self,
        plan: PreflightPlan,
        request_id: swallowtail_runtime::RequestId,
        working_resource: swallowtail_runtime::WorkingResourceRef,
        deadline: swallowtail_runtime::Deadline,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ClaudeAgentSdkSessionListing>, RuntimeFailure>> {
        Box::pin(async move {
            if self.registered.is_some() {
                return Err(unsupported("registered tools on session listing"));
            }
            if self.selected_skill.is_some() {
                return Err(unsupported("selected skill bundles on session listing"));
            }
            let request = OpenSessionRequest::from_plan(
                &plan,
                request_id.clone(),
                working_resource.clone(),
                Some(deadline),
            )
            .map_err(|_| {
                failure(
                    "swallowtail.claude-agent.sdk.listing_request_invalid",
                    "Claude Agent SDK session listing request did not match its prepared route",
                )
            })?;
            validate_open(&plan, &request, &services, &self.credential, self.profile)?;
            let bounded = HostBound::new(
                services
                    .time()
                    .cloned()
                    .expect("validated sidecar time service"),
                deadline,
            );
            if bounded.expired() {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.listing_timed_out",
                    "Claude Agent SDK session listing reached its host deadline before dispatch",
                ));
            }
            let open_scope = guard_scope("listing-open-guard", request_id.as_str())?;
            let close_scope = guard_scope("listing-close-guard", request_id.as_str())?;
            let session_scope = guard_scope("listing-session", request_id.as_str())?;
            let open_reservation = crate::sdk::guardian::reserve_reap(&services, &open_scope)?;
            let close_reservation = crate::sdk::guardian::reserve_reap(&services, &close_scope)?;
            let pump_reservation = crate::sdk::guardian::reserve_reap(&services, &session_scope)?;
            let close_guardian = crate::sdk::guardian::SessionGuardian::arm(
                &services,
                close_reservation,
                close_scope,
                request_id.as_str(),
            )?;
            let (guard, lease) = OpenGuard::arm(
                &services,
                open_reservation,
                open_scope,
                request_id.as_str(),
                deadline,
            )?;
            let started = bounded
                .run(self.acquire_and_list(
                    &plan,
                    &request,
                    services.clone(),
                    &guard,
                    lease,
                    Reservations {
                        pump: pump_reservation,
                        pump_scope: session_scope,
                        close_guardian,
                    },
                ))
                .await;
            let (pending, listing) = match started {
                Some(Ok(value)) => value,
                Some(Err(error)) => {
                    let cleaned = guard.fire(&bounded, &services).await.is_some();
                    return Err(if cleaned {
                        error
                    } else {
                        failure(
                            "swallowtail.claude-agent.sdk.listing_cleanup_unconfirmed",
                            "Claude Agent SDK session listing cleanup was not confirmed",
                        )
                    });
                }
                None => {
                    let cleaned = guard.fire(&bounded, &services).await.is_some();
                    return Err(if cleaned {
                        failure(
                            "swallowtail.claude-agent.sdk.listing_timed_out",
                            "Claude Agent SDK session listing reached its host deadline",
                        )
                    } else {
                        failure(
                            "swallowtail.claude-agent.sdk.listing_cleanup_unconfirmed",
                            "Claude Agent SDK session listing cleanup was not confirmed",
                        )
                    });
                }
            };
            pending.connection.begin_close().await;
            let cleaned = guard.fire(&bounded, &services).await.is_some();
            if !cleaned {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.listing_cleanup_unconfirmed",
                    "Claude Agent SDK session listing cleanup was not confirmed",
                ));
            }
            Ok(listing
                .into_iter()
                .map(ClaudeAgentSdkSessionListing::from_parts)
                .collect())
        })
    }

    fn open_with_start(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
        start: SessionStart,
    ) -> BoxFuture<'_, Result<ClaudeAgentSdkSessionHandle, ClaudeAgentSdkOpenRejection>> {
        Box::pin(async move {
            match &start {
                SessionStart::Fresh => {
                    validate_open(&plan, &request, &services, &self.credential, self.profile)
                        .map_err(OpenFailure::admission)?;
                }
                SessionStart::Resume {
                    binding,
                    resume_session_at,
                } => {
                    if self.registered.is_some() {
                        return Err(open_rejection(
                            OpenFailure::admission(unsupported(
                                "registered tools on resumed sessions",
                            )),
                            ClaudeAgentSdkFailedOpenCleanup::not_acquired(),
                        ));
                    }
                    if self.selected_skill.is_some() {
                        return Err(open_rejection(
                            OpenFailure::admission(unsupported(
                                "selected skill bundles on resumed sessions",
                            )),
                            ClaudeAgentSdkFailedOpenCleanup::not_acquired(),
                        ));
                    }
                    let resume = ResumeSessionRequest::from_plan(
                        &plan,
                        request.request_id().clone(),
                        *binding.clone(),
                        request
                            .working_resource()
                            .expect("validated resume working resource")
                            .clone(),
                        request.deadline(),
                    )
                    .map(|resume| resume.with_options(request.options().clone()))
                    .map_err(|_| {
                        open_rejection(
                            OpenFailure::admission(failure(
                                "swallowtail.claude-agent.sdk.resume_binding_mismatch",
                                "Claude Agent SDK resume request could not be reconstructed from its bound plan",
                            )),
                            ClaudeAgentSdkFailedOpenCleanup::not_acquired(),
                        )
                    })?;
                    validate_resume(
                        &plan,
                        &resume,
                        &services,
                        &self.credential,
                        self.profile,
                        resume_session_at.as_deref(),
                    )
                    .map_err(OpenFailure::admission)?;
                }
            }
            let deadline = request.deadline().expect("validated open deadline");
            let bounded = HostBound::new(
                services
                    .time()
                    .cloned()
                    .expect("validated sidecar time service"),
                deadline,
            );
            if bounded.expired() {
                // Nothing was armed or acquired, so no cleanup was owed.
                return Err(open_rejection(
                    OpenFailure::deadline(false),
                    ClaudeAgentSdkFailedOpenCleanup::not_acquired(),
                ));
            }
            // Reap authority first, before anything else exists. An
            // unsupported, closing, or capacity-exhausted host refuses here,
            // with no credential taken, no resource resolved, no process
            // started, no task spawned, and no provider contact. Both
            // guardians this session can ever need are admitted now, so
            // neither later transfer can be refused while its work is live.
            let open_scope = guard_scope("open-guard", request.request_id().as_str())
                .map_err(OpenFailure::admission)?;
            let close_scope = guard_scope("close-guard", request.request_id().as_str())
                .map_err(OpenFailure::admission)?;
            let session_scope = guard_scope("session", request.request_id().as_str())
                .map_err(OpenFailure::admission)?;
            let open_reservation = crate::sdk::guardian::reserve_reap(&services, &open_scope)
                .map_err(OpenFailure::admission)?;
            let close_reservation = crate::sdk::guardian::reserve_reap(&services, &close_scope)
                .map_err(OpenFailure::admission)?;
            let pump_reservation = crate::sdk::guardian::reserve_reap(&services, &session_scope)
                .map_err(OpenFailure::admission)?;
            // Starting a host worker is the fallible half, and it is done here
            // as well: the enclosing cleanup guardian exists before the first
            // acquisition, so activating it at close can never fail while a
            // live process, pump, and two leases are already owned.
            let close_guardian = crate::sdk::guardian::SessionGuardian::arm(
                &services,
                close_reservation,
                close_scope,
                request.request_id().as_str(),
            )
            .map_err(OpenFailure::admission)?;
            // Armed before the first acquisition. From here on, every lease,
            // process, and task the open path takes is recorded in the guard,
            // so the caller's deadline can drop this future at any point
            // without stranding what was already acquired.
            // The lease travels inside the bounded future: dropping that
            // future, however it ends, is what tells cleanup that no further
            // acquisition can arrive.
            let (guard, lease) = OpenGuard::arm(
                &services,
                open_reservation,
                open_scope,
                request.request_id().as_str(),
                deadline,
            )
            .map_err(OpenFailure::admission)?;
            let opened = bounded
                .run(self.acquire_and_start(
                    &plan,
                    &request,
                    &start,
                    services.clone(),
                    &guard,
                    lease,
                    Reservations {
                        pump: pump_reservation,
                        pump_scope: session_scope,
                        close_guardian,
                    },
                ))
                .await;
            match opened {
                Some(Ok((pending, readiness))) => match guard.claim() {
                    Some(acquired) => Ok(pending.into_handle(
                        &plan,
                        readiness,
                        acquired,
                        start.resume_binding().cloned(),
                    )),
                    // Readiness landed on the boundary and cleanup won the one
                    // atomic transition. What this open acquired is already
                    // being terminated, so reporting success would be a lie.
                    None => {
                        let report = guard.fire(&bounded, &services).await;
                        let cleanup = report.as_ref().map_or_else(
                            ClaudeAgentSdkFailedOpenCleanup::unconfirmed,
                            ClaudeAgentSdkFailedOpenCleanup::from_report,
                        );
                        Err(open_rejection(
                            if report.is_some() {
                                OpenFailure::deadline(true)
                            } else {
                                OpenFailure::deadline_unconfirmed_cleanup(true)
                            },
                            cleanup,
                        ))
                    }
                },
                Some(Err(failure)) => {
                    // A failure that only happened because the guard already
                    // terminated at the deadline is reported as the deadline,
                    // not as whatever the collapsing connection said next.
                    let expired = bounded.expired() || guard.deadline_fired();
                    let report = guard.fire(&bounded, &services).await;
                    let cleanup = report.as_ref().map_or_else(
                        ClaudeAgentSdkFailedOpenCleanup::unconfirmed,
                        ClaudeAgentSdkFailedOpenCleanup::from_report,
                    );
                    Err(match (expired, report.is_some()) {
                        // Cleanup is still outstanding: the underlying route
                        // code and observations stay, the error says the
                        // termination could not be confirmed.
                        (_, false) => open_rejection(
                            failure.with_replaced_error(open_cleanup_unconfirmed()),
                            cleanup,
                        ),
                        (true, true) => open_rejection(
                            failure.with_replaced_error(open_deadline_elapsed()),
                            cleanup,
                        ),
                        (false, true) => open_rejection(failure, cleanup),
                    })
                }
                None => {
                    // The bound expired inside acquisition or startup. The
                    // guard still terminates and releases under host ownership;
                    // this future returns now either way.
                    let report = guard.fire(&bounded, &services).await;
                    let cleanup = report.as_ref().map_or_else(
                        ClaudeAgentSdkFailedOpenCleanup::unconfirmed,
                        ClaudeAgentSdkFailedOpenCleanup::from_report,
                    );
                    Err(open_rejection(
                        if report.is_some() {
                            OpenFailure::deadline(false)
                        } else {
                            OpenFailure::deadline_unconfirmed_cleanup(false)
                        },
                        cleanup,
                    ))
                }
            }
        })
    }

    /// The whole provider-facing open: acquisition, launch, and readiness.
    ///
    /// It is one future so the caller's deadline covers all of it, and every
    /// acquisition inside is recorded in the guard before the next await.
    #[allow(clippy::too_many_arguments)]
    async fn acquire_and_start(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        start: &SessionStart,
        services: HostServices,
        guard: &OpenGuard,
        lease: crate::sdk::guardian::RecordingLease,
        reservations: Reservations,
    ) -> Result<(PendingSession, startup::SessionReadiness), OpenFailure> {
        // Held for exactly this future's lifetime, including an early return or
        let _recording = lease;
        let mut pending = self
            .spawn_session(
                plan,
                SessionLaunch {
                    request_id: request.request_id().clone(),
                    working_resource: request
                        .working_resource()
                        .expect("validated sidecar working resource")
                        .clone(),
                    access_policy: request.access_policy(),
                    reservations,
                },
                services,
                guard,
            )
            .await
            .map_err(OpenFailure::admission)?;
        let mut registered_pending = None;
        let mut registered_courier = None;
        if let Some(binding) = &self.registered {
            let pending_registered =
                registered::prepare_registered(binding, plan, request, &pending.services)
                    .await
                    .map_err(OpenFailure::registered_admission)?;
            registered_courier = Some(pending_registered.declaration());
            registered_pending = Some(pending_registered);
        }
        let readiness = match start {
            SessionStart::Fresh => match startup::open(
                &pending.connection,
                plan,
                &pending.leased_cwd,
                self.profile,
                &self.mcp_servers,
                registered_courier,
                self.selected_skill.as_deref(),
            )
            .await
            {
                Ok(readiness) => readiness,
                Err(error) => {
                    if let Some(pending_registered) = registered_pending.take()
                        && let Some(lease) = pending_registered.into_unclaimed_lease()
                    {
                        guard.ledger().record_registered(lease);
                    }
                    return Err(error);
                }
            },
            SessionStart::Resume {
                binding,
                resume_session_at,
            } => {
                startup::resume(
                    &pending.connection,
                    plan,
                    &pending.leased_cwd,
                    self.profile,
                    &self.mcp_servers,
                    None,
                    binding.provider_session_ref(),
                    resume_session_at.as_deref(),
                )
                .await?
            }
        };
        if let Some(mut pending_registered) = registered_pending.take() {
            if let Err(error) = pending_registered.wait_until_ready() {
                if let Some(lease) = pending_registered.into_unclaimed_lease() {
                    guard.ledger().record_registered(lease);
                }
                return Err(OpenFailure::after_readiness(error));
            }
            pending.registered = Some(pending_registered.claim());
        }
        Ok((pending, readiness))
    }

    async fn acquire_and_list(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: HostServices,
        guard: &OpenGuard,
        lease: crate::sdk::guardian::RecordingLease,
        reservations: Reservations,
    ) -> Result<(PendingSession, Vec<startup::SessionListing>), RuntimeFailure> {
        let _recording = lease;
        let pending = self
            .spawn_session(
                plan,
                SessionLaunch {
                    request_id: request.request_id().clone(),
                    working_resource: request
                        .working_resource()
                        .expect("validated listing working resource")
                        .clone(),
                    access_policy: request.access_policy(),
                    reservations,
                },
                services,
                guard,
            )
            .await?;
        let listing = startup::list(
            &pending.connection,
            request.request_id().as_str(),
            &pending.leased_cwd,
            1_000,
            0,
        )
        .await?;
        Ok((pending, listing))
    }
}

enum SessionStart {
    Fresh,
    Resume {
        binding: Box<SessionResumeBinding>,
        resume_session_at: Option<String>,
    },
}

impl SessionStart {
    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        match self {
            Self::Fresh => None,
            Self::Resume { binding, .. } => Some(binding),
        }
    }
}

pub(in crate::sdk) fn open_deadline_elapsed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.open_deadline_elapsed",
        "Claude Agent SDK sidecar session reached its host deadline before readiness",
    )
}

/// The open failed and its cleanup could not finish inside the same caller
/// bound. Termination was requested; completion is unconfirmed, and saying so
/// is the honest report.
pub(in crate::sdk) fn open_cleanup_unconfirmed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed",
        "Claude Agent SDK sidecar termination was requested, but cleanup did not complete inside \
         the caller's open deadline",
    )
}

/// Builds one exact guardian scope, so the reservation, the spawn, and the
/// later transfer all name the same operation scope.
fn guard_scope(
    role: &str,
    request_id: &str,
) -> Result<swallowtail_runtime::ScopeId, RuntimeFailure> {
    swallowtail_runtime::ScopeId::new(format!("claude-agent-sdk:{role}:{request_id}"))
        .map_err(|_| scope_invalid())
}

fn scope_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.scope_invalid",
        "Claude Agent SDK sidecar scope was invalid",
    )
}

pub use descriptor::claude_agent_sdk_descriptor;

pub(crate) use validation::{ACCESS_NAMESPACE, ENDPOINT_AUDIENCE};
