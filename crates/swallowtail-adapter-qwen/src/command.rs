use crate::budgets::QwenHeadlessBudgets;
use crate::plan_mode;
use swallowtail_core::{HarnessMode, ModelId};

pub(crate) fn arguments(
    model: &ModelId,
    budgets: QwenHeadlessBudgets,
    harness_mode: Option<HarnessMode>,
) -> Vec<String> {
    insert_budget_values(
        [
            "--input-format",
            "text",
            "--output-format",
            "stream-json",
            "--include-partial-messages",
            "--safe-mode",
            "--approval-mode",
            plan_mode::approval_arg(harness_mode),
            "--model",
            model.as_str(),
            "--core-tools",
            "read_file,grep_search,glob,list_directory,lsp",
            "--exclude-tools",
            "run_shell_command,monitor,edit,write_file,notebook_edit,agent,web_fetch,save_memory,skill,workflow,artifact,record_artifact,cron_create,cron_delete,create_sub_session,task_create,task_update,task_stop,team_create,team_delete,send_message,enter_worktree,exit_worktree",
            "--max-wall-time",
            "60s",
            "--max-tool-calls",
            "--max-session-turns",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        budgets,
    )
}

fn insert_budget_values(mut arguments: Vec<String>, budgets: QwenHeadlessBudgets) -> Vec<String> {
    let tool_index = arguments
        .iter()
        .position(|argument| argument == "--max-tool-calls")
        .expect("tool-call flag is always present");
    arguments.insert(tool_index + 1, budgets.tool_calls_arg());
    let turn_index = arguments
        .iter()
        .position(|argument| argument == "--max-session-turns")
        .expect("session-turn flag is always present");
    arguments.insert(turn_index + 1, budgets.session_turns_arg());
    arguments
}

pub(crate) fn resumed_arguments(
    model: &ModelId,
    session_id: &str,
    budgets: QwenHeadlessBudgets,
    harness_mode: Option<HarnessMode>,
) -> Vec<String> {
    let mut arguments = arguments(model, budgets, harness_mode);
    arguments.extend(["--resume".to_owned(), session_id.to_owned()]);
    arguments
}

pub(crate) fn reasoning_arguments(
    model: &ModelId,
    budgets: QwenHeadlessBudgets,
    harness_mode: Option<HarnessMode>,
) -> Vec<String> {
    let mut arguments = arguments(model, budgets, harness_mode);
    replace_input_format(&mut arguments);
    arguments
}

pub(crate) fn resumed_reasoning_arguments(
    model: &ModelId,
    session_id: &str,
    budgets: QwenHeadlessBudgets,
    harness_mode: Option<HarnessMode>,
) -> Vec<String> {
    let mut arguments = reasoning_arguments(model, budgets, harness_mode);
    arguments.extend(["--resume".to_owned(), session_id.to_owned()]);
    arguments
}

fn replace_input_format(arguments: &mut [String]) {
    if let Some(input_format) = arguments
        .iter_mut()
        .find(|argument| argument.as_str() == "text")
    {
        *input_format = "stream-json".to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::{arguments, resumed_arguments};
    use crate::budgets::QwenHeadlessBudgets;
    use swallowtail_core::{HarnessMode, ModelId};

    fn model() -> ModelId {
        ModelId::new("qwen3-coder-plus").expect("valid model")
    }

    #[test]
    fn omission_keeps_explicit_default_approval_mode() {
        let args = arguments(&model(), QwenHeadlessBudgets::omitted(), None);
        assert_eq!(
            args.windows(2)
                .find(|pair| pair[0] == "--approval-mode")
                .map(|pair| pair[1].as_str()),
            Some("default")
        );
        assert!(args.iter().any(|argument| argument == "--safe-mode"));
        assert!(!args.iter().any(|argument| argument == "plan"));
    }

    #[test]
    fn plan_replaces_only_the_approval_value() {
        let args = arguments(
            &model(),
            QwenHeadlessBudgets::omitted(),
            Some(HarnessMode::Plan),
        );
        assert_eq!(
            args.windows(2)
                .find(|pair| pair[0] == "--approval-mode")
                .map(|pair| pair[1].as_str()),
            Some("plan")
        );
        assert!(args.iter().any(|argument| argument == "--safe-mode"));
        let resumed = resumed_arguments(
            &model(),
            "session-fixture-1",
            QwenHeadlessBudgets::omitted(),
            Some(HarnessMode::Plan),
        );
        assert_eq!(
            resumed
                .windows(2)
                .find(|pair| pair[0] == "--approval-mode")
                .map(|pair| pair[1].as_str()),
            Some("plan")
        );
        assert_eq!(
            resumed
                .windows(2)
                .find(|pair| pair[0] == "--resume")
                .map(|pair| pair[1].as_str()),
            Some("session-fixture-1")
        );
    }
}
