use crate::output::failure;
use swallowtail_runtime::RuntimeFailure;

pub(super) fn closed_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.closed",
        "Watcher bridge lease is closed",
    )
}

pub(super) fn frozen_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.admission_frozen",
        "Watcher bridge admission is frozen",
    )
}

pub(super) fn foreign_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.foreign_lease",
        "Watcher bridge request did not match the bound lease",
    )
}

pub(super) fn duplicate_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.duplicate_correlation",
        "Watcher bridge request correlation was already used",
    )
}

pub(super) fn busy_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.busy",
        "Watcher bridge rejected work beyond its positive bound",
    )
}

pub(super) fn unauthorized_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.unauthorized",
        "Watcher bridge request was not authenticated",
    )
}

pub(super) fn malformed_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.malformed",
        "Watcher bridge request was malformed",
    )
}

pub(super) fn oversized_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.oversized",
        "Watcher bridge request exceeded its positive bound",
    )
}

pub(super) fn unknown_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.unknown",
        "Watcher bridge request used an unknown protocol surface",
    )
}

pub(super) fn identity_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.identity_rejected",
        "Watcher bridge rejected a required identity",
    )
}

pub(super) fn handshake_failure() -> RuntimeFailure {
    failure(
        "swallowtail.watcher_bridge.handshake_required",
        "Watcher bridge request ran before protocol initialization",
    )
}
