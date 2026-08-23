use crate::budgets::QwenHeadlessBudgets;
use swallowtail_core::ModelId;

pub(crate) fn arguments(model: &ModelId, budgets: QwenHeadlessBudgets) -> Vec<String> {
    insert_budget_values(
        [
            "--input-format",
            "text",
            "--output-format",
            "stream-json",
            "--include-partial-messages",
            "--safe-mode",
            "--approval-mode",
            "default",
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
) -> Vec<String> {
    let mut arguments = arguments(model, budgets);
    arguments.extend(["--resume".to_owned(), session_id.to_owned()]);
    arguments
}

pub(crate) fn reasoning_arguments(model: &ModelId, budgets: QwenHeadlessBudgets) -> Vec<String> {
    let mut arguments = arguments(model, budgets);
    replace_input_format(&mut arguments);
    arguments
}

pub(crate) fn resumed_reasoning_arguments(
    model: &ModelId,
    session_id: &str,
    budgets: QwenHeadlessBudgets,
) -> Vec<String> {
    let mut arguments = reasoning_arguments(model, budgets);
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
