# 030 Anthropic Messages Addable Descriptor

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../011-hosted-api-key-anthropic-messages.md`
Depends on: completed g04.010

## Goal

Expose an adapter-local hosted addable-route descriptor for
`anthropic.messages`.

## Scope

1. Add `AddableRouteDescriptor` in `swallowtail-adapter-anthropic`.
2. Topology is hosted, not `ExecutionLayer`.
3. Credential field: secret API key, optional env name `ANTHROPIC_API_KEY`.
4. Config field: API endpoint as an opaque host-owned field.
5. Availability is `Available` or `Unavailable(HostService)`. Absence of the
   descriptor still means the crate is unlinked.

## Out Of Scope

- admission and API-key collection (card 031)
- refresh, subject, overlay (card 032)
- OAuth, Codex, Ollama
- live provider calls

## Acceptance Criteria

- [ ] descriptor topology is hosted
- [ ] credential field carries no secret bytes
- [ ] endpoint config is a field id, not a URL
- [ ] `ExecutionLayer::DirectModelInference` is unchanged

## Validation

- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-runtime`
- `git diff --check`

## Auto-Continuation

Yes, into card 031.

## Stop Conditions

- Stop if an umbrella registry crate appears.
- Stop if topology is folded into `ExecutionLayer`.
