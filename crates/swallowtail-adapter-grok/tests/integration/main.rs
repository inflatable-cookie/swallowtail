#[path = "../discovery_suite.rs"]
mod discovery_suite;

#[path = "../acp_suite.rs"]
mod acp_suite;

#[path = "../compatibility_corpus.rs"]
mod compatibility_corpus;

#[path = "../catalogue_suite.rs"]
mod catalogue_suite;

#[path = "../grok_1_0_70_catalogue_identity.rs"]
mod grok_1_0_70_catalogue_identity;

#[path = "../grok_1_0_41_identity.rs"]
mod grok_1_0_41_identity;

#[path = "../grok_1_0_46_identity.rs"]
mod grok_1_0_46_identity;

#[path = "../grok_1_0_70_acp_identity.rs"]
mod grok_1_0_70_acp_identity;

#[cfg(feature = "live-probes")]
#[path = "../grok_acp_client_mcp_probe.rs"]
mod grok_acp_client_mcp_probe;

#[cfg(feature = "live-probes")]
#[path = "../live_installed_probe.rs"]
mod live_installed_probe;

#[cfg(feature = "live-probes")]
#[path = "../live_catalogue_probe.rs"]
mod live_catalogue_probe;
