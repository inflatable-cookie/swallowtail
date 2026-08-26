# 205 Cline Headless Model-Selection Binding

Status: planned
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.074 Cline Headless Model Selection](../074-cline-headless-model-selection.md)
Depends on: card 204; promoted Research 221 with a non-empty deliver-now set

## Goal

Bind only Research 221's exact Cline headless provider/model rows through an
immutable `ModelRoute`, prepared evidence, fail-closed driver validation, and
canonical child argv.

## Scope

1. Extend `ClineHeadlessRunProfileInput` with only the exact model-route input
   admitted by Research 221. Expose no arbitrary string, caller provider
   choice, API key, catalogue default, alias, or fallback.
2. Bind configured-instance identity, local-account audience, provider/model
   identities, route identity, exact package/behavior, and access evidence
   through the existing preflight boundary.
3. Carry the selected model route through immutable plan, prepared evidence,
   derived request state where required, prepared run, driver, and exact child
   arguments.
4. Validate exact Research 221 membership and request/route/plan/evidence/
   driver agreement before spawn. Reject stale, mismatched, open, or unsupported
   rows without invoking Cline.
5. Emit only Research 221's canonical fixed provider/model argv and placement.
   Omission must retain exact `--json --auto-approve false` plus optional
   `--plan`, then `-c <cwd> <prompt>`.
6. Keep the route immutable across the complete one-child run. Do not add a
   model picker, fallback, catalogue read, runtime mutation, or resume.
7. Preserve local-account access, `Ambient`, `AmbientHost`, explicit
   `--auto-approve false`, read-only working-resource policy, and optional
   portable Plan. Add no configuration authority.
8. Preserve activity, cancellation, deadline, terminal, failure, provider
   retention, process ownership, and joined cleanup. Advance an adapter-private
   behavior revision only when Research 221 requires it.

## Acceptance Criteria

- [ ] only Research 221 deliver-now rows prepare
- [ ] instance, access audience, route, plan/evidence, driver, and exact argv
      agree
- [ ] omission retains exact prior argv and ambient provider/model behavior
- [ ] unsupported, mismatched, drifting, fallback-prone, or mutating rows
      reject before process work
- [ ] Plan composition remains independent and exact
- [ ] provider choice, catalogue, entitlement, configuration, reasoning,
      retention, lifecycle, and cleanup claims do not widen

## Validation

```sh
cargo fmt -p swallowtail-adapter-cline
effigy validate:focused swallowtail-adapter-cline
effigy package:verify-affected swallowtail-adapter-cline
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 206 only when exact preparation, model-route binding,
argv, omission, rejection, Plan composition, and lifecycle proof passes.

## Stop Conditions

- existing prepared structured-run state cannot express the admitted route
  without a generic or breaking surface
- provider/model truth can drift after preparation or cannot be checked before
  process work
- implementation needs settings mutation, catalogue access, caller provider
  selection, API keys, sibling-route work, or a shared contract change

## Out Of Scope

- thinking delivery, shared closeout selection, another Cline feature/route,
  live provider work, currentness, release, merge, rollover, or g04 closure
