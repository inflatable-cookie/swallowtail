fn ordinary_success_jsonl(version: &str) -> String {
    include_str!("../fixtures/qwen-code-v0.19.11/success.jsonl").replace(
        "\"qwen_code_version\":\"0.19.11\"",
        &format!("\"qwen_code_version\":\"{version}\""),
    )
}

fn flag_value<'a>(arguments: &'a [String], flag: &str) -> Option<&'a str> {
    arguments.windows(2).find_map(|pair| {
        (pair[0] == flag).then_some(pair[1].as_str())
    })
}

fn budget_run_input(host_suffix: &str) -> QwenRunProfileInput {
    QwenRunProfileInput::new(
        RequestId::new(format!("qwen-budget-run-{host_suffix}")).expect("valid request"),
        QwenModelSelection::new(
            ModelRouteId::new(format!("qwen.budget.route.{host_suffix}")).expect("valid route"),
            ModelRouteRevision::new("1").expect("valid route revision"),
            ProviderId::new("alibaba-modelstudio").expect("valid provider"),
            ModelId::new("qwen3-coder-plus").expect("valid model"),
        ),
        OperationContent::new("budget fixture prompt").expect("valid prompt"),
        WorkingResourceRef::new("qwen.budget.workspace").expect("valid resource"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}
