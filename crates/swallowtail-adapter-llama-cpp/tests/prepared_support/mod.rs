#![allow(dead_code)]

#[path = "../support/fixture.rs"]
mod fixture;
#[path = "../support/owned_fixture.rs"]
mod owned_fixture;
#[path = "../support/owned_services.rs"]
mod owned_services;
#[path = "../support/server.rs"]
mod server;
#[path = "../support/services.rs"]
mod services;

pub use fixture::Fixture;
pub use owned_fixture::OwnedFixture;
pub use owned_services::{OwnedCall, ProcessStop, ScriptedOwnedServices};
pub use server::{FixtureServer, PropertiesFixture, StreamFixture};
