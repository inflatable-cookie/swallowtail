use super::fields::{
    error, object, optional_text, required_array, required_identifier, required_str, required_text,
    required_u64,
};
use crate::activity::{
    AcpBoundedText, AcpCommand, AcpCost, AcpOptionalUpdate, AcpPlanEntry, AcpPlanEntryPriority,
    AcpPlanEntryStatus, AcpSessionUpdate, AcpUsage, ActivityDecodeError, ActivityDecodeErrorKind,
    ActivityDecodeLimits,
};
use serde_json::{Map, Value};

pub(super) fn plan(
    update: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<Vec<AcpPlanEntry>, ActivityDecodeError> {
    required_array(
        update,
        "entries",
        ActivityDecodeErrorKind::PlanEntriesInvalid,
        limits,
    )?
    .iter()
    .map(|entry| {
        let entry = object(entry, ActivityDecodeErrorKind::PlanEntriesInvalid)?;
        let priority = match required_str(
            entry,
            "priority",
            ActivityDecodeErrorKind::PlanEntriesInvalid,
        )? {
            "high" => AcpPlanEntryPriority::High,
            "medium" => AcpPlanEntryPriority::Medium,
            "low" => AcpPlanEntryPriority::Low,
            _ => return Err(error(ActivityDecodeErrorKind::PlanEntriesInvalid)),
        };
        let status =
            match required_str(entry, "status", ActivityDecodeErrorKind::PlanEntriesInvalid)? {
                "pending" => AcpPlanEntryStatus::Pending,
                "in_progress" => AcpPlanEntryStatus::InProgress,
                "completed" => AcpPlanEntryStatus::Completed,
                _ => return Err(error(ActivityDecodeErrorKind::PlanEntriesInvalid)),
            };
        Ok(AcpPlanEntry {
            content: required_text(
                entry,
                "content",
                ActivityDecodeErrorKind::PlanEntriesInvalid,
            )?,
            priority,
            status,
        })
    })
    .collect()
}

pub(super) fn commands(
    update: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<Vec<AcpCommand>, ActivityDecodeError> {
    required_array(
        update,
        "availableCommands",
        ActivityDecodeErrorKind::MetadataInvalid,
        limits,
    )?
    .iter()
    .map(|command| {
        let command = object(command, ActivityDecodeErrorKind::MetadataInvalid)?;
        let input_hint = command
            .get("input")
            .filter(|input| !input.is_null())
            .map(|input| {
                object(input, ActivityDecodeErrorKind::MetadataInvalid).and_then(|input| {
                    optional_text(input, "hint", ActivityDecodeErrorKind::MetadataInvalid)
                })
            })
            .transpose()?
            .flatten();
        Ok(AcpCommand {
            name: required_identifier(
                command,
                "name",
                ActivityDecodeErrorKind::MetadataInvalid,
                limits,
            )?,
            description: required_text(
                command,
                "description",
                ActivityDecodeErrorKind::MetadataInvalid,
            )?,
            input_hint,
        })
    })
    .collect()
}

pub(super) fn session_info(
    update: &Map<String, Value>,
) -> Result<AcpSessionUpdate, ActivityDecodeError> {
    Ok(AcpSessionUpdate::SessionInfo {
        title: optional_update(update, "title")?,
        updated_at: optional_update(update, "updatedAt")?,
    })
}

pub(super) fn usage(
    update: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<AcpUsage, ActivityDecodeError> {
    let used = required_u64(update, "used", ActivityDecodeErrorKind::UsageInvalid)?;
    let size = required_u64(update, "size", ActivityDecodeErrorKind::UsageInvalid)?;
    if size == 0 || used > size {
        return Err(error(ActivityDecodeErrorKind::UsageInvalid));
    }
    let cost = update
        .get("cost")
        .filter(|cost| !cost.is_null())
        .map(|cost| {
            let cost = object(cost, ActivityDecodeErrorKind::UsageInvalid)?;
            let amount = cost
                .get("amount")
                .and_then(Value::as_f64)
                .filter(|amount| amount.is_finite() && *amount >= 0.0)
                .ok_or_else(|| error(ActivityDecodeErrorKind::UsageInvalid))?;
            let currency = required_identifier(
                cost,
                "currency",
                ActivityDecodeErrorKind::UsageInvalid,
                limits,
            )?;
            Ok(AcpCost { amount, currency })
        })
        .transpose()?;
    Ok(AcpUsage { used, size, cost })
}

fn optional_update(
    object: &Map<String, Value>,
    field: &str,
) -> Result<AcpOptionalUpdate<AcpBoundedText>, ActivityDecodeError> {
    match object.get(field) {
        None => Ok(AcpOptionalUpdate::Unchanged),
        Some(Value::Null) => Ok(AcpOptionalUpdate::Cleared),
        Some(Value::String(value)) => Ok(AcpOptionalUpdate::Set(AcpBoundedText(value.clone()))),
        Some(_) => Err(error(ActivityDecodeErrorKind::MetadataInvalid)),
    }
}
