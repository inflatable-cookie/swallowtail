#![allow(dead_code)]

#[allow(unused_imports)]
mod claude_code_support;
mod support;

#[path = "consumer_route_projection/acknowledgement.rs"]
mod acknowledgement;
#[path = "consumer_route_projection/assembly.rs"]
mod assembly;
#[path = "consumer_route_projection/fixtures.rs"]
mod fixtures;
#[path = "consumer_route_projection/ledger.rs"]
mod ledger;
#[path = "consumer_route_projection/mixture.rs"]
mod mixture;
#[path = "consumer_route_projection/naming.rs"]
mod naming;
#[path = "consumer_route_projection/posture.rs"]
mod posture;
#[path = "consumer_route_projection/proof.rs"]
mod proof;
