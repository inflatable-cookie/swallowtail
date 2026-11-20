#[path = "../support/discovery.rs"]
mod discovery_support;
#[path = "../headless_support/mod.rs"]
mod headless_support;
#[path = "../live_support/mod.rs"]
mod live_support;
#[path = "../support/mod.rs"]
mod support;

#[cfg(feature = "live-probes")]
#[path = "../live_installed_probe.rs"]
mod live_installed_probe;

#[path = "../http_mcp_live_harness.rs"]
mod http_mcp_live_harness;

#[cfg(feature = "live-probes")]
#[path = "../http_mcp_live_gate.rs"]
mod http_mcp_live_gate;

#[path = "../acp_suite.rs"]
mod acp_suite;

#[path = "../headless_suite.rs"]
mod headless_suite;

#[path = "../live_suite.rs"]
mod live_suite;

#[path = "../consumer_route_projection.rs"]
mod consumer_route_projection;

#[path = "../gemini_cli_0_63_0_delta_ledger.rs"]
mod gemini_cli_0_63_0_delta_ledger;

#[path = "../gemini_cli_0_63_0_headless_authority_stop.rs"]
mod gemini_cli_0_63_0_headless_authority_stop;
