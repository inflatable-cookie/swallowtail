# 033 Codex App-Server Addable Descriptor

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../012-installed-codex-app-server.md`
Depends on: completed g04.011

## Goal

Expose an adapter-local installed addable-route descriptor for
`codex.app-server`.

## Scope

1. Add `AddableRouteDescriptor` in `swallowtail-adapter-codex`.
2. Topology is installed, not `ExecutionLayer`.
3. Config fields: binary path and opaque environment ref. Values stay
   host-private.
4. ChatGPT subscription path advertises no API-key credential field.
5. Availability is `Available` when the Process host service exists, else
   `Unavailable(HostService)`. Discovery of the executable stays Contract
   008, not the addable row.

## Out Of Scope

- admission and prepare (card 034)
- refresh, update observation, overlay (card 035)
- hosted OAuth
- live `codex --version` probes in this card
- inventing catalogue `provider_id`

## Acceptance Criteria

- [ ] descriptor topology is installed
- [ ] config fields carry no paths or env bodies
- [ ] ChatGPT path has no secret credential field
- [ ] `ExecutionLayer::HarnessInteraction` is unchanged
- [ ] discovery candidates are not catalog rows

## Validation

- `effigy validate:focused swallowtail-adapter-codex swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Yes, into card 034.

## Stop Conditions

- Stop if topology is folded into `ExecutionLayer`.
- Stop if the addable row runs discovery.
