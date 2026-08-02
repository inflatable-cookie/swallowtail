use super::ObservedProcessRequest;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

#[derive(Clone, Copy)]
pub enum AppServerMode {
    CompleteTurn,
    HoldCatalog,
    HoldTurn,
    RequestCallback,
    ObserveApproval,
    ObserveUserInput,
    ExchangeUserInput,
    ExchangeUserInputNumericRequestId,
    DynamicToolCall,
    HoldDynamicToolCall,
    DisconnectTurn,
    MismatchedTurnSession,
    SubstituteResume,
    LifecycleSuccess,
    LifecycleReject,
    LifecycleDisconnect,
    LifecycleHold,
    LifecycleMalformed,
    LifecycleCleanupFailure,
    LifecycleWrongNotification,
    ThreadCatalogue(ThreadCatalogueMode),
}

#[derive(Clone, Copy)]
pub enum ThreadCatalogueMode {
    Available,
    WrongResource,
    Missing,
    Active,
    Changed,
    Mismatched,
    Hold,
    Disconnect,
    CleanupFailure,
}

#[derive(Default)]
pub struct AppServerState {
    request: Mutex<Option<ObservedProcessRequest>>,
    messages: Mutex<Vec<serde_json::Value>>,
    output: Mutex<VecDeque<ProcessOutputChunk>>,
    input: Mutex<Vec<u8>>,
    active_thread: Mutex<Option<String>>,
    experimental_api: AtomicBool,
    closed: AtomicBool,
    forced: AtomicBool,
    waited: AtomicBool,
}

impl AppServerState {
    pub fn started(&self) -> bool {
        self.request
            .lock()
            .expect("request lock is available")
            .is_some()
    }

    pub fn request(&self) -> ObservedProcessRequest {
        self.request
            .lock()
            .expect("request lock is available")
            .clone()
            .expect("process request was captured")
    }

    pub fn methods(&self) -> Vec<String> {
        self.messages
            .lock()
            .expect("messages lock is available")
            .iter()
            .filter_map(|message| message.get("method")?.as_str().map(str::to_owned))
            .collect()
    }

    pub fn messages(&self) -> Vec<serde_json::Value> {
        self.messages
            .lock()
            .expect("messages lock is available")
            .clone()
    }

    pub fn forced(&self) -> bool {
        self.forced.load(Ordering::SeqCst)
    }

    pub fn waited(&self) -> bool {
        self.waited.load(Ordering::SeqCst)
    }

    fn push(&self, message: serde_json::Value) {
        let mut bytes = serde_json::to_vec(&message).expect("fixture JSON is valid");
        bytes.push(b'\n');
        self.output
            .lock()
            .expect("output lock is available")
            .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
    }
}

pub struct ScriptedAppServer {
    state: Arc<AppServerState>,
    mode: AppServerMode,
    enforce_experimental_gate: bool,
    started: AtomicBool,
}

impl ScriptedAppServer {
    pub fn new(mode: AppServerMode) -> (Arc<Self>, Arc<AppServerState>) {
        Self::with_experimental_gate(mode, false)
    }

    pub fn gate_enforcing(mode: AppServerMode) -> (Arc<Self>, Arc<AppServerState>) {
        Self::with_experimental_gate(mode, true)
    }

    fn with_experimental_gate(
        mode: AppServerMode,
        enforce_experimental_gate: bool,
    ) -> (Arc<Self>, Arc<AppServerState>) {
        let state = Arc::new(AppServerState::default());
        (
            Arc::new(Self {
                state: Arc::clone(&state),
                mode,
                enforce_experimental_gate,
                started: AtomicBool::new(false),
            }),
            state,
        )
    }
}

impl ProcessService for ScriptedAppServer {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        assert!(!self.started.swap(true, Ordering::SeqCst));
        *self
            .state
            .request
            .lock()
            .expect("request lock is available") = Some(ObservedProcessRequest {
            executable: request.executable().as_host_value().to_owned(),
            arguments: request.arguments().map(str::to_owned).collect(),
            environments: request
                .environment()
                .map(|value| value.as_host_value().to_owned())
                .collect(),
            working_resource: request
                .working_resource()
                .map(|value| value.as_host_value().to_owned()),
        });
        let handle = ScriptedAppServerHandle {
            state: Arc::clone(&self.state),
            mode: self.mode,
            enforce_experimental_gate: self.enforce_experimental_gate,
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct ScriptedAppServerHandle {
    state: Arc<AppServerState>,
    mode: AppServerMode,
    enforce_experimental_gate: bool,
}

include!("app_server/handle.rs");
include!("app_server/process_handle.rs");
include!("app_server/thread_catalogue.rs");
