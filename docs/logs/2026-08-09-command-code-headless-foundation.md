# Command Code Headless Foundation

Date: 2026-08-09
Roadmap: g03.059
Card: 184

## Outcome

Swallowtail current source now contains one separately selectable Command Code
adapter and one exact `command-code.headless` route. It binds npm release
`1.15.1` on axis `command-code.npm`, provider-owned local account state without
a credential lease, one explicit model, plan mode, disabled session retention
and auto-update, stdin prompt delivery, and NDJSON projection of thinking,
text, tool lifecycle, usage, and namespaced unknowns.

The public guide, prepared example, route and feature matrices, activity
inventory, architecture, package contract, and release tooling agree. Current
source has 29 packages and 35 production routes. Immutable `v0.3.1` evidence
remains 28 packages and 34 routes. Command Code has a separate unreleased
semantic API baseline; no tagged baseline was rewritten.

## Live Finding

The installed probe first failed because approving the npm shebang script alone
cannot find `node` once the process environment drops `PATH`. Binding an
interpreted launch—exact `node` plus the resolved `command-code` launcher as a
fixed prefix—fixed discovery and the authenticated turn without relaxing
qualification.

## Validation

- `effigy validate:focused swallowtail-adapter-command-code` — 24 passed;
  warnings denied
- `effigy package:verify-affected swallowtail-adapter-command-code` — extracted
  package passed
- `effigy package:metadata` — 29 current packages; immutable baseline 28
- `effigy package:api` — 28 immutable package APIs plus one unreleased Command
  Code API
- `effigy qa:guides` — 35 routes, 24 route guides, 34 examples
- `effigy qa:routes` — 35 routes, 28 solutions, 68 activity operations
- `effigy qa:docs` / `effigy qa:consumer-docs` — tagged/current front-door
  distinction passed
- `effigy probe:command-code-installed` — exact installed payload passed
- operator-gated `effigy probe:command-code-plan` with
  `SWALLOWTAIL_LIVE_COMMAND_CODE_MODEL=deepseek/deepseek-v4-flash` — prepared
  plan-mode turn completed

All authenticated work ran with plan mode, no session retention, no
auto-update, no skills, and no `--yolo`. No login, logout, workspace mutation,
version bump, tag, GitHub Release, or registry mutation ran.

Source remains an uncommitted worktree on `main` atop
`b5881192b7b87596a0c49eb4bb3c9e2aea239940`.

## Next

Hold at the g03.059 planning checkpoint. Reassess resume/continue, session
export, and Provider API only under a separate operator-selected roadmap.
