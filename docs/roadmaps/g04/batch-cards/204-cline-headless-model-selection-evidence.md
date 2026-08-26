# 204 Cline Headless Model-Selection Evidence

Status: ready
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.074 Cline Headless Model Selection](../074-cline-headless-model-selection.md)
Depends on: Research 147, 190, and 220; Contracts 008, 020, 029, 033

## Goal

Determine whether exact qualified Cline `3.0.55` exposes any closed
provider/model row that `cline.headless` can bind as an immutable `ModelRoute`
without ambient provider drift, open model strings, silent fallback, live
catalogue authority, or unauthorized settings mutation. Promote an honest
empty set when it does not.

## Work

1. Retrieve and digest the published `cline@3.0.55` wrapper, annotated tag
   `cli-v3.0.55`, exact commit
   `ad442cbb6a81d21773ceabc1398ea5eb58170718`, and decisive tagged sources.
   Reconcile identity with Research 147; current docs/main are leads only.
2. Freeze `-m` / `--model` and `-P` / `--provider` parsing: canonical
   spellings, missing/empty/whitespace values, repeats, conflicts, option
   termination, placement with `--json`, `--auto-approve false`, optional
   `--plan`, `-c`, and positional prompt, omission, and local parse failures.
3. Trace provider resolution from explicit argument, last-used provider
   settings, and built-in `cline` fallback. Determine whether the existing
   configured-instance and local-account audience fix any provider before
   spawn, or whether a selected model remains ambient-provider-dependent.
4. Trace model resolution from explicit argument, selected-provider settings,
   resolved `knownModels`, catalogue-first selection, and hardcoded fallback.
   Freeze aliases, normalization, unknown/invalid identifiers, provider/model
   mismatch, and any silent fallback or late rejection.
5. Freeze exact model membership evidence for every candidate row. Separate
   package-static tables, bundled defaults, mutable provider catalogue,
   persisted state, account/entitlement evidence, and display metadata. A live
   or mutable catalogue cannot become a closed preflight allowlist.
6. Trace the resolved provider/model through `Config`, session construction,
   provider client/request dispatch, result and error paths, JSON output, and
   cleanup. Record whether selected provider/model appears only in verbose
   `run_start` and do not synthesize effective observation.
7. Trace `saveProviderSettings` and every related write: timing, inputs,
   failure handling, durable targets, omission behavior, whether explicit
   selection changes retained state, and whether an exact invocation can
   disable or operation-scope the write. Reconcile with Contract 033.
8. Audit production prepared input, configured instance, access audience,
   `ModelRoute`, preflight plan/evidence, request, driver, command builder,
   fixtures, guide, feature matrix, example, and API baseline. Name the
   smallest exact binding or the reason none exists.
9. Prove omission retains exact current argv and behavior. Determine whether
   an admitted pair can be validated before spawn and held immutable for the
   one-child run, including optional Plan composition.
10. Keep g04.042 thinking separate. Record only whether a delivered exact
    provider/model route would remove its selection dependency; do not claim
    reasoning support or reopen blocked cards.
11. Classify exact route/version/provider/model rows as deliver now,
    evidence-gated, intentionally withheld, or not applicable. Promote
    Research 221 with a non-empty exact table or explicit empty set.

## Acceptance Criteria

- [ ] exact artifact/source identities and decisive digests are frozen
- [ ] parser, provider/model precedence, membership, fallback, application,
      persistence, failure, output, and lifecycle truth is settled
- [ ] configured-instance, access-audience, provider, model, and route
      agreement is explicit
- [ ] requested, planned, dispatched, parsed, selected, applied/effective, and
      observed state is not conflated
- [ ] configuration reads and writes are reconciled with Contract 033 rather
      than treated as incidental provider behavior
- [ ] production plan/evidence, driver, argv, fixtures, docs, and API seams are
      audited
- [ ] Research 221 contains a non-empty exact table or honest empty set
- [ ] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes
- [ ] focused Cline validation, Northstar QA, research indexes, and diff checks
      pass

## Validation

```sh
effigy validate:focused swallowtail-adapter-cline
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Auto-continue to card 205 only when Research 221 admits a non-empty exact
`cline.headless` `3.0.55` provider/model row with preflight-validatable
membership, configuration authority, and complete route agreement.

## Stop Conditions

- provider remains ambient or model membership remains open, dynamic,
  account-scoped, fallback-prone, or post-spawn
- explicit selection unavoidably changes ambient provider settings without
  separately authorized configuration handling
- deterministic proof needs login, account inspection, live catalogue access,
  provider prompting, paid work, or configuration mutation

## Out Of Scope

- production binding, ACP, caller provider selection, API keys, thinking,
  catalogue implementation, live provider work, currentness, release, merge,
  rollover, or g04 closure
