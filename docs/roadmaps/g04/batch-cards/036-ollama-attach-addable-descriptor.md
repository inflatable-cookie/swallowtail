# 036 Ollama Attach Addable Descriptor

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../013-local-ollama-attach.md`
Depends on: completed g04.012

## Goal

Expose an adapter-local local-runtime addable-route descriptor for
`ollama.attached`.

## Scope

1. Add `AddableRouteDescriptor` in `swallowtail-adapter-ollama`.
2. Topology is local-runtime, not `ExecutionLayer`.
3. Config field: API endpoint as an opaque host-owned field.
4. No credential field. No sign-in action.
5. Availability is `Available` when the Network host service exists, else
   `Unavailable(HostService)`. Runtime reachability stays
   `prepare_ollama_attached`, not the addable row.

## Out Of Scope

- admission and prepare (card 037)
- refresh, update observation, overlay (card 038)
- live `/api/version` probes in this card
- inventing catalogue `provider_id`
- starting or installing Ollama

## Acceptance Criteria

- [ ] descriptor topology is local-runtime
- [ ] config field carries no URL
- [ ] no secret credential field
- [ ] `ExecutionLayer::DirectModelInference` is unchanged
- [ ] the addable row does not probe the runtime

## Validation

- `effigy validate:focused swallowtail-adapter-ollama swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Yes, into card 037.

## Stop Conditions

- Stop if topology is folded into `ExecutionLayer`.
- Stop if the addable row starts or probes Ollama.
