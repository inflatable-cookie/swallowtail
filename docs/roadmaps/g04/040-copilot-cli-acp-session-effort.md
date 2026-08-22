# g04.040 Copilot CLI ACP Session Effort

Status: ready
Owner: Tom
Created: 2026-08-22
Depends on: per-route feature completion programme
Vision tags: explicit selection, provider truth, route-local controls
Contract refs: 011, 020, 029, 037, 040, 041, 052
Research: 049, 149, 159; 188 to be produced by card 110

## Problem

`copilot-cli.acp` owns one bounded ACP stdio child and one interactive session,
but its spawn path deliberately omits the official server-start effort flags.
Consumers cannot request an initial reasoning effort through the prepared
session even though the route's process lifetime already bounds the setting.

Current official ACP-server documentation names `--effort` and
`--reasoning-effort` with `low`, `medium`, `high`, `xhigh`, and `max`. It also
says every session opened against that server inherits the startup setting and
`session/new` cannot change it. That current documentation is a lead, not proof
for the route's exact qualified package `1.0.80`. The exact package, syntax,
value semantics, and no-model-route Contract 040 fit must be frozen before
binding.

## Generation Runway Goal

Deliver the next route-local control family from the per-route feature
programme: one exact preparation-fixed Copilot CLI ACP reasoning selection on
the existing one-child/one-session route, if Research 188 proves it.

## Goals

- [ ] freeze current official ACP-server documentation and exact `1.0.80`
      package evidence for effort flags, syntax, values, scope, and defaults
- [ ] decide whether the interface-level session effort can map to portable
      `ReasoningSelection` without selected-model inference or value clamping
- [ ] classify the canonical `--effort` form and its
      `--reasoning-effort` alias without exposing raw argv choice
- [ ] bind only Research 188 deliver-now values through prepared input,
      immutable plan/evidence, driver configuration, and child argv
- [ ] keep one selection fixed for the owned process, every prompt, and fresh
      context-losing replacement
- [ ] preserve exact `copilot --acp --stdio` behavior when effort is absent
- [ ] reject unsupported values and request/plan/evidence/driver drift before
      process work
- [ ] publish dispatch truth without claiming provider acceptance or effective
      reasoning depth
- [ ] leave shared architecture, matrices, changelog, programme, indexes, and
      roadmap-front-door deltas for orchestrator closeout after merge

## Non-Goals

- server-start `--available-tools` or `--excluded-tools`
- `--yolo`, `--allow-all`, permission broadening, or ambient approval
- TCP `--port`, Copilot IDE/API routes, login, account inspection, or BYOK
- ACP `session/new` reasoning mutation, per-turn overrides, or model selection
- session load/resume/close, provider-session management, or durable recovery
- usage, output-token limits, structured output, attachments, callbacks, or
  another feature family
- a compatibility-ceiling change, live provider prompt, release, or publication

## Named Scope

The milestone is restricted to the existing `copilot-cli.acp` route, exact
`copilot-cli.package` `1.0.80`, ACP v1 stdio, host-account access profile, and
one bounded prepared interactive session.

Card 110 must freeze official current documentation plus secret-free exact
`1.0.80` package/help/source specimens. It must distinguish startup dispatch,
session inheritance, provider acceptance, and effective reasoning. The five
named values are candidates only until Research 188 classifies them.

The route does not select a model. Research 188 must decide whether the
official interface-level session setting is sufficient for exact portable
reasoning dispatch under Contract 040. It must stop rather than infer model
capability, accept upstream clamping, or turn an interface default into a
selected value.

Because Swallowtail owns one server child for one prepared session, an admitted
startup effort may be preparation-fixed for the entire session. Fresh
context-losing replacement must spawn a new child with the same prepared value.

## Execution Plan

### Batch 40.1 — Exact Package And Contract Evidence

- [ ] Execute card 110.
- [ ] freeze official and exact `1.0.80` effort specimens and digests
- [ ] promote Research 188 with value, syntax, profile, lifetime, and claim
      dispositions

### Batch 40.2 — Prepared Session Binding

- [ ] Execute card 111 only after card 110 admits a useful exact subset.
- [ ] bind optional portable reasoning through input, plan, evidence, request,
      driver, and canonical argv
- [ ] preserve the absent path and reject drift before process work

### Batch 40.3 — Dispatch And Acceptance

- [ ] Execute card 112 only after card 111.
- [ ] prove startup argv, first/later prompt inheritance, fresh replacement,
      failures, and unchanged ACP lifecycle
- [ ] update route-local guidance and report the deferred shared closeout delta

## Acceptance Criteria

- [ ] only Research 188 deliver-now values prepare
- [ ] request, plan constraint, prepared evidence, driver, and child argv agree
      exactly
- [ ] one selection spans the owned child/session and fresh replacement
- [ ] absent effort retains current argv and public behavior
- [ ] no alias, raw string, clamp, default substitution, or model inference
      enters the public mapping
- [ ] known failures occur before process or provider work
- [ ] deterministic QA uses no login, credential, account, or prompt
- [ ] docs stop at the exact evidence state proved

## Lane Runway

- predecessors: g04.035-039 initial per-route feature sequence
- this milestone: Copilot CLI ACP startup/session effort
- execution topology: one serial worker lane, cards 110-112
- next route family: selected by the orchestrator after merge from the remaining
  promoted inventory

## Decision Gates

- Stop if exact `1.0.80` does not expose the documented effort surface.
- Stop if values are clamped, substituted, or model-dependent without an exact
  supported subset.
- Stop if a model route or new portable contract is required.
- Stop if the setting cannot remain immutable across the owned session and
  fresh replacement.
- Stop if adding the control requires unresolved facade/version segmentation or
  a breaking public change.
- Stop before promoting tool filters, permissions, TCP, or another Copilot
  surface.

## Batch Cards

- [110-copilot-cli-acp-effort-evidence.md](batch-cards/110-copilot-cli-acp-effort-evidence.md) — ready
- [111-copilot-cli-acp-effort-binding.md](batch-cards/111-copilot-cli-acp-effort-binding.md) — ready after 110
- [112-copilot-cli-acp-effort-acceptance.md](batch-cards/112-copilot-cli-acp-effort-acceptance.md) — ready after 111

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application And Enforcement](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 041 Input Callback And Provider Tool Admission](../../contracts/041-input-callback-and-provider-tool-admission.md)
- [Copilot CLI ACP Prepared Integration](../../guides/copilot-cli-acp-prepared-integration.md)
- [GitHub Copilot CLI ACP Server](https://docs.github.com/en/copilot/reference/copilot-cli-reference/acp-server)
