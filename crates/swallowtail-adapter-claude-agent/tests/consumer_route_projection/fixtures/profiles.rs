use super::{AGENT_HOST, agent_delete, agent_run, agent_session, code_run, response_run};
use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{CleanupOutcome, ConsumerRouteProjectionContribution};

use crate::ledger::{
    AGENT_DELETE, AGENT_OBSERVED, AGENT_ROUTE, AGENT_RUN, AGENT_SESSION, CODE_ROUTE, CODE_RUN,
    RESPONSE_ROUTE, RESPONSE_RUN,
};
use crate::naming::{RowIdentity, identities, source};

pub(crate) fn profile_contributions() -> BTreeMap<&'static str, ConsumerRouteProjectionContribution>
{
    let observed_fixture = FixtureHost::new(Scenario::Success, "0.61.0");
    let observed = match block_on(
        agent_session(Some("low"), true).open_session_with_projection(
            source("projection.agent.observed.prepared"),
            source("projection.agent.observed.active"),
            observed_fixture.cleanup_request(),
            observed_fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        ),
    ) {
        Ok(outcome) => outcome,
        Err(failure) => panic!(
            "exact acknowledgement failed: {}",
            failure.failure().diagnostic().code()
        ),
    };
    let (session, observed_contribution) = observed.into_parts();
    assert_eq!(
        block_on(session.close(
            observed_fixture.cleanup_request(),
            observed_fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        )),
        CleanupOutcome::Clean
    );

    BTreeMap::from([
        (
            AGENT_RUN,
            agent_run(Some("low"), true, true)
                .consumer_route_projection_contribution(source("projection.agent.run"))
                .expect("agent run contributes"),
        ),
        (
            AGENT_SESSION,
            agent_session(Some("low"), true)
                .consumer_route_projection_contribution(source("projection.agent.session"))
                .expect("agent session contributes"),
        ),
        (
            AGENT_DELETE,
            agent_delete()
                .consumer_route_projection_contribution(source("projection.agent.delete"))
                .expect("agent delete contributes"),
        ),
        (AGENT_OBSERVED, observed_contribution),
        (
            CODE_RUN,
            code_run(Some("low"), Some(3))
                .consumer_route_projection_contribution(source("projection.code.run"))
                .expect("headless run contributes"),
        ),
        (
            RESPONSE_RUN,
            response_run(Some("low"))
                .consumer_route_projection_contribution(source("projection.response.run"))
                .expect("response-only run contributes"),
        ),
    ])
}

pub(crate) fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    profile_contributions()
        .into_iter()
        .map(|(profile, contribution)| {
            let route = match profile {
                AGENT_RUN | AGENT_SESSION | AGENT_DELETE | AGENT_OBSERVED => AGENT_ROUTE,
                CODE_RUN => CODE_ROUTE,
                RESPONSE_RUN => RESPONSE_ROUTE,
                other => panic!("unknown projection profile {other}"),
            };
            (profile, identities(route, &contribution))
        })
        .collect()
}
