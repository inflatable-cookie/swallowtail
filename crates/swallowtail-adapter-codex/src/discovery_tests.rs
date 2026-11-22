use super::{
    CODEX_INSTALL_GUIDANCE, ExitSignal, attach_install_guidance, parse_version, probe_process,
    stop_and_classify, wait_for_exit,
};
use crate::selection::CODEX_CLI_AXIS;
use crate::{CodexAppServerDriver, CodexExecDriver};
use std::sync::Arc;
use swallowtail_core::{
    DiscoveryOutcome, DiscoveryStatus, ExecutionHostId, InterfaceVersionAxis, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, DiscoveryCancellation, ExecutableRef, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, MonotonicInstant, ProcessExit,
    ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessRequest, ProcessService,
    RequestId, RuntimeFailure, ScopeId, TimeService,
};

#[test]
fn absent_guidance_is_frozen_and_present_outcomes_have_none() {
    let absent = attach_install_guidance(DiscoveryOutcome::new(DiscoveryStatus::Absent, None));
    let guidance = absent.install_guidance().expect("absent guidance");
    assert_eq!(guidance, &CODEX_INSTALL_GUIDANCE);
    assert_eq!(CodexExecDriver::install_guidance(), CODEX_INSTALL_GUIDANCE);
    assert_eq!(
        CodexAppServerDriver::install_guidance(),
        CODEX_INSTALL_GUIDANCE
    );
    assert_eq!(guidance.display_name(), "Codex CLI");
    assert_eq!(
        guidance.command(),
        "curl -fsSL https://chatgpt.com/codex/install.sh | sh"
    );
    assert_eq!(
        guidance.documentation_url(),
        "https://github.com/openai/codex/blob/main/README.md#installing-and-running-codex-cli"
    );
    assert_eq!(guidance.frozen_on(), "2026-09-06");

    let present = attach_install_guidance(DiscoveryOutcome::new(DiscoveryStatus::Discovered, None));
    assert!(present.install_guidance().is_none());
}

#[test]
fn parser_accepts_one_exact_codex_cli_semver() {
    let binding = parse_version(
        b"codex-cli 0.145.0\n",
        InterfaceVersionAxis::new(CODEX_CLI_AXIS).unwrap(),
    )
    .expect("version parses");
    assert_eq!(binding.version().as_str(), "0.145.0");
}

#[test]
fn parser_rejects_raw_payload_variants() {
    for output in [
        b"0.145.0".as_slice(),
        b"codex-cli latest".as_slice(),
        b"codex-cli 0.145.0 extra".as_slice(),
        b"private payload".as_slice(),
    ] {
        assert!(
            parse_version(output, InterfaceVersionAxis::new(CODEX_CLI_AXIS).unwrap()).is_none()
        );
    }
}

struct ExitedOnGracefulStop;

impl ProcessHandle for ExitedOnGracefulStop {
    fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async { Ok(None) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "test.already_exited",
                "process already exited after graceful stop",
            )))
        })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

struct HangingWaitProcess;

impl ProcessHandle for HangingWaitProcess {
    fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async { Ok(None) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(std::future::pending())
    }
}

struct ImmediateTime;

impl TimeService for ImmediateTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(10)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}

struct HangingWaitProcessService;

impl ProcessService for HangingWaitProcessService {
    fn start(
        &self,
        _scope: ScopeId,
        _request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        Box::pin(async { Ok(Box::new(HangingWaitProcess) as Box<dyn ProcessHandle>) })
    }
}

#[test]
fn stop_and_classify_preserves_desired_outcome_when_force_stop_errors_after_clean_graceful_stop() {
    let result = futures_executor::block_on(stop_and_classify(
        &ExitedOnGracefulStop,
        &ImmediateTime,
        DiscoveryOutcome::new(DiscoveryStatus::TimedOut, None),
    ));
    assert_eq!(result.status(), DiscoveryStatus::TimedOut);
}

#[test]
fn wait_for_exit_bounds_hanging_wait_by_deadline() {
    let deadline_instant = MonotonicInstant::from_ticks(1);
    let mut deadline: BoxFuture<'static, DeadlineObservation> = Box::pin(async move {
        DeadlineObservation::new(Deadline::at(deadline_instant), deadline_instant)
    });
    let mut cancelled: BoxFuture<'static, ()> = Box::pin(std::future::pending());
    let signal = futures_executor::block_on(wait_for_exit(
        &HangingWaitProcess,
        &mut deadline,
        &mut cancelled,
    ));
    assert!(matches!(signal, ExitSignal::TimedOut));
}

struct HangingStopProcess {
    hang_graceful: bool,
}

impl ProcessHandle for HangingStopProcess {
    fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async { Ok(None) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        if self.hang_graceful {
            Box::pin(std::future::pending())
        } else {
            Box::pin(async { Ok(()) })
        }
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        if self.hang_graceful {
            Box::pin(async { Ok(()) })
        } else {
            Box::pin(std::future::pending())
        }
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(std::future::pending())
    }
}

struct HangingStopProcessService {
    hang_graceful: bool,
}

impl ProcessService for HangingStopProcessService {
    fn start(
        &self,
        _scope: ScopeId,
        _request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        let hang_graceful = self.hang_graceful;
        Box::pin(async move {
            Ok(Box::new(HangingStopProcess { hang_graceful }) as Box<dyn ProcessHandle>)
        })
    }
}

#[test]
fn probe_process_bounds_hanging_wait_through_teardown_and_reports_cleanup_failed() {
    let host_id = ExecutionHostId::new("codex.discovery.teardown-test").expect("host id is valid");
    let services = HostServices::new(host_id.clone())
        .with_time(Arc::new(ImmediateTime))
        .with_process(Arc::new(HangingWaitProcessService));
    let axis = InterfaceVersionAxis::new(CODEX_CLI_AXIS).expect("axis is valid");
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("codex.discovery.teardown-req").expect("request id is valid"),
        ScopeId::new("codex.discovery.teardown-scope").expect("scope is valid"),
        host_id,
        InstalledExecutableTarget::new(
            ExecutableRef::new("codex.fixture").expect("executable ref is valid"),
            axis,
        ),
        Deadline::at(MonotonicInstant::from_ticks(20)),
        DiscoveryCancellation::new(),
    );
    let claim = crate::selection::codex_exec_claim();
    let result = futures_executor::block_on(probe_process(request, services, claim))
        .expect("probe returns bounded outcome instead of hanging");
    assert_eq!(result.status(), DiscoveryStatus::CleanupFailed);
}

#[test]
fn probe_process_bounds_hanging_stop_requests_through_teardown() {
    for hang_graceful in [true, false] {
        let host_id =
            ExecutionHostId::new("codex.discovery.stop-hang-test").expect("host id is valid");
        let services = HostServices::new(host_id.clone())
            .with_time(Arc::new(ImmediateTime))
            .with_process(Arc::new(HangingStopProcessService { hang_graceful }));
        let axis = InterfaceVersionAxis::new(CODEX_CLI_AXIS).expect("axis is valid");
        let request = InstalledExecutableDiscoveryRequest::new(
            RequestId::new("codex.discovery.stop-hang-req").expect("request id is valid"),
            ScopeId::new("codex.discovery.stop-hang-scope").expect("scope is valid"),
            host_id,
            InstalledExecutableTarget::new(
                ExecutableRef::new("codex.fixture").expect("executable ref is valid"),
                axis,
            ),
            Deadline::at(MonotonicInstant::from_ticks(20)),
            DiscoveryCancellation::new(),
        );
        let claim = crate::selection::codex_exec_claim();
        let result = futures_executor::block_on(probe_process(request, services, claim))
            .expect("probe returns bounded outcome instead of hanging on pending stop");
        assert_eq!(result.status(), DiscoveryStatus::CleanupFailed);
    }
}
