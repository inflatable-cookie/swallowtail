// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../claude_code_support/mod.rs"]
#[allow(dead_code, unused_imports)]
mod claude_code_support;
#[path = "../discovery_support.rs"]
mod discovery_support;
#[path = "../support/mod.rs"]
mod support;

#[path = "../consumer_route_projection/acknowledgement.rs"]
mod acknowledgement;
#[path = "../consumer_route_projection/assembly.rs"]
mod assembly;
#[path = "../consumer_route_projection/fixtures.rs"]
mod fixtures;
#[path = "../consumer_route_projection/ledger.rs"]
mod ledger;
#[path = "../consumer_route_projection/mixture.rs"]
mod mixture;
#[path = "../consumer_route_projection/model.rs"]
mod model;
#[path = "../consumer_route_projection/naming.rs"]
mod naming;
#[path = "../consumer_route_projection/posture.rs"]
mod posture;
#[path = "../consumer_route_projection/proof.rs"]
mod proof;

#[path = "../acp_driver.rs"]
mod acp_driver;
#[path = "../activity_corpus.rs"]
mod activity_corpus;
#[path = "../addable_route.rs"]
mod addable_route;
#[path = "../claude_agent_acp_0_100_0_identity.rs"]
mod claude_agent_acp_0_100_0_identity;
#[path = "../claude_agent_acp_0_106_0_inventory.rs"]
mod claude_agent_acp_0_106_0_inventory;
#[path = "../claude_agent_acp_0_109_1_identity.rs"]
mod claude_agent_acp_0_109_1_identity;
#[path = "../claude_agent_acp_0_81_2_delta_ledger.rs"]
mod claude_agent_acp_0_81_2_delta_ledger;
#[path = "../claude_agent_acp_identity.rs"]
mod claude_agent_acp_identity;
#[path = "../claude_agent_sdk_identity.rs"]
mod claude_agent_sdk_identity;
#[path = "../claude_agent_sdk_mcp_status_rows.rs"]
mod claude_agent_sdk_mcp_status_rows;
#[path = "../claude_agent_sdk_native_read_mediation_limitation.rs"]
mod claude_agent_sdk_native_read_mediation_limitation;
#[path = "../claude_code_headless_autocompaction_identity.rs"]
mod claude_code_headless_autocompaction_identity;
#[path = "../claude_code_headless_identity.rs"]
mod claude_code_headless_identity;
#[path = "../claude_code_headless_maximum_turns_identity.rs"]
mod claude_code_headless_maximum_turns_identity;
#[path = "../claude_code_headless_watcher_seam_identity.rs"]
mod claude_code_headless_watcher_seam_identity;
#[path = "../claude_code_response_only.rs"]
mod claude_code_response_only;
#[path = "../claude_code_response_only_identity.rs"]
mod claude_code_response_only_identity;
#[path = "../claude_code_response_only_project_location.rs"]
mod claude_code_response_only_project_location;
#[path = "../claude_code_watcher_admission_identity.rs"]
mod claude_code_watcher_admission_identity;
#[path = "../claude_code_watcher_isolation_identity.rs"]
mod claude_code_watcher_isolation_identity;
#[path = "../compatibility_corpus.rs"]
mod compatibility_corpus;
#[path = "../connection_lifecycle.rs"]
mod connection_lifecycle;
#[path = "../installed_probe.rs"]
mod installed_probe;
#[path = "../lifecycle_portability.rs"]
mod lifecycle_portability;
#[path = "../prepared_facade.rs"]
mod prepared_facade;
#[path = "../structured_run.rs"]
mod structured_run;
#[path = "../usage_corpus.rs"]
mod usage_corpus;

#[cfg(feature = "live-probes")]
#[path = "../live_response_only_probe.rs"]
mod live_response_only_probe;

#[cfg(feature = "live-probes")]
#[path = "../live_watcher_probe.rs"]
mod live_watcher_probe;

#[path = "../http_mcp_live_harness.rs"]
mod http_mcp_live_harness;

#[cfg(feature = "live-probes")]
#[path = "../http_mcp_live_gate.rs"]
mod http_mcp_live_gate;

#[path = "../claude_agent_sdk_native_proof_harness.rs"]
mod claude_agent_sdk_native_proof_harness;

#[path = "../claude_agent_sdk_0_3_322_identity.rs"]
mod claude_agent_sdk_0_3_322_identity;

#[path = "../claude_agent_sdk_node_22_23_3_identity.rs"]
mod claude_agent_sdk_node_22_23_3_identity;

#[path = "../sdk_support/mod.rs"]
mod sdk_support;
