# 055 llama.cpp Attached Refresh, Update, And Subject

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../019-local-llama-cpp-attached.md`
Depends on: card 054

## Goal

Refresh access status, project 029 update observation, and keep
authenticated subject Absent for an admitted llama.cpp attached instance.

## Scope

1. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement is
   unchanged. There is no credential dimension.
2. `observe_instance_update` reuses `llama_cpp_attached_runtime_claim`.
   032 stays unobserved unless an executable observation is supplied.
3. Subject is `Absent`.
4. Catalogue rows without `provider_id` stay unmarked. Do not invent a
   provider id so overlay can key.
5. Deterministic harnesses only. Exact opaque b9910/f5525f7e7 binding is
   unchanged. No unverified-newer.

## Out Of Scope

- live `/health` as a substitute for host-supplied refresh
- hosted OAuth
- owned-serving descriptors
- Contract 052 consumer-path publication

## Acceptance Criteria

- [ ] a disabled instance can refresh to ready access dimensions
- [ ] subject fields are Absent, not 047 fields
- [ ] update observation reuses 029; 032 may be unobserved
- [ ] overlay does not invent a llama.cpp catalogue provider id
- [ ] `public-api-0.3.3` stays immutable

## Validation

- `effigy validate:focused swallowtail-adapter-llama-cpp swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

No. This closes the second-proof addable-route expansion. Hosted OAuth
stays a remaining gate.

## Stop Conditions

- Stop if overlay invents a provider id or changes `Ready` / `NotReady`.
- Stop if subject becomes an instance id or routing key.
- Stop if Swallowtail starts or stops the attached server.
