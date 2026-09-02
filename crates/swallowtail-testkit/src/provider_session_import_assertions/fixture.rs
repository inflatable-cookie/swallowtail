#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContinuationEvent {
    Replay(u64),
    ReadyAfterLoad,
    ReadyAfterResume,
}

struct ContinuationFixtureDriver {
    events: Arc<Mutex<Vec<ContinuationEvent>>>,
}

impl InteractiveSessionDriver for ContinuationFixtureDriver {
    fn open_session(
        &self,
        _plan: swallowtail_core::PreflightPlan,
        _request: OpenSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure("fixture open is not used")) })
    }

    fn resume_session(
        &self,
        _plan: swallowtail_core::PreflightPlan,
        request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        self.events
            .lock()
            .expect("fixture event lock is valid")
            .push(ContinuationEvent::ReadyAfterResume);
        let handle = ContinuationFixtureHandle::new(
            request.request_id().clone(),
            request.resume_binding().clone(),
        );
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn InteractiveSessionHandle>) })
    }

    fn load_session(
        &self,
        _plan: swallowtail_core::PreflightPlan,
        request: LoadSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
        let provider_ref = request.provider_session_ref().clone();
        let replay = vec![
            SessionReplayItem::with_content(
                provider_ref.clone(),
                0,
                SessionReplayKind::UserMessage,
                OperationContent::new("private historical prompt")
                    .expect("fixture replay is valid"),
            ),
            SessionReplayItem::with_content(
                provider_ref,
                1,
                SessionReplayKind::AgentMessage,
                OperationContent::new("private historical response")
                    .expect("fixture replay is valid"),
            ),
        ];
        let mut events = self.events.lock().expect("fixture event lock is valid");
        events.extend(
            replay
                .iter()
                .map(|item| ContinuationEvent::Replay(item.sequence())),
        );
        events.push(ContinuationEvent::ReadyAfterLoad);
        drop(events);
        let handle = ContinuationFixtureHandle::new(
            request.request_id().clone(),
            request.resume_binding().clone(),
        );
        Box::pin(async move { Ok(LoadedSession::new(replay, Box::new(handle))) })
    }
}

struct ContinuationFixtureHandle {
    request_id: RequestId,
    session_id: RuntimeSessionId,
    binding: swallowtail_runtime::SessionResumeBinding,
    cancellation: ImmediateCancellation,
}

impl ContinuationFixtureHandle {
    fn new(request_id: RequestId, binding: swallowtail_runtime::SessionResumeBinding) -> Self {
        Self {
            request_id,
            session_id: RuntimeSessionId::new("fixture-imported-session")
                .expect("fixture runtime session id is valid"),
            binding,
            cancellation: ImmediateCancellation::new(CancellationScope::InteractiveSession),
        }
    }
}

impl InteractiveSessionHandle for ContinuationFixtureHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.session_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        Some(self.binding.provider_session_ref())
    }

    fn resume_binding(&self) -> Option<&swallowtail_runtime::SessionResumeBinding> {
        Some(&self.binding)
    }

    fn start_turn<'a>(
        &'a mut self,
        _request: TurnRequest,
        _services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure("fixture turn is not used")) })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(
        self: Box<Self>,
        request: swallowtail_runtime::SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        swallowtail_runtime::bound_session_cleanup(
            self.binding.execution_host_id().clone(),
            request,
            services,
            Box::pin(async { CleanupOutcome::Clean }),
        )
    }
}

fn request_id(input: &str) -> RequestId {
    RequestId::new(input).expect("fixture request id is valid")
}

fn session_ref(input: &str) -> SessionRef {
    SessionRef::new(input).expect("fixture provider session ref is valid")
}

fn fixture_failure(message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new("fixture.unused", message))
}

#[cfg(test)]
mod tests {
    #[test]
    fn provider_session_import_contract_passes() {
        super::assert_provider_session_import_contract();
    }
}
