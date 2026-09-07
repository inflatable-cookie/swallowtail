//! The one serialized registered-tool admission, dispatch, and progress kernel.
//!
//! This kernel is the only place a validated call binding is minted. It
//! serializes dispatch, result, progress, and revocation races behind one
//! admission point, keeps exactly-once result acceptance, and never claims
//! exactly-once remote execution.

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
use super::identity::RegisteredToolCallId;
use super::lease::{
    RegisteredToolAdmissionState, RegisteredToolCallChannel, RegisteredToolCompletionState,
    RegisteredToolLifecycleState,
};
use super::selection::RegisteredToolSelection;
use crate::{BoxFuture, RuntimeFailure};
use std::collections::{BTreeSet, VecDeque};
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Bounded ceiling on remembered call identities for duplicate rejection.
const MAX_REMEMBERED_CALL_IDS: usize = 256;

struct CallSlot {
    call_id: RegisteredToolCallId,
    cancelled: Arc<AtomicBool>,
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
}

/// One operation-scoped registered-tool kernel.
pub struct RegisteredToolOperationKernel {
    binding: ValidatedRegisteredToolBinding,
    selection: RegisteredToolSelection,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    state: Mutex<KernelState>,
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
    fn admit(&self, progress: RegisteredToolProgress) -> Result<(), RuntimeFailure> {
        self.kernel.admit_progress(progress)
    }
}

impl RegisteredToolOperationKernel {
    /// Creates one kernel bound to exactly one live validated binding.
    #[must_use]
    pub fn new(
        binding: ValidatedRegisteredToolBinding,
        selection: RegisteredToolSelection,
        dispatcher: Arc<dyn RegisteredToolDispatcher>,
    ) -> Self {
        Self {
            binding,
            selection,
            dispatcher,
            state: Mutex::new(KernelState {
                admission: RegisteredToolAdmissionState::Open,
                lifecycle: RegisteredToolLifecycleState::Ready,
                revoked: false,
                cleanup_failed: false,
                active: None,
                seen_calls: BTreeSet::new(),
                progress: VecDeque::new(),
            }),
        }
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, KernelState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Returns an object-safe call channel bound to this exact kernel.
    #[must_use]
    pub fn call_channel(self: &Arc<Self>) -> Arc<dyn RegisteredToolCallChannel> {
        Arc::new(Arc::clone(self))
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
    }

    /// Reports whether a cleanup attempt already failed for this kernel.
    #[must_use]
    pub fn cleanup_failed(&self) -> bool {
        self.locked().cleanup_failed
    }

    /// Returns the bounded outstanding-call count.
    #[must_use]
    pub fn outstanding_calls(&self) -> usize {
        usize::from(self.locked().active.is_some())
    }

    fn admit_progress(&self, progress: RegisteredToolProgress) -> Result<(), RuntimeFailure> {
        let mut state = self.locked();
        if state.revoked {
            return Err(fail(RegisteredToolFailureKind::Revoked));
        }
        match state.admission {
            RegisteredToolAdmissionState::Closed => {
                return Err(fail(RegisteredToolFailureKind::PostTerminalCorrelation));
            }
            RegisteredToolAdmissionState::Frozen => {
                return Err(fail(RegisteredToolFailureKind::PostTerminalCorrelation));
            }
            RegisteredToolAdmissionState::Open => {}
        }
        if !progress.binding().same_binding(&self.binding) {
            return Err(fail(RegisteredToolFailureKind::ForeignCorrelation));
        }
        let queued = state.progress.len();
        let bound = self.binding.effective_bounds().max_queued_progress_items();
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
        let sequence = progress.sequence().get();
        if sequence <= active.last_sequence {
            return Err(fail(RegisteredToolFailureKind::DuplicateCorrelation));
        }
        if queued >= bound {
            return Err(fail(RegisteredToolFailureKind::LimitExceeded));
        }
        active.last_sequence = sequence;
        state.progress.push_back(progress);
        Ok(())
    }

    /// Drains admitted progress after one live before-delivery admission check.
    pub async fn deliver_progress(&self) -> Result<Vec<RegisteredToolProgress>, RuntimeFailure> {
        self.require_current(AdmissionPhase::BeforeDelivery).await?;
        let mut state = self.locked();
        Ok(state.progress.drain(..).collect())
    }

    async fn require_current(&self, phase: AdmissionPhase) -> Result<(), RuntimeFailure> {
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
        Err(fail(RegisteredToolFailureKind::Revoked))
    }

    fn reserve(
        &self,
        request: &RegisteredToolCallRequest,
    ) -> Result<Arc<AtomicBool>, RuntimeFailure> {
        let mut state = self.locked();
        if state.revoked {
            return Err(fail(RegisteredToolFailureKind::Revoked));
        }
        match state.admission {
            RegisteredToolAdmissionState::Closed => {
                return Err(fail(RegisteredToolFailureKind::PostTerminalCorrelation));
            }
            RegisteredToolAdmissionState::Frozen => {
                return Err(fail(RegisteredToolFailureKind::PostTerminalCorrelation));
            }
            RegisteredToolAdmissionState::Open => {}
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
            last_sequence: 0,
            settled: false,
        });
        state.lifecycle = RegisteredToolLifecycleState::CallPending;
        Ok(cancelled)
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
        let cancelled = self.reserve(&request)?;
        let committed = self.require_current(AdmissionPhase::BeforeDispatch).await;
        if let Err(error) = committed {
            self.release();
            return Err(error);
        }
        let call = match RegisteredToolCall::mint(
            self.binding.clone(),
            request.call_id().clone(),
            request.tool().clone(),
            kind,
            request.arguments().clone(),
            request.deadline(),
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
        let settled = self.settle(&call, dispatched).await;
        self.release();
        settled
    }

    async fn settle(
        &self,
        call: &RegisteredToolCall,
        dispatched: Result<RegisteredToolOutcome, RuntimeFailure>,
    ) -> Result<RegisteredToolOutcome, RuntimeFailure> {
        let outcome = match dispatched {
            Ok(outcome) => outcome,
            Err(error) => return Err(error),
        };
        if outcome.call_id() != call.call_id()
            || outcome.tool() != call.tool()
            || outcome.kind() != call.kind()
        {
            return Err(fail(RegisteredToolFailureKind::ForeignCorrelation));
        }
        if let Err(error) = self.require_current(AdmissionPhase::BeforeDelivery).await {
            let disposition = match outcome.result() {
                Some(_) => RegisteredToolExecutionDisposition::Executed,
                None => outcome.disposition(),
            };
            let _ = error;
            return Ok(RegisteredToolOutcome::failed(
                call,
                reject(RegisteredToolFailureKind::Revoked),
                disposition,
            ));
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
}

impl RegisteredToolCallChannel for Arc<RegisteredToolOperationKernel> {
    fn issue(
        &self,
        request: RegisteredToolCallRequest,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        let kernel = Arc::clone(self);
        Box::pin(kernel.issue_now(request))
    }
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
