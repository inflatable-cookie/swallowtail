use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment,
};

/// Baseline opaque runtime revision retained in the attached binding.
pub const LLAMA_CPP_ATTACHED_RUNTIME_REVISION: &str = "b9910-f5525f7e7";
/// Opaque runtime revision qualified for host-owned serving.
pub const LLAMA_CPP_OWNED_RUNTIME_REVISION: &str = "b10069-178a6c449";

const ATTACHED_AXIS: &str = "llama.cpp.attached-runtime";
const OWNED_AXIS: &str = "llama.cpp.owned-runtime";
const ATTACHED_V0_6_0_RUNTIME_REVISION: &str = "b11429-d81235049";
const ATTACHED_BEHAVIOR_REVISION: &str = "llama-cpp.attached-openai-chat-b9910";

/// Returns the retained baseline attached-runtime interface binding.
#[must_use]
pub fn llama_cpp_attached_runtime_binding() -> InterfaceVersionBinding {
    binding(ATTACHED_AXIS, LLAMA_CPP_ATTACHED_RUNTIME_REVISION)
}

/// Returns the exact owned-runtime interface binding.
#[must_use]
pub fn llama_cpp_owned_runtime_binding() -> InterfaceVersionBinding {
    binding(OWNED_AXIS, LLAMA_CPP_OWNED_RUNTIME_REVISION)
}

/// Returns the bounded exact-set attached-runtime compatibility claim.
#[must_use]
pub fn llama_cpp_attached_runtime_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        valid(
            InterfaceCompatibilityClaimId::new,
            "llama-cpp.attached-runtime-window-1",
        ),
        valid(InterfaceVersionAxis::new, ATTACHED_AXIS),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [
            exact_member(
                LLAMA_CPP_ATTACHED_RUNTIME_REVISION,
                ATTACHED_BEHAVIOR_REVISION,
            ),
            exact_member(ATTACHED_V0_6_0_RUNTIME_REVISION, ATTACHED_BEHAVIOR_REVISION),
        ],
        [],
    )
    .expect("static llama.cpp attached compatibility claim is valid")
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
        [exact_member(version, behavior)],
        [],
    )
    .expect("static llama.cpp interface claim is valid")
}

fn exact_member(version: &str, behavior: &str) -> InterfaceVersionSegment {
    InterfaceVersionSegment::exact(
        valid(InterfaceVersion::new, version),
        valid(InterfaceBehaviorRevision::new, behavior),
        InterfaceSupportStatus::Maintained,
    )
}

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static llama.cpp interface identity is valid")
}
