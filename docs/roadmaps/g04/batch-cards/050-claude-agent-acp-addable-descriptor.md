# 050 Claude Agent ACP Addable Descriptor

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../018-installed-claude-agent-acp.md`
Depends on: completed g04.016

## Goal

Expose an adapter-local installed addable-route descriptor for
`claude-agent.acp`.

## Scope

1. Add `AddableRouteDescriptor` in `swallowtail-adapter-claude-agent`.
2. Topology is installed, not `ExecutionLayer`.
3. Config fields: binary path and opaque environment ref. Values stay
   host-private.
4. Local subscription path advertises no API-key credential field.
5. Availability is `Available` when the Process host service exists, else
   `Unavailable(HostService)`. Discovery of the executable stays Contract
   008, not the addable row.
6. Do not advertise `claude-code.headless` or
   `claude-code.response-only`.

## Out Of Scope

- admission and prepare (card 051)
- refresh, update observation, overlay (card 052)
- hosted OAuth
- live `--version` probes in this card
- inventing catalogue `provider_id`

## Acceptance Criteria

- [ ] descriptor topology is installed
- [ ] config fields carry no paths or env bodies
- [ ] subscription path has no secret credential field
- [ ] `ExecutionLayer::HarnessInteraction` is unchanged
- [ ] discovery candidates are not catalog rows

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Yes, into card 051.

## Stop Conditions

- Stop if topology is folded into `ExecutionLayer`.
- Stop if the addable row runs discovery.
- Stop if this row is treated as hosted OAuth.
