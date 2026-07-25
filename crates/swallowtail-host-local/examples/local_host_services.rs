use std::path::PathBuf;
use swallowtail_core::{ExecutionHostId, InterfaceVersionAxis};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::ExecutableRef;

fn main() {
    let executable = ExecutableRef::new("consumer.codex").expect("executable reference is valid");
    let axis = InterfaceVersionAxis::new("codex.cli").expect("version axis is valid");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable(executable, axis, PathBuf::from("/approved/codex"));
    let local = builder.build_services(
        ExecutionHostId::new("consumer.host.local").expect("execution host id is valid"),
    );

    let services = local.services();
    assert_eq!(services.execution_host_id().as_str(), "consumer.host.local");
    assert!(services.task().is_some());
    assert!(services.process().is_some());

    // Discovery receives this one explicit opaque target. It does not search PATH.
    let _selected_target = target;
}
