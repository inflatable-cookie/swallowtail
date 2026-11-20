// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../http_support/mod.rs"]
mod http_support;

#[cfg(feature = "live-probes")]
#[path = "../installed_probe.rs"]
mod installed_probe;

#[path = "../http_mcp_live_harness.rs"]
mod http_mcp_live_harness;

#[cfg(feature = "live-probes")]
#[path = "../http_mcp_live_gate.rs"]
mod http_mcp_live_gate;

#[path = "../runtime_suite.rs"]
mod runtime_suite;

#[path = "../protocol_suite.rs"]
mod protocol_suite;

#[path = "../opencode_http_1_19_5_delta_ledger.rs"]
mod opencode_http_1_19_5_delta_ledger;

#[path = "../acp_suite.rs"]
mod acp_suite;

#[path = "../acp_prepared_facade.rs"]
mod acp_prepared_facade;

#[path = "../opencode_acp_1_19_5_delta_ledger.rs"]
mod opencode_acp_1_19_5_delta_ledger;
