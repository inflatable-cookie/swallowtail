use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{NegotiatedReasoningSetup, RuntimeFailure};

use super::{KimiConfirmationRejection, KimiReasoningConfirmation};

use crate::failure::{failure, malformed};
use crate::selection::KimiAcpBehavior;

const OPTION_ID: &str = "thinking";
const OPTION_CATEGORY: &str = "thought_level";

pub(super) struct KimiReasoningSelection {
    setup: NegotiatedReasoningSetup,
    provider_value: String,
}

impl KimiReasoningSelection {
    pub(super) fn provider_value(&self) -> &str {
        &self.provider_value
    }

    pub(super) fn confirm(
        self,
        response: &Value,
        behavior: KimiAcpBehavior,
    ) -> Result<KimiReasoningConfirmation, KimiConfirmationRejection> {
        let option = parse_option(response, OptionPhase::Confirmation)
            .map_err(KimiConfirmationRejection::from)?;
        validate_behavior_shape(&option, behavior).map_err(KimiConfirmationRejection::from)?;
        let current = option.current.to_owned();
        let projected = match behavior {
            KimiAcpBehavior::LegacyReasoning => current.as_str(),
            KimiAcpBehavior::DeclaredEffort if self.setup.requested().as_str() == "on" => {
                if current == "off" {
                    return Err(KimiConfirmationRejection {
                        failure: effective_mismatch(),
                        provider_value: Some(current),
                    });
                }
                "on"
            }
            KimiAcpBehavior::DeclaredEffort => current.as_str(),
        };
        let mode = ReasoningMode::new(projected).map_err(|_| KimiConfirmationRejection {
            failure: malformed(),
            provider_value: Some(current.clone()),
        })?;
        let effective = self.setup.confirm(mode).map_err(|failure| KimiConfirmationRejection {
            failure,
            provider_value: Some(current.clone()),
        })?;
        Ok(KimiReasoningConfirmation {
            effective,
            provider_value: current,
        })
    }
}

pub(super) fn prepare_reasoning_selection(
    snapshot: &Value,
    behavior: KimiAcpBehavior,
    setup: NegotiatedReasoningSetup,
) -> Result<KimiReasoningSelection, RuntimeFailure> {
    let option = parse_option(snapshot, OptionPhase::Snapshot)?;
    validate_behavior_shape(&option, behavior)?;
    let requested = setup.requested().as_str().to_owned();
    let provider_value = match behavior {
        KimiAcpBehavior::LegacyReasoning => {
            if !option.values.contains(requested.as_str()) {
                return Err(unsupported_value());
            }
            requested.as_str()
        }
        KimiAcpBehavior::DeclaredEffort if requested == "on" => {
            if option.values.contains("on") || option.values.iter().any(|value| *value != "off") {
                "on"
            } else {
                return Err(unsupported_value());
            }
        }
        KimiAcpBehavior::DeclaredEffort => {
            if !admitted_declared_effort(requested.as_str())
                || !option.values.contains(requested.as_str())
            {
                return Err(unsupported_value());
            }
            requested.as_str()
        }
    };
    Ok(KimiReasoningSelection {
        setup,
        provider_value: provider_value.to_owned(),
    })
}

struct ThinkingOption<'a> {
    current: &'a str,
    values: BTreeSet<&'a str>,
}

#[derive(Clone, Copy)]
enum OptionPhase {
    Snapshot,
    Confirmation,
}

fn parse_option(root: &Value, phase: OptionPhase) -> Result<ThinkingOption<'_>, RuntimeFailure> {
    let options = root
        .get("configOptions")
        .and_then(Value::as_array)
        .ok_or_else(|| missing_option(phase))?;
    let mut matches = options
        .iter()
        .filter(|option| option.get("id").and_then(Value::as_str) == Some(OPTION_ID));
    let option = matches.next().ok_or_else(|| missing_option(phase))?;
    if matches.next().is_some() {
        return Err(failure(
            "swallowtail.kimi.acp.reasoning_option_ambiguous",
            "Kimi Code advertised more than one reasoning option",
        ));
    }
    if option.get("type").and_then(Value::as_str) != Some("select")
        || option.get("category").and_then(Value::as_str) != Some(OPTION_CATEGORY)
    {
        return Err(malformed_option());
    }
    let current = option
        .get("currentValue")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(malformed_option)?;
    let rows = option
        .get("options")
        .and_then(Value::as_array)
        .filter(|rows| !rows.is_empty())
        .ok_or_else(malformed_option)?;
    let mut values = BTreeSet::new();
    for row in rows {
        let value = row
            .get("value")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(malformed_option)?;
        if !values.insert(value) {
            return Err(malformed_option());
        }
    }
    if !values.contains(current) {
        return Err(malformed_option());
    }
    Ok(ThinkingOption { current, values })
}

fn validate_behavior_shape(
    option: &ThinkingOption<'_>,
    behavior: KimiAcpBehavior,
) -> Result<(), RuntimeFailure> {
    match behavior {
        KimiAcpBehavior::LegacyReasoning => {
            if option
                .values
                .iter()
                .any(|value| !matches!(*value, "off" | "on"))
                || !option.values.contains("on")
            {
                return Err(malformed_option());
            }
        }
        KimiAcpBehavior::DeclaredEffort => {
            // Foreign catalogue rows may coexist with the admitted subset.
            // Only admitted identifiers prepare; unknown rows do not make the
            // whole option malformed.
            let has_on = option.values.contains("on");
            let has_concrete = option
                .values
                .iter()
                .any(|value| !matches!(*value, "off" | "on"));
            if (has_on && has_concrete) || (!has_on && !has_concrete) {
                return Err(malformed_option());
            }
        }
    }
    Ok(())
}

fn admitted_declared_effort(value: &str) -> bool {
    matches!(
        value,
        "off" | "on" | "low" | "medium" | "high" | "xhigh" | "max"
    )
}

fn missing_option(phase: OptionPhase) -> RuntimeFailure {
    match phase {
        OptionPhase::Snapshot => failure(
            "swallowtail.kimi.acp.reasoning_option_missing",
            "Kimi Code did not advertise the requested reasoning option",
        ),
        OptionPhase::Confirmation => failure(
            "swallowtail.kimi.acp.reasoning_confirmation_missing",
            "Kimi Code did not confirm the selected reasoning option",
        ),
    }
}

fn malformed_option() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.acp.reasoning_option_malformed",
        "Kimi Code advertised an invalid reasoning option",
    )
}

fn unsupported_value() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.acp.reasoning_value_unsupported",
        "Kimi Code does not advertise the requested reasoning value",
    )
}

fn effective_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.negotiated_reasoning.effective_mismatch",
        "Harness reasoning confirmation does not match the requested mode",
    )
}
