//! Safe local failures for the registered-tool profile.

use crate::output::failure;
use swallowtail_runtime::RuntimeFailure;

pub(super) fn closed_failure() -> RuntimeFailure {
    failure(
        "swallowtail.registered_tool.closed",
        "Registered tool lease is closed",
    )
}

pub(super) fn foreign_failure() -> RuntimeFailure {
    failure(
        "swallowtail.registered_tool.foreign_correlation",
        "Registered tool request did not match the bound lease",
    )
}

pub(super) fn already_open_failure() -> RuntimeFailure {
    failure(
        "swallowtail.registered_tool.already_open",
        "Registered tool bridge already has an open lease for this turn",
    )
}

pub(super) fn identity_failure() -> RuntimeFailure {
    failure(
        "swallowtail.registered_tool.identity_rejected",
        "Registered tool bridge rejected a required identity",
    )
}

pub(super) fn unsupported_transport_failure() -> RuntimeFailure {
    failure(
        "swallowtail.registered_tool.unsupported_transport",
        "Selected registered-tool transport is not qualified",
    )
}
