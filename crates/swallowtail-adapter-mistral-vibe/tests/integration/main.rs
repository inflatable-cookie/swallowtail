// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../mistral_vibe_headless_identity.rs"]
mod mistral_vibe_headless_identity;

#[path = "../mistral_vibe_2_26_0_delta_ledger.rs"]
mod mistral_vibe_2_26_0_delta_ledger;

#[path = "../mistral_vibe_3_0_2_identity_stop.rs"]
mod mistral_vibe_3_0_2_identity_stop;

#[path = "../activity_corpus.rs"]
mod activity_corpus;

#[path = "../headless_suite.rs"]
mod headless_suite;

#[path = "../prepared_facade.rs"]
mod prepared_facade;
