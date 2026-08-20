# 045 DeepSeek Continuation Addable Descriptor

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../016-hosted-api-key-deepseek-continuation.md`
Depends on: completed g04.015

## Goal

Expose an adapter-local hosted addable-route descriptor for
`deepseek.continuation`.

## Scope

1. Add `AddableRouteDescriptor` in `swallowtail-adapter-deepseek`.
2. Topology is hosted, not `ExecutionLayer`.
3. Credential field: secret API key. Do not invent an environment name.
4. Config field: API endpoint as an opaque host-owned field.
5. Availability is `Available` or `Unavailable(HostService)`. Absence of the
   descriptor still means the crate is unlinked.

## Out Of Scope

- admission and API-key collection (card 046)
- refresh, subject, overlay (card 047)
- OAuth, Claude Agent, llama.cpp
- live provider calls

## Acceptance Criteria

- [ ] descriptor topology is hosted
- [ ] credential field carries no secret bytes and no invented env name
- [ ] endpoint config is a field id, not a URL
- [ ] `ExecutionLayer::DirectModelInference` is unchanged

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Yes, into card 046.

## Stop Conditions

- Stop if an umbrella registry crate appears.
- Stop if topology is folded into `ExecutionLayer`.
- Stop if an environment-variable name is invented.
