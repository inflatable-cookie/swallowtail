# 066 Optional Harness Isolation Rebaseline

Status: complete
Owner: Tom
Updated: 2026-07-21
Milestone: `../018-kimi-code-acp-portability-proof.md`

## Objective

Make ambient harness communication first-class, preserve exact optional
isolation claims, and reopen Kimi ACP without weakening bounded Codex routes.

## Governing References

- `../../../contracts/013-interactive-session-access-policy.md`
- `../../../contracts/017-provider-owned-session-load-replay-and-host-containment.md`
- `../../../research/011-kimi-macos-app-sandbox-runtime-compatibility.md`
- `../../../research/012-local-harness-orchestrator-isolation-posture.md`
- `057-harness-process-filesystem-containment.md`

## Scope

- current evidence from T3 Code and maintained harness permission or sandbox
  surfaces
- explicit `AmbientHost`, `ProviderEnforced`, and `HostEnforced` records
- no harness-isolation posture for direct routes without a local harness
- generic ambient default; enforced isolation requires explicit selection
- ambient external-host network evidence without a denied-network claim
- immutable preflight binding and no posture fallback
- reclassify Gemini ACP and attached OpenCode as ambient
- retain Codex read-only and bounded-workspace provider-enforced mappings
- roadmap 018 and Kimi card rebaseline

## Out Of Scope

- Kimi production driver
- new containment mechanism
- consumer UI or risk-copy policy
- Nucleus or Soundcheck changes

## Acceptance Criteria

- [x] harness communication does not require a process sandbox
- [x] ambient execution cannot claim a bounded filesystem
- [x] provider and host enforcement remain separate exact postures
- [x] direct inference does not receive a fake harness posture
- [x] Gemini and OpenCode no longer claim working-resource containment
- [x] Codex bounded profiles retain their exact provider sandbox mapping
- [x] focused records, driver, and profile tests pass
- [x] docs, Northstar, formatting, lint, compile, and diff checks pass

## Validation

- focused core, testkit, Gemini, OpenCode, and Codex tests
- `effigy check:rust`
- `effigy lint:rust`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy format:check`
- `git diff --check`

## Validation Result

- focused core, testkit, Codex, Gemini, and OpenCode tests pass
- full repository QA passes with 262 tests; installed/live probes remain gated
- warnings-denied clippy, all-target compile, formatting, docs, Northstar, and
  diff checks pass
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  7 errors

## Stop Conditions

- ambient execution is disguised as bounded access
- existing Codex isolation weakens
- provider permissions or callback scope are mislabeled as process isolation

## Auto-Continuation

No. Card 058 is the sole next task.
