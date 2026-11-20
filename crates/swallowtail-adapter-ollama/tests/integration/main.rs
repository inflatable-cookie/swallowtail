// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]

#[path = "../prepared_facade/fixtures.rs"]
mod prepared_fixtures;
#[path = "../support/mod.rs"]
mod support;

#[path = "../addable_route.rs"]
mod addable_route;

#[path = "../attached_driver.rs"]
mod attached_driver;

#[path = "../conformance.rs"]
mod conformance;

#[path = "../connection_lifecycle.rs"]
mod connection_lifecycle;

#[path = "../consumer_route_projection.rs"]
mod consumer_route_projection;

#[path = "../context_window_corpus.rs"]
mod context_window_corpus;

#[path = "../generation_control_corpus.rs"]
mod generation_control_corpus;

#[cfg(feature = "live-probes")]
#[path = "../installed_probe.rs"]
mod installed_probe;

#[path = "../interactive_session_corpus.rs"]
mod interactive_session_corpus;

#[path = "../ollama_0_34_4_identity.rs"]
mod ollama_0_34_4_identity;

#[path = "../ollama_0_40_12_identity.rs"]
mod ollama_0_40_12_identity;

#[path = "../ollama_0_40_1_identity.rs"]
mod ollama_0_40_1_identity;

#[path = "../ollama_llama_cpp_pin_identity.rs"]
mod ollama_llama_cpp_pin_identity;

#[path = "../ollama_runtime_identity.rs"]
mod ollama_runtime_identity;

#[path = "../prepared_facade.rs"]
mod prepared_facade;
