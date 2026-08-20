# 038 Ollama Attach Refresh, Update, And Subject

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../013-local-ollama-attach.md`
Depends on: card 037

## Goal

Refresh access status, project 029 update observation, and keep
authenticated subject Absent for an admitted Ollama attach instance.

## Scope

1. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement is
   unchanged. There is no credential dimension.
2. `observe_instance_update` reuses `ollama_runtime_claim`. 032 stays
   unobserved unless an executable observation is supplied.
3. Subject is `Absent`.
4. Catalogue rows without `provider_id` stay unmarked. Do not invent a
   provider id so overlay can key.
5. Deterministic harnesses only.

## Out Of Scope

- live `/api/version` as a substitute for host-supplied refresh
- hosted OAuth
- Codex or Anthropic descriptor edits
- Contract 052 consumer-path publication

## Acceptance Criteria

- [ ] a disabled instance can refresh to ready access dimensions
- [ ] subject fields are Absent, not 047 fields
- [ ] update observation reuses 029; 032 may be unobserved
- [ ] overlay does not invent an Ollama catalogue provider id
- [ ] `public-api-0.3.3` stays immutable

## Validation

- `effigy validate:focused swallowtail-adapter-ollama swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

No. Hosted OAuth stays a remaining gate. Compile Contract 052 consumer path
only after this local-runtime proof.

## Stop Conditions

- Stop if overlay invents a provider id or changes `Ready` / `NotReady`.
- Stop if subject becomes an instance id or routing key.
- Stop if Swallowtail starts Ollama.
