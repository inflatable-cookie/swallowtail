#[allow(dead_code)]
#[path = "installed_discovery/support.rs"]
mod discovery_support;
#[path = "local_server_interactive/fixture.rs"]
mod fixture;
#[allow(dead_code)]
#[path = "prepared_facade/fixtures.rs"]
mod fixtures;
#[path = "local_server_interactive_support/mod.rs"]
mod interactive_support;
#[path = "local_server_lifecycle_support/mod.rs"]
mod lifecycle_support;
#[path = "support/mod.rs"]
mod support;

#[path = "local_server_binding_import.rs"]
mod local_server_binding_import;
#[path = "local_server_corpus.rs"]
mod local_server_corpus;
#[path = "local_server_interactive.rs"]
mod local_server_interactive;
#[path = "local_server_lifecycle.rs"]
mod local_server_lifecycle;
#[path = "local_server_structured_run.rs"]
mod local_server_structured_run;
