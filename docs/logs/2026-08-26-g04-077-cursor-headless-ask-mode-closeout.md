# 2026-08-26 g04.077 Cursor Headless Ask Mode Closeout

Status: stopped after evidence
Owner: Tom
Milestone: g04.077
Cards: 213 complete; 214-215 blocked
Branch: `t3code/review-cursor-ask-mode-handoff`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-dcac6f7e`
Base: `4319e7ce1a7b5154226c5ffb405f6b2db079f38f` (`origin/main` at dispatch)
Planning base ancestor: `c12eeaf3ac041d66b31bd4cd26dd569efc1e6efd`
PR: pending

## Result

Card 213 completed an exact four-build artifact, parser, precedence,
application, read-only, observation, model-parameter, and production-seam
audit. Research 224 admits no deliver-now Cursor headless Ask row. Cards 214
and 215 are blocked and were not executed. The Cursor adapter, prepared
facade, child argv, fixtures, guide, matrices, and API baseline are unchanged.
No install, host-binary replacement, login, account inspection, catalogue,
provider prompt, tool execution, paid work, ambient config mutation, or live
model run was used.

## Evidence Stop

Selection truth is strong. All four qualified builds register
`--mode <mode>` with commander `.choices(["plan","ask"])` and no default.
Isolated parser cases are identical on every binary: `ask`, `plan`,
`--mode=ask`, `--mode=plan`, repeats, and every placement around the exact
production argv parse; `agent`, `ASK`, `Ask`, empty, `--help`-as-value, and
`ask,plan` reject. Persisted configuration holds no agent-mode key, no
environment variable selects the mode, and a headless session without
`--resume`/`--continue` starts fresh with no inherited mode metadata. Headless
refuses model-initiated switch-mode requests, so the value is immutable for
the run. `--plan` beats `--mode ask` in `chat.ts`.

Behavioral truth is not. Exact `run-agent.tsx` stores Ask as agent-store
metadata `"search"`, and `headless.ts` attaches `AgentMode.ASK` to the
outbound `UserMessage`. That is the whole application path. `getIsAskMode`
has one consumer: `shared/resources.ts` picks `workspace_readonly` instead of
`workspace_readwrite` for the shell-exec sandbox policy, gated on
`sandboxFeatureGateEnabled && isSandboxSupported() && "enabled" === resolved
sandbox mode`. Swallowtail sends no `--sandbox` and both the default and host
configs hold `sandbox.mode: "disabled"`, so the policy is `insecure_none` and
Ask has no local effect on this route. Where it would have one, ambient
`cli-config.json`, project state, team overlays, and feature gates control it.

No tool registry, approval path, or write refusal keys on Ask; tool exclusion
is the separate `--exclude-tools` mechanism. The qualified stream emits a
constant `permissionMode: "default"` and no mode field, so requested,
selected, applied, and effective mode are unobservable.

Every Research 183 deliver-now model tuple parses with `--mode ask` on all
four builds; `--mode` and `--model` stay independent.

Current production preparation is unchanged: `Read` dispatches exactly one
`--mode plan`, `ReadWrite` omits `--mode`, and the guide claims dispatch only.
Binding Ask at the surviving evidence tier would add a second read-mode token
whose only proved difference from Plan is the enum the backend receives.

## Changed Surfaces

- `docs/research/224-cursor-headless-ask-mode-evidence.md`: promoted exact
  identity, parser, precedence, application, read-only, observation,
  production audit, claim-strength, and empty deliver-now table
- cards 213-215, g04.077, programme, triage, indexes, Next Task, this closeout

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

No production code changed. Doctor was run once at dispatch and matches the
inherited baseline: `scan.god-files` 380 findings (334 warnings, 46 errors)
and `scan.generated-in-src` one warning. Docs-only edits do not move it.

## Continuation

Keep g04 open. No ready lane remains. Reassess the remaining per-route feature
inventory for the next serial lane unless the operator supplies a different
direction. Contract 029 currentness remains standing. Do not compile the next
family from this closeout.
