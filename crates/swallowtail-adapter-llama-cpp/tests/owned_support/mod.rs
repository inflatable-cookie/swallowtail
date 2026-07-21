#![allow(dead_code)]

#[path = "../support/owned_fixture.rs"]
mod owned_fixture;
#[path = "../support/owned_services.rs"]
mod owned_services;
#[path = "../support/server.rs"]
mod server;
#[path = "../support/services.rs"]
mod services;

pub use owned_fixture::OwnedFixture;
pub use owned_services::{OwnedCall, ProcessStop, ScriptedOwnedServices};
pub use server::{FixtureServer, PropertiesFixture, StreamFixture};

pub fn assert_order(actual: &[OwnedCall], expected: &[OwnedCall]) {
    let positions = expected
        .iter()
        .map(|expected| {
            actual
                .iter()
                .position(|call| call == expected)
                .expect("expected call exists")
        })
        .collect::<Vec<_>>();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
}
