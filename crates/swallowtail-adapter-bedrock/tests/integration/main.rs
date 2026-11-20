// The consolidated suites keep per-suite instances of shared helper files,
// exactly as the former one-binary-per-suite layout did, so the same source
// file is still loaded as more than one module.
#![allow(clippy::duplicate_mod)]
// `consumer_route_projection` still wraps its tests in a module of that name,
// which was the crate root before consolidation.
#![allow(clippy::module_inception)]

#[path = "../bedrock_catalogue_1_166_0_identity.rs"]
mod bedrock_catalogue_1_166_0_identity;

#[path = "../bedrock_runtime_1_153_0_identity.rs"]
mod bedrock_runtime_1_153_0_identity;

#[path = "../catalogue_sdk_fixtures.rs"]
mod catalogue_sdk_fixtures;

#[path = "../conformance.rs"]
mod conformance;

#[path = "../consumer_route_projection.rs"]
mod consumer_route_projection;

#[path = "../prepared_facade.rs"]
mod prepared_facade;

#[path = "../runtime_version_qualification.rs"]
mod runtime_version_qualification;

#[path = "../sdk_fixtures.rs"]
mod sdk_fixtures;
