// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../support/mod.rs"]
mod support;

#[path = "../activity_suite.rs"]
mod activity_suite;

#[path = "../app_server_suite.rs"]
mod app_server_suite;

#[path = "../connection_lifecycle_suite.rs"]
mod connection_lifecycle_suite;

#[path = "../exec_suite.rs"]
mod exec_suite;

#[path = "../codex_app_server_currentness_qualification.rs"]
mod codex_app_server_currentness_qualification;

#[path = "../codex_app_server_currentness_review.rs"]
mod codex_app_server_currentness_review;

#[path = "../codex_0_165_0_identity.rs"]
mod codex_0_165_0_identity;

#[path = "../codex_exec_0_171_0_identity.rs"]
mod codex_exec_0_171_0_identity;

#[path = "../codex_app_server_0_171_0_identity.rs"]
mod codex_app_server_0_171_0_identity;

#[path = "../exec_currentness_suite.rs"]
mod exec_currentness_suite;

#[path = "../legacy_corpus_support/mod.rs"]
mod legacy_corpus_support;
