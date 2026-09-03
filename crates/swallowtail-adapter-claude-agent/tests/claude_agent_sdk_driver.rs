//! Provider-free driver proofs for the `claude-agent.sdk` route.
//!
//! Every case runs against a fake sidecar and fake native children. No Node
//! runtime, SDK package, downloaded binary, login, or provider turn is
//! involved anywhere in this binary.

mod sdk_support;

mod claude_agent_sdk_driver {
    pub mod cancellation;
    pub mod framing;
    pub mod guardian;
    pub mod lifecycle;
    pub mod readiness;
    pub mod stalls;
}

use swallowtail_core::ExecutionHostId;

pub(crate) fn host_id(value: &str) -> ExecutionHostId {
    ExecutionHostId::new(value).expect("fixture execution host id is valid")
}
