use crate::failure::failure;
use serde_json::Value;
use swallowtail_runtime::RuntimeFailure;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CommandDisposition {
    Handled,
    Queued,
    Started,
}

pub(super) fn parse_command_disposition(
    data: Option<&Value>,
    command: &'static str,
    required: bool,
) -> Result<Option<CommandDisposition>, RuntimeFailure> {
    let Some(data) = data else {
        return missing_disposition(required);
    };
    let Some(disposition) = data.get("disposition") else {
        return missing_disposition(required);
    };
    let Some(disposition) = disposition.as_str() else {
        return Err(invalid_disposition());
    };

    let parsed = match (command, disposition) {
        ("prompt", "handled") | ("steer" | "follow_up", "handled") => CommandDisposition::Handled,
        ("prompt", "queued") | ("steer" | "follow_up", "queued") => CommandDisposition::Queued,
        ("prompt", "started") => CommandDisposition::Started,
        _ => return Err(invalid_disposition()),
    };
    Ok(Some(parsed))
}

fn missing_disposition(required: bool) -> Result<Option<CommandDisposition>, RuntimeFailure> {
    if required {
        Err(invalid_disposition())
    } else {
        Ok(None)
    }
}

fn invalid_disposition() -> RuntimeFailure {
    failure(
        "swallowtail.pi.rpc.response_disposition_invalid",
        "Pi RPC response omitted or changed its command disposition",
    )
}

#[cfg(test)]
mod tests {
    use super::{CommandDisposition, parse_command_disposition};
    use serde_json::json;

    #[test]
    fn prompt_dispositions_are_typed_and_handled_is_distinct() {
        assert_eq!(
            parse_command_disposition(Some(&json!({"disposition": "started"})), "prompt", true)
                .expect("started is valid"),
            Some(CommandDisposition::Started)
        );
        assert_eq!(
            parse_command_disposition(Some(&json!({"disposition": "queued"})), "prompt", true)
                .expect("queued is valid"),
            Some(CommandDisposition::Queued)
        );
        assert_eq!(
            parse_command_disposition(Some(&json!({"disposition": "handled"})), "prompt", true)
                .expect("handled is valid"),
            Some(CommandDisposition::Handled)
        );
    }

    #[test]
    fn queued_commands_reject_prompt_only_disposition_and_unknown_values() {
        assert_eq!(
            parse_command_disposition(Some(&json!({"disposition": "queued"})), "steer", true)
                .expect("steering is queued"),
            Some(CommandDisposition::Queued)
        );
        assert_eq!(
            parse_command_disposition(Some(&json!({"disposition": "handled"})), "follow_up", true,)
                .expect("follow-up can be handled"),
            Some(CommandDisposition::Handled)
        );
        assert!(
            parse_command_disposition(Some(&json!({"disposition": "started"})), "steer", true)
                .is_err()
        );
        assert!(
            parse_command_disposition(Some(&json!({"disposition": "future"})), "prompt", true)
                .is_err()
        );
        assert!(
            parse_command_disposition(Some(&json!({"disposition": 1})), "prompt", true).is_err()
        );
    }

    #[test]
    fn disposition_is_optional_only_for_legacy_versions() {
        assert_eq!(
            parse_command_disposition(None, "prompt", false).expect("legacy response"),
            None
        );
        assert_eq!(
            parse_command_disposition(Some(&json!({})), "prompt", false)
                .expect("legacy response without data"),
            None
        );
        assert!(parse_command_disposition(None, "prompt", true).is_err());
        assert!(parse_command_disposition(Some(&json!({})), "prompt", true).is_err());
    }
}
