use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.oh_my_pi.rpc.protocol_failed",
        "OhMyPi RPC stream did not match the qualified protocol",
    )
}

pub(crate) fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.oh_my_pi.rpc.unsupported_input",
        format!("OhMyPi RPC does not support {feature}"),
    )
}
