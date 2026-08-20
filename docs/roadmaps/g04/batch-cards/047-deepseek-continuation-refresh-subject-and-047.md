# 047 DeepSeek Continuation Refresh, Subject, And 047

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../016-hosted-api-key-deepseek-continuation.md`
Depends on: card 046

## Goal

Refresh access status and prove the 047 snapshot plus overlay path for an
admitted DeepSeek continuation instance without changing selection
readiness.

## Scope

1. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement is
   unchanged.
2. Authenticated subject is `Absent`. Do not probe Open Platform for
   identity.
3. Consumer-assembled 047 snapshot. Overlay keys to `deepseek` catalogue
   provider ids already present on continuation entries.
4. Deterministic harnesses only.

## Out Of Scope

- live provider identity or billing probes
- hosted OAuth
- Claude Agent or llama.cpp overlay `provider_id` repairs
- Contract 052 consumer-path publication

## Acceptance Criteria

- [x] a disabled instance can refresh to ready access dimensions
- [x] subject fields are Absent, not 047 fields
- [x] overlay can mark a DeepSeek catalogue model without changing
      `Ready` / `NotReady`
- [x] `public-api-0.3.3` stays immutable

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

No. Compile Claude Agent ACP or llama.cpp attached only after this proof.
Hosted OAuth stays gated.

## Stop Conditions

- Stop if overlay or refresh changes 047 `Ready` / `NotReady`.
- Stop if subject becomes an instance id or routing key.
- Stop if a live account probe starts.
