# 2026-09-01 g05.009 Cline Active-Observation Gate Compiled

Status: complete; card 032 ready
Owner: Tom
Date: 2026-09-01
Contracts: 037, 047, 057, 061

## Result

The operator approved both route-local Cline decisions: adapter-local
retention of exact effective/rejected Plan acknowledgement plus exact bounded
negotiated model options, and one additive adapter-owned projected-open seam
preserving `ClinePreparedSession::open_session`.

The gate fixes the public names and signatures, typed state, prepared and
active source identities, lifecycle, failure preservation, and provider-free
review oracles. It changes no runtime, testkit, or core public decision and
grants no authority to Kimi or another candidate.

Optional model metadata is observation, not legacy route admission. One exact
bounded row becomes `NegotiatedSessionModelOptions`; absence stays absent.
Invalid metadata preserves legacy-open success with no snapshot, while the
projected path closes the session and returns the exact negotiated-model
runtime failure with no contribution. Existing Plan failures remain unchanged.

Candidate G now passes the Batch 9.4 promotion rubric and is ready as card 032.
The implementation tranche owns 48 exact rows: 11 `cline.acp`, 8
`cline.headless`, 11 `command-code.headless`, 9 `copilot-cli.acp`, and 9
`goose.acp`. Its maximal ledger expects 38 emitted and 10 withheld rows.

## Current State

- g05.009 is `strict-ready`
- card 032 is the sole ready implementation card and sole Next Task
- 201 census rows remain proved; candidate G's 48 rows are not counted until
  implementation review and merge
- 566 rows remain unproved, including candidate G; 518 sit in candidates B,
  C, E, F, and I-L
- candidate F's three Kimi post-open families remain unapproved and unplanned
- no Rust, manifest, release baseline, contract, architecture, census, or
  provider claim changed
- no provider was contacted and no live probe ran

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Next Move

Implement card 032 as one four-package PR. Stop after its exact 48-row proof
for orchestrator review before reassessing another Batch 9.4 candidate.

## Authority

- [card 032](../roadmaps/g05/batch-cards/032-contract-061-cline-command-code-copilot-goose-package-completion.md)
- [public-baseline gate](../triage/2026-09-01-contract-061-cline-active-observation-public-baseline-gate.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.4 package expansion](../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
