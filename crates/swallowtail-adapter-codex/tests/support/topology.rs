use super::app_server_plan_for;
use swallowtail_adapter_codex::CodexAppServerDriver;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, DriverRole, PreflightPlan,
};
use swallowtail_runtime::{
    EnvironmentRef, OpenSessionRequest, RequestId, SchemaDocument, SessionAccessPolicy,
    SessionOptions, ToolDeclaration,
};
use swallowtail_testkit::ExecutionTopologyFixture;

pub fn driver() -> CodexAppServerDriver {
    CodexAppServerDriver::new(
        EnvironmentRef::new("codex-saved-login").expect("environment is valid"),
    )
}

pub fn plan_for(
    topology: &ExecutionTopologyFixture,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> PreflightPlan {
    app_server_plan_for(
        DriverRole::InteractiveSession,
        topology.execution_host_id().clone(),
        topology.configured_instance_id().clone(),
        topology.instance_target().clone(),
        capabilities,
        [],
    )
}

pub fn request_id(prefix: &str, topology: &ExecutionTopologyFixture) -> RequestId {
    RequestId::new(format!(
        "{prefix}:{}",
        topology.execution_host_id().as_str()
    ))
    .expect("request id is valid")
}

pub fn open_request(prefix: &str, topology: &ExecutionTopologyFixture) -> OpenSessionRequest {
    OpenSessionRequest::new(
        request_id(prefix, topology),
        topology.working_resource().clone(),
        None,
    )
    .with_access_policy(SessionAccessPolicy::read_only())
}

pub fn tool_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::ToolCalls,
        [
            CapabilityConstraint::ToolMaximumCount(2),
            CapabilityConstraint::ToolMaximumSchemaBytes(1024),
            CapabilityConstraint::tool_schema_dialect("json-schema-2020-12")
                .expect("dialect is valid"),
        ],
    )
}

pub fn tool_options() -> SessionOptions {
    let tool = ToolDeclaration::new(
        "task_ledger",
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1024).expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool declaration is valid");
    SessionOptions::default().with_tools([tool])
}
