use std::ffi::OsString;
use std::future::Future;
use std::io::{Read, Write};
use std::pin::pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    EnvironmentRef, ExecutableRef, ProcessHandle, ProcessOutputStream, ProcessRequest,
    ProcessService, RuntimeFailure, ScopeId, WorkingResourceRef,
};

const CHILD_MODE: &str = "SWALLOWTAIL_PROCESS_FIXTURE_MODE";
const CHILD_SECRET: &str = "SWALLOWTAIL_PROCESS_FIXTURE_SECRET";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn process_fixture() {
    let Ok(mode) = std::env::var(CHILD_MODE) else {
        return;
    };
    match mode.as_str() {
        "echo" => {
            let mut input = Vec::new();
            std::io::stdin()
                .read_to_end(&mut input)
                .expect("fixture reads stdin");
            std::io::stdout()
                .write_all(b"fixture-stdout:")
                .expect("fixture writes stdout");
            std::io::stdout()
                .write_all(&input)
                .expect("fixture echoes stdin");
            let secret = std::env::var(CHILD_SECRET).expect("fixture secret is present");
            std::io::stderr()
                .write_all(secret.as_bytes())
                .expect("fixture writes stderr");
            assert!(
                std::path::Path::new("fixture-marker").exists(),
                "fixture working resource is active"
            );
        }
        "overflow" => {
            std::io::stdout()
                .write_all(&[b'x'; 256])
                .expect("fixture writes bounded overflow");
        }
        "wait-for-eof" => {
            let mut input = Vec::new();
            std::io::stdin()
                .read_to_end(&mut input)
                .expect("fixture waits for eof");
        }
        "working-resource" => {
            std::fs::write("host-owned-marker", b"created")
                .expect("fixture writes inside the host-owned resource");
        }
        "sleep" => thread::sleep(Duration::from_secs(30)),
        "version" => {
            std::io::stdout()
                .write_all(b"fixture-harness 1.2.0\n")
                .expect("fixture writes version");
        }
        _ => panic!("unknown fixture mode"),
    }
}

pub(crate) fn fixture_host(
    mode: &str,
    limits: LocalProcessLimits,
    resource_directory: &std::path::Path,
) -> (
    LocalProcessHost,
    ExecutableRef,
    EnvironmentRef,
    WorkingResourceRef,
) {
    let executable = executable_ref();
    let environment = environment_ref();
    let resource = working_resource_ref();
    let host = LocalProcessHost::builder(limits)
        .approve_executable(
            executable.clone(),
            std::env::current_exe().expect("test executable"),
        )
        .approve_environment(environment.clone(), fixture_environment(mode))
        .approve_working_resource(resource.clone(), resource_directory)
        .build();
    (host, executable, environment, resource)
}

pub(crate) fn request(
    executable: &ExecutableRef,
    environment: &EnvironmentRef,
    resource: &WorkingResourceRef,
) -> ProcessRequest {
    ProcessRequest::new(executable.clone())
        .with_arguments(fixture_arguments())
        .with_environment([environment.clone()])
        .with_working_resource(resource.clone())
}

pub(crate) fn fixture_arguments() -> [String; 3] {
    [
        "--exact".to_owned(),
        "support::process_fixture".to_owned(),
        "--nocapture".to_owned(),
    ]
}

pub(crate) fn fixture_environment(mode: &str) -> [(OsString, OsString); 2] {
    [
        (OsString::from(CHILD_MODE), OsString::from(mode)),
        (
            OsString::from(CHILD_SECRET),
            OsString::from("stderr-secret"),
        ),
    ]
}

pub(crate) fn executable_ref() -> ExecutableRef {
    ExecutableRef::new("fixture-local-process").expect("executable reference is valid")
}

pub(crate) fn environment_ref() -> EnvironmentRef {
    EnvironmentRef::new("fixture-local-environment").expect("environment reference is valid")
}

pub(crate) fn working_resource_ref() -> WorkingResourceRef {
    WorkingResourceRef::new("fixture-working-resource").expect("resource reference is valid")
}

pub(crate) fn start(
    host: &LocalProcessHost,
    request: ProcessRequest,
) -> Result<Box<dyn ProcessHandle>, RuntimeFailure> {
    start_in_scope(host, process_scope(), request)
}

pub(crate) fn start_in_scope(
    host: &LocalProcessHost,
    scope: ScopeId,
    request: ProcessRequest,
) -> Result<Box<dyn ProcessHandle>, RuntimeFailure> {
    block_on(host.start(scope, request))
}

pub(crate) fn process_scope() -> ScopeId {
    ScopeId::new("fixture-process-scope").expect("scope is valid")
}

pub(crate) fn collect_output(
    process: &dyn ProcessHandle,
) -> Result<(Vec<u8>, Vec<u8>), RuntimeFailure> {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    while let Some(chunk) = block_on(process.read_output())? {
        match chunk.stream() {
            ProcessOutputStream::Stdout => stdout.extend_from_slice(chunk.bytes()),
            ProcessOutputStream::Stderr => stderr.extend_from_slice(chunk.bytes()),
        }
    }
    Ok((stdout, stderr))
}

pub(crate) fn assert_failure_code<T>(result: Result<T, RuntimeFailure>, expected: &'static str) {
    let failure = result.err().expect("operation must fail");
    assert_eq!(failure.diagnostic().code(), expected);
    assert!(!format!("{failure:?}").contains("stderr-secret"));
}

pub(crate) fn temporary_resource() -> std::path::PathBuf {
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time follows epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "swallowtail-process-{}-{nanos}-{sequence}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path).expect("fixture resource is created");
    path
}

struct ThreadWake(thread::Thread);

impl Wake for ThreadWake {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

pub(crate) fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWake(thread::current())));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        assert!(Instant::now() < deadline, "fixture future timed out");
        thread::park_timeout(Duration::from_millis(10));
    }
}
