use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment,
};

/// Opaque runtime revision qualified for externally attached serving.
pub const LLAMA_CPP_ATTACHED_RUNTIME_REVISION: &str = "b9910-f5525f7e7";
/// Opaque runtime revision qualified for host-owned serving.
pub const LLAMA_CPP_OWNED_RUNTIME_REVISION: &str = "b10069-178a6c449";

const ATTACHED_AXIS: &str = "llama.cpp.attached-runtime";
const OWNED_AXIS: &str = "llama.cpp.owned-runtime";

/// Returns the exact attached-runtime interface binding.
#[must_use]
pub fn llama_cpp_attached_runtime_binding() -> InterfaceVersionBinding {
    binding(ATTACHED_AXIS, LLAMA_CPP_ATTACHED_RUNTIME_REVISION)
}

/// Returns the exact owned-runtime interface binding.
#[must_use]
pub fn llama_cpp_owned_runtime_binding() -> InterfaceVersionBinding {
    binding(OWNED_AXIS, LLAMA_CPP_OWNED_RUNTIME_REVISION)
}

/// Returns the exact qualified attached-runtime compatibility claim.
#[must_use]
pub fn llama_cpp_attached_runtime_claim() -> InterfaceCompatibilityClaim {
    exact_claim(
        "llama-cpp.attached-runtime-window-1",
        ATTACHED_AXIS,
        LLAMA_CPP_ATTACHED_RUNTIME_REVISION,
        "llama-cpp.attached-openai-chat-b9910",
    )
}

/// Returns the exact qualified owned-runtime compatibility claim.
#[must_use]
pub fn llama_cpp_owned_runtime_claim() -> InterfaceCompatibilityClaim {
    exact_claim(
        "llama-cpp.owned-runtime-window-1",
        OWNED_AXIS,
        LLAMA_CPP_OWNED_RUNTIME_REVISION,
        "llama-cpp.owned-openai-chat-b10069",
    )
}

fn binding(axis: &str, version: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        valid(InterfaceVersionAxis::new, axis),
        valid(InterfaceVersion::new, version),
    )
}

fn exact_claim(id: &str, axis: &str, version: &str, behavior: &str) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        valid(InterfaceCompatibilityClaimId::new, id),
        valid(InterfaceVersionAxis::new, axis),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            valid(InterfaceVersion::new, version),
            valid(InterfaceBehaviorRevision::new, behavior),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static llama.cpp interface claim is valid")
}

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static llama.cpp interface identity is valid")
}
