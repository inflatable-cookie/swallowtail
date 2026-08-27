# 2026-08-27 g04.081 Pi SDK Sidecar Reasoning Selection Closeout

Status: stopped after evidence
Owner: Tom
Milestone: g04.081
Cards: 225 complete; 226-227 blocked
Branch: `t3code/pi-sdk-sidecar-selection`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-532827d3`
Base: `5cbb4d6ff4726364c7fe3bde6313fc248211f625` (planning commit on `origin/main`)
PR: pending

## Result

Card 225 froze exact Pi 0.84.2 thinking-level vocabulary, clamp behavior,
construction precedence, runtime replacement, sidecar snapshots, and current
Rust omission/validation posture. Research 228 admits no deliver-now row.
Cards 226 and 227 are blocked and were not executed. Production code, public
API, sidecar source tag, wire, fixtures, guide capability claims, and matrices
are unchanged. No provider prompt, credential use, account inspection, package
install, or ambient configuration mutation was used.

## Evidence Stop

Exact `createAgentSession` always calls `clampThinkingLevel` before readiness.
Unsupported, unknown, or out-of-map values silently substitute. Contract 040
forbids portable clamping. Deliver-now therefore requires a closed static
provider/model/value gate before process work plus `session.thinkingLevel`
agreement when clamp is identity.

The sidecar already accepts optional bootstrap `thinkingLevel`, forwards it
through the runtime factory on every replacement, and reports
`session.thinkingLevel` in bootstrap/state snapshots. Rust deliberately omits
and ignores that field today. Sidecar catalogue returns provider/id only. The
bundled `@earendil-works/pi-ai@0.84.2` corpus contains 1267 models and more
than four thousand non-`off` level rows; selectable models additionally depend
on configured auth. No route-local closed admission table survives those gates.

Omission retains exact prior bootstrap bytes, Pi default/stored behavior, and
no portable selection claim.

## Changed Surfaces

- `docs/research/228-pi-sdk-sidecar-reasoning-selection-evidence.md`: promoted
  exact sources, lifecycle matrix, gate table, and empty deliver-now set
- cards 225-227, g04.081, programme, triage, indexes, this closeout

No production code, public API, shared contract/runtime, guide capability,
matrix, or changelog edit.

## Validation

Pending on PR push:

- `effigy validate:focused swallowtail-adapter-pi`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

## Continuation

Keep g04 open. Reassess the remaining per-route feature inventory for the next
serial lane unless the operator supplies a different direction. Contract 029
currentness remains standing. Do not compile the next family from this
closeout.
