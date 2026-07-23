mod support;

use futures_executor::block_on;
use std::collections::BTreeSet;
use std::sync::Arc;
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::{FakeProcessService, app_server_plan, host_services, plan, working_resource};
use swallowtail_adapter_codex::{
    CodexAppServerDriver, CodexExecDriver, codex_app_server_descriptor, codex_exec_descriptor,
};
use swallowtail_core::{Capability, DriverRole};
use swallowtail_runtime::{
    DriverRegistration, EnvironmentRef, InteractiveSessionDriver, ModelCatalogDriver,
    OpenSessionRequest, OperationContent, RequestId, StructuredRunDriver, StructuredRunRequest,
};
use swallowtail_testkit::{
    ConformanceAssertion, ConformanceReport, run_long_lived_rpc_profile,
    run_one_shot_structured_cli_profile,
};

fn environment() -> EnvironmentRef {
    EnvironmentRef::new("codex-saved-login").expect("environment is valid")
}

#[test]
fn registrations_share_a_family_without_flattening_roles_or_transports() {
    let exec_descriptor = codex_exec_descriptor();
    let app_server_descriptor = codex_app_server_descriptor();
    assert_eq!(
        exec_descriptor.integration_family(),
        app_server_descriptor.integration_family()
    );
    assert_ne!(exec_descriptor.identity(), app_server_descriptor.identity());
    assert_ne!(
        exec_descriptor.transport_family(),
        app_server_descriptor.transport_family()
    );

    let exec_driver = Arc::new(CodexExecDriver::new(environment()));
    let exec_role: Arc<dyn StructuredRunDriver> = exec_driver;
    let exec_registration = DriverRegistration::new(exec_descriptor)
        .with_structured_run(exec_role)
        .expect("exec declares structured run");
    assert!(exec_registration.structured_run().is_some());
    assert!(exec_registration.model_catalog().is_none());
    assert!(exec_registration.interactive_session().is_none());

    let app_server_driver = Arc::new(CodexAppServerDriver::new(environment()));
    let catalog_role: Arc<dyn ModelCatalogDriver> = app_server_driver.clone();
    let session_role: Arc<dyn InteractiveSessionDriver> = app_server_driver;
    let app_server_registration = DriverRegistration::new(app_server_descriptor)
        .with_model_catalog(catalog_role)
        .expect("app-server declares model catalog")
        .with_interactive_session(session_role)
        .expect("app-server declares interactive session");
    assert!(app_server_registration.structured_run().is_none());
    assert!(app_server_registration.model_catalog().is_some());
    assert!(app_server_registration.interactive_session().is_some());

    let exec_capabilities = capabilities(&plan());
    let catalog_capabilities = capabilities(&app_server_plan(DriverRole::ModelCatalog));
    let session_capabilities = capabilities(&app_server_plan(DriverRole::InteractiveSession));
    assert_eq!(
        exec_capabilities,
        BTreeSet::from([Capability::StructuredRun])
    );
    assert_eq!(
        catalog_capabilities,
        BTreeSet::from([Capability::ModelCatalog])
    );
    assert_eq!(
        session_capabilities,
        BTreeSet::from([Capability::InteractiveSession])
    );
    assert!(!session_capabilities.contains(&Capability::StructuredOutput));
}

#[test]
fn drivers_reject_each_others_bound_plans_before_process_work() {
    let (exec_process, exec_state) = FakeProcessService::completed("");
    let exec_result = block_on(
        CodexExecDriver::new(environment()).start_run(
            app_server_plan(DriverRole::InteractiveSession),
            StructuredRunRequest::new(
                RequestId::new("cross-plan-exec").expect("request id is valid"),
                OperationContent::new("private prompt").expect("content is valid"),
                support::current_exec_policy(),
            )
            .with_working_resource(working_resource()),
            host_services(exec_process),
        ),
    );
    assert!(exec_result.is_err());
    assert!(!exec_state.started());

    let (app_process, app_state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let app_result = block_on(CodexAppServerDriver::new(environment()).open_session(
        plan(),
        OpenSessionRequest::new(
            RequestId::new("cross-plan-app-server").expect("request id is valid"),
            working_resource(),
            None,
        ),
        host_services(app_process),
    ));
    assert!(app_result.is_err());
    assert!(!app_state.started());
}

#[test]
fn selected_profiles_keep_identical_common_assertions() {
    let one_shot = run_one_shot_structured_cli_profile();
    let long_lived = run_long_lived_rpc_profile();

    assert!(one_shot.covers(ConformanceAssertion::ProcessLifecycle));
    assert!(!one_shot.covers(ConformanceAssertion::SessionLifecycle));
    assert!(long_lived.covers(ConformanceAssertion::SessionLifecycle));
    assert!(!long_lived.covers(ConformanceAssertion::ProcessLifecycle));
    assert_eq!(common_assertions(&one_shot), common_assertions(&long_lived));
}

fn capabilities(plan: &swallowtail_core::PreflightPlan) -> BTreeSet<Capability> {
    plan.requirements()
        .capabilities()
        .map(swallowtail_core::CapabilityRequirement::capability)
        .collect()
}

fn common_assertions(report: &ConformanceReport) -> BTreeSet<ConformanceAssertion> {
    report
        .passed()
        .filter(|assertion| {
            !matches!(
                assertion,
                ConformanceAssertion::ProcessLifecycle
                    | ConformanceAssertion::SessionLifecycle
                    | ConformanceAssertion::CallbackExchange
            )
        })
        .collect()
}
