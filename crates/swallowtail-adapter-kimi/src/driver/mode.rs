use serde_json::Value;
use swallowtail_core::{Capability, CapabilityConstraint, HarnessMode, PreflightPlan};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::{failure, unsupported};

const OPTION_ID: &str = "mode";
const OPTION_CATEGORY: &str = "mode";
const PLAN_VALUE: &str = "plan";
const PROVIDER_VALUES: &[&str] = &["default", "plan", "auto", "yolo"];

pub(super) fn prepare_plan_mode(snapshot: &Value) -> Result<(), RuntimeFailure> {
    parse_option(snapshot, OptionPhase::Snapshot).map(|_| ())
}

pub(super) fn confirm_plan_mode(response: &Value) -> Result<(), RuntimeFailure> {
    let option = parse_option(response, OptionPhase::Confirmation)?;
    if option.current == PLAN_VALUE {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.kimi.acp.harness_mode_mismatch",
            "Kimi Code harness-mode confirmation does not match the requested mode",
        ))
    }
}

pub(super) fn requested_plan_mode(options: &swallowtail_runtime::SessionOptions) -> bool {
    options.harness_mode() == Some(HarnessMode::Plan)
}

pub(super) fn validate_harness_mode_plan(
    plan: &PreflightPlan,
    options: &swallowtail_runtime::SessionOptions,
) -> Result<(), RuntimeFailure> {
    let requested = options.harness_mode();
    let planned = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == Capability::HarnessModeSelection)
        .and_then(|requirement| {
            requirement
                .constraints()
                .find_map(|constraint| match constraint {
                    CapabilityConstraint::HarnessMode(mode) => Some(*mode),
                    _ => None,
                })
        });
    if requested != planned {
        return Err(failure(
            "swallowtail.kimi.acp.harness_mode_mismatch",
            "Kimi ACP session harness mode does not match its preflight plan",
        ));
    }
    if requested.is_some_and(|mode| mode != HarnessMode::Plan) {
        return Err(unsupported("harness mode"));
    }
    Ok(())
}

pub(super) fn reject_attachment_harness_mode(
    options: &swallowtail_runtime::SessionOptions,
) -> Result<(), RuntimeFailure> {
    if options.harness_mode().is_some() {
        Err(failure(
            "swallowtail.kimi.acp.attachment_harness_mode_unsupported",
            "Kimi ACP load and resume cannot redeclare harness-mode selection",
        ))
    } else {
        Ok(())
    }
}

struct ModeOption<'a> {
    current: &'a str,
}

#[derive(Clone, Copy)]
enum OptionPhase {
    Snapshot,
    Confirmation,
}

fn parse_option(root: &Value, phase: OptionPhase) -> Result<ModeOption<'_>, RuntimeFailure> {
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
            "swallowtail.kimi.acp.harness_mode_option_ambiguous",
            "Kimi Code advertised more than one harness-mode option",
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
    let mut values = Vec::new();
    for row in rows {
        let value = row
            .get("value")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(malformed_option)?;
        if values.contains(&value) {
            return Err(malformed_option());
        }
        values.push(value);
    }
    if values.as_slice() != PROVIDER_VALUES || !values.contains(&current) {
        return Err(malformed_option());
    }
    Ok(ModeOption { current })
}

fn missing_option(phase: OptionPhase) -> RuntimeFailure {
    match phase {
        OptionPhase::Snapshot => failure(
            "swallowtail.kimi.acp.harness_mode_option_missing",
            "Kimi Code did not advertise the requested harness-mode option",
        ),
        OptionPhase::Confirmation => failure(
            "swallowtail.kimi.acp.harness_mode_confirmation_missing",
            "Kimi Code did not confirm the selected harness mode",
        ),
    }
}

fn malformed_option() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.acp.harness_mode_option_malformed",
        "Kimi Code advertised an invalid harness-mode option",
    )
}
