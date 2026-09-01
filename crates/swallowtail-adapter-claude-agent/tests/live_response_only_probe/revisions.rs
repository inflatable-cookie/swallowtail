use super::{PROBE_INSTANCE_REVISION, PROBE_ROUTE_REVISION};
use swallowtail_adapter_claude_agent::CLAUDE_CODE_RESPONSE_ONLY_VERSION;

#[test]
fn live_response_only_revisions_are_not_the_moving_ceiling() {
    assert_ne!(PROBE_INSTANCE_REVISION, CLAUDE_CODE_RESPONSE_ONLY_VERSION);
    assert_ne!(PROBE_ROUTE_REVISION, CLAUDE_CODE_RESPONSE_ONLY_VERSION);
}
