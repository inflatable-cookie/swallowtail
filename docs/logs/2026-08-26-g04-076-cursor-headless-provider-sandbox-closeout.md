# 2026-08-26 g04.076 Cursor Headless Provider Sandbox Closeout

Status: stopped after evidence
Owner: Tom
Milestone: g04.076
Cards: 210 complete; 211-212 blocked
Branch: `t3code/cursor-headless-provider-sandbox`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-a9727dc6`
Base: `bf8f16269f149af6b4a6f9238e1af46d4ff66f13` (`origin/main` at dispatch)
Planning base ancestor: `9afc07eede0ab175a3c8fc4b834448043c356f9b`
PR: pending

## Result

Card 210 completed an exact four-build artifact, parser, mode-resolution,
platform/backend, filesystem/network/subprocess, approval/escape, observation,
and production-seam audit. Research 223 admits no deliver-now
`HarnessIsolation::ProviderEnforced` row. Cards 211 and 212 are blocked and
were not executed. The Cursor adapter, prepared facade, child argv, fixtures,
guide, matrices, and API baseline are unchanged. No install, host-binary
replacement, login, account inspection, catalogue, provider prompt, tool
execution, paid work, ambient config mutation, or live model run was used.

## Evidence Stop

All four qualified builds parse `--sandbox <mode>` with choices
`enabled|disabled`. The help text says the flag overrides config. Isolated
parser cases reject missing, empty, invalid, and case-folded values and accept
print-mode placement. Repeats parse; last-versus-first application was not
observed.

Exact `PT` resolution is
`sandboxOverride ?? config.sandbox.mode ?? (cliSandboxDefaultEnabled ?
"enabled" : "disabled")`. Default config mode is `disabled`. CLI override is
not a closed boundary: persisted config, `sandbox.json` extra paths and
`allow_all` network, team `sandboxingDisabled`, and feature gate
`composer_sandbox_settings_visible` remain live under `Ambient` posture.

The colocated helper advertises itself as `Sandboxing helper for Everysphere
shell-exec`. Darwin `isSandboxSupported` is `/usr/bin/sandbox-exec` plus helper
path existence. `cursorsandbox --preflight-only` is not that check. Linux and
Windows backends are unbound from these darwin-arm64 artifacts.

Print mode without `--force` uses always-deny for approval. Unsupported
commands are denied, not contained. File, MCP, and fetch tools are not the
helper. Qualified stream-json has no sandbox-enforcement event.

Current production preparation keeps `AmbientHost` and omits `--sandbox`.
Omission retains exact no-flag argv.

## Changed Surfaces

- `docs/research/223-cursor-headless-provider-sandbox-evidence.md`:
  promoted exact artifact, parser, backend, production audit, claim strength,
  and empty deliver-now table
- cards 210-212, g04.076, programme, triage, indexes, this closeout

No production code, public API, shared contract/runtime, guide capability,
matrix, or changelog edit.

## Validation

Passed:

- `effigy validate:focused swallowtail-adapter-cursor`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

No production code changed. Doctor was not re-run for repair; the inherited
`scan.god-files` 380 findings (334 warnings, 46 errors) and
`scan.generated-in-src` one warning are unchanged by docs-only edits.

## Continuation

Keep g04 open. Reassess the remaining per-route feature inventory for the next
serial lane unless the operator supplies a different direction. Contract 029
currentness remains standing. Do not compile the next family from this closeout.
