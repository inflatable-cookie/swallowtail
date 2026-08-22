# 099 Ollama Num Ctx Binding

Status: ready after 098
Owner: Tom
Created: 2026-08-22
Milestone: [g04.036 Ollama Attached Context Window](../036-ollama-attached-context-window.md)
Depends on: card 098; promoted Research 184

## Goal

Add one typed Ollama-local context-window selection and bind the exact qualified
value through prepared evidence, the bound native driver, and `/api/chat`
encoding without opening a generic option surface.

## Scope

1. Add a documented adapter-local public value type for the exact positive
   domain admitted by Research 184. Reject zero, negative representation,
   overflow, and out-of-domain values at construction or preparation.
2. Add the value only to the prepared structured-run and/or session profile
   inputs marked deliver-now by Research 184. Preserve existing constructors and
   absent-control behavior.
3. Retain the selected value in `OllamaPreparedEvidence` and expose a safe
   getter. Inventory preparation records no context selection.
4. Configure the prepared operation's `OllamaNativeAttachedDriver` with the
   same typed value. Preserve `new()` as the no-override low-level path and add
   an exact typed low-level constructor or builder rather than raw JSON options.
5. Ensure existing `into_parts` extraction retains the value through evidence;
   add an additive bound-parts helper if needed. Do not break the existing tuple
   or silently drop the selection.
6. Extend native chat encoding with optional `num_ctx` beside `num_predict` in
   one `options` object. Keep the absent body byte-for-byte unchanged.
7. If sessions are admitted, bind one fixed value at session preparation and
   dispatch it on every replay turn and fresh restoration. Do not accept a
   per-turn raw override.
8. Keep the provider-neutral plan free of a context-window capability. The
   adapter-local evidence and driver are the exact authority for this
   route-local value.
9. Keep maximum output tokens, reasoning, structured output, model tag/digest,
   version assessment, residency, cancellation, and cleanup independent.
10. Reject any evidence/driver/value mismatch before endpoint authorization or
    catalogue observation. Add deterministic preparation and low-level tests.

## Compatibility Boundary

- Existing prepared input constructors, no-override driver construction, and
  request bodies remain unchanged.
- The typed value and builders are additive public API; run semantic API proof.
- An unverified-newer attempt may use only the latest qualified mapping and
  remains visibly unverified.
- Dispatch grants no runtime ownership, capacity, unload, or retry authority.

## Acceptance Criteria

- no generic map or raw `options` payload enters public API
- only Research 184 deliver-now profiles and numeric values prepare
- prepared evidence, configured driver, and encoded integer agree exactly
- absent selection preserves current request JSON and public behavior
- `num_ctx` remains independent from `num_predict` and model metadata
- all known validation failures occur before network work
- public docs state local-only, resource-sensitive, dispatch-only semantics

## Validation

```sh
cargo fmt -p swallowtail-adapter-ollama
effigy validate:focused swallowtail-adapter-ollama
effigy package:verify-affected swallowtail-adapter-ollama
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 100 when exact binding, extraction, and absent-path tests
pass.

## Stop Conditions

- the value requires a generic options map or provider-neutral capability
- prepared evidence and low-level driver cannot remain in exact agreement
- session support would allow value drift across turns or restoration
- preserving current constructors or absent JSON requires a breaking change
- implementation needs a live runtime or effective-value inference

## Out Of Scope

- guide, matrix, architecture, programme, or changelog closeout
- provider acceptance, effective context, resource-fit, or truncation claims
- server configuration, cloud, compatible endpoints, or another Ollama option
- live provider, install, pull, unload, or currentness work
