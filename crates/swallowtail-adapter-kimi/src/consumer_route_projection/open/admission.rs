use crate::driver::{KimiAcknowledgement, KimiOpenObservation};
use crate::selection::KimiAcpBehavior;
use swallowtail_runtime::{
    ConsumerRouteAcknowledgementState, ConsumerRouteCompoundAcknowledgement,
    ConsumerRouteEnumerableValue, ConsumerRouteStateSupport, RuntimeFailure,
};

const MAXIMUM_KIMI_PROVIDER_VALUE_BYTES: usize = 128;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact bounded provider confirmation retained for Kimi projection.
pub struct KimiProviderValue(String);

impl KimiProviderValue {
    fn new(value: &str) -> Result<Self, AdmissionFailure> {
        if value.trim().is_empty()
            || value.trim() != value
            || value.len() > MAXIMUM_KIMI_PROVIDER_VALUE_BYTES
            || value.chars().any(char::is_control)
        {
            Err(AdmissionFailure::Unbounded)
        } else {
            Ok(Self(value.to_owned()))
        }
    }

    /// Returns the retained provider token.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub(super) fn compound_acknowledgement(
    observation: &KimiOpenObservation,
) -> Result<
    (
        ConsumerRouteCompoundAcknowledgement,
        ConsumerRouteStateSupport,
    ),
    AdmissionFailure,
> {
    let (reasoning, reasoning_state) = state(&observation.reasoning, Some(observation.behavior))?;
    let (plan, plan_state) = state(&observation.plan, None)?;
    let acknowledgement = ConsumerRouteCompoundAcknowledgement::new(reasoning, plan)
        .map_err(|_| AdmissionFailure::Foreign)?;
    let mut support = ConsumerRouteStateSupport::descriptor_only().with_requested();
    if reasoning_state.0 || plan_state.0 {
        support = support.with_provider_effective();
    }
    if reasoning_state.1 || plan_state.1 {
        support = support.with_rejected();
    }
    Ok((acknowledgement, support))
}

fn state(
    state: &KimiAcknowledgement,
    behavior: Option<KimiAcpBehavior>,
) -> Result<(ConsumerRouteAcknowledgementState, (bool, bool)), AdmissionFailure> {
    Ok(match state {
        KimiAcknowledgement::Absent => {
            (ConsumerRouteAcknowledgementState::absent(), (false, false))
        }
        KimiAcknowledgement::RequestedNotDispatched => (
            ConsumerRouteAcknowledgementState::requested_not_dispatched(),
            (false, false),
        ),
        KimiAcknowledgement::Effective(value) => {
            let value = admit(value, behavior)?;
            (
                ConsumerRouteAcknowledgementState::effective(enumerable(&value)?),
                (true, false),
            )
        }
        KimiAcknowledgement::Rejected(value) => {
            let value = admit(value, behavior)?;
            (
                ConsumerRouteAcknowledgementState::rejected(enumerable(&value)?),
                (false, true),
            )
        }
    })
}

fn admit(
    value: &str,
    behavior: Option<KimiAcpBehavior>,
) -> Result<KimiProviderValue, AdmissionFailure> {
    let value = KimiProviderValue::new(value)?;
    let admitted = match behavior {
        Some(KimiAcpBehavior::LegacyReasoning) => matches!(value.as_str(), "off" | "on"),
        Some(KimiAcpBehavior::DeclaredEffort) => matches!(
            value.as_str(),
            "off" | "on" | "low" | "medium" | "high" | "xhigh" | "max"
        ),
        None => matches!(value.as_str(), "default" | "plan" | "auto" | "yolo"),
    };
    admitted.then_some(value).ok_or(AdmissionFailure::Foreign)
}

fn enumerable(value: &KimiProviderValue) -> Result<ConsumerRouteEnumerableValue, AdmissionFailure> {
    ConsumerRouteEnumerableValue::new(value.as_str()).map_err(|_| AdmissionFailure::Unbounded)
}

#[derive(Clone, Copy)]
pub(super) enum AdmissionFailure {
    Foreign,
    Unbounded,
}

impl AdmissionFailure {
    pub(super) fn runtime(self) -> RuntimeFailure {
        match self {
            Self::Foreign => crate::failure::failure(
                "swallowtail.kimi.acp.reasoning_value_foreign",
                "Kimi ACP confirmed a foreign reasoning value",
            ),
            Self::Unbounded => crate::failure::failure(
                "swallowtail.kimi.acp.reasoning_value_unbounded",
                "Kimi ACP confirmed an unretainable reasoning value",
            ),
        }
    }
}
