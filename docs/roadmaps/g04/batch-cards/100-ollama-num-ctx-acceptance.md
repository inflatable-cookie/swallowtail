# 100 Ollama Num Ctx Acceptance

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.036 Ollama Attached Context Window](../036-ollama-attached-context-window.md)
Depends on: card 099

## Goal

Prove exact local native context-window dispatch, publish bounded route truth,
and close the second per-route feature milestone.

## Scope

1. Add deterministic protocol and prepared-facade tests for the Research 184
   minimum, representative, and maximum admitted values.
2. Assert `options.num_ctx` and `options.num_predict` are distinct exact integer
   fields and the selected value agrees with prepared evidence and driver
   configuration.
3. Assert the absent selection preserves existing fixture request bodies
   byte-for-byte.
4. Assert zero, overflow, out-of-domain, raw options, and unqualified profile
   use fail before network work. Prepared-path tests prove evidence/driver/native
   agreement; low-level role dispatch remains caller authority under Contract 037.
5. If sessions are admitted, assert every clean replay turn and fresh
   restoration uses the fixed prepared value; failed turns still do not commit
   history. If not admitted, document and test the rejection.
6. Preserve remote/cloud-model rejection and the existing attached-runtime
   endpoint, model tag/digest, version, residency, cancellation, and cleanup
   proofs.
7. Update `docs/architecture/system-architecture.md`,
   `docs/guides/ollama-attached-prepared-integration.md`,
   `docs/guides/provider-route-matrix.md`, and the Ollama feature-matrix notes
   with exact local/profile truth. Do not add a portable context column.
8. State dispatch separately from acceptance and effective allocation. Note
   that requesting `num_ctx` may load/reload a runner, change memory pressure,
   or contribute to eviction on the externally owned runtime.
9. Add a `CHANGELOG.md` Unreleased entry and closeout log. Complete g04.036 and
   cards 098-100, update Research 184, programme progress, g04 and batch-card
   indexes, and the sole roadmaps Next Task.
10. Set Next Task to compile Anthropic Messages `output_config.effort` as the
    next numbered per-route milestone. Contract 029 currentness stays standing.

## Acceptance Criteria

- deterministic tests prove prepared-path evidence/driver/native-body agreement
- every admitted value boundary and failure class is covered
- default QA starts no runtime and sends no model request
- local attached and remote/cloud exclusions stay explicit
- docs do not claim provider acceptance, effective context, or resource fit
- architecture describes only realized behavior
- the advanced-route triage note remains promoted; programme progress records
  this milestone and points to Anthropic Messages effort
- named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-ollama
effigy validate:focused swallowtail-adapter-ollama
effigy package:verify-affected swallowtail-adapter-ollama
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

- deterministic proof cannot retain one exact typed value through dispatch
- upstream semantics require an unplanned version-segment or contract change
- session restoration or replay changes the fixed value
- docs would need to imply effective allocation from HTTP acceptance
- another Ollama option, provider route, or currentness family enters scope

## Out Of Scope

- live runtime/model verification
- cloud or OpenAI-compatible endpoints
- generic context controls, sampling options, or model administration
- release, publication, or consumer-repository changes
