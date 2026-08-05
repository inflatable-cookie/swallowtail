# 2026-08-05 Integration Guide System Acceptance

Roadmap: `../roadmaps/g03/042-complete-integration-guide-system.md`
Card: `../roadmaps/g03/batch-cards/123-integration-guide-coverage-acceptance.md`

## Changed

- added `scripts/check-integration-guide-coverage.py` behind
  `effigy qa:guides`
- made guide coverage part of ordinary `effigy qa:docs`
- compare all 33 production route ids with complete canonical route-guide and
  normal-path example rows
- compare all 34 feature-matrix headers plus nine named portable/operator
  surfaces with one complete canonical feature-family owner
- require every guide owner to exist locally and appear in the guide index
- fail on missing, duplicate, partial, unexpected, stale, or out-of-tree
  traceability
- reconciled root, guide, architecture, contract, roadmap, log, and script
  front doors
- closed all six g03.042 cards and returned g03 to its evidence gate

No provider capability, route behavior, access posture, version claim,
credential flow, fallback, retry, or consumer persistence policy changed.

## Validation

- `effigy qa:guides` — passed: 33 routes, 22 route guides, 32 distinct
  examples, 14 feature families, 11 feature guides, 43 tracked features
- `effigy validate:focused swallowtail-adapter-antigravity
  swallowtail-adapter-cursor swallowtail-adapter-grok` — 96 tests passed;
  warnings-denied clippy passed
- `effigy package:verify-affected swallowtail-adapter-antigravity
  swallowtail-adapter-cursor swallowtail-adapter-grok` — all three independently
  assembled and compiled
- `effigy check:examples` — passed
- `effigy qa:docs` — passed
- `effigy qa:routes` — passed
- `effigy format:check` — passed
- `git diff --check` — passed

`effigy doctor` still reports the previously recorded 22 structural
god-file errors. They remain outside this documentation lane. No live or
authenticated provider work ran.

## Next Move

Hold at the g03 evidence gate. Reassess only after a consumer defect, material
non-deferred stable drift, or explicit operator promotion supplies new
evidence.
