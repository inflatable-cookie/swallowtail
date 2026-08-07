#[test]
fn prepared_session_derives_tool_bounds_from_bounded_declarations() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let larger_schema = serde_json::to_vec(&serde_json::json!({
        "type": "object",
        "description": "x".repeat(8 * 1024),
    }))
    .expect("schema serializes");
    let larger_schema_bytes = u64::try_from(larger_schema.len()).expect("schema length fits");
    let larger_tool = ToolDeclaration::new(
        "large_lookup",
        SchemaDocument::inline(larger_schema, 16 * 1024).expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool declaration is valid");
    let profile = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("bounded-tools").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default().with_tools([tool("lookup"), larger_tool]),
        ))
        .expect("bounded tool declarations prepare");
    let requirement = profile
        .plan()
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == Capability::ToolCalls)
        .expect("tool capability is planned");

    assert!(
        requirement
            .constraints()
            .any(|constraint| constraint == &CapabilityConstraint::ToolMaximumCount(2))
    );
    assert!(requirement.constraints().any(|constraint| {
        constraint == &CapabilityConstraint::ToolMaximumSchemaBytes(larger_schema_bytes)
    }));

    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let handle = block_on(profile.open_session(support::host_services(process)))
        .expect("prepared session opens");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    let thread_start = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/start")
        })
        .expect("thread start is captured");
    assert_eq!(
        thread_start
            .pointer("/params/dynamicTools")
            .and_then(serde_json::Value::as_array)
            .map(Vec::len),
        Some(2)
    );
    assert!(state.waited());
}

