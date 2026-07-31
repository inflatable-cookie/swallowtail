# 021 Qwen Code 0.19.11 To 0.21.2 Range Corpus

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../009-qwen-code-installed-range-closure.md`
Depends on: card 018

## Goal

Freeze the exact stable Qwen Code interval and every selected-route behavior
milestone before widening the production claim.

## Scope

1. Classify stable `0.19.12`, `0.20.0`, `0.20.1`, `0.21.0`, `0.21.1`, and
   `0.21.2`; ignore prerelease, nightly, and experimental variants.
2. Compare exact selected command flags, safe-mode behavior, read-only tool
   registration, stream records, errors, catalogue control, activity fields,
   and exact-session resume behavior.
3. Freeze baseline, latest boundary, and material milestone fixtures with
   authoritative package metadata.
4. Name behavior revisions and any exact exclusions required by the evidence.
5. Add deterministic compatibility-corpus assertions without changing the
   production claim.

## Acceptance Criteria

- [x] every stable point in the interval has one evidence classification
- [x] source and package identities are exact and indexed
- [x] unchanged framing is distinguished from invocation or capability changes
- [x] fixture data contains no host paths, credentials, raw provider payloads,
  or consumer policy
- [x] the existing exact `0.19.11` production claim remains unchanged
- [x] focused Qwen tests pass
- [x] card 022 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-qwen`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no authenticated provider prompt

## Auto-Continuation

Yes. Continue to card 022 when the corpus settles every compatibility segment.

## Evidence

- Research 082 records seven stable points and exact source identities.
- one stream declaration blob spans the interval
- `0.21.0` introduces the only selected behavior milestone by filtering
  image-only catalogue entries
- 34 focused Qwen tests passed with warnings denied
- no live provider or workspace effect ran
