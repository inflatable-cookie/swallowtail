# 116 Portable Provider And Harness Failure Mappings

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../041-portable-failure-classification.md`
Depends on: card 115

## Goal

Preserve existing route diagnostics while promoting machine-readable provider
and harness failure evidence into the portable classification.

## Scope

1. Map typed direct-provider error kinds across current production adapters.
2. Map exact harness exit or event evidence where qualified.
3. Leave opaque provider and harness failures honestly unknown.
4. Freeze equivalent cross-route classes and unchanged exact diagnostic codes.

## Validation

- focused validation for every changed adapter package

## Stop Conditions

- stop on prose, stderr, or output heuristics
- stop if a route needs invented retry or remediation truth

## Auto-Continuation

Continue to card 117 after deterministic mappings pass.

## Completion

- typed direct-provider evidence maps across Anthropic, OpenAI, Kimi Platform,
  Bedrock, DeepSeek, llama.cpp, Ollama, xAI, and Alibaba Model Studio.
- exact harness evidence maps across Gemini, Claude Code, Qwen, Kimi local
  server, Pi, Oh My Pi, Cursor, and Antigravity.
- existing diagnostic codes and messages remain unchanged; opaque process and
  provider failures retain `Unknown` rather than parsing prose or output.
- focused validation passed for every changed adapter package.
