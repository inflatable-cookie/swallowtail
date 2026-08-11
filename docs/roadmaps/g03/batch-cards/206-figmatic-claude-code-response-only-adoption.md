# 206 Figmatic Claude Code Response-Only Adoption

Status: ready
Owner: Tom
Updated: 2026-08-11

## Goal

Adopt the exact Swallowtail response-only source identity in Figmatic and
replace the unsafe ACP proposal transport without changing Figmatic's compiler
or acceptance authority.

## Source Identity

- Swallowtail commit: `d8f9aae41b3604283676dc52c85b307723060f80`
- package: `swallowtail-adapter-claude-agent`
- route: `claude-code.response-only`
- executable claim: exact Claude Code `2.1.227`

## API

- `prepare_claude_code_response_only`
- `ClaudeCodeResponsePreparationInput`
- `ClaudeCodeResponsePreparationProbe`
- `ClaudeCodeResponsePreparedIntegration::prepare_run`
- `ClaudeCodeResponseProfileInput::new(request_id, model, prompt, deadline)`
- `ClaudeCodeResponsePreparedRun::start_run`
- terminal `OperationContent` is untrusted text

Use the compile-tested
[`prepared_claude_code_response_only` example](../../../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_code_response_only.rs).

## Acceptance

- Figmatic supplies one prompt and exact model selection
- Figmatic passes no schema, working resource, attachment, tool, or callback
- host environment preserves approved `HOME`, `USER`, and `LOGNAME`, with no
  `ANTHROPIC_API_KEY`
- events and terminal outcome drain concurrently; the run always closes
- Figmatic performs JSON extraction, schema-v4 deserialization, identity/value
  validation, deterministic compilation, gates, and operator acceptance
- no retry, continuation, fallback, or alternate route is selected

## Out Of Scope

Swallowtail release, Figmatic policy changes, provider-authored CSS, schema
emulation, and consumer-side repair.
