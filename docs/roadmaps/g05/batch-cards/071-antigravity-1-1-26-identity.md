# 071 Antigravity 1.1.26 Identity

Status: complete; stop. Official `1.1.26` identity frozen; `1.1.22` provider-managed retry is unbounded and unaccepted; ceiling stays `1.1.17`; card 072 not admitted
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../027-antigravity-1-1-26-useful-newer.md`
Depends on: Contract 029; Research 276; frozen `antigravity-cli-1.1.17` corpus; official stable `1.1.26`

## Goal

Freeze exact official `google-antigravity/antigravity-cli` identity for every
published stable from `1.1.18` through `1.1.26`, classify the selected
catalogue, print, and continuation surfaces hop by hop against the frozen
`1.1.17` corpus, and record one outcome without changing a claim or
executing a downloaded binary.

## Scope

1. Enumerate GitHub releases after `1.1.17`. Record tag commit, publication
   time, and `linux-x64` tarball plus extracted-binary SHA-256, size, ELF
   Build ID, and in-binary version literal for each hop through `1.1.26`.
   Record the first unpublished later stable.
2. Recompute rather than trust the frozen `1.1.17` corpus digests.
3. Use PR 182's `1.1.18..=1.1.24` evidence as a cross-check only: every
   digest it recorded must be recomputed here, and any disagreement is a
   stop.
4. Keep host `agy` observation-only. Do not install, update, or run it.
5. Compare the selected surfaces per hop as in-binary literals and shipped
   file inventory: `--print`, `--output-format`, `--model`, `--mode`,
   `--sandbox`, `--effort`, `--json-schema`, `--conversation`, and the
   `models` subcommand. Keep `--input-format`, `mcp`, `mic-serve`, voice,
   remote control, and sign-in extras unmapped.
6. Read each hop's changelog as discovery only. Classify any entry that
   touches mapped print or catalogue lifecycle as compatible-extension
   repair, private milestone, new revision, or authority change, with the
   artifact evidence that supports it. Trace any capability or process
   authority change against Contracts 017 and 023 before labelling it.
7. Add Research 283 and one secret-free `antigravity-cli-1.1.26` identity
   corpus with a delta-ledger test that also covers the intermediate hops.
8. Commit identity evidence before any selection, matrix, guide, changelog,
   or standing-lane claim edit.
9. Record exactly one outcome: compatible extension, private milestone, new
   revision, or stop.

## Out Of Scope

Production claim edits, `selection.rs`, the Contract 061 projection code in
the Antigravity crate, ACP-registry `antigravity-acp`, Gemini, provider
contact, install, host update, live probe, feature-specific widening,
release work, or execution of downloaded binaries.

## Acceptance Criteria

- every hop's identity is corroborated from official artifacts and
  recomputed digests
- PR 182's digests are either confirmed or the disagreement is recorded as
  a stop
- mapped and material unmapped additions are explicit per hop
- current production claims are byte-for-byte unchanged in this commit
- fixture provenance, digests, and negative boundaries are load-bearing
- card 072 continues only for an admitted segment

## Validation

- `effigy validate:focused swallowtail-adapter-antigravity`
- `effigy package:verify-affected swallowtail-adapter-antigravity`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: this commit changes evidence only, and every hop's verdict rests
on an artifact digest, not on PR 182 or a changelog sentence.

Smallest counterexample: a moved selection constant, a hop whose digest was
copied from PR 182 rather than recomputed, or a "compatible extension"
verdict for a hop whose selected-flag literal set changed.

## Auto-Continuation

Yes, to card 072 only after an admitted segment is recorded. A stop ends the
milestone at this card.

## Stop Conditions

Official latest moves during the run (stop and escalate); identity or digest
disagreement with recomputation; a selected surface or process authority
changes without deterministic mapping; a new driver or facade revision is
required.

## Result

Stop. Official `1.1.17..=1.1.26` release, tag, linux-x64 tarball, extracted
binary, Build ID, and in-binary version identity is frozen. Fresh downloads
match every frozen `1.1.17` corpus digest and every parked PR 182 digest.
Selected literals hold at every hop. `1.1.22` changes selected headless
failure lifecycle by retrying model-endpoint HTTP 502 responses, but official
evidence publishes no finite bound or disable control. Contract 023 requires
separate acceptance for provider-managed retry; none exists. The host deadline
does not substitute for that policy. Production claims remain unchanged, the
ceiling stays `1.1.17`, and card 072 is not admitted. Reopen when official
evidence exposes a deterministic retry policy and control, or the operator
accepts the exact retry behavior. Evidence: [Research
283](../../../research/283-antigravity-1-1-26-identity.md) and
`crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.26/`.
