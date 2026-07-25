use swallowtail_adapter_codex::{
    CodexExecProfileInput, CodexModelSelection, CodexPreparationInput, CodexPreparationProbe,
    CodexPreparedCatalogue, CodexPreparedDriver, CodexPreparedExec, CodexPreparedIntegration,
    CodexPreparedSession, CodexSessionProfileInput, prepare_codex,
};
use swallowtail_core::{
    AccessProfile, AccessStatus, ConfiguredInstanceId, ExecutionHostId, ExternalNetworkPolicy,
    ExternalSearchPolicy, InstanceRevision, ModelCatalogEntry, ModelId, ModelRouteId,
    ModelRouteRevision,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, HostServices, InstalledExecutableTarget,
    InteractiveSessionHandle, OperationContent, PreparationFailure, PreparedAccessEvidence,
    RequestId, RunHandle, RuntimeFailure, ScopeId, SessionOptions, WorkingResourceRef,
};

async fn prepare_installed_codex(
    host: ExecutionHostId,
    services: HostServices,
    target: InstalledExecutableTarget,
    environment: EnvironmentRef,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    deadline: Deadline,
) -> Result<CodexPreparedIntegration, PreparationFailure> {
    let input = CodexPreparationInput::new(
        CodexPreparedDriver::AppServer,
        ConfiguredInstanceId::new("codex.local").expect("instance id is valid"),
        InstanceRevision::new("1").expect("instance revision is valid"),
        host,
        target,
        environment,
        access_profile,
        PreparedAccessEvidence::caller_asserted(access_status),
    );
    let probe = CodexPreparationProbe::new(
        RequestId::new("codex-prepare").expect("request id is valid"),
        ScopeId::new("codex-prepare").expect("scope id is valid"),
        deadline,
        DiscoveryCancellation::new(),
    );
    prepare_codex(input, probe, services).await
}

fn prepare_read_only_session(
    prepared: &CodexPreparedIntegration,
    model: CodexModelSelection,
    working_resource: WorkingResourceRef,
) -> Result<CodexPreparedSession, PreparationFailure> {
    prepared.prepare_read_only_session(CodexSessionProfileInput::new(
        RequestId::new("codex-session").expect("request id is valid"),
        model,
        working_resource,
        None,
        SessionOptions::default(),
    ))
}

fn prepare_offline_exec(
    prepared: &CodexPreparedIntegration,
    model: CodexModelSelection,
    working_resource: WorkingResourceRef,
) -> Result<CodexPreparedExec, PreparationFailure> {
    prepared.prepare_structured_exec(CodexExecProfileInput::new(
        RequestId::new("codex-exec").expect("request id is valid"),
        OperationContent::new("consumer-owned prompt").expect("prompt is valid"),
        model,
        working_resource,
        ExternalNetworkPolicy::Denied,
        ExternalSearchPolicy::Disabled,
    ))
}

async fn list_models(
    catalogue: &CodexPreparedCatalogue,
    services: HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    catalogue.list_models(services).await
}

async fn open_session(
    session: &CodexPreparedSession,
    services: HostServices,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    session.open_session(services).await
}

async fn start_exec(
    exec: &CodexPreparedExec,
    services: HostServices,
) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
    exec.start_run(services).await
}

fn main() {
    let _ = prepare_installed_codex;
    let _ = prepare_read_only_session;
    let _ = prepare_offline_exec;
    let _ = list_models;
    let _ = open_session;
    let _ = start_exec;
    let _ = CodexModelSelection::new(
        ModelRouteId::new("codex-route").expect("route id is valid"),
        ModelRouteRevision::new("1").expect("route revision is valid"),
        ModelId::new("explicit-model").expect("model id is valid"),
    );
}
