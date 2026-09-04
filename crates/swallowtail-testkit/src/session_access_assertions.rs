use crate::{SessionAccessFixtureCase, SessionAccessPreflightFixture};
use swallowtail_core::{ExecutionHostId, PreflightDimension};

/// Proves consumer-tool exclusion follows a filesystem boundary claim, not
/// read/write resource access alone.
pub fn assert_consumer_tool_exclusion_keys_on_boundary_claim() {
    for case in [
        SessionAccessFixtureCase::ReadOnlyWithToolCalls,
        SessionAccessFixtureCase::AmbientReadWriteWithToolCalls,
        SessionAccessFixtureCase::AmbientMediatedReadWriteWithToolCalls,
    ] {
        SessionAccessPreflightFixture::for_case(case, fixture_host())
            .preflight()
            .expect("tool calls without a bounded writable boundary must pass");
    }

    let failure = SessionAccessPreflightFixture::for_case(
        SessionAccessFixtureCase::BoundedWorkspaceWithToolCalls,
        fixture_host(),
    )
    .preflight()
    .expect_err("bounded writable sessions with tool calls must fail");

    assert_eq!(failure.dimension(), PreflightDimension::SessionAccess);
    assert_eq!(
        failure.diagnostic().message(),
        "Bounded writable sessions cannot declare consumer tools"
    );
}

fn fixture_host() -> ExecutionHostId {
    ExecutionHostId::new("fixture.host.local").expect("fixture host id is valid")
}
