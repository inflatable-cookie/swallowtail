use crate::realtime_support;

use realtime_support::{RealtimeFixture, RealtimeScenario, TimeMode};

use super::ledger::*;
use super::naming::*;
use super::support::*;

#[test]
fn no_projected_realtime_row_carries_raw_endpoint_or_credential_data() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let session = prepared_session(&fixture, Some("high"));
    let contribution = session
        .consumer_route_projection_contribution(source(PREPARED_SOURCE))
        .expect("prepared contribution is admitted");
    let rendered = format!("{contribution:?}");
    for forbidden in [
        "openai-realtime-fixture-endpoint",
        "openai-realtime-fixture-key",
        "fixture-secret",
        "ws://",
    ] {
        assert!(
            !rendered.contains(forbidden),
            "a projected row must not carry raw target, endpoint, or credential data"
        );
    }
}
