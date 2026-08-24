# 154 Claude Code 2.1.241 Claim

Status: completed
Owner: Tom
Milestone: [g04.055 Claude Code 2.1.241 Useful Newer](../055-claude-code-2-1-241-useful-newer.md)
Created: 2026-08-24

## Task

Raise the Claude Code headless and response-only qualified ceilings from
`2.1.238` to official `2.1.241` after identity card 153 confirms
compatible-extension.

## Edit Set

In `crates/swallowtail-adapter-claude-agent/src/claude_code_selection.rs`
and `claude_code_response_selection.rs`:

- Change latest-qualified constants from `"2.1.238"` to `"2.1.241"`
- Keep claim ids `claude-code.headless.window-1` and
  `claude-code.response-only.window-1`
- Keep `AllowUnverified`
- Keep baselines `2.1.220` and `2.1.227`
- Keep behaviors `claude-code.headless.stream-json.v1` and
  `claude-code.response-only.stream-json.v1`
- Keep the empty response-only deny-list
- Unit tests: `2.1.238`, `2.1.239`, `2.1.240`, and `2.1.241` qualified;
  synthetic `UnverifiedNewer` is `2.1.242`

In tests:

- Add `2.1.241` identity corpus assertions
- Keep the `2.1.238` specimen and structured-output evidence stop
- Keep decoder corpora `claude-code-2.1.220`, `claude-code-2.1.227`,
  and `claude-code-2.1.228`
- Move synthetic later-stable UnverifiedNewer to `2.1.242`

In docs:

- Update Claude Code prepared-integration guide
- Update Claude Code route + feature matrix rows
- Add `CHANGELOG.md` Unreleased entry
- Write identity and claim logs
- Index family research and logs
- Do not rewrite `docs/roadmaps/README.md` Next Task
- Update the g04 milestone/checkpoint and batch-card indexes

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
```

Do not run workspace `qa`, broad `qa:docs`, live probes, MSRV, or
consumer checks.

## Acceptance

- Official `2.1.241` classifies as Qualified Maintained on both axes
- `2.1.220`, `2.1.227`, `2.1.238`, `2.1.239`, and `2.1.240` remain
  Qualified
- `2.1.242` remains permitted UnverifiedNewer
- Decoder specimens remain
- Named adapter gates pass

Auto-continuation: No. Next Task stays on the generation's actual work.

## Out Of Scope

- Codex
- Qwen
- Ollama
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer checks
- Mapping unused surfaces
- Flattening onto Claude Agent ACP
- Provider work
- Next Task changes, architecture, or contract edits
