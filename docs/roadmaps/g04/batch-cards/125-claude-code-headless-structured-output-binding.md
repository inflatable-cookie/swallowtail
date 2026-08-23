# 125 Claude Code Headless Structured Output Binding

Status: blocked; card 124 evidence stop
Owner: Tom
Created: 2026-08-23
Milestone: [g04.045 Claude Code Headless Structured Output](../045-claude-code-headless-structured-output.md)
Depends on: card 124; promoted Research 192 with a non-empty deliver-now set

Card 124 promoted Research 192 with an empty deliver-now set. No typed
structured-output input, prepared plan, policy, driver, argv, parser, or
behavior revision is admitted by this lane.

## Goal

Bind only Research 192's exact Claude Code headless structured-output subset
through typed prepared input, immutable plan/evidence, request policy, driver,
argv, and strict terminal parsing.

## Scope

1. Add one optional typed structured-output selection to
   `ClaudeCodeRunProfileInput`, using the existing portable
   `StructuredOutputDescriptor`. Preserve current constructors and exact
   schema-absent behavior.
2. Admit only the exact Research 192 version, schema dialect/subset,
   enforcement source, and attempt policy. Expose no raw settings, generic
   provider options, prompt convention, or retry knob.
3. Bind structured-output capability and constraints through route instance,
   operation requirements, preflight plan, prepared evidence, request policy,
   low-level driver, and command arguments.
4. Serialize the schema deterministically into the exact official flag form.
   Do not alter the user prompt or emulate schema enforcement.
5. Parse only the exact qualified terminal structured-result representation.
   A missing, null, malformed, duplicate, foreign, or schema-invalid result is
   an explicit failure even if the child exits zero.
6. Preserve fixed Plan mode, `Read,Glob,Grep`, read-only working resource,
   model/reasoning selection, no persistence, strict empty MCP, activity,
   usage, deadline, cancellation, and joined cleanup.
7. Bind a new exact Contract 029 point/private behavior revision when Research
   192 requires one. Do not backfill the current behavior or earlier versions.
8. Reject descriptor, dialect, enforcement, attempt, version, plan, evidence,
   request, driver, or argv drift before process work when knowable.

## Acceptance Criteria

- [ ] only Research 192 deliver-now schema rows prepare
- [ ] typed input, descriptor, constraints, plan, evidence, policy, driver,
      argv, and parser agree exactly
- [ ] schema absence preserves current command bytes and result behavior
- [ ] null/missing/invalid structured results never become ordinary success
- [ ] no unbounded hidden retry, prompt emulation, raw option map, or version
      inference enters the API
- [ ] all knowable mismatches reject before process work

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 126 when exact preparation, command, terminal parsing,
failure, absent-path, and zero-provider-work rejection tests pass.

## Stop Conditions

- exact portable vocabulary cannot carry the admitted schema semantics
- attempt bounds cannot remain immutable from preparation through result
- strict parsing requires invented provider fields or content inference
- route controls, cleanup, or compatibility truth would weaken
- implementation needs a contract change, live proof, or breaking API

## Out Of Scope

- shared docs/indexes, other Claude routes or flags, live provider work,
  release, or merge
