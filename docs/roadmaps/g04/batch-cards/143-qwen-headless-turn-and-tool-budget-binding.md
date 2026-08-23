# 143 Qwen Headless Turn And Tool Budget Binding

Status: conditional
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.051 Qwen Headless Turn And Tool Budgets](../051-qwen-headless-turn-and-tool-budgets.md)
Depends on: card 142; promoted Research 198 with a non-empty deliver-now set

## Goal

Bind only Research 198's exact Qwen caller-decreasing budget subset through
typed adapter-local input, immutable prepared state, driver validation, and
every admitted child command.

## Scope

1. Add the smallest typed adapter-local selection admitted by Research 198.
   Preserve current constructors and omission behavior.
2. Keep the control inside `swallowtail-adapter-qwen`. Do not add a shared
   `Capability`, portable generation control, generic provider-settings map, or
   sibling-route behavior.
3. Bind selected values through configured input, immutable plan/evidence,
   driver state, and command construction. Reject input/plan/evidence/driver/
   version/profile drift before process start or user prompt.
4. Emit only exact Research 198 values on exact package `0.21.15`. Omission
   must preserve current `--max-session-turns 24 --max-tool-calls 16` bytes.
5. Apply one selected pair consistently to structured-run children and every
   first, resumed, and fresh replacement session child admitted by Research
   198. Do not imply operation-wide counters if Qwen resets them per child.
6. Preserve exact ordinary text-stdin behavior and the existing
   initialize/set-effort-before-user reasoning transport. Budget binding must
   not alter model qualification, resume, prompt ordering, or replacement.
7. Preserve the native 60-second wall bound, mandatory host deadline, safe
   mode, read-only allowlist, excluded tools, credentials, environment,
   cancellation, failure, and cleanup.
8. Map over-budget terminal behavior only to Research 198's proved boundary.
   Do not convert stderr or an exit code into a stronger semantic claim.
9. Advance only exact feature-local revisions selected by Research 198.
   Preserve prior evidence as historical proof and do not widen currentness.

## Acceptance Criteria

- [ ] only Research 198 deliver-now values and profiles prepare
- [ ] selection, plan/evidence, driver, and every child argv agree exactly
- [ ] omission remains byte- and behavior-stable at `24` / `16`
- [ ] ordinary and reasoning-selected run/session behavior is unchanged
- [ ] invalid values, aliases, unlimited values, raised bounds, and knowable
      mismatches reject before effects
- [ ] wall time, host deadline, tool set, approval posture, credentials, model,
      resume, cancellation, and cleanup remain unchanged
- [ ] no shared runtime, portable capability, sibling route, retry, fallback,
      provider-acceptance, quality, latency, or billing claim enters the API

## Validation

```sh
cargo fmt -p swallowtail-adapter-qwen
effigy validate:focused swallowtail-adapter-qwen
effigy package:verify-affected swallowtail-adapter-qwen
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 144 when exact preparation, command, composition,
terminal, rejection, and lifecycle preservation pass.

## Stop Conditions

- implementation needs a shared capability, contract/currentness change, or
  breaking public API
- admitted values cannot remain exact across input, plan/evidence, driver, and
  every child command
- terminal truth, omission, reasoning transport, or a fixed safety boundary
  changes

## Out Of Scope

- route guide, shared closeout, live provider work, another version/profile,
  wall-time selection, release, publication, or merge

