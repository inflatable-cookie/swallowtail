# g04.076 Cursor Headless Provider Sandbox

Status: stopped after evidence
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Depends on: g04.035; per-route feature completion programme
Vision tags: harness isolation, native sandbox, route-local controls
Contract refs: 011, 023, 029, 033, 037, 040, 052
Research: 075, 077, 087, 135, 183, 223

## Problem

Production route `cursor-agent.headless` always prepares
`HarnessIsolation::AmbientHost` and omits Cursor's official `--sandbox` flag.
Every qualified exact Cursor build parses `--sandbox enabled|disabled`, and
current official documentation describes native filesystem, network, and
subprocess restrictions. Swallowtail exposes no provider-enforced sandbox
profile on this route.

The flag is a credible route-local lead, not containment proof. Cursor applies
the sandbox to supported terminal commands, can route unsupported commands to
approval, and reads user/project sandbox and network configuration. Exact
evidence must prove whether any qualified build, platform, operation profile,
and invocation form has a closed boundary that satisfies Contract 023 without
ambient configuration authority or a provider prompt.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind
`HarnessIsolation::ProviderEnforced` on `cursor-agent.headless` through
canonical `--sandbox enabled`. Preserve omission as exact `AmbientHost` with no
sandbox argument, keep resource access and Plan mode independent, and retain
the current model parameters, process lifecycle, durable provider-state truth,
and fail-closed preflight.

## Goals

- [x] freeze exact parser, precedence, platform, backend, filesystem, network,
      subprocess, escape, approval, configuration, output, and failure truth
      for the four qualified Cursor builds
- [x] distinguish requested, planned, argv-dispatched, parser-accepted,
      backend-active, enforced, effective, and observed sandbox state
- [x] classify `Read` and `ReadWrite` profiles independently and separate
      native isolation from Plan mode, permissions, tools, and workspace trust
- [x] promote Research 223 with an exact deliver-now table or honest empty set
- [ ] conditionally bind only admitted rows through prepared input, immutable
      plan/evidence, driver validation, and canonical argv
- [x] prove omission retains exact no-flag `AmbientHost` behavior
- [x] preserve exact model/parameter binding, activity, usage, cancellation,
      deadline, retention, terminal, and joined cleanup

## Non-Goals

- `--sandbox disabled`, raw sandbox strings, generic provider configuration,
  `sandbox.json` editing, network-mode selection, extra path policy, or host
  sandbox construction
- treating Plan, `--trust`, approvals, tool restrictions, `.cursorignore`, or
  resource access as process containment
- `--force`, `--yolo`, `--auto-review`, permission-policy selection, approval
  callbacks, MCP/plugin work, cloud workers, or worktree mode
- Cursor ACP/catalogue, another Cursor feature, model-parameter changes,
  currentness, install, login, account inspection, provider prompt, tool
  execution, paid work, release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `cursor-agent.headless`, driver
`swallowtail.cursor-agent.headless`, axis `cursor-agent.release-date`, exact
qualified pairs `2026.07.01-41b2de7`, `2026.07.23-e383d2b`,
`2026.08.04-aaa8809`, and `2026.08.11-e8db854`, one explicit-model structured
run, delegated Cursor subscription access, one working resource, one host
deadline, and the exact platforms for which artifact and host facts can be
bound before spawn.

Card 210 must reuse the exact identities frozen by Research 077, 087, and 135.
Current official Cursor documentation may corroborate current semantics but
cannot backport them to an exact build. Trace `--sandbox enabled` through argv
parsing, configuration precedence, backend selection, command classification,
filesystem/network policy, subprocess-tree application, approval or bypass
paths, activity/output, failure, and cleanup. Inventory every ambient user,
project, team, or hardcoded policy input that can weaken or widen the boundary.

The only candidate portable value is
`HarnessIsolation::ProviderEnforced`. Preparation may expose it only when
Research 223 proves one exact route/build/platform/profile row and the
immutable plan can bind every decisive platform and configuration fact before
process work. A flag, environment marker, backend name, successful command, or
official product label alone is insufficient.

Resource access remains independent. `Read` continues to select
`--mode plan`; `ReadWrite` continues to omit mode. An admitted sandbox profile
may add only `--sandbox enabled`. It cannot widen a working resource, grant
tools, approve an escape, or turn provider sandboxing into host-enforced
isolation.

Omission must preserve the exact current argv with no `--sandbox` token,
`HarnessIsolation::AmbientHost`, and ambient configuration truth. Explicit
`disabled`, provider defaults, persistent settings, or automatic fallback are
not substitutes for omission.

## Execution Plan

### Batch 76.1 — Exact Cursor Sandbox Evidence

- [x] Execute card 210.
- [x] freeze exact build/platform/configuration and native-boundary truth
- [x] promote Research 223 with a non-empty exact table or honest empty set

### Batch 76.2 — Conditional Provider-Enforced Binding

- [ ] Execute card 211 only when Research 223 admits a non-empty set.
- [ ] bind only admitted `ProviderEnforced` rows through immutable preparation
      and canonical `--sandbox enabled`

### Batch 76.3 — Route-Local Acceptance

- [ ] Execute card 212 only after card 211.
- [ ] prove dispatch, omission, rejection, access/mode composition, and
      unchanged lifecycle truth

## Acceptance Criteria

- [x] only Research 223 deliver-now rows prepare
- [ ] request, immutable plan, prepared evidence, driver, platform facts, and
      argv agree on the isolation posture
- [x] exact evidence proves the native boundary and every escape or approval
      path required by Contract 023
- [x] omission preserves exact no-flag `AmbientHost` behavior
- [ ] unsupported, mismatched, configurable, unavailable, fallback, or
      behaviorally weaker rows reject before process work when knowable
- [x] resource access, Plan mode, workspace trust, permissions, tools, model
      parameters, activity, terminal, retention, and cleanup remain separate
- [x] default QA performs no install, login, account inspection, provider
      prompt, tool execution, paid work, ambient config mutation, or live run

## Lane Runway

- predecessor: g04.075 Qwen headless Plan delivery
- this milestone: exact Cursor headless sandbox evidence and conditional
  provider-enforced binding
- execution topology: one serial worker lane, cards 210-212
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact artifacts cannot prove the full native boundary for any named
  build/platform/profile row.
- Stop if user, project, team, network, path, approval, or backend state can
  widen the boundary without an immutable preflight-bound fact.
- Stop if unsupported commands can execute outside the sandbox, an approval
  can be silently granted in print mode, or the route cannot observe and fail
  closed on backend unavailability.
- Stop if deterministic proof needs login, account inspection, provider
  prompting, tool execution, paid work, ambient config mutation, or a live
  model run.
- Stop if delivery needs raw configuration, host-isolation work, sibling-route
  changes, a shared contract change, currentness movement, or a breaking API.

## Batch Cards

- [210-cursor-headless-provider-sandbox-evidence.md](batch-cards/210-cursor-headless-provider-sandbox-evidence.md)
- [211-cursor-headless-provider-sandbox-binding.md](batch-cards/211-cursor-headless-provider-sandbox-binding.md)
- [212-cursor-headless-provider-sandbox-acceptance.md](batch-cards/212-cursor-headless-provider-sandbox-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 075 Cursor And Antigravity Refocus](../../research/075-cursor-and-antigravity-installed-harness-refocus.md)
- [Research 077 Cursor Headless Qualification](../../research/077-cursor-headless-installed-source-qualification.md)
- [Research 087 Cursor 2026.07.23 Checkpoint](../../research/087-cursor-agent-2026-07-23-range-checkpoint.md)
- [Research 135 Cursor 2026.08 Builds](../../research/135-cursor-agent-2026-08-04-2026-08-11-identity.md)
- [Research 183 Cursor Model Parameters](../../research/183-cursor-headless-model-parameter-evidence.md)
- [Research 223 Cursor Headless Provider Sandbox](../../research/223-cursor-headless-provider-sandbox-evidence.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Cursor Prepared Integration](../../guides/cursor-prepared-integration.md)
