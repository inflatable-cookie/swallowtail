// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../goose_acp_identity.rs"]
mod goose_acp_identity;

#[path = "../activity_corpus.rs"]
mod activity_corpus;

#[path = "../acp_suite.rs"]
mod acp_suite;

#[path = "../prepared_facade.rs"]
mod prepared_facade;

#[path = "../consumer_route_projection.rs"]
mod consumer_route_projection;

#[path = "../goose_1_50_0_delta_ledger.rs"]
mod goose_1_50_0_delta_ledger;

#[path = "../goose_acp_currentness_1_53_0.rs"]
mod goose_acp_currentness_1_53_0;

#[path = "../goose_acp_currentness_1_56_0.rs"]
mod goose_acp_currentness_1_56_0;

#[path = "../goose_acp_currentness_1_57_0.rs"]
mod goose_acp_currentness_1_57_0;

#[path = "../goose_acp_currentness_1_59_0.rs"]
mod goose_acp_currentness_1_59_0;
