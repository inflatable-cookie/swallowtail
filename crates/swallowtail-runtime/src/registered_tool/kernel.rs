//! The one serialized registered-tool admission, dispatch, and progress kernel.
//!
//! This kernel is the sole authority that mints a lease, a validated binding,
//! and a call. There is no other constructor: [`RegisteredToolOperationKernel::open`]
//! requires a [`RegisteredToolTopologyProof`] issued for the exact selection, so
//! every mounted open path passes the typed readiness gate before any binding
//! exists.
//!
//! It serializes dispatch, result, progress, and revocation races behind one
//! admission point, enforces the effective call deadline at the dispatch,
//! progress, and terminal boundaries, keeps exactly-once result acceptance, and
//! never claims exactly-once remote execution.

use super::admission::AdmissionPhase;
use super::call::{
    RegisteredToolCall, RegisteredToolCallRequest, RegisteredToolExecutionDisposition,
    RegisteredToolOutcome, RegisteredToolProgress, ValidatedRegisteredToolBinding,
};
use super::dispatch::{
    RegisteredToolCancellation, RegisteredToolCancellationSource, RegisteredToolDispatchContext,
    RegisteredToolDispatcher, RegisteredToolProgressChannel, RegisteredToolProgressSink,
};
use super::failure::{RegisteredToolFailureKind, fail, reject};
use super::gate::{AdmissionGate, AdmissionPermit};
use super::identity::{
    RegisteredToolCallId, RegisteredToolLeaseGeneration, RegisteredToolTransportGeneration,
};
use super::lease::{
    RegisteredToolAdmissionState, RegisteredToolBridgeLease, RegisteredToolCompletionState,
    RegisteredToolLifecycleState, RegisteredToolOpenRequest,
};
use super::readiness::RegisteredToolTopologyProof;
use super::selection::RegisteredToolSelection;
use crate::{BoxFuture, Deadline, MonotonicInstant, RuntimeFailure, TimeService};
use std::collections::{BTreeSet, VecDeque};
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Bounded ceiling on remembered call identities for duplicate rejection.
const MAX_REMEMBERED_CALL_IDS: usize = 256;

/// One progress notification that passed every pre-verdict check.
struct ReservedProgress {
    sequence: u64,
    epoch: u64,
}

struct CallSlot {
    call_id: RegisteredToolCallId,
    cancelled: Arc<AtomicBool>,
    expires_at: Deadline,
    last_sequence: u64,
    settled: bool,
}

struct KernelState {
    admission: RegisteredToolAdmissionState,
    lifecycle: RegisteredToolLifecycleState,
    revoked: bool,
    cleanup_failed: bool,
    active: Option<CallSlot>,
    seen_calls: BTreeSet<RegisteredToolCallId>,
    progress: VecDeque<RegisteredToolProgress>,
    /// Bumped whenever admission truth changes under the gate.
    ///
    /// A verdict obtained against one epoch can never commit against another,
    /// so a `Current` verdict that raced a freeze, revocation, or close is
    /// rejected instead of applied.
    epoch: u64,
}

impl KernelState {
    fn bump_epoch(&mut self) {
        self.epoch = self.epoch.saturating_add(1);
    }
}

/// One operation-scoped registered-tool kernel.
pub struct RegisteredToolOperationKernel {
    binding: ValidatedRegisteredToolBinding,
    selection: RegisteredToolSelection,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    time: Arc<dyn TimeService>,
    operation_deadline: Deadline,
    state: Mutex<KernelState>,
    gate: AdmissionGate,
}

struct CallCancellation {
    flag: Arc<AtomicBool>,
}

impl RegisteredToolCancellationSource for CallCancellation {
    fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }
}

struct ProgressGate {
    kernel: Arc<RegisteredToolOperationKernel>,
}

impl RegisteredToolProgressChannel for ProgressGate {
    fn admit(&self, progress: RegisteredToolProgress) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let kernel = Arc::clone(&self.kernel);
        Box::pin(async move { kernel.admit_progress(progress).await })
    }
}

impl RegisteredToolOperationKernel {
    /// Opens one kernel and its lease after a matching topology proof.
    ///
    /// The proof is the readiness gate: an unqualified carrier, an unqualified
    /// protocol version, a foreign execution host, a missing required service,
    /// or an absent port cannot produce one, so no binding or lease is minted.
    /// A proof issued for another selection is rejected here.
    pub fn open(
        request: RegisteredToolOpenRequest,
        proof: &RegisteredToolTopologyProof,
        dispatcher: Arc<dyn RegisteredToolDispatcher>,
        time: Arc<dyn TimeService>,
        generation: RegisteredToolLeaseGeneration,
        transport_generation: RegisteredToolTransportGeneration,
    ) -> Result<(Arc<Self>, RegisteredToolBridgeLease), RuntimeFailure> {
        if !proof.matches(request.selection()) {
            return Err(fail(RegisteredToolFailureKind::UnsupportedRegistration));
        }
        if request.execution_host_id() != request.selection().snapshot().execution_host_id() {
            return Err(fail(RegisteredToolFailureKind::UnsupportedRegistration));
        }
        let binding = ValidatedRegisteredToolBinding::mint(
            request.execution_host_id().clone(),
            request.configured_instance().clone(),
            request.scope().clone(),
            request.turn().clone(),
            request.selection().snapshot().server_id().clone(),
            request.selection().snapshot().revision().clone(),
            generation,
            request.selection().transport(),
            transport_generation,
            request.selection().protocol_version().clone(),
            request.effective_bounds(),
            request.admission().clone(),
        );
        let kernel = Arc::new(Self {
            binding,
            selection: request.selection().clone(),
            dispatcher,
            time,
            operation_deadline: request.deadline(),
            state: Mutex::new(KernelState {
                admission: RegisteredToolAdmissionState::Open,
                lifecycle: if request.selection().attachment()
                    == super::attachment::RegisteredToolAttachment::MediatedStdioProxy
                {
                    RegisteredToolLifecycleState::Prepared
                } else {
                    RegisteredToolLifecycleState::Ready
                },
                revoked: false,
                cleanup_failed: false,
                active: None,
                seen_calls: BTreeSet::new(),
                progress: VecDeque::new(),
                epoch: 0,
            }),
            gate: AdmissionGate::default(),
        });
        let lease = RegisteredToolBridgeLease::mint(
            &request,
            generation,
            transport_generation,
            Arc::clone(&kernel),
        );
        Ok((kernel, lease))
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, KernelState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Returns the live validated binding this kernel owns.
    #[must_use]
    pub const fn binding(&self) -> &ValidatedRegisteredToolBinding {
        &self.binding
    }

    /// Returns the bounded completion observation, freezing when clear.
    ///
    /// The gate observes and freezes; it never waits, cancels, or converts a
    /// provider-terminal response into success.
    #[must_use]
    pub fn observe_and_freeze_when_clear(&self) -> RegisteredToolCompletionState {
        let mut state = self.locked();
        if state.admission == RegisteredToolAdmissionState::Open && state.active.is_none() {
            state.admission = RegisteredToolAdmissionState::Frozen;
            state.lifecycle = RegisteredToolLifecycleState::Frozen;
            state.bump_epoch();
        }
        completion_state(&state)
    }

    /// Returns the bounded completion observation without changing admission.
    #[must_use]
    pub fn observe(&self) -> RegisteredToolCompletionState {
        completion_state(&self.locked())
    }

    /// Freezes new admission and signals cancellation to any issued call.
    pub fn freeze(&self) {
        let mut state = self.locked();
        if state.admission == RegisteredToolAdmissionState::Open {
            state.admission = RegisteredToolAdmissionState::Frozen;
            state.lifecycle = RegisteredToolLifecycleState::Frozen;
        }
        if let Some(active) = state.active.as_ref() {
            active.cancelled.store(true, Ordering::SeqCst);
        }
        state.progress.clear();
        state.bump_epoch();
    }

    /// Completes the mediated transport ready barrier.
    ///
    /// Host-mediated callback leases are ready at open. A mediated stdio
    /// lease stays prepared until its courier has authenticated and completed
    /// the fixed MCP negotiation, so no provider call can enter the kernel
    /// before that barrier.
    pub fn mark_ready(&self) -> Result<(), RuntimeFailure> {
        let mut state = self.locked();
        if state.admission != RegisteredToolAdmissionState::Open {
            return Err(fail(RegisteredToolFailureKind::PostTerminalCorrelation));
        }
        match state.lifecycle {
            RegisteredToolLifecycleState::Prepared => {
                state.lifecycle = RegisteredToolLifecycleState::Ready;
                state.bump_epoch();
                Ok(())
            }
            RegisteredToolLifecycleState::Ready => Ok(()),
            _ => Err(fail(RegisteredToolFailureKind::NotReady)),
        }
    }

    /// Marks the kernel closing after admission has frozen.
    pub fn begin_close(&self) {
        let mut state = self.locked();
        state.lifecycle = RegisteredToolLifecycleState::Closing;
    }

    /// Records a joined close that released every resource.
    pub fn record_closed(&self) {
        let mut state = self.locked();
        state.admission = RegisteredToolAdmissionState::Closed;
        state.lifecycle = RegisteredToolLifecycleState::Closed;
        state.progress.clear();
        state.bump_epoch();
    }

    /// Records a cleanup attempt that did not join within its bounded budget.
    ///
    /// The lease stays frozen and retains ownership of its resources. It never
    /// reports a clean close and never becomes reusable.
    pub fn record_cleanup_failed(&self) {
        let mut state = self.locked();
        state.cleanup_failed = true;
        state.admission = RegisteredToolAdmissionState::Frozen;
        state.lifecycle = RegisteredToolLifecycleState::Frozen;
        state.bump_epoch();
    }

    /// Reports whether a cleanup attempt already failed for this kernel.
    #[must_use]
    pub fn cleanup_failed(&self) -> bool {
        self.locked().cleanup_failed
    }

    /// Reports whether an issued call has expired against the host clock.
    #[must_use]
    pub fn expired_call_pending(&self) -> bool {
        let now = self.time.now();
        self.locked()
            .active
            .as_ref()
            .is_some_and(|active| reached(now, active.expires_at))
    }

    /// Returns the bounded outstanding-call count.
    #[must_use]
    pub fn outstanding_calls(&self) -> usize {
        usize::from(self.locked().active.is_some())
    }

    /// Returns the effective expiry of one call issued at an exact instant.
    ///
    /// A call expires at the earliest of the operation deadline, the caller's
    /// own deadline, and the declared maximum call duration.
    #[must_use]
    pub fn effective_call_deadline(
        &self,
        requested: Deadline,
        started: MonotonicInstant,
    ) -> Deadline {
        let max_duration_ticks = u64::try_from(
            self.binding
                .effective_bounds()
                .max_call_duration()
                .as_nanos(),
        )
        .unwrap_or(u64::MAX);
        let bounded = Deadline::at(MonotonicInstant::from_ticks(
            started.ticks().saturating_add(max_duration_ticks),
        ));
        [self.operation_deadline, requested, bounded]
            .into_iter()
            .min()
            .unwrap_or(bounded)
    }

    async fn admit_progress(&self, progress: RegisteredToolProgress) -> Result<(), RuntimeFailure> {
        // One serialized admission sequence: nothing else may admit, dispatch,
        // deliver, or commit between the pre-verdict checks, the awaited live
        // verdict, and the final commit.
        let permit = self.gate.acquire().await;
        let reserved = self.check_progress(&progress)?;
        // The pre-verdict checks reject sequence, correlation, deadline, and
        // bound violations before any live verdict is requested.
        self.require_current(AdmissionPhase::BeforeDelivery, &permit)
            .await?;
        self.commit_progress(progress, reserved, &permit)
    }

    fn commit_progress(
        &self,
        progress: RegisteredToolProgress,
        reserved: ReservedProgress,
        permit: &AdmissionPermit<'_>,
    ) -> Result<(), RuntimeFailure> {
        let _ = permit;
        let now = self.time.now();
        let mut state = self.locked();
        // A verdict obtained against an earlier epoch never commits.
        if state.epoch != reserved.epoch || state.revoked {
            return Err(fail(RegisteredToolFailureKind::Revoked));
        }
        if state.admission != RegisteredToolAdmissionState::Open {
            return Err(fail(RegisteredToolFailureKind::PostTerminalCorrelation));
        }
        if state.progress.len() >= self.binding.effective_bounds().max_queued_progress_items() {
            return Err(fail(RegisteredToolFailureKind::LimitExceeded));
        }
        let Some(active) = state.active.as_mut() else {
            return Err(fail(RegisteredToolFailureKind::ForeignCorrelation));
        };
        if active.call_id != *progress.call_id() || active.settled {
            return Err(fail(RegisteredToolFailureKind::ForeignCorrelation));
        }
        if active.cancelled.load(Ordering::SeqCst) {
            return Err(fail(RegisteredToolFailureKind::Cancelled));
        }
        if reached(now, active.expires_at) {
            active.cancelled.store(true, Ordering::SeqCst);
            return Err(fail(RegisteredToolFailureKind::DeadlineExceeded));
        }
        if reserved.sequence <= active.last_sequence {
            return Err(fail(RegisteredToolFailureKind::DuplicateCorrelation));
        }
        active.last_sequence = reserved.sequence;
        state.progress.push_back(progress);
        Ok(())
    }

    fn check_progress(
        &self,
        progress: &RegisteredToolProgress,
    ) -> Result<ReservedProgress, RuntimeFailure> {
        let now = self.time.now();
        let mut state = self.locked();
        if state.revoked {
            return Err(fail(RegisteredToolFailureKind::Revoked));
        }
        match state.admission {
            RegisteredToolAdmissionState::Closed | RegisteredToolAdmissionState::Frozen => {
                return Err(fail(RegisteredToolFailureKind::PostTerminalCorrelation));
            }
            RegisteredToolAdmissionState::Open => {}
        }
        if !progress.binding().same_binding(&self.binding) {
            return Err(fail(RegisteredToolFailureKind::ForeignCorrelation));
        }
        let queued = state.progress.len();
        let bound = self.binding.effective_bounds().max_queued_progress_items();
        let epoch = state.epoch;
        let Some(active) = state.active.as_mut() else {
            return Err(fail(RegisteredToolFailureKind::ForeignCorrelation));
        };
        if active.call_id != *progress.call_id() {
            return Err(fail(RegisteredToolFailureKind::ForeignCorrelation));
        }
        if active.cancelled.load(Ordering::SeqCst) {
            return Err(fail(RegisteredToolFailureKind::Cancelled));
        }
        if active.settled {
            return Err(fail(RegisteredToolFailureKind::PostTerminalCorrelation));
        }
        if reached(now, active.expires_at) {
            active.cancelled.store(true, Ordering::SeqCst);
            return Err(fail(RegisteredToolFailureKind::DeadlineExceeded));
        }
        let sequence = progress.sequence().get();
        if sequence <= active.last_sequence {
            return Err(fail(RegisteredToolFailureKind::DuplicateCorrelation));
        }
        if queued >= bound {
            return Err(fail(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(ReservedProgress { sequence, epoch })
    }

    /// Drains admitted progress after one live before-delivery admission check.
    ///
    /// Delivery takes the same serialized admission sequence as dispatch and
    /// progress, so a drain never interleaves with an in-flight verdict.
    pub async fn deliver_progress(&self) -> Result<Vec<RegisteredToolProgress>, RuntimeFailure> {
        let permit = self.gate.acquire().await;
        let epoch = self.locked().epoch;
        self.require_current(AdmissionPhase::BeforeDelivery, &permit)
            .await?;
        let mut state = self.locked();
        if state.epoch != epoch || state.revoked {
            return Err(fail(RegisteredToolFailureKind::Revoked));
        }
        Ok(state.progress.drain(..).collect())
    }

    /// Takes one live verdict inside an already held admission sequence.
    ///
    /// The permit is the proof that this await cannot interleave with another
    /// admission sequence on the same kernel.
    async fn require_current(
        &self,
        phase: AdmissionPhase,
        permit: &AdmissionPermit<'_>,
    ) -> Result<(), RuntimeFailure> {
        let _ = permit;
        if self.locked().revoked {
            return Err(fail(RegisteredToolFailureKind::Revoked));
        }
        let verdict = self.binding.admission().validate(phase).await?;
        if verdict.is_current() {
            return Ok(());
        }
        let mut state = self.locked();
        state.revoked = true;
        if state.admission == RegisteredToolAdmissionState::Open {
            state.admission = RegisteredToolAdmissionState::Frozen;
            state.lifecycle = RegisteredToolLifecycleState::Frozen;
        }
        state.progress.clear();
        state.bump_epoch();
        Err(fail(RegisteredToolFailureKind::Revoked))
    }

    fn reserve(
        &self,
        request: &RegisteredToolCallRequest,
        expires_at: Deadline,
        now: MonotonicInstant,
    ) -> Result<(Arc<AtomicBool>, u64), RuntimeFailure> {
        let mut state = self.locked();
        if state.revoked {
            return Err(fail(RegisteredToolFailureKind::Revoked));
        }
        match state.admission {
            RegisteredToolAdmissionState::Closed | RegisteredToolAdmissionState::Frozen => {
                return Err(fail(RegisteredToolFailureKind::PostTerminalCorrelation));
            }
            RegisteredToolAdmissionState::Open => {}
        }
        if state.lifecycle == RegisteredToolLifecycleState::Prepared {
            return Err(fail(RegisteredToolFailureKind::NotReady));
        }
        if reached(now, expires_at) {
            return Err(fail(RegisteredToolFailureKind::DeadlineExceeded));
        }
        if state.active.is_some() {
            return Err(fail(RegisteredToolFailureKind::LimitExceeded));
        }
        if state.seen_calls.contains(request.call_id()) {
            return Err(fail(RegisteredToolFailureKind::DuplicateCorrelation));
        }
        if state.seen_calls.len() >= MAX_REMEMBERED_CALL_IDS {
            return Err(fail(RegisteredToolFailureKind::LimitExceeded));
        }
        state.seen_calls.insert(request.call_id().clone());
        let cancelled = Arc::new(AtomicBool::new(false));
        state.active = Some(CallSlot {
            call_id: request.call_id().clone(),
            cancelled: Arc::clone(&cancelled),
            expires_at,
            last_sequence: 0,
            settled: false,
        });
        state.lifecycle = RegisteredToolLifecycleState::CallPending;
        let epoch = state.epoch;
        Ok((cancelled, epoch))
    }

    fn release(&self) {
        let mut state = self.locked();
        if let Some(active) = state.active.as_mut() {
            active.settled = true;
        }
        state.active = None;
        state.progress.clear();
        if state.admission == RegisteredToolAdmissionState::Open {
            state.lifecycle = RegisteredToolLifecycleState::Ready;
        }
    }

    /// Issues one bounded call through the kernel's serialized admission point.
    pub fn issue<'kernel>(
        kernel: &'kernel Arc<Self>,
        request: RegisteredToolCallRequest,
    ) -> BoxFuture<'kernel, Result<RegisteredToolOutcome, RuntimeFailure>> {
        let kernel = Arc::clone(kernel);
        Box::pin(async move { kernel.issue_now(request).await })
    }

    async fn issue_now(
        self: Arc<Self>,
        request: RegisteredToolCallRequest,
    ) -> Result<RegisteredToolOutcome, RuntimeFailure> {
        let declaration = self
            .selection
            .snapshot()
            .declaration(request.tool())
            .filter(|_| self.selection.contains(request.tool()))
            .ok_or_else(|| fail(RegisteredToolFailureKind::UnsupportedTool))?;
        let kind = declaration.kind();
        if !kind.is_host_dispatchable() {
            return Err(fail(RegisteredToolFailureKind::UnsupportedTool));
        }
        let started = self.time.now();
        let expires_at = self.effective_call_deadline(request.deadline(), started);
        // The dispatch admission sequence runs under the same serialized gate.
        // It is released before the dispatcher runs so a dispatcher publishing
        // progress cannot deadlock against its own call.
        let (cancelled, epoch) = {
            let permit = self.gate.acquire().await;
            let (cancelled, epoch) = self.reserve(&request, expires_at, started)?;
            if let Err(error) = self
                .require_current(AdmissionPhase::BeforeDispatch, &permit)
                .await
            {
                self.release();
                return Err(error);
            }
            if self.locked().epoch != epoch {
                self.release();
                return Err(fail(RegisteredToolFailureKind::Revoked));
            }
            if reached(self.time.now(), expires_at) {
                cancelled.store(true, Ordering::SeqCst);
                self.release();
                return Err(fail(RegisteredToolFailureKind::DeadlineExceeded));
            }
            (cancelled, epoch)
        };
        let _ = epoch;
        let call = match RegisteredToolCall::mint(
            self.binding.clone(),
            request.call_id().clone(),
            request.tool().clone(),
            kind,
            request.arguments().clone(),
            expires_at,
        ) {
            Ok(call) => call,
            Err(error) => {
                self.release();
                return Err(error.into_runtime_failure());
            }
        };
        let context = RegisteredToolDispatchContext::new(
            RegisteredToolCancellation::new(
                call.call_id().clone(),
                Arc::new(CallCancellation {
                    flag: Arc::clone(&cancelled),
                }),
            ),
            RegisteredToolProgressSink::new(
                call.clone(),
                Arc::new(ProgressGate {
                    kernel: Arc::clone(&self),
                }),
            ),
        );
        let dispatched = self.dispatcher.dispatch(call.clone(), context).await;
        let settled = self.settle(&call, expires_at, dispatched).await;
        self.release();
        settled
    }

    async fn settle(
        &self,
        call: &RegisteredToolCall,
        expires_at: Deadline,
        dispatched: Result<RegisteredToolOutcome, RuntimeFailure>,
    ) -> Result<RegisteredToolOutcome, RuntimeFailure> {
        let outcome = dispatched?;
        if outcome.call_id() != call.call_id()
            || outcome.tool() != call.tool()
            || outcome.kind() != call.kind()
        {
            return Err(fail(RegisteredToolFailureKind::ForeignCorrelation));
        }
        if reached(self.time.now(), expires_at) {
            // The call outran its effective deadline. Whether the linked host
            // completed the effect is not knowable here, so it is reported as
            // unknown and never replays automatically.
            return Ok(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::DeadlineExceeded),
                RegisteredToolExecutionDisposition::Unknown,
            ));
        }
        // The terminal commit runs as one serialized admission sequence, exactly
        // like dispatch, progress, and delivery: the epoch and terminal truth
        // are snapshotted under the permit, the live verdict is awaited under
        // the same permit, and the result is revalidated before it is accepted.
        let permit = self.gate.acquire().await;
        let epoch = self.locked().epoch;
        if let Err(error) = self
            .require_current(AdmissionPhase::BeforeDelivery, &permit)
            .await
        {
            let _ = error;
            return Ok(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::Revoked),
                self.honest_disposition(&outcome),
            ));
        }
        if let Some(stale) =
            self.terminal_commit_failure(call, epoch, expires_at, &outcome, &permit)
        {
            return Ok(stale);
        }
        if let Some(result) = outcome.result()
            && result.payload().byte_len() > self.binding.effective_bounds().max_result_bytes()
        {
            return Ok(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::LimitExceeded),
                RegisteredToolExecutionDisposition::Executed,
            ));
        }
        Ok(outcome)
    }

    /// Reports the honest disposition of an outcome that cannot be accepted.
    ///
    /// A dispatcher that produced a result demonstrably executed. Otherwise the
    /// dispatcher's own disposition stands, and it never becomes a claim that
    /// the effect did not happen.
    fn honest_disposition(
        &self,
        outcome: &RegisteredToolOutcome,
    ) -> RegisteredToolExecutionDisposition {
        match outcome.result() {
            Some(_) => RegisteredToolExecutionDisposition::Executed,
            None => outcome.disposition(),
        }
    }

    /// Revalidates terminal truth after the awaited verdict, under the permit.
    ///
    /// A stale successful outcome never commits: an epoch change, freeze,
    /// revocation, close, failed cleanup, prior settlement, cancellation,
    /// correlation change, or an effective deadline that elapsed during the
    /// verdict all reject it. The reported disposition stays honest, so a call
    /// whose effect may have landed is never reported as not executed.
    fn terminal_commit_failure(
        &self,
        call: &RegisteredToolCall,
        epoch: u64,
        expires_at: Deadline,
        outcome: &RegisteredToolOutcome,
        permit: &AdmissionPermit<'_>,
    ) -> Option<RegisteredToolOutcome> {
        let _ = permit;
        let now = self.time.now();
        let state = self.locked();
        let disposition = self.honest_disposition(outcome);
        if state.revoked {
            return Some(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::Revoked),
                disposition,
            ));
        }
        if state.cleanup_failed {
            return Some(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::TeardownFailed),
                disposition,
            ));
        }
        if state.admission != RegisteredToolAdmissionState::Open {
            return Some(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::PostTerminalCorrelation),
                disposition,
            ));
        }
        let Some(active) = state.active.as_ref() else {
            return Some(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::ForeignCorrelation),
                disposition,
            ));
        };
        if active.call_id != *call.call_id() || active.settled {
            return Some(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::ForeignCorrelation),
                disposition,
            ));
        }
        if active.cancelled.load(Ordering::SeqCst) {
            return Some(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::Cancelled),
                disposition,
            ));
        }
        if reached(now, active.expires_at) || reached(now, expires_at) {
            return Some(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::DeadlineExceeded),
                RegisteredToolExecutionDisposition::Unknown,
            ));
        }
        if state.epoch != epoch {
            // Terminal truth changed under a verdict taken against an earlier
            // epoch. The outcome is stale and never commits.
            return Some(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::PostTerminalCorrelation),
                disposition,
            ));
        }
        None
    }
}

impl std::fmt::Debug for RegisteredToolOperationKernel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self.locked();
        formatter
            .debug_struct("RegisteredToolOperationKernel")
            .field("admission", &state.admission)
            .field("lifecycle", &state.lifecycle)
            .field("outstanding_calls", &usize::from(state.active.is_some()))
            .field("cleanup_failed", &state.cleanup_failed)
            .finish()
    }
}

fn reached(now: MonotonicInstant, deadline: Deadline) -> bool {
    now.ticks() >= deadline.instant().ticks()
}

fn completion_state(state: &KernelState) -> RegisteredToolCompletionState {
    RegisteredToolCompletionState::new(
        state.admission,
        state.lifecycle,
        usize::from(state.active.is_some()),
        state.cleanup_failed,
    )
}

/// Returns the first admissible progress sequence for a new call.
#[must_use]
pub const fn first_progress_sequence() -> NonZeroU64 {
    NonZeroU64::MIN
}
