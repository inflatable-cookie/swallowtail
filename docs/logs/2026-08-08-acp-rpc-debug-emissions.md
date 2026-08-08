# ACP And Harness-RPC Debug Emissions

Date: 2026-08-08
Roadmap: g03.056
Card: 173

## Outcome

ACP and harness-RPC connections carry `HostServices` and emit `ProtocolParse`
observations on pump transport failures. Claude Agent also emits from
turn-local `fail` for malformed usage. Observer fixture proves capture.

## Validation

Focused validation passed for claude-agent, pi, cursor, gemini, grok, kimi,
and oh-my-pi. Public API baseline unchanged.
