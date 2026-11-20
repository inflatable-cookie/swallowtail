// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../qoder_headless_identity.rs"]
mod qoder_headless_identity;

#[path = "../activity_corpus.rs"]
mod activity_corpus;

#[path = "../headless_suite.rs"]
mod headless_suite;

#[path = "../prepared_facade.rs"]
mod prepared_facade;

#[path = "../consumer_route_projection.rs"]
mod consumer_route_projection;

#[path = "../qoder_1_1_54_reopen.rs"]
mod qoder_1_1_54_reopen;

#[path = "../qoder_1_1_65_delta_ledger.rs"]
mod qoder_1_1_65_delta_ledger;
