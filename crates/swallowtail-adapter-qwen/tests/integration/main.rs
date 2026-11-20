// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../compatibility_corpus.rs"]
mod compatibility_corpus;

#[path = "../conformance.rs"]
mod conformance;

#[path = "../current_qwen_headless.rs"]
mod current_qwen_headless;

#[path = "../driver.rs"]
mod driver;

#[path = "../interactive_session_corpus.rs"]
mod interactive_session_corpus;

#[cfg(feature = "live-probes")]
#[path = "../live_installed_probe.rs"]
mod live_installed_probe;

#[path = "../prepared_facade.rs"]
mod prepared_facade;

#[path = "../protocol_fixtures.rs"]
mod protocol_fixtures;

#[path = "../qwen_code_0_27_2_identity.rs"]
mod qwen_code_0_27_2_identity;

#[path = "../qwen_headless_identity.rs"]
mod qwen_headless_identity;
