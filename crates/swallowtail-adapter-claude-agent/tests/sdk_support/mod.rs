// Every integration binary compiles this module tree but uses only a subset;
// unused-fixture lints fire per binary, not per module.
#[allow(dead_code)]
mod cancel;
#[allow(dead_code)]
mod capture;
#[allow(dead_code)]
mod host;
#[allow(dead_code)]
mod selection;

#[allow(unused_imports)]
pub use cancel::{drop_within, poll_once};
#[allow(unused_imports)]
pub use capture::{
    SDK_RESULT_FIELD_NAMES, SanitizedCaptureJournal, SanitizedHarnessRecord, SanitizedWireCapture,
    captured_services, captured_services_with_journal, record_open_failure, record_success,
};
#[allow(unused_imports)]
pub use host::{CleanupEvent, SdkFixtureHost, SdkScenario, Stall};
#[allow(unused_imports)]
pub use selection::{
    cleanup_request, expired_cleanup_request, preparation, prepared_session, prepared_session_for,
    prepared_session_with, turn_request,
};
