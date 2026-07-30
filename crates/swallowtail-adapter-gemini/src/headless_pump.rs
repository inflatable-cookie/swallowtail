use crate::headless_events::GeminiHeadlessEventParser;
use crate::headless_handle::GeminiHeadlessCancellation;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{ModelId, OwnedRemoteResourceKind, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, EnvironmentRef,
    ExecutableRef, ProcessHandle, ProcessOutputChunk, ProcessOutputStream, ProcessRequest,
    ProcessService, RemoteResourceDeletionOutcome, RuntimeEventSender, RuntimeFailure, ScopeId,
    TerminalOutcome, TerminalStatus, WorkingResourceRef,
};

const MANAGEMENT_OUTPUT_LIMIT: usize = 64 * 1024;

pub(crate) struct TranscriptCleanup {
    pub(crate) process_service: Arc<dyn ProcessService>,
    pub(crate) executable: ExecutableRef,
    pub(crate) environment: EnvironmentRef,
    pub(crate) working_resource: WorkingResourceRef,
    pub(crate) session_id: String,
    pub(crate) deadline: BoxFuture<'static, DeadlineObservation>,
}

pub(crate) struct HeadlessProjection {
    pub(crate) model: ModelId,
    pub(crate) session_id: String,
    pub(crate) operation_id: ActivityOperationId,
}

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<GeminiHeadlessCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    projection: HeadlessProjection,
    cleanup: Option<TranscriptCleanup>,
) -> TerminalOutcome {
    let outcome = pump_run(process, events, cancellation, deadline, projection).await;
    let Some(cleanup) = cleanup else {
        return outcome;
    };
    let (deletion, cleanup_outcome) = delete_transcript(cleanup).await;
    replace_cleanup(outcome, cleanup_outcome)
        .with_remote_resource_deletion(OwnedRemoteResourceKind::Session, deletion)
}

async fn pump_run(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<GeminiHeadlessCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    projection: HeadlessProjection,
) -> TerminalOutcome {
    let mut parser = GeminiHeadlessEventParser::new(
        projection.model,
        projection.session_id,
        projection.operation_id,
    );
    let mut deadline = Some(deadline);
    loop {
        match next_output(process.as_ref(), cancellation.as_ref(), &mut deadline).await {
            NextOutput::Deadline => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return TerminalOutcome::new(TerminalStatus::TimedOut, cleanup);
            }
            NextOutput::Process(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        if send_all(&events, parsed).is_err() {
                            let cleanup = force_cleanup(process.as_ref()).await;
                            return event_delivery_failed(cleanup);
                        }
                    }
                    Err(error) => {
                        let cleanup = force_cleanup(process.as_ref()).await;
                        return TerminalOutcome::new(
                            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                            cleanup,
                        );
                    }
                }
            }
            NextOutput::Process(Ok(Some(_))) => {}
            NextOutput::Process(Ok(None)) => break,
            NextOutput::Process(Err(error)) => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return TerminalOutcome::new(
                    TerminalStatus::HostFailed(error.diagnostic().clone()),
                    cleanup,
                );
            }
        }
    }
    let exit = process.wait().await;
    if cancellation.is_requested() {
        return TerminalOutcome::new(TerminalStatus::Cancelled, cleanup_from_wait(&exit));
    }
    match (parser.finish(), exit) {
        (Ok((trailing, parsed)), Ok(exit)) => {
            if send_all(&events, trailing).is_err() {
                event_delivery_failed(CleanupOutcome::Clean)
            } else {
                parsed.outcome(exit)
            }
        }
        (Err(error), exit) => TerminalOutcome::new(
            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
            cleanup_from_wait(&exit),
        ),
        (_, Err(_)) => TerminalOutcome::new(
            TerminalStatus::HostFailed(SafeDiagnostic::new(
                "swallowtail.gemini.headless.process_wait_failed",
                "Gemini headless process wait failed",
            )),
            process_cleanup_failed(),
        ),
    }
}

async fn delete_transcript(
    cleanup: TranscriptCleanup,
) -> (RemoteResourceDeletionOutcome, CleanupOutcome) {
    let mut deadline = Some(cleanup.deadline);
    let delete = run_management_process(
        cleanup.process_service.as_ref(),
        &cleanup.executable,
        &cleanup.environment,
        &cleanup.working_resource,
        crate::headless_command::delete_session_arguments(&cleanup.session_id),
        "delete",
        &mut deadline,
    )
    .await;
    if delete.is_err() {
        return unconfirmed_deletion();
    }
    let list = run_management_process(
        cleanup.process_service.as_ref(),
        &cleanup.executable,
        &cleanup.environment,
        &cleanup.working_resource,
        crate::headless_command::list_sessions_arguments(),
        "reconcile",
        &mut deadline,
    )
    .await;
    match list {
        Ok(result)
            if result.exit.success()
                && !contains_exact_session_id(&result.combined(), &cleanup.session_id) =>
        {
            (
                RemoteResourceDeletionOutcome::Confirmed,
                CleanupOutcome::Clean,
            )
        }
        _ => unconfirmed_deletion(),
    }
}

pub(crate) async fn run_management_process(
    service: &dyn ProcessService,
    executable: &ExecutableRef,
    environment: &EnvironmentRef,
    working_resource: &WorkingResourceRef,
    arguments: Vec<String>,
    phase: &str,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> Result<ManagementProcessResult, RuntimeFailure> {
    let scope = ScopeId::new(format!("gemini-headless:transcript-{phase}")).map_err(|_| {
        RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.gemini.headless.cleanup_scope_invalid",
            "Gemini transcript cleanup scope was invalid",
        ))
    })?;
    let process = service
        .start(
            scope,
            ProcessRequest::new(executable.clone())
                .with_arguments(arguments)
                .with_environment([environment.clone()])
                .with_working_resource(working_resource.clone()),
        )
        .await?;
    process.close_stdin().await?;
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    loop {
        let mut read = process.read_output();
        let signal = poll_fn(|context| {
            if let Some(wait) = deadline.as_mut()
                && wait.as_mut().poll(context).is_ready()
            {
                return Poll::Ready(ManagementSignal::Deadline);
            }
            read.as_mut().poll(context).map(ManagementSignal::Output)
        })
        .await;
        match signal {
            ManagementSignal::Deadline => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                return Err(management_failure());
            }
            ManagementSignal::Output(Ok(Some(chunk))) => {
                if stdout
                    .len()
                    .saturating_add(stderr.len())
                    .saturating_add(chunk.bytes().len())
                    > MANAGEMENT_OUTPUT_LIMIT
                {
                    let _ = process.force_stop().await;
                    let _ = process.wait().await;
                    return Err(management_failure());
                }
                match chunk.stream() {
                    ProcessOutputStream::Stdout => stdout.extend_from_slice(chunk.bytes()),
                    ProcessOutputStream::Stderr => stderr.extend_from_slice(chunk.bytes()),
                }
            }
            ManagementSignal::Output(Ok(None)) => break,
            ManagementSignal::Output(Err(_)) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                return Err(management_failure());
            }
        }
    }
    Ok(ManagementProcessResult {
        exit: process.wait().await?,
        stdout,
        stderr,
    })
}

pub(crate) struct ManagementProcessResult {
    pub(crate) exit: swallowtail_runtime::ProcessExit,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
}

impl ManagementProcessResult {
    pub(crate) fn combined(&self) -> Vec<u8> {
        let mut combined = Vec::with_capacity(self.stdout.len() + self.stderr.len());
        combined.extend_from_slice(&self.stdout);
        combined.extend_from_slice(&self.stderr);
        combined
    }
}

enum ManagementSignal {
    Output(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

pub(crate) fn contains_exact_session_id(output: &[u8], session_id: &str) -> bool {
    let needle = session_id.as_bytes();
    output
        .windows(needle.len())
        .enumerate()
        .any(|(index, value)| {
            value == needle
                && index
                    .checked_sub(1)
                    .is_none_or(|before| !is_session_id_byte(output[before]))
                && output
                    .get(index + needle.len())
                    .is_none_or(|after| !is_session_id_byte(*after))
        })
}

fn is_session_id_byte(value: u8) -> bool {
    value.is_ascii_alphanumeric() || matches!(value, b'-' | b'_')
}

fn management_failure() -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.gemini.headless.transcript_deletion_unconfirmed",
        "Gemini stored-transcript deletion could not be confirmed",
    ))
}

fn unconfirmed_deletion() -> (RemoteResourceDeletionOutcome, CleanupOutcome) {
    (
        RemoteResourceDeletionOutcome::Unconfirmed,
        CleanupOutcome::Degraded(SafeDiagnostic::new(
            "swallowtail.gemini.headless.transcript_deletion_unconfirmed",
            "Gemini stored-transcript deletion could not be confirmed",
        )),
    )
}

fn replace_cleanup(outcome: TerminalOutcome, cleanup: CleanupOutcome) -> TerminalOutcome {
    let cleanup = merge_cleanup(outcome.cleanup().clone(), cleanup);
    let mut replaced = TerminalOutcome::new(outcome.status().clone(), cleanup);
    if let Some(output) = outcome.output().cloned() {
        replaced = replaced.with_output(output);
    }
    if let Some(cancellation) = outcome.provider_cancellation() {
        replaced = replaced.with_provider_cancellation(cancellation);
    }
    for (kind, deletion) in outcome.remote_resource_deletions() {
        replaced = replaced.with_remote_resource_deletion(kind, deletion);
    }
    replaced
}

fn merge_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(error), _) | (_, CleanupOutcome::Failed(error)) => {
            CleanupOutcome::Failed(error)
        }
        (CleanupOutcome::Degraded(error), _) | (_, CleanupOutcome::Degraded(error)) => {
            CleanupOutcome::Degraded(error)
        }
        (CleanupOutcome::Clean, CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
        | (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}

enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &GeminiHeadlessCancellation,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> NextOutput {
    let mut read = process.read_output();
    poll_fn(|context| {
        if !cancellation.is_requested()
            && let Some(wait) = deadline.as_mut()
            && wait.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(NextOutput::Deadline);
        }
        read.as_mut().poll(context).map(NextOutput::Process)
    })
    .await
}

fn send_all(
    sender: &RuntimeEventSender,
    events: impl IntoIterator<Item = swallowtail_runtime::RuntimeEvent>,
) -> Result<(), RuntimeFailure> {
    for event in events {
        sender.send(event)?;
    }
    Ok(())
}

pub(crate) async fn cleanup_failed_start(process: &dyn ProcessHandle) {
    let _ = force_cleanup(process).await;
}

async fn force_cleanup(process: &dyn ProcessHandle) -> CleanupOutcome {
    let force = process.force_stop().await;
    let wait = process.wait().await;
    if force.is_err() || wait.is_err() {
        process_cleanup_failed()
    } else {
        CleanupOutcome::Clean
    }
}

fn cleanup_from_wait(
    exit: &Result<swallowtail_runtime::ProcessExit, RuntimeFailure>,
) -> CleanupOutcome {
    if exit.is_ok() {
        CleanupOutcome::Clean
    } else {
        process_cleanup_failed()
    }
}

fn event_delivery_failed(cleanup: CleanupOutcome) -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
            "swallowtail.gemini.headless.event_delivery_failed",
            "Gemini headless event delivery failed",
        )),
        cleanup,
    )
}

fn process_cleanup_failed() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.gemini.headless.process_cleanup_failed",
        "Gemini headless process cleanup failed",
    ))
}
