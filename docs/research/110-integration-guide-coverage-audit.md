# 110 Integration Guide Coverage Audit

Status: promoted
Owner: Tom
Updated: 2026-08-05

## Question

Can another agent or operator integrate every production route and every
portable feature from Swallowtail's guides without reverse-engineering source?

## Current Inventory

The guide tree contains 31 files:

- 18 route-oriented prepared-integration guides
- one 33-route selection matrix
- one 26-solution, 34-feature matrix
- one solution activity matrix
- six consumer feature guides for activity, provider-session import,
  reconciliation, detachment, working-state restoration, and failures
- one realtime route-family guide
- two maintainer/operator guides for prepared-facade authoring and validation
- one guide index

The 18 route guides cover 26 of 33 production route IDs and 22 of 26 solution
rows. Seven route IDs have no canonical route guide:

- `antigravity.catalogue`
- `antigravity.headless`
- `cursor-agent.catalogue`
- `cursor-agent.acp`
- `cursor-agent.headless`
- `grok-build.acp`
- `oh-my-pi.rpc`

Thirty-one adapter examples cover 27 route IDs. Antigravity's two routes,
Cursor's three routes, and Grok's route have no normal-path example. Oh My Pi
has a compiling example but no route guide.

## Depth Finding

Existing route guides are useful but structurally inconsistent. They range
from 55 to 248 lines. Most link to compiling examples, but only a minority
carry inline Rust. There is no common requirement for installation, access,
host services, event draining, cancellation, cleanup, persistence, failure,
unsupported behavior, or deterministic validation.

No existing route guide can be declared complete against a shared standard
because no standard or traceability check existed before this audit.

## Feature Finding

Dedicated cross-cutting guidance exists for:

- observable activity and child-work projection
- provider-session catalogue and import
- interrupted-operation reconciliation
- controlled detachment
- working-state restoration
- portable failure handling
- realtime route preparation and rollover differences

The remaining feature matrix columns are described mainly in route notes,
contracts, examples, or adapter source. Material missing runbooks include:

- configured provider instances, access readiness, and model catalogues
- structured-run and interactive-session lifecycle
- ordinary event draining, terminal outcomes, cleanup, cancellation, usage,
  and billed cost
- model, reasoning, output-bound, and schema controls
- attachments, consumer tools, permission exchange, question exchange,
  working resources, bounded writes, and external search
- load, resume, native close, persistence, archive, restore, deletion, and
  provider-owned cleanup
- retained background work, stream reattachment, provider-managed recovery,
  and owned-runtime lifecycle

Plan mode, task lists, and subagent topology are present in the activity
contract and guide but need a task-oriented consumer path. The configured
provider-instance catalogue and harness question example exist in code but
have no indexed consumer guide.

## Validation Gap

`effigy qa:docs` checks links, front doors, next-action posture, and forbidden
defaults. `effigy check:examples` compiles examples. Neither proves that every
route and feature maps to a guide and example.

## Promotion

- Contract 052 defines route, feature, example, operator, and validation
  completeness.
- Roadmap g03.042 owns missing route guides, existing-guide deepening,
  cross-cutting feature runbooks, and a coverage check.
- `docs/guides/integration-guide-map.md` is the live non-authoritative
  traceability front door.

No runtime or provider behavior changes are required.
