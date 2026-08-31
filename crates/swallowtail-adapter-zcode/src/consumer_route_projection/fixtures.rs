use crate::ZcodeAppServerMode;
use crate::prepared::projection_fixture;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_core::OperationShape;
use swallowtail_runtime::ConsumerRouteProjectionContribution;

use super::ledger::{BUILD, PLAN, ZCODE_PROFILES};
use super::naming::{RowIdentity, identities, source};

pub(super) fn contribution(
    run: &crate::ZcodePreparedRun,
    source_id: &str,
) -> ConsumerRouteProjectionContribution {
    run.consumer_route_projection_contribution(source(source_id))
        .expect("prepared ZCode app-server run contributes")
}

/// Returns the exact prepared run for one named ledger profile.
pub(super) fn profile(name: &str) -> crate::ZcodePreparedRun {
    match name {
        PLAN => projection_fixture::run(ZcodeAppServerMode::plan()),
        BUILD => projection_fixture::run(ZcodeAppServerMode::build()),
        other => panic!("unknown prepared ZCode profile {other}"),
    }
}

/// Collects the exact census identities each prepared profile emits.
pub(super) fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    ZCODE_PROFILES
        .into_iter()
        .map(|name| {
            (
                name,
                identities(&contribution(&profile(name), "zcode.app-server.ledger")),
            )
        })
        .collect()
}

/// Returns the exact prepared operation shape each profile binds its rows to.
pub(super) fn prepared_operation_shapes() -> BTreeMap<&'static str, OperationShape> {
    ZCODE_PROFILES
        .into_iter()
        .map(|name| {
            (
                name,
                operation_shape_of(&contribution(&profile(name), "zcode.app-server.shape")),
            )
        })
        .collect()
}

/// Returns the operation shape every row of one contribution is bound to.
fn operation_shape_of(contribution: &ConsumerRouteProjectionContribution) -> OperationShape {
    for row in contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
    {
        assert_eq!(
            row.applicability(),
            contribution.applicability(),
            "{:?} is not bound to the contribution's exact applicability",
            row.identity()
        );
    }
    contribution.applicability().operation_shape()
}
