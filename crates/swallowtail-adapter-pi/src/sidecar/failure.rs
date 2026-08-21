use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    crate::failure::failure(code, message)
}

pub(crate) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.pi.sdk-sidecar.protocol_failed",
        "Pi SDK sidecar stream did not match the qualified protocol",
    )
}

pub(crate) fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.pi.sdk-sidecar.unsupported_input",
        format!("Pi SDK sidecar does not support {feature}"),
    )
}
