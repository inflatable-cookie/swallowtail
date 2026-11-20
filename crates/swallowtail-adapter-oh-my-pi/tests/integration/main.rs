#[path = "../compatibility_corpus.rs"]
mod compatibility_corpus;

#[path = "../driver.rs"]
mod driver;

#[path = "../input_callback_corpus.rs"]
mod input_callback_corpus;

#[cfg(feature = "live-probes")]
#[path = "../live_installed_probe.rs"]
mod live_installed_probe;

#[path = "../oh_my_pi_18_13_2_identity.rs"]
mod oh_my_pi_18_13_2_identity;

#[path = "../oh_my_pi_package_identity.rs"]
mod oh_my_pi_package_identity;

#[path = "../prepared_facade.rs"]
mod prepared_facade;

#[path = "../structured_run.rs"]
mod structured_run;

#[path = "../usage_corpus.rs"]
mod usage_corpus;
