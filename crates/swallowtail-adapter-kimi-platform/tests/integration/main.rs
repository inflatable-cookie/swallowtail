// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../support/mod.rs"]
mod support;

#[path = "../addable_route.rs"]
mod addable_route;

#[path = "../conformance.rs"]
mod conformance;

#[path = "../connection_lifecycle.rs"]
mod connection_lifecycle;

#[path = "../direct_driver.rs"]
mod direct_driver;

#[path = "../prepared_facade.rs"]
mod prepared_facade;
