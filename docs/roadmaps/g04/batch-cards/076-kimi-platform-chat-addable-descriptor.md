# 076 Kimi Platform Chat Addable Descriptor

Status: completed
Owner: Tom
Created: 2026-08-21
Milestone: `../024-hosted-api-key-kimi-platform-chat.md`
Depends on: completed g04.023

## Goal

Expose one adapter-local hosted addable-route descriptor for
`kimi-platform.chat` without changing its prepared facade.

## Scope

1. Add and export the descriptor from `swallowtail-adapter-kimi-platform`.
2. Bind the existing `swallowtail.kimi-platform.direct-chat` driver identity.
3. Use hosted topology, distinct from `ExecutionLayer`.
4. Describe one secret Platform API-key field. Do not invent an environment
   name.
5. Describe the approved API endpoint as an opaque host-owned config field.
6. Report `Available` only when the required Credential host service exists;
   otherwise report the named missing host service.

## Out Of Scope

- admission or API-key collection (card 077)
- preparation, catalogue, inference, refresh, subject, or 047 assembly
- Kimi Membership, Kimi Code, Kimi local server, regional Platform access
- browser OAuth, provider calls, billing checks, install, or login
- public route-matrix changes or an umbrella registry

## Acceptance Criteria

- descriptor route id is exactly `kimi-platform.chat`
- topology is hosted and driver identity is unchanged
- credential metadata carries no secret bytes and no environment name
- endpoint metadata carries a field id, not the URL value
- absence of the adapter descriptor still means the crate is unlinked

## Validation

- `effigy validate:focused swallowtail-adapter-kimi-platform swallowtail-runtime`
- `git diff --check`
- `effigy package:api` if public API changes

## Auto-Continuation

Yes, into card 077.

## Stop Conditions

- Stop if a portable record receives a key or endpoint value.
- Stop if Platform, Membership, Code, or regional credentials are flattened.
- Stop if topology is folded into `ExecutionLayer`.
