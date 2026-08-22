# 102 Anthropic Messages Effort Binding

Status: completed
Owner: Tom
Created: 2026-08-22
Milestone: [g04.037 Anthropic Messages Effort](../037-anthropic-messages-effort.md)
Depends on: card 101; promoted Research 185

## Goal

Bind only the Research 185 deliver-now Anthropic Messages effort selections
through prepared input, immutable plan/evidence, the low-level driver, and exact
request encoding.

## Scope

1. Add optional portable `ReasoningMode` input only to the structured and/or
   session profiles admitted by Research 185. Preserve existing constructors.
2. Reject unsupported values, models, and operation profiles at preparation.
3. Add exact `ReasoningSelection` capability constraints to the selected model
   route and immutable plan; do not advertise reasoning when absent.
4. Retain the selected mode in `AnthropicPreparedEvidence` and configure the
   low-level driver with the same exact value. Preserve the no-selection driver.
5. Encode `output_config: {"effort": "<exact>"}` once on every applicable
   Messages request. Do not add or modify `thinking`.
6. Preserve current request bodies byte-for-byte when effort is absent.
7. If sessions are admitted, fix the value at preparation and emit it on every
   attempt and fresh restoration without per-turn override.
8. Reject request/plan/evidence/driver mismatch before endpoint authorization or
   credential use. Add deterministic preparation and protocol tests.

## Acceptance Criteria

- [x] only Research 185 deliver-now combinations prepare
- [x] input, plan constraint, evidence, driver, and wire value agree exactly
- [x] absent selection preserves current JSON and public behavior
- [x] no raw provider value or generic options object enters public API
- [x] `thinking`, output limits, search, attachments, model identity, cancellation,
  and cleanup stay independent
- [x] known failures occur before network work

## Validation

```sh
cargo fmt -p swallowtail-adapter-anthropic
effigy validate:focused swallowtail-adapter-anthropic
effigy package:verify-affected swallowtail-adapter-anthropic
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 103 when exact binding and absent-path tests pass.

## Stop Conditions

- exact selection cannot be represented by portable reasoning under Contract 040
- prepared evidence and driver cannot remain in exact agreement
- session selection can drift between attempts or restoration
- compatibility requires a breaking public change or new facade segment

## Out Of Scope

- guide, matrix, architecture, programme, or changelog closeout
- provider acceptance or effective-effort claims
- Messages thinking, web-search updates, or live provider work
