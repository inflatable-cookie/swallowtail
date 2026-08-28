# g05.001 Harness Skill And Watcher Surface Inventory

Status: completed
Owner: Tom
Created: 2026-08-28
Depends on: completed g04
Vision tags: harness skills, process observability, consumer integration
Contract refs: 013, 017, 023, 029, 034, 041, 044, 047, 052, 058, 059
Research: 255 promoted
Planning state: cards 001-003 completed

## Problem

Harness distributions may ship skills or background-process mechanisms, but
Swallowtail does not know which exact surfaces exist, which are visible to the
selected model, or which state can be exposed safely to a consumer. Ambient
files and model prose cannot prove either capability. A watcher skill cannot
enforce process ownership, cleanup, or a turn-completion gate by itself.

## Goal

Freeze exact prompt-free evidence across production harness routes. Separate
distribution membership, host configuration, session visibility, watcher
control, process ownership, activity projection, and completion enforcement.
Use that evidence to prepare operator decisions before architecture or contract
promotion.

## Execution Plan

### Batch 1.1 — Surface Inventory

- [x] execute card 001
- [x] inventory exact route/version official listing, manifest, protocol, and
      process-control surfaces
- [x] promote Research 255 with a closed evidence matrix

### Batch 1.2 — Boundary And Decision Packet

- [x] execute card 002 after Research 255 promotion
- [x] classify portable, provider-local, host-owned, consumer-owned, unsafe,
      and unknown seams
- [x] return unresolved policy choices to the operator

### Batch 1.3 — Promotion And Runway Selection

- [x] execute card 003 after recorded operator decisions
- [x] select architecture, contract, research, or stop dispositions
- [x] compile proof routes only from promoted testable boundaries

## Acceptance Criteria

- [x] every production harness route has an exact evidence disposition
- [x] skill distribution membership stays separate from model visibility
- [x] native watchers stay separate from a Swallowtail-owned mechanism
- [x] process ownership, stop, join, output, and turn completion remain explicit
- [x] no public type or implementation is selected before operator decisions
- [x] one clear continuation checkpoint remains

## Stop Conditions

- evidence needs credentials, a provider prompt, paid work, install/update, or
  ambient host mutation
- listing requires recursive user-home or project scanning
- route/version identity cannot be bounded
- a watcher claim depends only on prompt compliance
- the evidence cannot separate provider, harness, Swallowtail, host, and
  consumer authority

## Non-Goals

- skill injection, watcher tools, process registries, or turn gates
- arbitrary PID inspection or kill authority
- consumer UI design or raw log streaming
- new routes, currentness, parked Bedrock work, release, or publication

## Batch Cards

- [001 Production Harness Skill And Watcher Surface Census](batch-cards/001-production-harness-skill-and-watcher-surface-census.md)
- [002 Boundary Classification And Operator Decision Packet](batch-cards/002-boundary-classification-and-operator-decision-packet.md)
- [003 Promotion And Proof-Route Selection](batch-cards/003-promotion-and-proof-route-selection.md)

## References

- [Research 255 Production Harness Skill And Watcher Surface Census](../../research/255-production-harness-skill-and-watcher-surface-census.md)
- [Harness Skill Discovery And Process Watchers](../../triage/2026-08-27-harness-skill-discovery-and-process-watchers.md)
- [Contract 013 Interactive Session Access Policy](../../contracts/013-interactive-session-access-policy.md)
- [Contract 017 Provider-Owned Session Load](../../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 041 Input Callback And Tool Admission](../../contracts/041-input-callback-and-provider-tool-admission.md)
- [Contract 044 Observable Agent Activity](../../contracts/044-observable-agent-activity-and-disclosure.md)
