mod support;

#[path = "corpus/artifact.rs"]
mod artifact;
#[path = "corpus/common.rs"]
mod common;
#[path = "corpus/identity.rs"]
mod identity;
#[path = "corpus/prepared.rs"]
mod prepared;
#[path = "corpus/rejection.rs"]
mod rejection;

include!("consumer_route_projection/mod.rs");
