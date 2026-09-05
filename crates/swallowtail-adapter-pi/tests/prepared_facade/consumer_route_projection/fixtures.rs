use std::collections::{BTreeMap, BTreeSet};

use swallowtail_adapter_pi::{PiCatalogueProfileInput, PiRunProfileInput, PiSessionProfileInput};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId, Deadline,
    MonotonicInstant, OperationContent, RequestId, SessionOptions, WorkingResourceRef,
};

use super::ledger::*;
use super::naming::{RowIdentity, identities};

fn source(id: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(id).expect("fixture source id is valid")
}

fn contribution(
    contribution: ConsumerRouteProjectionContribution,
    id: &str,
) -> ConsumerRouteProjectionContribution {
    let _ = source(id);
    contribution
}

fn rpc_catalogue() -> ConsumerRouteProjectionContribution {
    let prepared = super::pi_prepared();
    let catalogue = prepared
        .prepare_catalogue(PiCatalogueProfileInput::new(
            RequestId::new("pi-ledger-catalogue").expect("request id"),
        ))
        .expect("Pi catalogue prepares");
    contribution(
        catalogue
            .consumer_route_projection_contribution(source("pi.rpc.catalogue"))
            .expect("Pi catalogue contributes"),
        RPC_CATALOGUE,
    )
}

fn rpc_run(with_attachments: bool) -> ConsumerRouteProjectionContribution {
    let prepared = super::pi_prepared();
    let input = PiRunProfileInput::new(
        RequestId::new(if with_attachments {
            "pi-ledger-run-attachments"
        } else {
            "pi-ledger-run-minimal"
        })
        .expect("request id"),
        crate::model("pi.ledger.run"),
        OperationContent::new("private Pi ledger prompt").expect("content"),
        WorkingResourceRef::new("pi.ledger.workspace").expect("resource"),
        Deadline::at(MonotonicInstant::from_ticks(100_000)),
    );
    let input = if with_attachments {
        input.with_attachments([crate::image("pi.ledger.image")])
    } else {
        input
    };
    let run = prepared.prepare_run(input).expect("Pi run prepares");
    let profile = if with_attachments {
        RPC_RUN_ATTACHMENTS
    } else {
        RPC_RUN_MINIMAL
    };
    contribution(
        run.consumer_route_projection_contribution(source(profile))
            .expect("Pi run contributes"),
        profile,
    )
}

fn rpc_session(with_attachments: bool) -> ConsumerRouteProjectionContribution {
    let prepared = super::pi_prepared();
    let input = PiSessionProfileInput::new(
        RequestId::new(if with_attachments {
            "pi-ledger-session-attachments"
        } else {
            "pi-ledger-session-minimal"
        })
        .expect("request id"),
        crate::model("pi.ledger.session"),
        WorkingResourceRef::new("pi.ledger.workspace").expect("resource"),
        SessionOptions::default(),
    );
    let input = if with_attachments {
        input.with_image_attachments()
    } else {
        input
    };
    let session = prepared
        .prepare_session(input)
        .expect("Pi session prepares");
    let profile = if with_attachments {
        RPC_SESSION_ATTACHMENTS
    } else {
        RPC_SESSION_MINIMAL
    };
    contribution(
        session
            .consumer_route_projection_contribution(source(profile))
            .expect("Pi session contributes"),
        profile,
    )
}

pub(super) fn observed_rpc() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([
        (RPC_CATALOGUE, identities(&rpc_catalogue(), RPC_ROUTE)),
        (RPC_RUN_MINIMAL, identities(&rpc_run(false), RPC_ROUTE)),
        (RPC_RUN_ATTACHMENTS, identities(&rpc_run(true), RPC_ROUTE)),
        (
            RPC_SESSION_MINIMAL,
            identities(&rpc_session(false), RPC_ROUTE),
        ),
        (
            RPC_SESSION_ATTACHMENTS,
            identities(&rpc_session(true), RPC_ROUTE),
        ),
    ])
}

fn sidecar(with_attachments: bool, reasoning: bool) -> ConsumerRouteProjectionContribution {
    let options = if reasoning {
        super::reasoning_options("medium")
    } else {
        SessionOptions::default()
    };
    let prepared = super::sidecar_prepared(with_attachments, options);
    let profile = match (with_attachments, reasoning) {
        (false, false) => SIDECAR_MINIMAL,
        (false, true) => SIDECAR_REASONING,
        (true, false) => SIDECAR_ATTACHMENTS,
        (true, true) => SIDECAR_REASONING_ATTACHMENTS,
    };
    contribution(
        prepared
            .consumer_route_projection_contribution(source(profile))
            .expect("Pi sidecar contributes"),
        profile,
    )
}

pub(super) fn observed_sidecar() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([
        (
            SIDECAR_MINIMAL,
            identities(&sidecar(false, false), SIDECAR_ROUTE),
        ),
        (
            SIDECAR_REASONING,
            identities(&sidecar(false, true), SIDECAR_ROUTE),
        ),
        (
            SIDECAR_ATTACHMENTS,
            identities(&sidecar(true, false), SIDECAR_ROUTE),
        ),
        (
            SIDECAR_REASONING_ATTACHMENTS,
            identities(&sidecar(true, true), SIDECAR_ROUTE),
        ),
    ])
}
