# g04.072 Grok Build ACP Subagents Disabled

Status: stopped after evidence
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Depends on: g04.057; per-route feature completion programme
Vision tags: explicit selection, provider truth, bounded child topology
Contract refs: 009, 011, 023, 029, 033, 037, 044, 045, 052
Research: 070, 130, 163, 204, 219

## Problem

Production route `grok-build.acp` owns one exact Grok Build ACP stdio child but
always launches `grok --no-auto-update agent stdio`. Consumers cannot request
the official launch-time `--no-subagents` restriction for the child lifetime.

Exact installed `1.0.5` help accepts `--no-subagents` as a global option before
`agent stdio`; the ACP subcommand exposes no protocol option for the same
control. Parser acceptance is only a lead. Exact package/source evidence must
still prove where the flag is applied, whether it suppresses every subagent
spawn path for ACP sessions, and whether ambient configuration or later
session metadata can override it.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind one adapter-local
subagents-disabled launch profile on the existing Grok Build ACP route. Keep
this as a provider-native topology restriction. Do not turn it into child
observation, direct child control, permission, sandbox, or process-containment
authority.

## Goals

- [x] freeze exact maintained-package parser, configuration, agent-construction,
      subagent-tool, ACP-session, override, and failure truth
- [x] determine whether `--no-subagents` is effective and immutable across the
      complete owned child lifetime without a provider prompt
- [x] distinguish requested restriction, argv dispatch, parser acceptance,
      configuration application, tool removal, attempted spawn, and effect
- [x] promote Research 219 with an exact deliver-now table or honest empty set
- [ ] conditionally expose only the adapter-local disabled profile admitted by
      Research 219
- [ ] bind the selection through prepared input, immutable evidence, driver,
      exact argv, and fresh replacement
- [x] preserve current argv and behavior when the selection is absent
- [x] preserve existing ACP activity, permission, cancellation, failure, and
      joined-cleanup truth

## Non-Goals

- enabling subagents, selecting agents, `--agents` JSON, task delegation,
  teammate control, child messages, targeted interruption, or child history
- claiming child identity, parentage, lifecycle, attribution, collaboration
  actions, or direct operator control under Contract 045
- `--disable-web-search`, `--tools`, `--disallowed-tools`, allow/deny rules,
  effort, max turns, plan mode, model changes, or structured output
- `--always-approve`, permission bypass, sandbox widening, filesystem/network
  restriction, descendant-process containment, or read-only claims
- account inspection, login, credential capture, provider prompt, paid work,
  currentness, release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `grok-build.acp`, driver
`swallowtail.grok-build.acp`, axis `grok-build.executable`, maintained exact
packages `1.0.4..=1.0.5`, model `grok-4.6`, ACP v1 stdio, delegated
subscription access, and the route's existing structured-run and interactive-
session shapes.

Card 198 must inspect exact official package artifacts and exact-version source
when available. Current public source or documentation may corroborate, but it
cannot replace exact `1.0.4`/`1.0.5` truth. Freeze global-option placement,
duplicates, aliases, environment/config precedence, default state, and local
parse failures. Trace the selected value into the ACP agent, every subagent
registration and spawn seam, new sessions, operation-private sessions, later
prompts, attachment recovery, and fresh replacement.

The only candidate public shape is an adapter-local disabled selection. It may
be a closed enum or a named builder selected by Research 219. There is no raw
boolean, string, generic subagent map, portable `Capability`, or explicit
enabled value. Omission must retain exact current argv and behavior. An
admitted disabled selection is preparation-fixed for the owned child and every
session attached to it.

`--no-subagents` is provider-native behavior under Contract 023. Even when
effective, it proves only the exact provider spawn restriction. It does not
prove OS descendant-process containment, remove ordinary process tools, grant
or answer permissions, or create a portable child-control role. Contract 045
observation and operator-control claims stay unchanged.

## Execution Plan

### Batch 72.1 — Exact Disabled-Topology Evidence

- [x] Execute card 198.
- [x] freeze exact package, parser, configuration, subagent registration/spawn,
      lifecycle, override, and failure truth
- [x] promote Research 219 with a non-empty exact table or honest empty set

### Batch 72.2 — Conditional Adapter-Local Binding

- [ ] Execute card 199 only when Research 219 admits a non-empty set.
- [ ] bind only exact version/profile/lifecycle rows through prepared evidence
      and child argv

### Batch 72.3 — Route-Local Acceptance

- [ ] Execute card 200 only after card 199.
- [ ] prove dispatch, omission, rejection, replacement, topology separation,
      lifecycle, docs, and API truth

## Acceptance Criteria

- [ ] only Research 219 deliver-now rows prepare
- [ ] input, plan/evidence, driver, and child argv agree exactly
- [ ] omission preserves prior argv and route behavior
- [ ] unsupported, mismatched, drifting, and overrideable rows reject before
      process work when knowable
- [ ] disabled subagents never becomes observation, direct control,
      permission, sandbox, filesystem/network, or process-containment authority
- [ ] existing initialization, session, prompt, activity, permission,
      cancellation, deadline, failure, replacement, and cleanup truth remains
      exact
- [ ] default QA performs no install, login, account inspection, provider
      prompt, tool execution, or paid request

## Lane Runway

- predecessor: g04.071 Copilot CLI ACP built-in-tool allowlist evidence stop
- this milestone: Grok Build ACP launch-time subagents-disabled evidence and
  conditional binding
- execution topology: one serial worker lane, cards 198-200
- generation boundary: g04 remains open; no closure or rollover is authorized
- stop: card 198 promoted Research 219 as an empty set; cards 199-200 blocked;
  current `grok --no-auto-update agent stdio` argv retained

## Decision Gates

- Stop if exact `1.0.4`/`1.0.5` package/source cannot prove the flag reaches
  every ACP subagent registration and spawn path.
- Stop if the flag is TUI/headless-only, parser-only, advisory, prompt-based,
  overridden by ambient config/session metadata, or ineffective after startup.
- Stop if proving effectiveness requires a provider prompt, account inspection,
  subagent work, arbitrary tool execution, or paid inference.
- Stop if delivery needs raw flags, explicit enabling, generic topology
  controls, Contract 045 expansion, sibling-route changes, currentness movement,
  or a breaking lifecycle.

## Batch Cards

- [198-grok-build-acp-subagents-disabled-evidence.md](batch-cards/198-grok-build-acp-subagents-disabled-evidence.md)
- [199-grok-build-acp-subagents-disabled-binding.md](batch-cards/199-grok-build-acp-subagents-disabled-binding.md)
- [200-grok-build-acp-subagents-disabled-acceptance.md](batch-cards/200-grok-build-acp-subagents-disabled-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 070 Grok Build Authenticated ACP Qualification](../../research/070-grok-build-0-2-114-authenticated-acp-qualification.md)
- [Research 130 Grok 1.0.4 Milestone Handshake](../../research/130-grok-1-0-4-milestone-handshake.md)
- [Research 163 Grok 1.0.5 Identity](../../research/163-grok-1-0-5-identity.md)
- [Research 204 Grok Build ACP Reasoning Selection](../../research/204-grok-build-acp-reasoning-selection-evidence.md)
- [Research 219 Grok Build ACP Subagents Disabled](../../research/219-grok-build-acp-subagents-disabled-evidence.md) — promoted empty deliver-now set
- [Contract 023 Harness Operation Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 045 Subagent Topology](../../contracts/045-subagent-topology-observation-and-control.md)
- [Grok Build Prepared Integration](../../guides/grok-build-prepared-integration.md)
- [Grok Build CLI Reference](https://docs.x.ai/build/cli/reference)
