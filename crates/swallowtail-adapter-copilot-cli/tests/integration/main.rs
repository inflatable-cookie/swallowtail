// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../copilot_cli_acp_identity.rs"]
mod copilot_cli_acp_identity;

#[path = "../copilot_acp_offline_proof.rs"]
mod copilot_acp_offline_proof;

#[path = "../activity_corpus.rs"]
mod activity_corpus;

#[path = "../acp_suite.rs"]
mod acp_suite;

#[path = "../prepared_facade.rs"]
mod prepared_facade;

#[path = "../consumer_route_projection.rs"]
mod consumer_route_projection;
