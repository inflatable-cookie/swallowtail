// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../antigravity_release_identity.rs"]
mod antigravity_release_identity;

#[path = "../foundation.rs"]
mod foundation;

#[path = "../headless_suite.rs"]
mod headless_suite;

#[path = "../continuation_suite.rs"]
mod continuation_suite;

#[path = "../prepared_suite.rs"]
mod prepared_suite;

#[path = "../consumer_route_projection.rs"]
mod consumer_route_projection;
