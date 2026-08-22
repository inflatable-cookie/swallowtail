# 096 Cursor Headless Model Parameter Binding

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.035 Cursor Headless Model Parameters](../035-cursor-headless-model-parameters.md)
Depends on: card 095; promoted Research 183

## Goal

Add typed Cursor-local model parameters and bind the exact qualified selection
through preparation without opening a generic generation-parameter surface.

## Scope

1. Add documented adapter-local public types for the exact deliver-now subset
   from Research 183. Use closed types for Fast, context, and effort; do not
   expose raw parameter names or values.
2. Extend `CursorHeadlessModelSelection` with a fallible typed-parameter path.
   Preserve `CursorHeadlessModelSelection::new` for plain model ids.
3. Keep the typed parameters private inside the selection until preparation,
   then validate the exact base-model/parameter/value combination against the
   frozen allowlist.
4. Reject bracket, comma, equals, or other reserved parameter grammar supplied
   through the plain model-id path before provider work. Do not parse or bless
   caller-assembled parameter strings.
5. Render non-empty typed parameters once, in the canonical order fixed by
   Research 183, into one exact `ModelId`. Use that rendered id in the immutable
   `ModelRoute`, prepared evidence, validation, and driver dispatch.
6. Keep Fast and context Cursor-local. They do not add portable capabilities or
   constraints.
7. For a qualified effort tuple, bind exact `ReasoningMode` in the structured
   request policy and add `Capability::ReasoningSelection` with the matching
   constraint to the prepared instance, requirements, and model route.
8. Fail closed when effort request, plan constraint, rendered model parameter,
   or selected tuple disagrees. Do not infer provider acceptance or effective
   application.
9. Add deterministic preparation, public-API, and validation tests for plain,
   individual, combined, invalid, and mismatched selections.

## Compatibility Boundary

- The existing plain-model constructor and resulting argv remain unchanged for
  valid plain ids.
- Typed selection is additive public API; run `effigy package:api`.
- No parameter tuple may exceed Research 183 merely because its scalar type is
  representable.
- If the exact allowlist needs a new adapter-private behavior revision, add it
  without changing the Contract 029 ceiling. Otherwise preserve the current
  revision and explain why the mapping is an additive prepared-input seam.

## Acceptance Criteria

- no generic map or public raw provider grammar exists
- only Research 183 deliver-now tuples prepare
- raw bracket syntax cannot bypass typed validation
- canonical model id in the plan equals the eventual dispatched model id
- qualified effort appears identically in request policy and plan constraint
- plain-model preparation remains byte-for-byte equivalent
- all failure paths occur before host process work
- public API documentation states model dependence and evidence bounds

## Validation

```sh
cargo fmt -p swallowtail-adapter-cursor
effigy validate:focused swallowtail-adapter-cursor
effigy package:verify-affected swallowtail-adapter-cursor
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 097 when the typed path and plain-path compatibility
tests pass.

## Stop Conditions

- a generic provider-parameter map is required
- raw parameter grammar cannot be distinguished and rejected
- exact tuple validation would require an authenticated catalogue or prompt
- effort cannot satisfy Contract 040 request/plan/dispatch agreement
- preserving the plain-model path requires a breaking public change

## Out Of Scope

- guide, matrix, architecture, programme, or changelog closeout
- provider acceptance or effective-value confirmation
- Cursor ACP or catalogue changes
- sandbox, force, ask mode, or another route feature
- live provider, login, install, or currentness work
