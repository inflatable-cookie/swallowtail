//! Contract 061 disposition proof for the exact eight `qoder.headless` rows.

#![allow(dead_code)]

#[path = "support/discovery.rs"]
mod discovery_support;
#[path = "support/headless.rs"]
mod headless_support;

#[path = "consumer_route_projection/claims.rs"]
mod claims;
#[path = "consumer_route_projection/fixtures.rs"]
mod fixtures;
#[path = "consumer_route_projection/ledger.rs"]
mod ledger;
#[path = "consumer_route_projection/naming.rs"]
mod naming;
#[path = "consumer_route_projection/proof.rs"]
mod proof;
