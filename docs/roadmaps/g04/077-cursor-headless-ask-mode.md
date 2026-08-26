# g04.077 Cursor Headless Ask Mode

Status: complete
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Depends on: g04.035; g04.076 closeout; per-route feature completion programme
Vision tags: explicit behavior, read-only exploration, route-local controls
Contract refs: 010, 011, 023, 029, 033, 034, 037, 040, 052
Research: 075, 077, 087, 135, 183, 223, 224

## Problem

Production route `cursor-agent.headless` maps `ResourceAccess::Read` to
canonical `--mode plan`. Every qualified exact Cursor build also exposes
`--mode ask` as its read-only exploration and Q&A posture, but Swallowtail
cannot select it. Consumers that need investigation rather than plan
generation must bypass typed preparation or accept the wrong provider
behavior.

Ask is not a portable `HarnessMode`. It is an adapter-local Cursor behavior
whose exact parser, precedence, read-only boundary, output truth, and
composition with existing Plan, access, model parameters, ambient
configuration, and one-child lifecycle must be qualified before binding.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind Cursor headless Ask as a
closed adapter-local read-mode selection on `cursor-agent.headless`. Preserve
the existing `Read` to `--mode plan` default, reject Ask with `ReadWrite`, and
retain exact model-parameter, configuration, isolation, working-resource,
retention, deadline, cancellation, activity, terminal, and cleanup truth.

## Goals

- [x] freeze exact parser, alias, placement, repetition, precedence,
      configuration, source, and output truth for `--mode ask`
- [x] prove the exact behavioral and read-only boundary independently from
      resource access, process isolation, permissions, tools, and trust
- [x] promote Research 224 with an exact deliver-now table or honest empty set
- [x] conditionally add one closed adapter-local Ask selection with no raw mode
      string or provider-neutral API widening
- [x] preserve omission as the exact current `Read` to `--mode plan` mapping
      and `ReadWrite` no-mode argv
- [x] prove Ask composes with every qualified Cursor headless model-parameter
      tuple without changing its support set
- [x] preserve exact one-child lifecycle and default-QA boundaries

## Non-Goals

- adding `Ask` to portable `HarnessMode`, inferring Ask from `ResourceAccess`,
  or creating a generic provider-mode string
- Cursor Agent mode, `--force`, `--yolo`, `--auto-review`, sandboxing,
  approvals, permission-policy selection, tool policy, or write authority
- claiming process containment, filesystem mediation, provider configuration
  suppression, or working-resource containment from Ask
- changing Cursor ACP/catalogue, model parameters, the Contract 029 ceiling,
  installed identity, or currentness
- login, account inspection, model catalogue, provider prompt, tool execution,
  paid work, release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `cursor-agent.headless`, driver
`swallowtail.cursor-agent.headless`, axis `cursor-agent.release-date`, and the
four exact qualified builds:

- `2026.07.01-41b2de7`
- `2026.07.23-e383d2b`
- `2026.08.04-aaa8809`
- `2026.08.11-e8db854`

Card 213 must reuse the exact identities frozen by Research 077, 087, and 135
and the model-parameter set from Research 183. Current official Cursor docs
may corroborate current mode vocabulary but cannot backport behavior to an
exact artifact. Freeze `--mode ask`, `--mode plan`, `--plan`, omission,
invalid/case/repeated values, option placement, and every configuration source
that can override or weaken the selected behavior.

The only candidate public control is a closed Cursor-local selection for Ask.
Existing construction remains Plan for `ResourceAccess::Read` and no mode for
`ResourceAccess::ReadWrite`. An explicit Ask selection must be immutable in
the prepared result and low-level driver binding, dispatch exactly one
`--mode ask`, and reject before process work when access, exact version,
prepared evidence, driver state, or command intent disagrees. No raw provider
value enters core or runtime.

Ask may be reported only at the evidence level the exact interface proves.
Requested, prepared, dispatched, parser-accepted, applied, effective, and
observed state remain distinct. A help label or successful parse is
insufficient to claim an effective read-only Q&A posture.

## Execution Plan

### Batch 77.1 — Exact Cursor Ask Evidence

- [x] Execute card 213.
- [x] freeze exact parser, precedence, configuration, behavioral, output, and
      access truth
- [x] promote Research 224 with a non-empty exact table or honest empty set

### Batch 77.2 — Conditional Adapter-Local Binding

- [x] Execute card 214 only when Research 224 admits a non-empty set.
- [x] bind only admitted Ask rows through typed preparation and canonical argv

### Batch 77.3 — Route-Local Acceptance

- [x] Execute card 215 only after card 214.
- [x] prove dispatch, defaults, rejection, model composition, and unchanged
      lifecycle truth

## Acceptance Criteria

- [x] only Research 224 deliver-now rows prepare Ask
- [x] Ask selection is closed, Cursor-local, immutable, and exactly dispatched
- [x] existing `Read` still dispatches `--mode plan`; `ReadWrite` still omits
      `--mode`; Ask plus `ReadWrite` rejects before process work
- [x] docs distinguish qualified dispatch from parser acceptance, application,
      effectiveness, and observation
- [x] access, working resource, isolation, configuration, tools, permissions,
      trust, retention, and lifecycle claims do not widen
- [x] all qualified model-parameter rows retain exact membership and argv
- [x] default QA performs no install, login, account inspection, provider
      prompt, tool execution, paid work, ambient config mutation, or live run

## Outcome

Complete through cards 213-215.

Research 224 admits four deliver-now rows: `--mode ask` with
`ResourceAccess::Read` on each exact qualified build, at qualified dispatch
and application only. Selection is exact, case-sensitive, closed to `plan` and
`ask`, has no persisted-config or environment competitor, and cannot be
switched mid-run in print mode.

`CursorHeadlessReadMode` binds it as one closed Cursor-local selection.
Omission preserves the exact defaults; `with_read_mode` rejects any selection
on `ReadWrite`; `prepare_run` rejects Ask on a release that is not exactly
qualified; the resolved mode is frozen on the prepared run and carried to the
low-level driver, which re-validates access and release before process work
and never falls back.

Effective and observed mode are withheld. Ask's only local consumer selects a
shell-exec sandbox policy type that is inert on this route's argv and
ambient-controlled otherwise, and the qualified stream reports no mode. Docs,
matrices, and tests claim qualified dispatch and application only, never a
locally enforced read-only boundary.

## Lane Runway

- predecessor: g04.076 Cursor headless provider-sandbox evidence stop
- this milestone: exact Cursor headless Ask evidence and conditional binding
- execution topology: one serial worker lane, cards 213-215
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact artifacts cannot prove a useful Ask behavior beyond parser
  acceptance or current mutable documentation.
- Stop if Ask can be overridden, widened, or switched by ambient state that
  cannot be bound or rejected before process work.
- Stop if ReadWrite can execute under Ask, Ask grants write or approval
  authority, or exact source does not establish the claimed read-only boundary.
- Stop if deterministic proof needs login, account inspection, provider
  prompting, tool execution, paid work, ambient config mutation, or a live
  model run.
- Stop if delivery needs a portable `HarnessMode` change, raw configuration,
  shared contract/runtime work, sibling-route changes, currentness movement,
  or a breaking API.

## Batch Cards

- [213-cursor-headless-ask-mode-evidence.md](batch-cards/213-cursor-headless-ask-mode-evidence.md)
- [214-cursor-headless-ask-mode-binding.md](batch-cards/214-cursor-headless-ask-mode-binding.md)
- [215-cursor-headless-ask-mode-acceptance.md](batch-cards/215-cursor-headless-ask-mode-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 077 Cursor Headless Qualification](../../research/077-cursor-headless-installed-source-qualification.md)
- [Research 087 Cursor 2026.07.23 Checkpoint](../../research/087-cursor-agent-2026-07-23-range-checkpoint.md)
- [Research 135 Cursor 2026.08 Builds](../../research/135-cursor-agent-2026-08-04-2026-08-11-identity.md)
- [Research 183 Cursor Model Parameters](../../research/183-cursor-headless-model-parameter-evidence.md)
- [Research 224 Cursor Headless Ask Mode](../../research/224-cursor-headless-ask-mode-evidence.md)
- [Contract 010 Execution Host Services And Inputs](../../contracts/010-execution-host-services-and-inputs.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation Control](../../contracts/040-generation-control-application-and-enforcement.md)
- [Cursor Prepared Integration](../../guides/cursor-prepared-integration.md)
