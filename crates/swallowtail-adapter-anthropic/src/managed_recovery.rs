use crate::failure::failure;
use swallowtail_core::{OwnedRemoteResourceKind, PreflightPlan, RunRef};
use swallowtail_runtime::{
    ProviderRecoveredResourceCleanupBinding, ProviderRunCheckpoint, RuntimeFailure, RuntimeRunId,
};

const VERSION: u8 = 1;
const MAXIMUM_ID_BYTES: usize = 1_024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ManagedRecoveryResources {
    pub environment_id: String,
    pub session_id: String,
}

pub(crate) fn records(
    plan: &PreflightPlan,
    runtime_run_id: RuntimeRunId,
    environment_id: &str,
    session_id: &str,
) -> Result<
    (
        ProviderRunCheckpoint,
        ProviderRecoveredResourceCleanupBinding,
    ),
    RuntimeFailure,
> {
    let encoded = encode(environment_id, session_id)?;
    let provider_run_ref = RunRef::new(session_id).map_err(|_| invalid())?;
    let checkpoint = ProviderRunCheckpoint::new(
        plan,
        runtime_run_id.clone(),
        provider_run_ref.clone(),
        &encoded,
    )
    .map_err(|_| invalid())?;
    let cleanup = ProviderRecoveredResourceCleanupBinding::new(
        plan,
        runtime_run_id,
        provider_run_ref,
        [
            OwnedRemoteResourceKind::Environment,
            OwnedRemoteResourceKind::Session,
        ],
        encoded,
    )
    .map_err(|_| invalid())?;
    Ok((checkpoint, cleanup))
}

pub(crate) fn from_checkpoint(
    checkpoint: &ProviderRunCheckpoint,
) -> Result<ManagedRecoveryResources, RuntimeFailure> {
    let resources = decode(checkpoint.cursor())?;
    if checkpoint.provider_run_ref().as_provider_value() != resources.session_id {
        return Err(invalid());
    }
    Ok(resources)
}

pub(crate) fn from_cleanup_binding(
    binding: &ProviderRecoveredResourceCleanupBinding,
) -> Result<ManagedRecoveryResources, RuntimeFailure> {
    let expected = [
        OwnedRemoteResourceKind::Environment,
        OwnedRemoteResourceKind::Session,
    ];
    if binding.resource_kinds().collect::<Vec<_>>() != expected {
        return Err(invalid());
    }
    let resources = decode(binding.provider_resource_binding())?;
    if binding.provider_run_ref().as_provider_value() != resources.session_id {
        return Err(invalid());
    }
    Ok(resources)
}

fn encode(environment_id: &str, session_id: &str) -> Result<Vec<u8>, RuntimeFailure> {
    let mut output = vec![VERSION];
    for value in [environment_id, session_id] {
        if value.trim().is_empty() || value.len() > MAXIMUM_ID_BYTES {
            return Err(invalid());
        }
        let length = u16::try_from(value.len()).map_err(|_| invalid())?;
        output.extend_from_slice(&length.to_be_bytes());
        output.extend_from_slice(value.as_bytes());
    }
    Ok(output)
}

fn decode(input: &[u8]) -> Result<ManagedRecoveryResources, RuntimeFailure> {
    if input.first().copied() != Some(VERSION) {
        return Err(invalid());
    }
    let mut offset = 1;
    let environment_id = read_text(input, &mut offset)?;
    let session_id = read_text(input, &mut offset)?;
    if offset != input.len() {
        return Err(invalid());
    }
    Ok(ManagedRecoveryResources {
        environment_id,
        session_id,
    })
}

fn read_text(input: &[u8], offset: &mut usize) -> Result<String, RuntimeFailure> {
    let length_end = offset.checked_add(2).ok_or_else(invalid)?;
    let length = input.get(*offset..length_end).ok_or_else(invalid)?;
    *offset = length_end;
    let length = usize::from(u16::from_be_bytes([length[0], length[1]]));
    if length == 0 || length > MAXIMUM_ID_BYTES {
        return Err(invalid());
    }
    let end = offset.checked_add(length).ok_or_else(invalid)?;
    let value = std::str::from_utf8(input.get(*offset..end).ok_or_else(invalid)?)
        .map_err(|_| invalid())?
        .to_owned();
    *offset = end;
    Ok(value)
}

fn invalid() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.recovery_binding_invalid",
        "Anthropic Managed Agents recovery binding is invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn private_resource_binding_round_trips_and_rejects_trailing_data() {
        let encoded = encode("environment", "session").expect("binding encodes");
        let decoded = decode(&encoded).expect("binding decodes");
        assert_eq!(decoded.environment_id, "environment");
        assert_eq!(decoded.session_id, "session");

        let mut malformed = encoded;
        malformed.push(0);
        assert!(decode(&malformed).is_err());
    }
}
