#[test]
fn unsupported_inputs_fail_before_process_start() {
    let topology = ExecutionTopologyFixture::local();
    let (process, state) = FakeProcessService::completed("");
    let (services, _) = host_services_for(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let request = request_for("gemini-tools-rejected", topology.working_resource().clone())
        .with_tools([swallowtail_runtime::ToolDeclaration::new(
            "fixture-tool",
            swallowtail_runtime::SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1_024)
                .expect("schema is valid"),
            "application/schema+json",
            "json-schema-2020-12",
        )
        .expect("tool is valid")]);
    let result = block_on(driver().start_run(plan_for(&topology), request, services));

    assert!(result.is_err());
    assert!(!state.started());
}
