use serde_json::{Value, json};
use std::collections::BTreeSet;
use swallowtail_protocol_acp::{
    ACP_PROTOCOL_VERSION, AcpSessionUpdate, AcpToolCallStatus, AcpToolKind, decode_session_update,
};

const IDENTITY: &str = include_str!("fixtures/acp-schema-v1.24.1/identity.json");
const INVENTORY: &str = include_str!("fixtures/acp-schema-v1.24.1/artifact-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/acp-schema-v1.24.1/protocol.json");

#[test]
fn stable_identity_freezes_compatible_segment_and_preserves_old_artifacts() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture is valid JSON");

    assert_eq!(identity["axis"], "acp.schema");
    assert_eq!(
        identity["source"]["repository"],
        "https://github.com/agentclientprotocol/agent-client-protocol"
    );
    assert_eq!(
        identity["source"]["runtime_identity"],
        "data-only schema artifacts; no runtime"
    );
    assert!(identity["source"]["host_observation"].is_null());
    assert_eq!(
        identity["claim_at_observation"]["historical_artifact_pins"],
        json!(["schema-v1.19.0", "schema-v1.19.1", "schema-v1.20.0"])
    );
    assert_eq!(identity["official_latest"]["tag"], "schema-v1.24.1");
    assert_eq!(
        identity["official_latest"]["source_commit"],
        "1761180eeddf0828d4ecc367106a632c61be06d9"
    );
    assert_eq!(
        identity["official_latest"]["tag_signature"]["verified"],
        true
    );
    assert_eq!(
        identity["published_hops_after_ceiling"],
        json!([
            "schema-v1.21.0",
            "schema-v1.22.0",
            "schema-v1.23.0",
            "schema-v1.24.0",
            "schema-v1.24.1"
        ])
    );
    assert_eq!(identity["unpublished_next_stable"], "schema-v1.24.2");
    assert_eq!(identity["qualification"]["shape"], "compatible-extension");
    assert_eq!(
        identity["qualification"]["segment"]["from_existing_ceiling"],
        "schema-v1.20.0"
    );
    assert_eq!(
        identity["qualification"]["segment"]["through"],
        "schema-v1.24.1"
    );
    assert_eq!(
        identity["qualification"]["wire_protocol_version"],
        ACP_PROTOCOL_VERSION
    );
    assert_eq!(
        identity["qualification"]["provider_route_claims_preserved"],
        true
    );
    assert_eq!(
        identity["qualification"]["historical_artifact_pins_preserved"],
        true
    );
}

#[test]
fn release_asset_inventory_freezes_every_published_hop() {
    let inventory: Value =
        serde_json::from_str(INVENTORY).expect("inventory fixture is valid JSON");
    let releases = inventory["release_assets"]
        .as_object()
        .expect("release assets are an object");
    assert_eq!(
        inventory["compared"],
        json!([
            "schema-v1.20.0",
            "schema-v1.21.0",
            "schema-v1.22.0",
            "schema-v1.23.0",
            "schema-v1.24.0",
            "schema-v1.24.1"
        ])
    );
    assert_eq!(inventory["asset_count_per_release"], 4);

    let expected = [
        (
            "schema-v1.20.0",
            "5e89c71497fe07dd4ae633c181a17224f4a8956d",
            [
                (
                    "meta.json",
                    1059,
                    "e0bf36f8123b2544b499174197fdc371ec49a1b4572a35114513d56492741599",
                ),
                (
                    "meta.unstable.json",
                    1852,
                    "3026898232badf413624010d1343e20bef853e6705c62d6b56387cf9de6b0543",
                ),
                (
                    "schema.json",
                    198609,
                    "92c1dfcda10dd47e99127500a3763da2b471f9ac61e12b9bf0430c32cf953796",
                ),
                (
                    "schema.unstable.json",
                    364909,
                    "f71fbcb7beeae82770e9c33d1e5969999868789cacec509289331a1205816838",
                ),
            ],
        ),
        (
            "schema-v1.21.0",
            "272bf799f35a258c6a4107a0410ed361e83683d3",
            [
                (
                    "meta.json",
                    1159,
                    "061edb6efa8fb2aa2792459a86ec7268de5fe665bba48b2ffe7939df01481f88",
                ),
                (
                    "meta.unstable.json",
                    1852,
                    "3026898232badf413624010d1343e20bef853e6705c62d6b56387cf9de6b0543",
                ),
                (
                    "schema.json",
                    246569,
                    "caf62ff962ada396878372ced11efb2c6764e59d90919a38583c319948931a42",
                ),
                (
                    "schema.unstable.json",
                    370113,
                    "7f77702b34e0a0558e77220e9007bf8ee161a976bb8ac5021aba1b7e7b2c5708",
                ),
            ],
        ),
        (
            "schema-v1.22.0",
            "9b4c23b966fa3e64b21b6b2718d8dd71fcb64882",
            [
                (
                    "meta.json",
                    1159,
                    "061edb6efa8fb2aa2792459a86ec7268de5fe665bba48b2ffe7939df01481f88",
                ),
                (
                    "meta.unstable.json",
                    1852,
                    "3026898232badf413624010d1343e20bef853e6705c62d6b56387cf9de6b0543",
                ),
                (
                    "schema.json",
                    247168,
                    "3c17bd6385d90cf672d8a661fddc359d73422cf8b8ce6865213d25cfd4c0eca7",
                ),
                (
                    "schema.unstable.json",
                    372939,
                    "b74a7e07a962d3834029b14e371e088aca52dc72ae50fc3669c8e0b760e36bac",
                ),
            ],
        ),
        (
            "schema-v1.23.0",
            "6d08f412a7a1370d3cc9a124e3be3d6acf92641e",
            [
                (
                    "meta.json",
                    1159,
                    "061edb6efa8fb2aa2792459a86ec7268de5fe665bba48b2ffe7939df01481f88",
                ),
                (
                    "meta.unstable.json",
                    1852,
                    "3026898232badf413624010d1343e20bef853e6705c62d6b56387cf9de6b0543",
                ),
                (
                    "schema.json",
                    247168,
                    "3c17bd6385d90cf672d8a661fddc359d73422cf8b8ce6865213d25cfd4c0eca7",
                ),
                (
                    "schema.unstable.json",
                    373903,
                    "2a920d3c0f76443e07ffa7801443e3cdf008e2a3095e565581a0433fd728ce41",
                ),
            ],
        ),
        (
            "schema-v1.24.0",
            "cb50abaa3ed455e2113a9d3d4323e17c5a9785cf",
            [
                (
                    "meta.json",
                    1159,
                    "061edb6efa8fb2aa2792459a86ec7268de5fe665bba48b2ffe7939df01481f88",
                ),
                (
                    "meta.unstable.json",
                    1778,
                    "2c85c87aeedbd10bdac671b2f9274666e2833d38b8835decb741ca19a9d7fc5f",
                ),
                (
                    "schema.json",
                    247168,
                    "3c17bd6385d90cf672d8a661fddc359d73422cf8b8ce6865213d25cfd4c0eca7",
                ),
                (
                    "schema.unstable.json",
                    390753,
                    "f1a445dc9d91938dbecb657a950b3c4c1148624586f93b4c816c0113fad563ed",
                ),
            ],
        ),
        (
            "schema-v1.24.1",
            "1761180eeddf0828d4ecc367106a632c61be06d9",
            [
                (
                    "meta.json",
                    1159,
                    "061edb6efa8fb2aa2792459a86ec7268de5fe665bba48b2ffe7939df01481f88",
                ),
                (
                    "meta.unstable.json",
                    1778,
                    "2c85c87aeedbd10bdac671b2f9274666e2833d38b8835decb741ca19a9d7fc5f",
                ),
                (
                    "schema.json",
                    247168,
                    "3c17bd6385d90cf672d8a661fddc359d73422cf8b8ce6865213d25cfd4c0eca7",
                ),
                (
                    "schema.unstable.json",
                    390855,
                    "6449a87a3b3c42aa0abd30033fc9bd3236cd785078ad084ce3675766be09109e",
                ),
            ],
        ),
    ];

    let expected_tags = expected
        .iter()
        .map(|(tag, _, _)| (*tag).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        releases.keys().cloned().collect::<BTreeSet<_>>(),
        expected_tags
    );
    for (tag, commit, assets) in expected {
        let release = &releases[tag];
        assert_eq!(release["source_commit"], commit);
        let files = release["files"]
            .as_object()
            .expect("release files are an object");
        let expected_files = assets
            .iter()
            .map(|(name, _, _)| (*name).to_owned())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            files.keys().cloned().collect::<BTreeSet<_>>(),
            expected_files
        );
        for (name, size, digest) in assets {
            assert_eq!(files[name]["size_bytes"], size);
            assert_eq!(files[name]["sha256"], digest);
        }
    }

    assert_eq!(
        inventory["hops"],
        json!([
            {"from":"schema-v1.20.0","to":"schema-v1.21.0","added":[],"removed":[],"changed":["meta.json","schema.json","schema.unstable.json"],"identical":["meta.unstable.json"]},
            {"from":"schema-v1.21.0","to":"schema-v1.22.0","added":[],"removed":[],"changed":["schema.json","schema.unstable.json"],"identical":["meta.json","meta.unstable.json"]},
            {"from":"schema-v1.22.0","to":"schema-v1.23.0","added":[],"removed":[],"changed":["schema.unstable.json"],"identical":["meta.json","meta.unstable.json","schema.json"]},
            {"from":"schema-v1.23.0","to":"schema-v1.24.0","added":[],"removed":[],"changed":["meta.unstable.json","schema.unstable.json"],"identical":["meta.json","schema.json"]},
            {"from":"schema-v1.24.0","to":"schema-v1.24.1","added":[],"removed":[],"changed":["schema.unstable.json"],"identical":["meta.json","meta.unstable.json","schema.json"]}
        ])
    );
    assert_eq!(
        inventory["file_classifications"],
        json!({
            "schema.json": "Selected stable ACP v1 schema; classify the exact definitions in protocol.json.",
            "meta.json": "Selected stable ACP v1 method-name metadata; stable method additions are classified in protocol.json.",
            "schema.unstable.json": "Unselected unstable schema artifact; no stable-axis inference is made from its changes.",
            "meta.unstable.json": "Unselected unstable method metadata; no stable-axis inference is made from its changes."
        })
    );
}

#[test]
fn selected_schema_deltas_and_unmapped_additions_are_exact() {
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture is valid JSON");
    assert_eq!(protocol["wire_protocol_version"], ACP_PROTOCOL_VERSION);
    let definition_hashes = &protocol["selected_definition_sha256"];
    let all_versions = [
        "schema-v1.20.0",
        "schema-v1.21.0",
        "schema-v1.22.0",
        "schema-v1.23.0",
        "schema-v1.24.0",
        "schema-v1.24.1",
    ];
    let later_versions = [
        "schema-v1.21.0",
        "schema-v1.22.0",
        "schema-v1.23.0",
        "schema-v1.24.0",
        "schema-v1.24.1",
    ];
    let unchanged_definitions = [
        (
            "ProtocolVersion",
            "7281d37a5654e372e490b26a92b3e06ba8be15ebeabb7f9fab6cb4df2c589b61",
        ),
        (
            "RequestPermissionRequest",
            "8d28e54c28364666fbb9a6db2877f0c51f1b54745192ddcd8737ab640afb6336",
        ),
        (
            "RequestPermissionResponse",
            "bd893d42a9ab8d11c255e51e3cd8f2b3d2c60658cecedd09e365d3936dfc3761",
        ),
        (
            "NewSessionRequest",
            "4de7b5fbf49ec24e9d3dc31502ec680dc8bb598fa742cd86c8b53396f42ae89b",
        ),
        (
            "CloseSessionRequest",
            "6886ed7e9efc4259c24b4582c4b0d696b992e35c617849512e4b99766a434f17",
        ),
        (
            "DeleteSessionRequest",
            "83c13033c90361612f135e2ced4a1699dec076dbfeb60590abf34c9d88739805",
        ),
        (
            "UsageUpdate",
            "b1489421f07f88d3b10bdca14a162639d89ede379c851ba30378814063e09b86",
        ),
    ];
    for (definition, digest) in unchanged_definitions {
        let version_hashes = definition_hashes[definition]
            .as_object()
            .expect("definition hashes are an object");
        assert_eq!(
            version_hashes.keys().cloned().collect::<BTreeSet<_>>(),
            all_versions
                .iter()
                .map(|version| (*version).to_owned())
                .collect()
        );
        for version in all_versions {
            assert_eq!(version_hashes[version], digest);
        }
    }
    let configuration = &protocol["selected_configuration_definition_sha256"];
    assert_eq!(
        configuration["versions"],
        json!([
            "schema-v1.20.0",
            "schema-v1.21.0",
            "schema-v1.22.0",
            "schema-v1.23.0",
            "schema-v1.24.0",
            "schema-v1.24.1"
        ])
    );
    let config_definitions = configuration["definitions"]
        .as_object()
        .expect("configuration definition hashes are an object");
    let expected_configuration = [
        (
            "NewSessionResponse",
            "873f144438af821b59d07936c496ecfbdf9846e0a49a8d26b7b2a8afc6094f50",
        ),
        (
            "SessionConfigOptionsCapabilities",
            "1270bd76d71ef9a7f54630b5207fe7f7d426414b58bdabb4ebdea911b50f1f24",
        ),
        (
            "BooleanConfigOptionCapabilities",
            "6b960b7f5cb02388f0ac844015e7ef22bed47a37d907b1b8df2423cbce600e99",
        ),
        (
            "SessionConfigOption",
            "4ceeac6f47f9644f960ecf4ffc5948dd63b7f89fa7d422ad1c45acba3e21bca1",
        ),
        (
            "SessionConfigSelect",
            "8d2736d67b86bd81fb94b71966c4cfc9eb0a4f3c58a96214327ca87627f62fd7",
        ),
        (
            "SessionConfigBoolean",
            "25c2db89c82f3a8e953e50803a77fec71e289bd9c7483342a7dd3b206db2abe8",
        ),
        (
            "SessionConfigSelectOption",
            "974516119c9c5bc76124a9a70153413c4d22f48213ca69c656b99c5419480c3a",
        ),
        (
            "SessionConfigSelectGroup",
            "4470fc2cf0803bd1c9fb7e47a1fe7b7917a4a66c6a9a0c04477fd84e5611ab46",
        ),
        (
            "SessionConfigSelectOptions",
            "15d7eac7b072d6bc8ecc05de0d4cbe4fd57666753c39fb065a4e3107a388bfcb",
        ),
        (
            "SessionConfigId",
            "5347c9b1fd2a3ba5510f64b5238c8be63dd014b594ca17a7119efcfcb4b95214",
        ),
        (
            "SessionConfigValueId",
            "6e59e44dd7b219b1327207f7dccce0042f49e5eea0f499a1b76af8a7be1fea99",
        ),
        (
            "SessionConfigOptionCategory",
            "7ffd6cc01a1718d9c381a738f0b1986d0efb159280fb80668752f5dadda2da40",
        ),
        (
            "SessionConfigGroupId",
            "b96973eb7bd4c7d50da5696b85093e554a8599d2ea5cd876e50b955ee37657b1",
        ),
        (
            "SetSessionConfigOptionRequest",
            "6dc9121aafade9f245b3c26094673b9f972ddb1e885cfec1c5c88503b9a59390",
        ),
        (
            "ConfigOptionUpdate",
            "b52532c7a360f3dfa9dd944d08a6eb38aad11c4d027152c7e371a2ab90d29200",
        ),
    ];
    assert_eq!(
        config_definitions.keys().cloned().collect::<BTreeSet<_>>(),
        expected_configuration
            .iter()
            .map(|(name, _)| (*name).to_owned())
            .collect()
    );
    for (definition, digest) in expected_configuration {
        assert_eq!(config_definitions[definition], digest);
    }
    assert_eq!(
        protocol["selected_surfaces"]["configuration"]["definitions"],
        json!([
            "NewSessionResponse",
            "SessionConfigOptionsCapabilities",
            "BooleanConfigOptionCapabilities",
            "SessionConfigOption",
            "SessionConfigSelect",
            "SessionConfigBoolean",
            "SessionConfigSelectOption",
            "SessionConfigSelectGroup",
            "SessionConfigSelectOptions",
            "SessionConfigId",
            "SessionConfigValueId",
            "SessionConfigOptionCategory",
            "SessionConfigGroupId",
            "SetSessionConfigOptionRequest",
            "ConfigOptionUpdate"
        ])
    );
    assert_eq!(
        protocol["selected_surfaces"]["configuration"]["selected_shape_changed"],
        false
    );
    let capability_hashes = &definition_hashes["ClientCapabilities"];
    assert_eq!(
        capability_hashes["schema-v1.20.0"],
        "6ea96fededacfd11c503138c1c013faf57c11fb061acf3d265cc28bc881e8e50"
    );
    for version in later_versions {
        assert_eq!(
            capability_hashes[version],
            "e6431319715016c5cbfe7d3fbf4376f09e6fc11c71f592dd3fb409ceafef942c"
        );
    }
    let initialize_hashes = &definition_hashes["InitializeRequest"];
    assert_eq!(
        initialize_hashes["schema-v1.20.0"],
        "fbf498f5062fe0cee0193b489e386c1e64f8d5f785e3c5bd9d0f75e206d16b51"
    );
    for version in later_versions {
        assert_eq!(
            initialize_hashes[version],
            "4f0fb7df2275303f1aa9e88bdd1b959e48054b8c78a851f4061502cbe6e35982"
        );
    }
    for (definition, digest) in [
        (
            "CreateElicitationRequest",
            "35455e98e0780f1ff251c40bad2778208b2bd191e410dc5dab4b37a96c224742",
        ),
        (
            "ElicitationFormMode",
            "327bb8f2c15e770379a0cb61d5b63c56186ab31fc1cdb4138baf1bb4bba1cbba",
        ),
    ] {
        let version_hashes = definition_hashes[definition]
            .as_object()
            .expect("form definition hashes are an object");
        assert_eq!(
            version_hashes.keys().cloned().collect::<BTreeSet<_>>(),
            later_versions
                .iter()
                .map(|version| (*version).to_owned())
                .collect()
        );
        for version in later_versions {
            assert_eq!(version_hashes[version], digest);
        }
    }
    for (version, digest) in [
        (
            "schema-v1.20.0",
            "76859d2d28d49d0427920d867758b88b56f48316b00b4733423e834c4c84e807",
        ),
        (
            "schema-v1.21.0",
            "76859d2d28d49d0427920d867758b88b56f48316b00b4733423e834c4c84e807",
        ),
        (
            "schema-v1.22.0",
            "f8dc46888da9e4fa1edfc9cd88f93bc074a839ecc6d08ddb6d2d2ac12fe568fa",
        ),
        (
            "schema-v1.23.0",
            "f8dc46888da9e4fa1edfc9cd88f93bc074a839ecc6d08ddb6d2d2ac12fe568fa",
        ),
        (
            "schema-v1.24.0",
            "f8dc46888da9e4fa1edfc9cd88f93bc074a839ecc6d08ddb6d2d2ac12fe568fa",
        ),
        (
            "schema-v1.24.1",
            "f8dc46888da9e4fa1edfc9cd88f93bc074a839ecc6d08ddb6d2d2ac12fe568fa",
        ),
    ] {
        assert_eq!(definition_hashes["ToolCall"][version], digest);
    }
    for (version, digest) in [
        (
            "schema-v1.20.0",
            "e05a7f01e93b0e10f5b28b7cb3e7fe3cdd452a2d13790ff44c13ad85ef7daee2",
        ),
        (
            "schema-v1.21.0",
            "e05a7f01e93b0e10f5b28b7cb3e7fe3cdd452a2d13790ff44c13ad85ef7daee2",
        ),
        (
            "schema-v1.22.0",
            "89464d21aa887ee0a13ffe3df39ae23f1598eb6a9a15a0a2af14b46f7e4b4968",
        ),
        (
            "schema-v1.23.0",
            "89464d21aa887ee0a13ffe3df39ae23f1598eb6a9a15a0a2af14b46f7e4b4968",
        ),
        (
            "schema-v1.24.0",
            "89464d21aa887ee0a13ffe3df39ae23f1598eb6a9a15a0a2af14b46f7e4b4968",
        ),
        (
            "schema-v1.24.1",
            "89464d21aa887ee0a13ffe3df39ae23f1598eb6a9a15a0a2af14b46f7e4b4968",
        ),
    ] {
        assert_eq!(definition_hashes["ToolCallUpdate"][version], digest);
    }

    let first_hop = &protocol["stable_hop_deltas"][0];
    assert_eq!(
        protocol["stable_hop_deltas"]
            .as_array()
            .expect("hop deltas are an array")
            .len(),
        5
    );
    assert_eq!(first_hop["from"], "schema-v1.20.0");
    assert_eq!(first_hop["to"], "schema-v1.21.0");
    assert_eq!(
        first_hop["added_definitions"],
        json!([
            "AuthCapabilities",
            "AuthMethodTerminal",
            "BooleanPropertySchema",
            "CompleteElicitationNotification",
            "CreateElicitationRequest",
            "CreateElicitationResponse",
            "ElicitationAcceptAction",
            "ElicitationCapabilities",
            "ElicitationContentValue",
            "ElicitationFormCapabilities",
            "ElicitationFormMode",
            "ElicitationId",
            "ElicitationPropertySchema",
            "ElicitationRequestScope",
            "ElicitationSchema",
            "ElicitationSchemaType",
            "ElicitationSessionScope",
            "ElicitationUrlCapabilities",
            "ElicitationUrlMode",
            "EnumOption",
            "IntegerPropertySchema",
            "MultiSelectItems",
            "MultiSelectPropertySchema",
            "NumberPropertySchema",
            "StringFormat",
            "StringMultiSelectItems",
            "StringPropertySchema",
            "TitledMultiSelectItems"
        ])
    );
    assert_eq!(
        first_hop["changed_definitions"],
        json!([
            "AgentNotification",
            "AgentRequest",
            "AuthMethod",
            "AuthMethodAgent",
            "ClientCapabilities",
            "ClientRequest",
            "ClientResponse",
            "InitializeRequest"
        ])
    );
    assert_eq!(first_hop["removed_definitions"], json!([]));
    assert_eq!(
        first_hop["added_client_methods"],
        json!(["elicitation_complete", "elicitation_create"])
    );
    assert_eq!(
        first_hop["selected_additions"],
        json!([
            "ClientCapabilities.elicitation.form",
            "elicitation/create form mode"
        ])
    );
    assert_eq!(
        first_hop["unmapped_additions"],
        json!([
            "ClientCapabilities.auth",
            "elicitation/complete",
            "elicitation/create URL mode",
            "form field types and option details outside the existing choice-and-Other subset"
        ])
    );

    let second_hop = &protocol["stable_hop_deltas"][1];
    assert_eq!(second_hop["from"], "schema-v1.21.0");
    assert_eq!(second_hop["to"], "schema-v1.22.0");
    assert_eq!(second_hop["added_definitions"], json!([]));
    assert_eq!(second_hop["removed_definitions"], json!([]));
    assert_eq!(
        second_hop["changed_definitions"],
        json!(["ToolCall", "ToolCallUpdate"])
    );
    assert_eq!(second_hop["added_client_methods"], json!([]));
    assert_eq!(second_hop["selected_additions"], json!([]));
    assert_eq!(
        second_hop["unmapped_additions"],
        json!(["ToolCall.name", "ToolCallUpdate.name"])
    );
    for (hop, (from, to)) in protocol["stable_hop_deltas"]
        .as_array()
        .expect("stable hop ledger is an array")
        .iter()
        .skip(2)
        .zip([
            ("schema-v1.22.0", "schema-v1.23.0"),
            ("schema-v1.23.0", "schema-v1.24.0"),
            ("schema-v1.24.0", "schema-v1.24.1"),
        ])
    {
        assert_eq!(hop["from"], from);
        assert_eq!(hop["to"], to);
        assert_eq!(hop["added_definitions"], json!([]));
        assert_eq!(hop["removed_definitions"], json!([]));
        assert_eq!(hop["changed_definitions"], json!([]));
        assert_eq!(hop["added_client_methods"], json!([]));
        assert_eq!(hop["selected_additions"], json!([]));
        assert_eq!(hop["unmapped_additions"], json!([]));
    }

    assert_eq!(
        protocol["selected_surfaces"]["initialize"]["wire_version_type_and_value"],
        "integer 1"
    );
    assert_eq!(
        protocol["selected_surfaces"]["initialize"]["consumer"],
        "claude-agent.acp"
    );
    assert_eq!(
        protocol["selected_surfaces"]["elicitation"]["selected_method"],
        "elicitation/create"
    );
    assert_eq!(
        protocol["selected_surfaces"]["elicitation"]["selected_mode"],
        "form"
    );
    assert_eq!(
        protocol["selected_surfaces"]["lifecycle"]["selected_shape_changed"],
        false
    );
    assert_eq!(
        protocol["selected_surfaces"]["activity"]["selected_shape_changed"],
        false
    );
    assert_eq!(
        protocol["selected_surfaces"]["permission"]["selected_shape_changed"],
        false
    );
    assert_eq!(
        protocol["selected_surfaces"]["usage"]["selected_shape_changed"],
        false
    );
    assert_eq!(
        protocol["selected_surfaces"]["tool"]["selected_name_field"],
        false
    );
    assert_eq!(
        protocol["selected_surfaces"]["tool"]["selected_shape_changed"],
        false
    );
    assert_eq!(
        protocol["provider_and_unstable_boundary"]["schema.unstable.json"],
        "Excluded; it is a separate unstable artifact and does not supply stable v1 proof."
    );
}

#[test]
fn v1_22_tool_names_stay_outside_the_typed_activity_projection() {
    let create = decode_session_update(&json!({
        "sessionId": "schema-tool-name-fixture",
        "update": {
            "sessionUpdate": "tool_call",
            "toolCallId": "tool-1",
            "title": "Read source",
            "name": "private_tool_name_create",
            "kind": "read",
            "status": "in_progress"
        }
    }))
    .expect("tool creation with the v1.22 name extension decodes");
    let AcpSessionUpdate::ToolCall(create) = create.update else {
        panic!("tool creation remains a typed tool call");
    };
    assert_eq!(create.tool_call_id.as_str(), "tool-1");
    assert_eq!(create.title.as_str(), "Read source");
    assert_eq!(create.kind, AcpToolKind::Read);
    assert_eq!(create.status, AcpToolCallStatus::InProgress);
    assert!(!format!("{create:?}").contains("private_tool_name_create"));

    let update = decode_session_update(&json!({
        "sessionId": "schema-tool-name-fixture",
        "update": {
            "sessionUpdate": "tool_call_update",
            "toolCallId": "tool-1",
            "name": "private_tool_name_update"
        }
    }))
    .expect("tool refinement with the v1.22 name extension decodes");
    let AcpSessionUpdate::ToolCallUpdate(update) = update.update else {
        panic!("tool refinement remains a typed tool update");
    };
    assert_eq!(update.tool_call_id.as_str(), "tool-1");
    assert!(update.title.is_none());
    assert!(update.kind.is_none());
    assert!(update.status.is_none());
    assert!(!format!("{update:?}").contains("private_tool_name_update"));
}
