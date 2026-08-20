# 032 Anthropic Messages Refresh, Subject, And 047

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../011-hosted-api-key-anthropic-messages.md`
Depends on: card 031

## Goal

Refresh access status and prove the 047 snapshot plus overlay path for an
admitted Anthropic Messages instance without changing selection readiness.

## Scope

1. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement is
   unchanged.
2. Authenticated subject is `Absent` unless the adapter already discloses a
   field. Do not probe Messages for email.
3. Consumer-assembled 047 snapshot. Overlay keys to `anthropic` catalogue
   provider ids already present on Messages entries.
4. Deterministic harnesses only.

## Out Of Scope

- live provider identity or billing probes
- hosted OAuth
- Codex or Ollama overlay `provider_id` repairs
- Contract 052 consumer-path publication

## Acceptance Criteria

- [ ] a disabled instance can refresh to ready access dimensions
- [ ] subject fields are Absent, not 047 fields
- [ ] overlay can mark an Anthropic catalogue model without changing
      `Ready` / `NotReady`
- [ ] `public-api-0.3.3` stays immutable

## Validation

- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

No. Compile Codex, Ollama, or hosted OAuth only after this first proof.

## Stop Conditions

- Stop if overlay or refresh changes 047 `Ready` / `NotReady`.
- Stop if subject becomes an instance id or routing key.
- Stop if a live account probe starts.
