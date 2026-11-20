// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../corpus.rs"]
mod corpus;

#[path = "../command_code_1_79_1_identity.rs"]
mod command_code_1_79_1_identity;

#[path = "../consumer_route_projection.rs"]
mod consumer_route_projection;

#[cfg(feature = "live-probes")]
#[path = "../live_installed_probe.rs"]
mod live_installed_probe;
