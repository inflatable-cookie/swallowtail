retained_execution_classifications = {
    ("retained_background_execution", "antigravity.catalogue + antigravity.headless"): "operation_shape_not_applicable",
    ("stream_reattachment", "antigravity.catalogue + antigravity.headless"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "antigravity.catalogue + antigravity.headless"): "upstream_unsupported",
    ("provider_managed_recovery", "cursor-agent.catalogue + cursor-agent.acp + cursor-agent.headless"): "upstream_unsupported",
    ("retained_background_execution", "qwen.headless"): "operation_shape_not_applicable",
    ("stream_reattachment", "qwen.headless"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "qwen.headless"): "upstream_unsupported",
    ("retained_background_execution", "alibaba.conversations"): "upstream_unsupported",
    ("stream_reattachment", "alibaba.conversations"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "alibaba.conversations"): "upstream_unsupported",
    ("retained_background_execution", "bedrock.catalogue; bedrock.runtime"): "separate_route_and_contract_required",
    ("stream_reattachment", "bedrock.catalogue; bedrock.runtime"): "upstream_unsupported",
    ("provider_managed_recovery", "bedrock.catalogue; bedrock.runtime"): "upstream_unsupported",
    ("retained_background_execution", "claude-agent.acp"): "operation_shape_not_applicable",
    ("stream_reattachment", "claude-agent.acp"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "claude-agent.acp"): "upstream_unsupported",
    ("retained_background_execution", "claude-code.headless"): "operation_shape_not_applicable",
    ("stream_reattachment", "claude-code.headless"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "claude-code.headless"): "upstream_unsupported",
    ("retained_background_execution", "anthropic.managed-agent"): "operation_shape_not_applicable",
    ("retained_background_execution", "anthropic.messages"): "separate_route_and_contract_required",
    ("stream_reattachment", "anthropic.messages"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "anthropic.messages"): "upstream_unsupported",
    ("retained_background_execution", "pi.rpc"): "operation_shape_not_applicable",
    ("stream_reattachment", "pi.rpc"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "pi.rpc"): "upstream_unsupported",
    ("retained_background_execution", "deepseek.continuation"): "upstream_unsupported",
    ("stream_reattachment", "deepseek.continuation"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "deepseek.continuation"): "upstream_unsupported",
    ("retained_background_execution", "gemini-cli.acp + gemini-cli.headless"): "operation_shape_not_applicable",
    ("stream_reattachment", "gemini-cli.acp + gemini-cli.headless"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "gemini-cli.acp + gemini-cli.headless"): "upstream_unsupported",
    ("retained_background_execution", "gemini.live"): "operation_shape_not_applicable",
    ("stream_reattachment", "gemini.live"): "upstream_unsupported",
    ("provider_managed_recovery", "gemini.live"): "upstream_unsupported",
    ("retained_background_execution", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("stream_reattachment", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("retained_background_execution", "kimi-code.acp + kimi-code.headless"): "operation_shape_not_applicable",
    ("stream_reattachment", "kimi-code.acp + kimi-code.headless"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "kimi-code.acp + kimi-code.headless"): "shared_contract_and_corpus_required",
    ("retained_background_execution", "kimi-code.local-server"): "operation_shape_not_applicable",
    ("stream_reattachment", "kimi-code.local-server"): "shared_contract_and_corpus_required",
    ("provider_managed_recovery", "kimi-code.local-server"): "shared_contract_and_corpus_required",
    ("retained_background_execution", "kimi-platform.chat"): "upstream_unsupported",
    ("stream_reattachment", "kimi-platform.chat"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "kimi-platform.chat"): "upstream_unsupported",
    ("retained_background_execution", "ollama.attached"): "operation_shape_not_applicable",
    ("stream_reattachment", "ollama.attached"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "ollama.attached"): "operation_shape_not_applicable",
    ("retained_background_execution", "codex.app-server; codex.exec"): "operation_shape_not_applicable",
    ("stream_reattachment", "codex.app-server; codex.exec"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "codex.app-server; codex.exec"): "upstream_unsupported",
    ("retained_background_execution", "openai.realtime"): "operation_shape_not_applicable",
    ("stream_reattachment", "openai.realtime"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "openai.realtime"): "upstream_unsupported",
    ("provider_managed_recovery", "openai.background"): "shared_contract_and_corpus_required",
    ("retained_background_execution", "opencode.http"): "operation_shape_not_applicable",
    ("stream_reattachment", "opencode.http"): "upstream_unsupported",
    ("provider_managed_recovery", "opencode.http"): "upstream_unsupported",
    ("retained_background_execution", "xai.responses-websocket"): "operation_shape_not_applicable",
    ("stream_reattachment", "xai.responses-websocket"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "xai.responses-websocket"): "upstream_unsupported",
    ("retained_background_execution", "grok-build.acp"): "operation_shape_not_applicable",
    ("stream_reattachment", "grok-build.acp"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "grok-build.acp"): "upstream_unsupported",
}

working_resource_write_classifications = {
    ("bounded_workspace_text_write", "qwen.headless"): "upstream_unsupported",
    ("working_resource", "alibaba.conversations"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "alibaba.conversations"): "operation_shape_not_applicable",
    ("working_resource", "bedrock.catalogue; bedrock.runtime"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "bedrock.catalogue; bedrock.runtime"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "claude-agent.acp"): "upstream_unsupported",
    ("bounded_workspace_text_write", "claude-code.headless"): "upstream_unsupported",
    ("working_resource", "anthropic.managed-agent"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "anthropic.managed-agent"): "operation_shape_not_applicable",
    ("working_resource", "anthropic.messages"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "anthropic.messages"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "pi.rpc"): "upstream_unsupported",
    ("working_resource", "deepseek.continuation"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "deepseek.continuation"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "gemini-cli.acp + gemini-cli.headless"): "contract_or_corpus_required",
    ("working_resource", "gemini.live"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "gemini.live"): "operation_shape_not_applicable",
    ("working_resource", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "kimi-code.local-server"): "upstream_unsupported",
    ("working_resource", "kimi-platform.chat"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "kimi-platform.chat"): "operation_shape_not_applicable",
    ("working_resource", "ollama.attached"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "ollama.attached"): "operation_shape_not_applicable",
    ("working_resource", "openai.realtime"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "openai.realtime"): "operation_shape_not_applicable",
    ("working_resource", "openai.background"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "openai.background"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "opencode.http"): "upstream_unsupported",
    ("working_resource", "xai.responses-websocket"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "xai.responses-websocket"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "grok-build.acp"): "upstream_unsupported",
}

owned_runtime_not_applicable = {
    "antigravity.catalogue + antigravity.headless",
    "cursor-agent.catalogue + cursor-agent.acp + cursor-agent.headless",
    "qwen.headless",
    "alibaba.conversations",
    "bedrock.catalogue; bedrock.runtime",
    "claude-agent.acp",
    "claude-code.headless",
    "anthropic.managed-agent",
    "anthropic.messages",
    "pi.rpc",
    "deepseek.continuation",
    "gemini-cli.acp + gemini-cli.headless",
    "gemini.live",
    "grok-build.acp",
    "llama-cpp.attached",
    "kimi-code.acp + kimi-code.headless",
    "kimi-platform.chat",
    "ollama.attached",
    "codex.app-server; codex.exec",
    "openai.realtime",
    "openai.background",
    "opencode.http",
    "xai.responses-websocket",
}
rollover_not_applicable = {
    "antigravity.catalogue + antigravity.headless",
    "cursor-agent.catalogue + cursor-agent.acp + cursor-agent.headless",
    "qwen.headless",
    "alibaba.conversations",
    "bedrock.catalogue; bedrock.runtime",
    "claude-agent.acp",
    "claude-code.headless",
    "anthropic.managed-agent",
    "anthropic.messages",
    "pi.rpc",
    "deepseek.continuation",
    "gemini-cli.acp + gemini-cli.headless",
    "grok-build.acp",
    "llama-cpp.attached",
    "kimi-code.acp + kimi-code.headless",
    "kimi-code.local-server",
    "kimi-platform.chat",
    "ollama.attached",
    "codex.app-server; codex.exec",
    "openai.background",
    "opencode.http",
    "xai.responses-websocket",
}
runtime_rollover_classifications = {
    **{
        ("owned_runtime_lifecycle", route): "operation_shape_not_applicable"
        for route in owned_runtime_not_applicable
    },
    **{
        ("planned_connection_rollover", route): "operation_shape_not_applicable"
        for route in rollover_not_applicable
    },
    (
        "planned_connection_rollover",
        "openai.realtime",
    ): "selected_surface_absence",
}

residual_interface_not_runtime_ordered = {
    ("unverified_newer_allowed", route)
    for route in {
        "alibaba.conversations",
        "bedrock.catalogue; bedrock.runtime",
        "anthropic.managed-agent",
        "anthropic.messages",
        "deepseek.continuation",
        "gemini.live",
        "kimi-platform.chat",
        "openai.realtime",
        "openai.background",
        "xai.responses-websocket",
    }
}
residual_contract_or_corpus = {
    ("interactive_session", "qwen.headless"),
    ("interactive_session", "bedrock.catalogue; bedrock.runtime"),
    ("interactive_session", "claude-code.headless"),
    ("interactive_session", "anthropic.managed-agent"),
    ("unverified_newer_allowed", "llama-cpp.attached"),
    ("interactive_session", "llama-cpp.attached"),
    ("unverified_newer_allowed", "llama-cpp.owned"),
    ("interactive_session", "kimi-platform.chat"),
    ("interactive_session", "ollama.attached"),
}
residual_operation_not_applicable = {
    ("interactive_session", route)
    for route in {
        "gemini.live",
        "openai.realtime",
        "openai.background",
    }
} | {
    ("realtime_media_session", route)
    for route in {
        "antigravity.catalogue + antigravity.headless",
        "qwen.headless",
        "claude-agent.acp",
        "claude-code.headless",
        "anthropic.managed-agent",
        "anthropic.messages",
        "pi.rpc",
        "deepseek.continuation",
        "gemini-cli.acp + gemini-cli.headless",
        "llama-cpp.attached",
        "kimi-code.acp + kimi-code.headless",
        "kimi-code.local-server",
        "kimi-platform.chat",
        "ollama.attached",
        "codex.app-server; codex.exec",
        "openai.background",
        "opencode.http",
        "grok-build.acp",
    }
}
residual_separate_route = {
    ("realtime_media_session", "alibaba.conversations"),
    ("realtime_media_session", "bedrock.catalogue; bedrock.runtime"),
    ("realtime_media_session", "xai.responses-websocket"),
}
residual_no_provider_billing = {
    ("billed_cost_evidence", route)
    for route in {
        "claude-code.headless",
        "llama-cpp.attached",
        "kimi-code.acp + kimi-code.headless",
        "kimi-code.local-server",
        "ollama.attached",
    }
}
residual_selected_surface_absence = {
    ("billed_cost_evidence", route)
    for route in {
        "antigravity.catalogue + antigravity.headless",
        "qwen.headless",
        "alibaba.conversations",
        "bedrock.catalogue; bedrock.runtime",
        "anthropic.managed-agent",
        "anthropic.messages",
        "deepseek.continuation",
        "gemini-cli.acp + gemini-cli.headless",
        "gemini.live",
        "kimi-platform.chat",
        "codex.app-server; codex.exec",
        "openai.realtime",
        "openai.background",
        "cursor-agent.catalogue + cursor-agent.acp + cursor-agent.headless",
    }
}
residual_non_authoritative_cost = {
    ("billed_cost_evidence", route)
    for route in {
        "claude-agent.acp",
        "pi.rpc",
        "opencode.http",
        "grok-build.acp",
    }
}
residual_feature_classifications = {
    **{
        cell: "interface_axis_not_runtime_ordered"
        for cell in residual_interface_not_runtime_ordered
    },
    **{
        cell: "contract_or_corpus_required"
        for cell in residual_contract_or_corpus
    },
    **{
        cell: "operation_shape_not_applicable"
        for cell in residual_operation_not_applicable
    },
    **{
        cell: "separate_route_and_contract_required"
        for cell in residual_separate_route
    },
    **{
        cell: "no_provider_billing_boundary"
        for cell in residual_no_provider_billing
    },
    **{
        cell: "selected_surface_absence"
        for cell in residual_selected_surface_absence
    },
    **{
        cell: "non_authoritative_cost_evidence"
        for cell in residual_non_authoritative_cost
    },
}
