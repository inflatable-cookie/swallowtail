# 206 Figmatic Claude Code Response-Only Adoption

Status: superseded by card 215
Owner: Tom
Updated: 2026-08-12

Exact Claude Code `2.1.228` replaced this `v0.3.2` / `2.1.227` adoption
target before Figmatic completed the card. Card 215 carries the current exact
source identity and packaged `g04.005` smoke.

## Goal

Adopt the exact Swallowtail response-only source identity in Figmatic and
replace the unsafe ACP proposal transport without changing Figmatic's compiler
or acceptance authority.

## Source Identity

- Swallowtail tag: `v0.3.2`
- peeled commit: `a859d56b47b1bc2975df7d0516ca96fd8e485b35`
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
- medium effort may emit content-free `ProgressSnapshot` events before the
  one assistant text result; consumers need not interpret them

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
- replay Figmatic unit `fc335758-3c1a-4bda-bb71-a8c6119fe876` after relinking
  the exact tag

## Out Of Scope

Swallowtail release, Figmatic policy changes, provider-authored CSS, schema
emulation, and consumer-side repair.
