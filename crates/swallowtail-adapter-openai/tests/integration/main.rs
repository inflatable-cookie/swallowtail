// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../realtime_reasoning_prepare.rs"]
mod realtime_reasoning_prepare;
#[path = "../realtime_support/mod.rs"]
mod realtime_support;
#[path = "../support/server.rs"]
mod server;
#[path = "../support/services.rs"]
mod services;
#[path = "../support/mod.rs"]
mod support;

#[path = "../catalogue_suite.rs"]
mod catalogue_suite;

#[path = "../direct_suite.rs"]
mod direct_suite;

#[path = "../realtime_suite.rs"]
mod realtime_suite;

#[path = "../direct_support/fixture.rs"]
mod fixture;
