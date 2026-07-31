use swallowtail_core::{HarnessIsolation, ModelId, ReasoningMode, ResourceAccess};

pub(crate) struct HeadlessCommand<'a> {
    pub(crate) prompt: &'a str,
    pub(crate) model: &'a ModelId,
    pub(crate) access: ResourceAccess,
    pub(crate) isolation: HarnessIsolation,
    pub(crate) effort: Option<&'a ReasoningMode>,
    pub(crate) schema: Option<&'a str>,
    pub(crate) conversation_id: Option<&'a str>,
}

pub(crate) fn arguments(command: HeadlessCommand<'_>) -> Vec<String> {
    let mut arguments = [
        "--print".to_owned(),
        command.prompt.to_owned(),
        "--output-format".to_owned(),
        "stream-json".to_owned(),
        "--model".to_owned(),
        command.model.as_str().to_owned(),
    ]
    .into_iter()
    .collect::<Vec<_>>();
    if command.access == ResourceAccess::Read {
        arguments.extend(["--mode".to_owned(), "plan".to_owned()]);
    }
    if command.isolation == HarnessIsolation::ProviderEnforced {
        arguments.push("--sandbox".to_owned());
    }
    if let Some(effort) = command.effort {
        arguments.extend(["--effort".to_owned(), effort.as_str().to_owned()]);
    }
    if let Some(schema) = command.schema {
        arguments.extend(["--json-schema".to_owned(), schema.to_owned()]);
    }
    if let Some(conversation_id) = command.conversation_id {
        arguments.extend(["--conversation".to_owned(), conversation_id.to_owned()]);
    }
    arguments
}

#[cfg(test)]
mod tests {
    use super::{HeadlessCommand, arguments};
    use swallowtail_core::{HarnessIsolation, ModelId, ReasoningMode, ResourceAccess};

    #[test]
    fn ambient_and_sandboxed_profiles_never_bypass_permissions() {
        let model = ModelId::new("gemini-3.6-flash-high").expect("model");
        let effort = ReasoningMode::new("high").expect("effort");
        let ambient = arguments(HeadlessCommand {
            prompt: "private prompt",
            model: &model,
            access: ResourceAccess::Read,
            isolation: HarnessIsolation::AmbientHost,
            effort: Some(&effort),
            schema: Some("{\"type\":\"object\"}"),
            conversation_id: None,
        });
        let sandboxed = arguments(HeadlessCommand {
            prompt: "private prompt",
            model: &model,
            access: ResourceAccess::ReadWrite,
            isolation: HarnessIsolation::ProviderEnforced,
            effort: None,
            schema: None,
            conversation_id: Some("exact-conversation"),
        });

        assert!(ambient.windows(2).any(|pair| pair == ["--mode", "plan"]));
        assert!(!ambient.iter().any(|value| value == "--sandbox"));
        assert!(ambient.windows(2).any(|pair| pair == ["--effort", "high"]));
        assert!(ambient.iter().any(|value| value == "--json-schema"));
        assert!(sandboxed.iter().any(|value| value == "--sandbox"));
        assert!(!sandboxed.iter().any(|value| value == "--mode"));
        assert!(
            sandboxed
                .windows(2)
                .any(|pair| pair == ["--conversation", "exact-conversation"])
        );
        for values in [&ambient, &sandboxed] {
            assert!(
                values
                    .windows(2)
                    .any(|pair| pair == ["--print", "private prompt"])
            );
            assert!(
                !values
                    .iter()
                    .any(|value| value == "--dangerously-skip-permissions")
            );
            assert!(!values.iter().any(|value| value == "--continue"));
        }
    }
}
