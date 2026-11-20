// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../installed_discovery/support.rs"]
mod discovery_support;
#[path = "../local_server_interactive/fixture.rs"]
mod fixture;
#[path = "../prepared_facade/fixtures.rs"]
mod fixtures;
#[path = "../local_server_interactive_support/mod.rs"]
mod interactive_support;
#[path = "../local_server_lifecycle_support/mod.rs"]
mod lifecycle_support;
#[path = "../provider_session_import.rs"]
mod provider_session_import;

#[path = "../support/mod.rs"]
mod support;

#[cfg(feature = "live-probes")]
#[path = "../live_installed_probe.rs"]
mod live_installed_probe;

#[path = "../acp_suite.rs"]
mod acp_suite;

#[path = "../kimi_code_2_1_1_acp_identity.rs"]
mod kimi_code_2_1_1_acp_identity;

#[path = "../headless_suite.rs"]
mod headless_suite;

#[path = "../local_suite.rs"]
mod local_suite;

#[path = "../identity_suite.rs"]
mod identity_suite;
