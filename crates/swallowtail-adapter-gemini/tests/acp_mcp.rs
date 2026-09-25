use crate::support;

use futures_executor::block_on;
use support::{FixtureHost, Scenario, close_session, selection};
use swallowtail_adapter_gemini::{
    GEMINI_ACP_MCP_SERVER_NAME, GeminiAcpDriver, GeminiAcpHttpMcpPlacement,
};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    InteractiveSessionDriver, OpenSessionRequest, RequestId, SessionPlanAgreement,
};

const CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-035-redaction-canary";
const CANARY_HEADER: &str = "Bearer g06-035-redaction-canary";

#[test]
fn omission_keeps_session_new_mcp_servers_byte_identical() {
    let host_id = ExecutionHostId::new("fixture.host.mcp-omission").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let driver = GeminiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("gemini.fixture.isolated")
            .expect("valid environment"),
        selected.credential.clone(),
    );
    let session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("gemini-mcp-omission").expect("valid request"),
            selected.resource,
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(
                    swallowtail_core::ResourceAccess::Read,
                ),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services.clone(),
    ))
    .expect("session opens");
    let session_new = host
        .writes()
        .into_iter()
        .find(|message| message["method"] == "session/new")
        .expect("session/new is sent");
    assert_eq!(
        serde_json::to_string(&session_new["params"]).expect("params serialize"),
        r#"{"cwd":"/private/fixture","mcpServers":[]}"#
    );
    assert_eq!(
        block_on(close_session(session, services)),
        swallowtail_runtime::CleanupOutcome::Clean
    );
}

#[test]
fn http_mcp_entry_reaches_session_new_verbatim() {
    let host_id = ExecutionHostId::new("fixture.host.mcp-http").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let admitted = GeminiAcpHttpMcpPlacement::new(
        GEMINI_ACP_MCP_SERVER_NAME,
        CANARY_URL,
        vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
    );
    let driver = GeminiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("gemini.fixture.isolated")
            .expect("valid environment"),
        selected.credential.clone(),
    )
    .with_http_mcp_placement(admitted)
    .expect("route-owned http placement is admitted");
    let session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("gemini-mcp-http").expect("valid request"),
            selected.resource,
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(
                    swallowtail_core::ResourceAccess::Read,
                ),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services.clone(),
    ))
    .expect("session opens");
    let session_new = host
        .writes()
        .into_iter()
        .find(|message| message["method"] == "session/new")
        .expect("session/new is sent");
    assert_eq!(session_new["params"]["mcpServers"][0]["type"], "http");
    assert_eq!(
        session_new["params"]["mcpServers"][0]["name"],
        GEMINI_ACP_MCP_SERVER_NAME
    );
    assert_eq!(session_new["params"]["mcpServers"][0]["url"], CANARY_URL);
    assert_eq!(
        session_new["params"]["mcpServers"][0]["headers"][0]["name"],
        "Authorization"
    );
    assert_eq!(
        session_new["params"]["mcpServers"][0]["headers"][0]["value"],
        CANARY_HEADER
    );
    assert_eq!(
        block_on(close_session(session, services)),
        swallowtail_runtime::CleanupOutcome::Clean
    );
}

#[test]
fn unauthenticated_gate_fails_typed_and_never_drops_the_entry() {
    let host_id = ExecutionHostId::new("fixture.host.mcp-auth").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::AuthRequired);
    let services = host.services(host_id);
    let admitted = GeminiAcpHttpMcpPlacement::new(
        GEMINI_ACP_MCP_SERVER_NAME,
        CANARY_URL,
        vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
    );
    let driver = GeminiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("gemini.fixture.isolated")
            .expect("valid environment"),
        selected.credential.clone(),
    )
    .with_http_mcp_placement(admitted)
    .expect("route-owned http placement is admitted");
    let error = match block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("gemini-mcp-auth").expect("valid request"),
            selected.resource,
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(
                    swallowtail_core::ResourceAccess::Read,
                ),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services,
    )) {
        Err(error) => error,
        Ok(_) => panic!("the authRequired gate must fail the open"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.gemini.acp.auth_required"
    );
    assert!(
        !format!("{error:?}").contains("g06-035-redaction-canary"),
        "typed failure leaked a consumer secret"
    );
    let writes = host.writes();
    assert!(
        writes.iter().any(|message| {
            message["method"] == "session/new"
                && message["params"]["mcpServers"][0]["type"] == "http"
                && message["params"]["mcpServers"][0]["url"] == CANARY_URL
        }),
        "the refused open must still carry the entry, not drop it"
    );
    assert_eq!(host.releases(), 1);
}

#[test]
fn foreign_names_and_invalid_structures_refuse_before_spawn() {
    let host_id = ExecutionHostId::new("fixture.host.mcp-refusal").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let collision = GeminiAcpHttpMcpPlacement::new(
        "other-server",
        CANARY_URL,
        vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
    );
    let error = match GeminiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("gemini.fixture.isolated")
            .expect("valid environment"),
        selected.credential.clone(),
    )
    .with_http_mcp_placement(collision)
    {
        Err(error) => error,
        Ok(_) => panic!("foreign MCP names collide"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.gemini.acp.mcp_name_collision"
    );
    assert!(
        !error
            .diagnostic()
            .message()
            .contains("g06-035-redaction-canary")
    );

    let invalid_url = GeminiAcpHttpMcpPlacement::new(
        GEMINI_ACP_MCP_SERVER_NAME,
        "not-a-url",
        Vec::<(String, String)>::new(),
    );
    let error = match GeminiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("gemini.fixture.isolated")
            .expect("valid environment"),
        selected.credential.clone(),
    )
    .with_http_mcp_placement(invalid_url)
    {
        Err(error) => error,
        Ok(_) => panic!("relative URLs are refused"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.gemini.acp.mcp_http_invalid"
    );
    assert!(host.writes().is_empty());
}
