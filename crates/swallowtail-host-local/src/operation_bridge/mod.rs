//! One private-operation bridge kernel shared by both closed profiles.
//!
//! Contract 060's watcher profile and Contract 063's registered-tool profile
//! reuse this kernel for operation-private authority material, per-turn lease
//! generation and registry ownership, and bounded joined teardown. Neither
//! profile owns a second lease manager, generation counter, or secret source.

mod join;
mod secret;
mod table;

pub(crate) use join::join_within;
pub(crate) use secret::generate_operation_secret;
pub(crate) use table::LeaseTable;
