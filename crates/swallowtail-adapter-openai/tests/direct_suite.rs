#[path = "direct_support/fixture.rs"]
mod fixture;
#[allow(unused_imports)]
use crate::server;
#[allow(unused_imports)]
use crate::services;

#[path = "background_projection.rs"]
mod background_projection;
#[path = "direct_driver.rs"]
mod direct_driver;
#[path = "prepared_facade.rs"]
mod prepared_facade;
#[path = "reconciliation.rs"]
mod reconciliation;
#[path = "retained_execution_corpus.rs"]
mod retained_execution_corpus;
