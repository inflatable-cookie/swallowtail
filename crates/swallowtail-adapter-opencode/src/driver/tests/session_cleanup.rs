use super::*;
use futures_channel::oneshot;
use futures_util::FutureExt;
use std::future::{pending, ready};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::atomic::AtomicUsize;
use swallowtail_core::{
    ConfiguredInstanceId, CredentialRef, EndpointAudience, ModelId, ModelRouteId,
};
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, DelegatedCredential, OperationContent, ScopedTaskService,
    WorkingResourceRef,
};
use swallowtail_testkit::{RecordedHostCall, RecordingHostServices, RecordingOutcome};

fn session(services: HostServices, endpoint: String) -> OpenCodeSessionHandle {
    let model_id = ModelId::new("fixture").expect("model id");
    let working_resource = WorkingResourceRef::new("opencode.fixture.workspace").expect("resource");
    let binding = SessionResumeBinding::new(
        SessionRef::new("fixture-session").expect("session ref"),
        ConfiguredInstanceId::new("opencode.fixture").expect("instance id"),
        services.execution_host_id().clone(),
        ModelRouteId::new("opencode.fixture.route").expect("model route"),
        model_id.clone(),
        working_resource.clone(),
        SessionAccessPolicy::read_only(),
    );
    let active = Arc::new(Mutex::new(None));
    let lease_scope = ScopeId::new("opencode.fixture.leases").expect("lease scope");
    OpenCodeSessionHandle {
        request_id: RequestId::new("fixture-request").expect("request id"),
        runtime_id: RuntimeSessionId::new("fixture-runtime").expect("runtime session id"),
        resume_binding: binding,
        provider_id: ProviderId::new("fixture").expect("provider id"),
        model_id,
        provider_session_id: "fixture-session".to_owned(),
        directory: "/workspace".to_owned(),
        services,
        transport: CurlTransport,
        access: Some(AccessLeases {
            endpoint: endpoint.clone(),
            resource: Some(ResourceLease::consumer_owned(
                lease_scope.clone(),
                working_resource,
                ResourceAccess::Read,
                ResourceRepresentation::Filesystem,
            )),
            credential: Some(CredentialLease::Delegated(DelegatedCredential::new(
                lease_scope,
                CredentialRef::new("opencode.fixture.credential").expect("credential ref"),
                EndpointAudience::new("opencode.fixture.audience").expect("audience"),
            ))),
            directory: None,
        }),
        endpoint,
        active: Arc::clone(&active),
        cancellation: Arc::new(SessionCancellation::new(active)),
        reasoning_mode: None,
        structured_output: None,
        image_attachments: false,
        provider_callbacks: false,
        active_turn_detachment: false,
        callback_run_id: None,
    }
}

struct StalledDelete {
    dispatched: AtomicBool,
}

impl BlockingWorkService for StalledDelete {
    fn run(
        &self,
        scope: ScopeId,
        _job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        assert_eq!(scope.as_str(), "opencode-http:run-delete:fixture-request");
        self.dispatched.store(true, Ordering::SeqCst);
        Box::pin(pending())
    }
}

#[test]
fn stalled_run_delete_is_signalled_and_both_leases_are_released() {
    let host = RecordingHostServices::new(RecordingOutcome::Succeed);
    let worker = Arc::new(StalledDelete {
        dispatched: AtomicBool::new(false),
    });
    let services = host.services().clone().with_blocking_work(worker.clone());
    let session = session(services, "http://127.0.0.1:1".to_owned());
    let cancelled = Arc::new(AtomicBool::new(false));

    let result = session
        .close_and_delete(Arc::clone(&cancelled))
        .now_or_never()
        .expect("the recording clock expires the stalled deletion");
    assert!(worker.dispatched.load(Ordering::SeqCst));
    assert!(cancelled.load(Ordering::SeqCst));
    assert_eq!(result.deletion, RemoteResourceDeletionOutcome::Unconfirmed);
    assert_eq!(
        result.cleanup.diagnostic().map(SafeDiagnostic::code),
        Some("swallowtail.opencode.run_delete_timed_out")
    );
    assert!(host.calls().ends_with(&[
        RecordedHostCall::WorkingResourceRelease,
        RecordedHostCall::CredentialRelease,
    ]));
}

#[test]
fn dropping_in_flight_run_delete_signals_its_worker() {
    let host = RecordingHostServices::new(RecordingOutcome::Succeed);
    let worker = Arc::new(StalledDelete {
        dispatched: AtomicBool::new(false),
    });
    let services = HostServices::new(host.services().execution_host_id().clone())
        .with_blocking_work(worker.clone());
    let mut session = session(services, "http://127.0.0.1:1".to_owned());
    let cancelled = Arc::new(AtomicBool::new(false));
    let mut deleting = Box::pin(session.delete_run_session(Arc::clone(&cancelled)));
    assert!(deleting.as_mut().now_or_never().is_none());
    assert!(worker.dispatched.load(Ordering::SeqCst));
    assert!(!cancelled.load(Ordering::SeqCst));

    drop(deleting);
    assert!(cancelled.load(Ordering::SeqCst));
}

struct PromptGate {
    calls: AtomicUsize,
    prompt_ready: Mutex<Option<oneshot::Receiver<()>>>,
}

impl BlockingWorkService for PromptGate {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        match self.calls.fetch_add(1, Ordering::SeqCst) {
            // Only the prompt makes a network exchange. The SSE and abort
            // jobs serve as independently observable dispatches.
            0 | 2 => Box::pin(ready(Ok(()))),
            1 => {
                let prompt_ready = self
                    .prompt_ready
                    .lock()
                    .expect("prompt gate lock")
                    .take()
                    .expect("one prompt");
                Box::pin(async move {
                    prompt_ready.await.expect("prompt released by test");
                    job()
                })
            }
            _ => panic!("unexpected OpenCode HTTP work"),
        }
    }
}

#[derive(Default)]
struct JoinPump(Arc<AtomicBool>);

struct PumpTask {
    task: BoxFuture<'static, ()>,
    joined: Arc<AtomicBool>,
}

impl ScopedTaskService for JoinPump {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(PumpTask {
            task,
            joined: Arc::clone(&self.0),
        }))
    }
}

impl JoinedTask for PumpTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.task.await;
            self.joined.store(true, Ordering::SeqCst);
            Ok(())
        })
    }
}

#[test]
fn session_cancel_during_prompt_setup_aborts_and_joins_the_unpublished_turn() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback prompt fixture");
    let endpoint = format!("http://{}", listener.local_addr().expect("local address"));
    let server = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().expect("accept prompt request");
        let reader = BufReader::new(&mut socket);
        let mut headers = reader.lines();
        let request_line = headers
            .next()
            .expect("request line")
            .expect("valid request");
        assert!(request_line.starts_with("POST /session/fixture-session/prompt_async?"));
        for header in headers {
            if header.expect("valid HTTP header").is_empty() {
                break;
            }
        }
        socket
            .write_all(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n")
            .expect("acknowledge prompt");
    });

    let host = RecordingHostServices::new(RecordingOutcome::Succeed);
    let (release_prompt, prompt_ready) = oneshot::channel();
    let blocking = Arc::new(PromptGate {
        calls: AtomicUsize::new(0),
        prompt_ready: Mutex::new(Some(prompt_ready)),
    });
    let tasks = Arc::new(JoinPump::default());
    let services = host
        .services()
        .clone()
        .with_blocking_work(blocking.clone())
        .with_task(tasks.clone());
    let mut session = session(services.clone(), endpoint);
    let cancellation = Arc::clone(&session.cancellation);
    let turn = TurnRequest::new(
        RuntimeTurnId::new("fixture-turn").expect("turn id"),
        OperationContent::new("fixture prompt").expect("prompt"),
    );
    let mut starting = session.start_turn(turn, services);
    assert!(starting.as_mut().now_or_never().is_none());
    assert_eq!(blocking.calls.load(Ordering::SeqCst), 2);

    assert_eq!(
        cancellation
            .request()
            .now_or_never()
            .expect("no active turn")
            .expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    release_prompt.send(()).expect("prompt still pending");
    let error = futures_executor::block_on(starting)
        .err()
        .expect("cancelled session rejects the in-flight start");
    server.join().expect("prompt fixture joined");
    assert_eq!(error.diagnostic().code(), "swallowtail.opencode.session_cancelled");
    assert_eq!(
        blocking.calls.load(Ordering::SeqCst),
        3,
        "remote abort was dispatched"
    );
    assert!(tasks.0.load(Ordering::SeqCst), "the turn pump was joined");
    assert!(session.active.lock().expect("active slot lock").is_none());
}
