// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../foundation.rs"]
mod foundation;

#[path = "../acp_suite.rs"]
mod acp_suite;

#[path = "../headless_suite.rs"]
mod headless_suite;

#[path = "../headless_ask_suite.rs"]
mod headless_ask_suite;

#[path = "../prepared_suite.rs"]
mod prepared_suite;

#[path = "../compatibility_corpus.rs"]
mod compatibility_corpus;

#[path = "../consumer_route_projection.rs"]
mod consumer_route_projection;

#[path = "../cursor_agent_release_identity.rs"]
mod cursor_agent_release_identity;

#[path = "../cursor_agent_2026_10_22_identity.rs"]
mod cursor_agent_2026_10_22_identity;

#[path = "../cursor_agent_2026_10_01_acp_identity.rs"]
mod cursor_agent_2026_10_01_acp_identity;

#[path = "../cursor_agent_headless_2026_10_01_identity.rs"]
mod cursor_agent_headless_2026_10_01_identity;

#[path = "../cursor_agent_acp_headless_2026_10_22_identity.rs"]
mod cursor_agent_acp_headless_2026_10_22_identity;
