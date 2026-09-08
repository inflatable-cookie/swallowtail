# 108 Claude Code Response-Only Project Location

Status: ready; the `v0.4.3` tag exists, so its gate is satisfied
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
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
