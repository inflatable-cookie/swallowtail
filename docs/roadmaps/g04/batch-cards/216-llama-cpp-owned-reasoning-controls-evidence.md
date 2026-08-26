# 216 llama.cpp Owned Reasoning Controls Evidence

Status: complete
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.078 llama.cpp Owned Reasoning Controls](../078-llama-cpp-owned-reasoning-controls.md)
Depends on: g04.056; g04.077 closeout

## Goal

Freeze exact `b10069-178a6c449` parser, precedence, model/template,
application, and observation truth for `--reasoning` and
`--reasoning-budget`, then promote Research 225 with a non-empty exact
deliver-now table or an honest empty set.

## Work

1. Reuse and verify the exact runtime identity, source revision, artifact
   digest, driver id, compatibility axis, and owned-server launch from Research
   203. Digest every decisive parser, params, template, reasoning, server,
   readiness, and output source. Current `master` may corroborate only.
2. Freeze parser truth for `--reasoning on|off|auto` and
   `--reasoning-budget -1|0|N`: aliases, separators, missing/empty/invalid and
   overflow values, negative domain, repeated flags, mixed forms, placement,
   defaults, diagnostics, exit status, and help/source agreement.
3. Freeze precedence across CLI, environment, configuration, model metadata,
   chat templates, request payloads, and server defaults. Name any value that
   can replace, weaken, or make the prepared selection inert after launch.
4. Trace `on`, `off`, and `auto` from parse through server params, chat-template
   detection, request handling, rendered prompt, reasoning extraction, and
   response framing. Distinguish selected enum, applied template behavior,
   effective model behavior, and output observation.
5. Trace budget `-1`, `0`, and positive `N` through parser bounds and template
   application. Prove dependencies on reasoning start/end tags, template
   capabilities, preservation, and `--reasoning-format`; record no-op,
   fallback, warning, and failure behavior.
6. Determine the exact preflight model/template evidence available before
   process work. Audit operator-supplied GGUF metadata, prepared inputs,
   immutable plan/evidence, startup arguments, `/props`, readiness, fixtures,
   and any local prompt-free source or artifact probe.
7. Freeze observation truth. Record whether startup output, `/health`,
   `/props`, or another deterministic prompt-free channel reports requested,
   parsed, applied, or effective reasoning selection and budget. Do not infer
   inference behavior from server readiness.
8. Audit the current prepared input/result, context-size binding, driver state,
   behavior revision, command builder, exact-version assessment, guide,
   matrices, examples, fixtures, and API baseline. Name the smallest closed
   binding or the missing preflight fact.
9. Prove omission remains the exact current launch: no reasoning arguments,
   unchanged context-size placement and bounds, model path, host/port,
   readiness, working resource, `AmbientHost`, provider state, and one-child
   lifecycle.
10. Classify reasoning selection and budget independently and in composition.
    Every exact value/model-template row must be deliver now, evidence-gated,
    intentionally withheld, or not applicable.
11. Keep serving and inference separate. Do not change the attached route,
    infer a portable reasoning capability, or weaken current rejection of
    unqualified reasoning content.
12. Promote Research 225 with exact source digests, matrices, and a non-empty
    deliver-now table or explicit empty set. Update milestone/card state and
    close out honestly.

## Acceptance Criteria

- [x] exact identity, source digests, parser, precedence, and defaults are
      frozen
- [x] selection, budget, template/model applicability, application, and
      observation have exact dispositions
- [x] requested, prepared, dispatched, parser-accepted, applied, effective,
      and observed state remain distinct
- [x] production preparation, driver, argv, context-size composition,
      fixtures, docs, and API seams are audited
- [x] Research 225 contains a non-empty exact table or honest empty set
- [x] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-llama-cpp
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Auto-continue to card 217 only when Research 225 admits a non-empty exact row
whose model/template applicability and selected behavior are deterministic
without a live model run or wider authority.

## Stop Conditions

- exact source, precedence, model/template applicability, or observation
  remains ambiguous
- a selected value may silently become inert or drift after preparation
- deterministic proof needs model download/load, prompting, inference, paid
  work, ambient configuration mutation, or a shared contract change

## Out Of Scope

- production binding, portable reasoning APIs, raw provider controls, attached
  route work, live inference, currentness, release, merge, rollover, or g04
  closure
