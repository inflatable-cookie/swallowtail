const OPTION_ID: &str = "mode";
const OPTION_CATEGORY: &str = "mode";
const PLAN_VALUE: &str = "plan";
const ACT_VALUE: &str = "act";
const PROVIDER_VALUES: &[&str] = &["plan", "act"];

fn prepare_plan_mode(snapshot: &Value) -> Result<(), RuntimeFailure> {
    let modes_current = parse_modes(snapshot)?;
    let option = parse_mode_option(snapshot, OptionPhase::Snapshot)?;
    if modes_current != option.current {
        return Err(failure(
            "swallowtail.cline.acp.harness_mode_option_ambiguous",
            "Cline ACP advertised contradictory current harness-mode truth",
        ));
    }
    if modes_current != ACT_VALUE {
        return Err(failure(
            "swallowtail.cline.acp.harness_mode_option_malformed",
            "Cline ACP new-session snapshot is not the frozen default Act row",
        ));
    }
    Ok(())
}

fn confirm_plan_mode(response: &Value) -> Result<(), RuntimeFailure> {
    let option = parse_mode_option(response, OptionPhase::Confirmation)?;
    if option.current == PLAN_VALUE {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.cline.acp.harness_mode_mismatch",
            "Cline ACP harness-mode confirmation does not match the requested mode",
        ))
    }
}

fn requested_plan_mode(options: &swallowtail_runtime::SessionOptions) -> bool {
    options.harness_mode() == Some(HarnessMode::Plan)
}

fn validate_harness_mode_plan(
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
            "swallowtail.cline.acp.harness_mode_mismatch",
            "Cline ACP session harness mode does not match its preflight plan",
        ));
    }
    if requested.is_some_and(|mode| mode != HarnessMode::Plan) {
        return Err(unsupported("harness mode"));
    }
    Ok(())
}

struct ModeOption<'a> {
    current: &'a str,
}

#[derive(Clone, Copy)]
enum OptionPhase {
    Snapshot,
    Confirmation,
}

fn parse_modes(root: &Value) -> Result<&str, RuntimeFailure> {
    let modes = root
        .get("modes")
        .ok_or_else(|| missing_mode_option(OptionPhase::Snapshot))?;
    let available = modes
        .get("availableModes")
        .and_then(Value::as_array)
        .filter(|rows| !rows.is_empty())
        .ok_or_else(malformed_mode_option)?;
    let mut values = Vec::new();
    for row in available {
        let value = row
            .get("id")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(malformed_mode_option)?;
        if values.contains(&value) {
            return Err(failure(
                "swallowtail.cline.acp.harness_mode_option_ambiguous",
                "Cline ACP advertised more than one matching harness mode",
            ));
        }
        values.push(value);
    }
    if values.as_slice() != PROVIDER_VALUES {
        return Err(malformed_mode_option());
    }
    let current = modes
        .get("currentModeId")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(malformed_mode_option)?;
    if !values.contains(&current) {
        return Err(malformed_mode_option());
    }
    Ok(current)
}

fn parse_mode_option(root: &Value, phase: OptionPhase) -> Result<ModeOption<'_>, RuntimeFailure> {
    let options = root
        .get("configOptions")
        .and_then(Value::as_array)
        .ok_or_else(|| missing_mode_option(phase))?;
    let mut matches = options
        .iter()
        .filter(|option| option.get("id").and_then(Value::as_str) == Some(OPTION_ID));
    let option = matches.next().ok_or_else(|| missing_mode_option(phase))?;
    if matches.next().is_some() {
        return Err(failure(
            "swallowtail.cline.acp.harness_mode_option_ambiguous",
            "Cline ACP advertised more than one harness-mode option",
        ));
    }
    if option.get("type").and_then(Value::as_str) != Some("select")
        || option.get("category").and_then(Value::as_str) != Some(OPTION_CATEGORY)
    {
        return Err(malformed_mode_option());
    }
    let current = option
        .get("currentValue")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(malformed_mode_option)?;
    let rows = option
        .get("options")
        .and_then(Value::as_array)
        .filter(|rows| !rows.is_empty())
        .ok_or_else(malformed_mode_option)?;
    let mut values = Vec::new();
    for row in rows {
        let value = row
            .get("value")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(malformed_mode_option)?;
        if values.contains(&value) {
            return Err(malformed_mode_option());
        }
        values.push(value);
    }
    if values.as_slice() != PROVIDER_VALUES || !values.contains(&current) {
        return Err(malformed_mode_option());
    }
    Ok(ModeOption { current })
}

fn missing_mode_option(phase: OptionPhase) -> RuntimeFailure {
    match phase {
        OptionPhase::Snapshot => failure(
            "swallowtail.cline.acp.harness_mode_option_missing",
            "Cline ACP did not advertise the requested harness-mode option",
        ),
        OptionPhase::Confirmation => failure(
            "swallowtail.cline.acp.harness_mode_confirmation_missing",
            "Cline ACP did not confirm the selected harness mode",
        ),
    }
}

fn malformed_mode_option() -> RuntimeFailure {
    failure(
        "swallowtail.cline.acp.harness_mode_option_malformed",
        "Cline ACP advertised an invalid harness-mode option",
    )
}
