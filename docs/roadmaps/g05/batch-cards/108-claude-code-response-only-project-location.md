# 108 Claude Code Response-Only Project Location

Status: complete; PR 289; optional read-only working resource as the response-only child project location; absence byte-identical; cwd-relative ambient CLI behaviour frozen from the scratch-directory fixture; no isolation claim; route stays AmbientHost
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-08
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: `v0.4.3` tagged; Contract 013 Ambient Harness ("the working resource may select a project location"); the Desktop card 297 gap report 2026-09-06

## Goal

Let a consumer give `claude-code.response-only` an optional `Read` working
resource used solely as the native child's cwd, so a tool-free bounded
response no longer inherits the host process's current directory. No
isolation claim: the route stays `AmbientHost`.

## Scope

1. Additive optional working resource on the response-only prepared input;
   host-local already applies cwd from an approved resource
   (`src/process/launch.rs` `apply_working_resource`), so the change is in
   the prepared path and the process request, not the host.
2. Freeze, with a scratch-directory fixture, what the native CLI still does
   cwd-relatively with tools suppressed (project settings, `CLAUDE.md`
   discovery, git context) and record it in the guide as ambient behaviour,
   not as a boundary.
3. Default unchanged: no resource means inherited cwd, documented as such.
4. Guide, matrix cell, changelog `[Unreleased]`, additive baseline.

## Out Of Scope

Any isolation posture change; tools; working-resource write access;
`claude-code.headless`.

## Acceptance Criteria

- [ ] optional `Read` resource sets the child cwd; absence keeps today's behaviour
- [ ] cwd-relative native behaviours with tools suppressed are frozen from a fixture
- [ ] no boundary or isolation claim added anywhere
- [ ] guide, matrix, changelog, additive baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the response-only child runs in the leased directory when one is
given and nothing in the route claims that directory is a boundary. Smallest
counterexample: a matrix cell that says isolated.

## Stop Conditions

The CLI reads or writes outside the given cwd with tools suppressed (record;
return to Chatterbox).

## Auto-Continuation

No.

## Result

`ClaudeCodeResponseProfileInput::with_working_resource` binds one optional
host-approved working resource on the prepared response-only run input.
`prepare_run` advertises `Capability::WorkingResource` with
`ResourceAccess::Read` and `ResourceRepresentation::Filesystem` plus the
`WorkingResource` host service only when a resource is bound, and the driver
forwards the resource onto the process request, where the host applies it as
the native child's working directory. Omission is byte-identical: no
capability, no service requirement, no process-request field, and an unbound
request still rejects any plan advertising `WorkingResource`. A host without
working-resource authority fails closed at preparation; services that lose
the working-resource service fail closed at run start before any process.

The scratch-directory fixture (`claude_code_response_only_project_location`)
freezes the Swallowtail-owned half of the child-side view over a real local
host: the child runs in the leased directory, its argv keeps `--tools ""`,
`--safe-mode`, the empty `--mcp-config`, and `--no-session-persistence`, and
cwd-relative resolution anchors there — project settings from the directory
itself, while upward `CLAUDE.md` discovery and git context are exercised
against an ancestor anchor holding the marker and the git repository root.
The recorded native CLI behaviours come from the Desktop card 297 gap
report; they are ambient, not a boundary: nothing in the route, guide, or
fixtures claims isolation, and the route stays `AmbientHost`. The guide row
and the ambient-behaviour record carry the same wording; the changelog and
the additive `v0.4.4` baseline (`with_working_resource`) record the surface.

Validation tier: `cargo fmt -p swallowtail-adapter-claude-agent -- --check`,
`effigy validate:focused swallowtail-adapter-claude-agent`,
`effigy package:verify-affected swallowtail-adapter-claude-agent`,
`effigy package:api` (one additive line; immutable v0.4.3 intact),
`effigy qa:northstar`, and `git diff --check` all pass. The full adapter
suite stays at 462 passing tests with no behavioural change on the unbound
path.

Independent same-workspace cross-model review of PR 289 (codex
`gpt-5.6-terra`, high reasoning) checked the diff against this card, the
Card 108 manifest, and Contracts 013 and 039. Round one raised one blocker:
the fixture hard-coded the recorded behaviours instead of exercising them,
and the guide attributed the behaviour list to the fixture rather than to
the recorded provider evidence. The repair (7fd124db) seeds the `CLAUDE.md`
marker and the git repository root in an ancestor anchor so upward
discovery and git resolution are genuinely exercised from the leased
directory, and the guide now attributes the three behaviours to the Desktop
card 297 gap report while the fixture freezes the Swallowtail-owned half.
Round two returned accept with no defects.
