use super::*;
use swallowtail_adapter_codex::CodexSessionManagementInput;
use swallowtail_core::{
    ProviderSessionAffectedScope, ProviderSessionBindingOrigin, ProviderSessionDeletionStrength,
    ProviderSessionEffectTruth,
};
use swallowtail_runtime::{CancellationControl, ProviderSessionManagementBinding};

mod authority;
mod control;
mod failure;
mod mapping;
pub(super) mod support;
mod topology;
