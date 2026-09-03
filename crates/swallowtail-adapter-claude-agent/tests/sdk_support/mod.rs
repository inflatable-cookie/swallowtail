// Every integration binary compiles this module tree but uses only a subset;
// unused-fixture lints fire per binary, not per module.
#[allow(dead_code)]
mod host;
#[allow(dead_code)]
mod selection;

#[allow(unused_imports)]
pub use host::{CleanupEvent, SdkFixtureHost, SdkScenario};
#[allow(unused_imports)]
pub use selection::{
    cleanup_request, expired_cleanup_request, prepared_session, prepared_session_with, turn_request,
};
