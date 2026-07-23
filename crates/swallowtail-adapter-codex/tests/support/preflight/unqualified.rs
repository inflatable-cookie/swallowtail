use super::app_server_plan_for_policy;
use swallowtail_core::{
    ConfiguredInstanceId, DriverRole, ExecutionHostId, InstanceTargetRef, PreflightPlan,
    SessionAccessPolicy,
};

pub fn unqualified_app_server_plan(role: DriverRole, version: Option<&str>) -> PreflightPlan {
    app_server_plan_for_policy(
        role,
        ExecutionHostId::new("host.local").expect("host id is valid"),
        ConfiguredInstanceId::new("codex.app-server.local").expect("instance id is valid"),
        InstanceTargetRef::new("codex-app-server-executable").expect("target is valid"),
        version,
        None,
        [],
        [],
        SessionAccessPolicy::read_only(),
        [],
    )
}
