# g04.042 Cline Thinking Controls

Status: ready
Owner: Tom
Created: 2026-08-22
Depends on: per-route feature completion programme; g03.086-087
Vision tags: explicit selection, provider truth, route-local controls
Contract refs: 011, 020, 029, 033, 037, 040, 050, 052
Research: 143, 146, 147, 159, 190

## Problem

`cline.acp` and `cline.headless` share the exact `cline.package` `3.0.55`
axis, but they are distinct transports and operation shapes. Current official
Cline documentation names `--thinking none|low|medium|high|xhigh` on the CLI.
Swallowtail passes no thinking argument on either route and therefore rejects
portable reasoning selection.

The documentation is not enough to bind a portable control. Current official
pages disagree about omission and default behavior, the exact qualified
package may normalize values or depend on the selected provider/model, and the
ACP flag is fixed at child spawn while headless selection is per run. Exact
package evidence must settle those boundaries independently.

## Generation Runway Goal

Qualify and, only where exact evidence permits, bind Cline thinking controls
on `cline.acp` and `cline.headless` without flattening their transport,
lifetime, model, default, or acceptance truth.

## Goals

- [ ] freeze current official documentation and exact `3.0.55` package source
      for syntax, omission, defaults, normalization, model/provider support,
      and precedence
- [ ] classify ACP spawn and headless run transport independently
- [ ] classify `none`, `low`, `medium`, `high`, and `xhigh` independently,
      including whether `none` can map exactly to portable `off`
- [ ] reject aliases, nearest-value mapping, model defaults, provider/model
      substitution, and inferred support
- [ ] decide whether either transport needs a private Contract 029 behavior
      revision at exact `3.0.55`
- [ ] bind only Research 190 deliver-now route/value rows through typed input,
      immutable plan/evidence, request/session policy, driver, and argv
- [ ] preserve exact existing commands and behavior when thinking is absent
- [ ] repeat an admitted ACP selection on fresh context-losing replacement
      and keep a headless selection local to its one run
- [ ] publish only requested, planned, dispatched, accepted, effective, and
      observed truth proved by the exact surface

## Non-Goals

- Cline model selection, plan mode, timeout, retries, config roots, or data dirs
- `--auto-approve true`, YOLO, permission bypass, teams, or background hub work
- generic argv, environment, settings, or generation-parameter maps
- ACP session load/resume, headless provider-session continuity, or sibling
  Cline-family routes
- compatibility-currentness changes, install, login, prompt, or live provider
  work

## Named Scope

The milestone is restricted to production routes `cline.acp` and
`cline.headless`, axis `cline.package`, exact qualified point `3.0.55`, and
their existing `cline.acp.stdio-v1` and `cline.headless.stdio-json-v1`
behaviors. Sharing one package and executable does not share a capability
claim.

Research 190 must freeze the exact `3.0.55` parse and application paths. It
must resolve the conflict between current official descriptions of omission,
determine whether the package treats `--thinking` as optional or defaulted,
and prove whether each selected value survives provider/model resolution
without normalization or substitution. If the exact route selects no model
and all values are model-entitled, that route stops after card 116.

The portable mapping may use only canonical `ReasoningMode` values with exact
semantics. `none` is an upstream value, not automatic proof that portable
`off` is exact. `minimal` and `max` are not documented Cline values and remain
outside the candidate set. Omission retains the existing Swallowtail path; it
is not a selected reasoning value.

## Execution Plan

### Batch 42.1 — Exact Package And Two-Transport Evidence

- [ ] Execute card 116.
- [ ] freeze current official and exact `3.0.55` specimens and digests
- [ ] promote Research 190 with separate route/value, omission, model,
      normalization, version, and lifetime dispositions

### Batch 42.2 — Conditional Prepared Binding

- [ ] Execute card 117 only if card 116 admits at least one deliver-now row.
- [ ] bind only those exact rows through the owning route's prepared and
      low-level surfaces
- [ ] preserve absent argv exactly and reject plan/evidence/driver drift before
      process work

### Batch 42.3 — Route-Local Acceptance

- [ ] Execute card 118 only after card 117.
- [ ] prove each admitted route/value row, ACP replacement, headless run scope,
      absent behavior, and fail-closed rejection
- [ ] update both route guides and report the deferred shared closeout delta

## Acceptance Criteria

- [ ] ACP and headless dispositions are explicit and independent
- [ ] only Research 190 deliver-now rows prepare
- [ ] portable mode, plan constraint, prepared evidence, request/session
      policy, configured driver, and exact argv agree
- [ ] no alias, clamp, default, provider/model inference, or sibling-route
      promotion enters the mapping
- [ ] absent thinking retains current `cline --acp` and
      `cline --json --auto-approve false ...` behavior
- [ ] every known mismatch fails before provider work at the earliest boundary
      the exact process surface permits
- [ ] deterministic QA uses no install, login, credential, account, or prompt
- [ ] docs do not infer provider-effective reasoning from CLI acceptance or
      emitted reasoning text

An empty Research 190 deliver-now set, or a set for only one transport, is an
honest result. It does not justify weakening Contract 040 or copying one
route's claim onto the other.

## Lane Runway

- predecessor: g04.041 Qwen headless reasoning effort
- this milestone: Cline ACP and headless thinking evidence plus conditional
  route-local binding
- execution topology: one serial worker lane, cards 116-118
- next route family: selected by the orchestrator after evidence and merge
  closeout; no later family is precompiled here

## Decision Gates

- Stop a route if exact `3.0.55` lacks or contradicts the documented value
  surface.
- Stop a route if every candidate depends on an unselected provider/model,
  clamps, aliases, substitutes a default, or is silently ignored.
- Stop if `none` cannot map exactly to portable `off`; do not translate by
  label resemblance.
- Stop if ACP spawn state can change underneath the prepared child or cannot
  be repeated on fresh replacement.
- Stop if headless selection leaks through ambient or persisted configuration
  instead of the one run argv.
- Stop if support needs config mutation, a synthetic config root, a generic
  parameter map, a contract change, or a breaking public lifecycle change.

## Batch Cards

- [116-cline-thinking-control-evidence.md](batch-cards/116-cline-thinking-control-evidence.md) — ready
- [117-cline-thinking-control-binding.md](batch-cards/117-cline-thinking-control-binding.md) — conditional
- [118-cline-thinking-control-acceptance.md](batch-cards/118-cline-thinking-control-acceptance.md) — conditional

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 146 Cline ACP Identity](../../research/146-cline-acp-3-0-55-identity.md)
- [Research 147 Cline Headless Identity](../../research/147-cline-headless-3-0-55-identity.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 050 Working-State Restoration](../../contracts/050-working-state-restoration-facade.md)
- [Cline ACP Prepared Integration](../../guides/cline-acp-prepared-integration.md)
- [Cline Headless Prepared Integration](../../guides/cline-headless-prepared-integration.md)
- [Cline CLI Reference](https://docs.cline.bot/cli/cli-reference)
- [Cline ACP](https://docs.cline.bot/usage/acp)
