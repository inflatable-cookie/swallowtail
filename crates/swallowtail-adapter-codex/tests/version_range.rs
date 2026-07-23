mod support;

use futures_executor::block_on;
use support::{
    FakeProcessService, current_exec_policy, host_services, unqualified_app_server_plan,
    unqualified_exec_plan, working_resource,
};
use swallowtail_adapter_codex::{
    CODEX_CLI_AXIS, CodexAppServerDriver, CodexExecDriver, codex_app_server_claim,
    codex_app_server_descriptor, codex_cli_binding, codex_exec_claim, codex_exec_descriptor,
};
use swallowtail_core::{DriverRole, InterfaceSupportStatus, InterfaceVersionAxis};
use swallowtail_runtime::{
    EnvironmentRef, ModelCatalogDriver, ModelCatalogRequest, OperationContent, RequestId,
    StructuredRunDriver, StructuredRunRequest,
};

#[test]
fn descriptors_publish_independent_closed_claims_on_one_observed_axis() {
    let axis = InterfaceVersionAxis::new(CODEX_CLI_AXIS).expect("axis is valid");
    let exec = codex_exec_descriptor();
    let app = codex_app_server_descriptor();
    assert_eq!(
        exec.interface_compatibility(&axis),
        Some(&codex_exec_claim())
    );
    assert_eq!(
        app.interface_compatibility(&axis),
        Some(&codex_app_server_claim())
    );
    assert_ne!(codex_exec_claim().id(), codex_app_server_claim().id());
    assert!(exec.supports_interface_version(&codex_cli_binding("0.80.0")));
    assert!(exec.supports_interface_version(&codex_cli_binding("0.121.0")));
    assert!(app.supports_interface_version(&codex_cli_binding("0.121.0")));
    for version in ["0.82.0", "0.83.0", "0.108.0", "0.109.0"] {
        assert!(!exec.supports_interface_version(&codex_cli_binding(version)));
        assert!(!app.supports_interface_version(&codex_cli_binding(version)));
    }
    for claim in [codex_exec_claim(), codex_app_server_claim()] {
        assert_eq!(
            claim
                .classify(codex_cli_binding("0.80.0").version())
                .unwrap()
                .support_status(),
            InterfaceSupportStatus::Deprecated
        );
        assert_eq!(
            claim
                .classify(codex_cli_binding("0.122.0").version())
                .unwrap()
                .support_status(),
            InterfaceSupportStatus::Maintained
        );
    }
}

#[test]
fn exec_rejects_missing_and_unqualified_versions_before_process_work() {
    for (version, expected_code) in [
        (None, "swallowtail.codex.exec.version_missing"),
        (
            Some("0.82.0"),
            "swallowtail.codex.exec.version_incompatible",
        ),
        (
            Some("0.108.0"),
            "swallowtail.codex.exec.version_incompatible",
        ),
        (
            Some("0.146.0-alpha.4"),
            "swallowtail.codex.exec.version_incompatible",
        ),
        (
            Some("0.146.0"),
            "swallowtail.codex.exec.version_incompatible",
        ),
    ] {
        let (process, state) = FakeProcessService::completed("");
        let failure = block_on(exec_driver().start_run(
            unqualified_exec_plan(version),
            run_request("exec-version-rejection"),
            host_services(process),
        ))
        .err()
        .expect("unqualified version fails");
        assert_eq!(failure.diagnostic().code(), expected_code);
        assert!(!state.started());
    }
}

#[test]
fn app_server_rejects_missing_and_unqualified_versions_before_process_work() {
    for (version, expected_code) in [
        (None, "swallowtail.codex.app_server.version_missing"),
        (
            Some("0.82.0"),
            "swallowtail.codex.app_server.version_incompatible",
        ),
        (
            Some("0.108.0"),
            "swallowtail.codex.app_server.version_incompatible",
        ),
        (
            Some("0.146.0-alpha.4"),
            "swallowtail.codex.app_server.version_incompatible",
        ),
        (
            Some("0.146.0"),
            "swallowtail.codex.app_server.version_incompatible",
        ),
    ] {
        let (process, state) = FakeProcessService::completed("");
        let failure = block_on(app_driver().list_models(
            unqualified_app_server_plan(DriverRole::ModelCatalog, version),
            ModelCatalogRequest::new(RequestId::new("app-version-rejection").unwrap()),
            host_services(process),
        ))
        .expect_err("unqualified version fails");
        assert_eq!(failure.diagnostic().code(), expected_code);
        assert!(!state.started());
    }
}

fn exec_driver() -> CodexExecDriver {
    CodexExecDriver::new(EnvironmentRef::new("codex-saved-login").unwrap())
}

fn app_driver() -> CodexAppServerDriver {
    CodexAppServerDriver::new(EnvironmentRef::new("codex-saved-login").unwrap())
}

fn run_request(id: &str) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("private prompt").expect("content is valid"),
        current_exec_policy(),
    )
    .with_working_resource(working_resource())
}
