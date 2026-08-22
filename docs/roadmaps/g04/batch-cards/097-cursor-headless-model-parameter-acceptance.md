# 097 Cursor Headless Model Parameter Acceptance

Status: ready
Owner: Tom
Created: 2026-08-22
Milestone: [g04.035 Cursor Headless Model Parameters](../035-cursor-headless-model-parameters.md)
Depends on: card 096

## Goal

Prove exact Cursor headless parameter dispatch, publish bounded route truth,
and close the first per-route feature milestone.

## Scope

1. Add deterministic driver and prepared-facade tests for every Research 183
   deliver-now tuple, representative combined selections, and canonical key
   ordering.
2. Assert the rendered parameterized model stays one `--model` argv and agrees
   exactly with the immutable plan.
3. Assert plain-model dispatch remains unchanged.
4. Assert unknown model/value combinations, raw grammar, plan/request effort
   mismatch, duplicate semantics, and unsupported controls fail before process
   work.
5. Keep `--force`, `--sandbox`, `--mode ask`, ambient parameter discovery, and
   sibling Cursor routes absent.
6. Update `docs/architecture/system-architecture.md`,
   `docs/guides/cursor-prepared-integration.md`,
   `docs/guides/provider-route-matrix.md`, and
   `docs/guides/provider-solution-feature-matrix.csv` with headless-only truth.
   Mark reasoning selection Yes only for the exact qualified effort subset;
   state that Fast and context remain route-local selected-model parameters.
7. State qualified dispatch separately from provider acceptance and effective
   value. Provider rejection of a qualified-dispatch tuple is returned as
   provider truth, not remapped or retried.
8. Add a `CHANGELOG.md` Unreleased entry and closeout log. Complete g04.035 and
   cards 095-097, update the programme progress, g04 and batch-card indexes,
   and the sole roadmaps Next Task.
9. Set Next Task to define the next numbered per-route feature milestone for
   Ollama attached `num_ctx`. Contract 029 currentness remains a standing lane.

## Acceptance Criteria

- deterministic tests prove plan/request/argv agreement
- every exact deliver-now tuple and failure boundary is covered
- no provider call, account inspection, live catalogue, install, or update is
  part of default QA
- matrices distinguish headless from Cursor ACP and catalogue
- docs do not claim acceptance or effective-value confirmation
- architecture describes only realized behavior
- the advanced-route triage note remains promoted; programme progress records
  this milestone and points to Ollama `num_ctx`
- named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-cursor
effigy validate:focused swallowtail-adapter-cursor
effigy package:verify-affected swallowtail-adapter-cursor
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

- deterministic dispatch cannot prove one exact `--model` argument
- route docs would need to generalize Cursor names into portable aliases
- reasoning selection would exceed exact Research 183 model evidence
- provider acceptance or effective values are needed to justify the claim
- acceptance requires a provider prompt or authenticated catalogue

## Out Of Scope

- live provider, authenticated catalogue, login, install, or host update
- Cursor ACP or catalogue behavior
- sandbox, force, ask mode, session management, or unrelated route features
- Contract 029 ceiling changes
- Ollama implementation
- workspace `qa`, broad `qa:docs`, live probes, MSRV, consumer, release, or
  publication checks
