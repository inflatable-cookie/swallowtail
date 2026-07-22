use crate::failure::AlibabaProtocolFailure;
use serde_json::Value;
use swallowtail_core::ProviderRequestRef;

pub fn parse_request_correlation(
    input: &[u8],
) -> Result<ProviderRequestRef, AlibabaProtocolFailure> {
    let value: Value = serde_json::from_slice(input)
        .map_err(|_| AlibabaProtocolFailure::invalid("response headers"))?;
    let request = value
        .get("x-request-id")
        .and_then(Value::as_str)
        .ok_or_else(|| AlibabaProtocolFailure::invalid("request correlation"))?;
    ProviderRequestRef::new(request.to_owned())
        .map_err(|_| AlibabaProtocolFailure::invalid("request correlation"))
}

pub fn parse_provider_failure(
    input: &[u8],
) -> Result<AlibabaProtocolFailure, AlibabaProtocolFailure> {
    if input.len() > 512 * 1024 {
        return Err(AlibabaProtocolFailure::invalid("provider failure"));
    }
    let value: Value = serde_json::from_slice(input)
        .map_err(|_| AlibabaProtocolFailure::invalid("provider failure"))?;
    if value
        .pointer("/error/code")
        .and_then(Value::as_str)
        .is_none()
        || value
            .pointer("/error/message")
            .and_then(Value::as_str)
            .is_none()
    {
        return Err(AlibabaProtocolFailure::invalid("provider failure"));
    }
    Ok(AlibabaProtocolFailure::provider())
}
