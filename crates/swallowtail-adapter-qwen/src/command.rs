use swallowtail_core::ModelId;

pub(crate) fn arguments(model: &ModelId) -> Vec<String> {
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
        "16",
        "--max-session-turns",
        "24",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
