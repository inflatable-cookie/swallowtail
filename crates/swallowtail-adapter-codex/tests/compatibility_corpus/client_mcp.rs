fn is_client_mcp_declaration(name: &str) -> bool {
    let folded = name.replace(['-', '/'], "_").to_ascii_lowercase();
    folded.contains("mcp_server") || folded.contains("mcpserver") || folded.contains("client_mcp")
}

fn refute_client_mcp_names(names: &[&str], label: &str) {
    for name in names {
        assert!(
            !is_client_mcp_declaration(name),
            "{label} {name} is not a client-declared MCP server surface"
        );
    }
}

#[test]
fn frozen_app_server_corpus_has_no_client_mcp_declaration_surface() {
    let evidence = json(APP_SERVER_CLIENT_MCP_RANGE);
    assert_eq!(evidence["axis"], "codex.cli");
    assert_eq!(evidence["route"], "codex.app-server");
    assert_eq!(evidence["decision"], "provider_limitation");
    assert_eq!(evidence["deliver_now_row_count"], 0);
    assert!(evidence["fake_transcript"].is_null());
    assert!(evidence["binding_card"].is_null());
    assert_eq!(evidence["provider_prompt_sent"], false);
    assert_eq!(evidence["live_session"], false);
    assert_eq!(evidence["downloaded_binaries_executed"], false);
    assert_eq!(
        evidence["supported_consumer_tool_path"]["experimental_field"],
        "dynamicTools"
    );
    assert_eq!(
        evidence["withheld"]["provider_direct_mcp_registration"],
        "swallowtail.codex.app_server.registered_mcp_withheld"
    );

    let legacy = json(LEGACY_APP_SERVER_RELEASES);
    refute_client_mcp_names(&strings(&legacy["selected_methods"]), "legacy selected method");
    assert_eq!(
        strings(&legacy["selected_methods"]),
        strings(&evidence["segments"][0]["selected_methods"])
    );

    let current = json(APP_SERVER_RELEASES);
    let mut union = BTreeSet::new();
    for release in current["releases"]
        .as_array()
        .expect("current releases are an array")
    {
        let fields = strings(&release["experimental_thread_fields"]);
        refute_client_mcp_names(&fields, "experimental thread field");
        assert!(
            fields.contains(&"dynamicTools"),
            "Card 117 dynamicTools remains the experimental consumer-tool field"
        );
        union.extend(fields);
    }
    assert_eq!(
        union,
        string_set(&evidence["segments"][1]["experimental_thread_fields_union"])
    );

    for protocol in [
        json(include_str!("../fixtures/codex-cli-0.148.0/protocol.json")),
        json(include_str!("../fixtures/codex-cli-0.149.0/protocol.json")),
        json(include_str!("../fixtures/codex-cli-0.149.1/protocol.json")),
        json(include_str!("../fixtures/codex-cli-0.151.0/protocol.json")),
        json(include_str!("../fixtures/codex-cli-0.152.0/protocol.json")),
        json(include_str!("../fixtures/codex-cli-0.152.1/protocol.json")),
    ] {
        let methods = strings(&protocol["schema"]["methods_present"]);
        refute_client_mcp_names(&methods, "identity protocol method");
        assert_eq!(
            methods,
            strings(&evidence["segments"][2]["methods_present"])
        );
        if let Some(properties) = protocol["schema"].get("thread_resume_properties") {
            refute_client_mcp_names(&strings(properties), "thread/resume property");
        }
    }
}
