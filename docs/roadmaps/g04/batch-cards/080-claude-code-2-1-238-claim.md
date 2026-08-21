# 080 Claude Code 2.1.238 Claim

Status: completed
Owner: Tom
Milestone: [g04.028 Claude Code 2.1.238 Useful Newer](../028-claude-code-2-1-238-useful-newer.md)
Created: 2026-08-21

## Task

Raise the Claude Code headless and response-only qualified ceilings from
`2.1.235` to official `2.1.238` after identity card 079 confirms
compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-claude-agent/src/claude_code_selection.rs`
and `claude_code_response_selection.rs`:

- Change latest-qualified constants from `"2.1.235"` to `"2.1.238"`
- Keep claim ids `claude-code.headless.window-1` and
  `claude-code.response-only.window-1`
- Keep `AllowUnverified`
- Keep baselines `2.1.220` and `2.1.227`
- Keep behaviors `claude-code.headless.stream-json.v1` and
  `claude-code.response-only.stream-json.v1`
- Keep the empty response-only deny-list
- Unit tests: `2.1.235`, `2.1.236`, `2.1.237`, and `2.1.238` qualified;
  synthetic `UnverifiedNewer` is `2.1.239`

In tests:

- Add `2.1.238` identity corpus assertions
- Keep the `2.1.235` specimen
- Keep decoder corpora `claude-code-2.1.220`, `claude-code-2.1.227`,
  and `claude-code-2.1.228`
- Move synthetic later-stable UnverifiedNewer to `2.1.239`

In docs:

- Update Claude Code prepared-integration guide
- Update Claude Code route + feature matrix rows
- Add `CHANGELOG.md` Unreleased entry
- Write identity and claim logs
- Index family research, logs, cards, the g04 milestone/checkpoint, and the
  standing currentness pointer
- Keep the Next Task on g04.024; do not edit architecture or contracts

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or
consumer checks.

## Acceptance

- Official `2.1.238` classifies as Qualified Maintained on both axes
- `2.1.220`, `2.1.227`, `2.1.235`, `2.1.236`, and `2.1.237` remain
  Qualified
- `2.1.239` remains permitted UnverifiedNewer
- Decoder specimens remain
- Named adapter gates pass
- Named route, Northstar, and index gates pass

Auto-continuation: No. Keep the Next Task on g04.024.

## Out Of Scope

- Gemini requalification (deferred)
- Codex
- Qwen
- Ollama
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Flattening onto Claude Agent ACP
- Provider work
- Kimi Platform implementation, Next Task changes, architecture, or contract
  edits
