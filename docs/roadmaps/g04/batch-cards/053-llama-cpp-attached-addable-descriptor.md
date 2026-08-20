# 053 llama.cpp Attached Addable Descriptor

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../019-local-llama-cpp-attached.md`
Depends on: completed g04.018

## Goal

Expose an adapter-local local-runtime addable-route descriptor for
`llama-cpp.attached`.

## Scope

1. Add `AddableRouteDescriptor` in `swallowtail-adapter-llama-cpp`.
2. Topology is local-runtime, not `ExecutionLayer`.
3. Config field: API endpoint as an opaque host-owned field.
4. No credential field. No sign-in action.
5. Availability is `Available` when the Network host service exists, else
   `Unavailable(HostService)`. Runtime reachability stays
   `prepare_llama_cpp_attached`, not the addable row.
6. Do not advertise `llama-cpp.owned`.

## Out Of Scope

- admission and prepare (card 054)
- refresh, update observation, overlay (card 055)
- live `/health` probes in this card
- inventing catalogue `provider_id`
- starting, stopping, or installing the server
- owned-serving descriptor edits

## Acceptance Criteria

- [x] descriptor topology is local-runtime
- [x] config field carries no URL
- [x] no secret credential field
- [x] `ExecutionLayer::DirectModelInference` is unchanged
- [x] the addable row does not probe the runtime
- [x] the row is not `llama-cpp.owned`

## Validation

- `effigy validate:focused swallowtail-adapter-llama-cpp swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Yes, into card 054.

## Stop Conditions

- Stop if topology is folded into `ExecutionLayer`.
- Stop if the addable row starts or probes llama.cpp.
- Stop if owned serving is advertised from this row.
