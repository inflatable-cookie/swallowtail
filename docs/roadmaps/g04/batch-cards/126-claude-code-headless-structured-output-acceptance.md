# 126 Claude Code Headless Structured Output Acceptance

Status: conditional; awaiting card 125
Owner: Tom
Created: 2026-08-23
Milestone: [g04.045 Claude Code Headless Structured Output](../045-claude-code-headless-structured-output.md)
Depends on: card 125

## Goal

Prove exact Claude Code headless structured output, preserved absent behavior,
and fail-closed result handling, then produce one review-ready route-local
closeout.

## Scope

1. Add deterministic exact-version corpus and prepared-facade coverage for
   every Research 192 deliver-now schema row.
2. Assert exact schema serialization, dialect, enforcement source, attempt
   constraint, model, reasoning, facade, policy, capability, plan, and argv.
3. Assert the absent path retains the current command and ordinary terminal
   text behavior.
4. Prove valid, invalid, unsatisfiable, null, missing, malformed, duplicate,
   foreign, post-terminal, non-zero-exit, deadline, cancellation, process-loss,
   usage, and cleanup behavior.
5. Preserve fixed Plan-mode tools, read-only working-resource authority,
   activity projection, model matching, no persistence, and strict empty MCP.
6. Update the Claude Agent prepared guide, Research 192, cards 124-126,
   g04.045, the reserved route-local closeout, and package-specific unreleased
   API baseline when applicable.
7. Record the exact required architecture, route/feature matrix, changelog,
   programme, index, matrix-assertion, and Next Task delta in the closeout and
   PR body. Do not edit those shared surfaces on the worker branch.

## Acceptance Criteria

- [ ] every admitted schema and failure class has deterministic coverage
- [ ] default QA performs no credential, account, install, external request,
      provider prompt, or paid work
- [ ] docs distinguish CLI acceptance from returned and schema-valid output
- [ ] no sibling route, schema draft, retry promise, or compatibility point is
      inferred
- [ ] closeout records PR/head truth without claiming merge
- [ ] worker changes stay inside the route-local boundary
- [ ] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
effigy check:examples
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy package:api
git diff --check
```

Auto-continuation: No.

## Stop Conditions

- exact schema, attempt, terminal, usage, or cleanup truth cannot be proved
- docs would need to infer enforcement, effectiveness, or provider behavior
- another route, flag family, currentness lane, contract, or release enters
  scope

## Out Of Scope

- live provider verification, publication, merge, shared front-door edits, or
  later feature selection

