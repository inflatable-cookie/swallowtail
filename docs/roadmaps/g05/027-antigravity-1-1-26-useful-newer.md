# g05.027 Antigravity 1.1.26 Useful Newer

Status: complete; evidence stop at card 071; card 072 claim remains gated behind an admitted segment
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Depends on: Contract 029; Research 276; parked PR 182 evidence; completed g05.026; tagged `v0.4.0`
Vision tags: route currentness, Antigravity, compatibility

## Purpose

Qualify official GitHub `google-antigravity/antigravity-cli` `1.1.26` as one
Contract 029 family on the `antigravity.catalogue` and `antigravity.headless`
axes, preserving identity-before-claim, baseline `1.1.9`, independently
unqualified `1.1.8`, and `AllowUnverified`. The current ceiling is `1.1.17`.
Research 276 ranked this family behind Kimi; g05.026 resolved as an evidence
stop, so this is the next currentness lane.

Closed PR 182 (branch `cursor/antigravity-1-1-24-useful-newer-29a5`, head
`562225db`) froze `1.1.17` through `1.1.24` as a compatible extension and was
parked under the v0.4.0 freeze. Its evidence is an input, not merged
qualification: recompute it rather than trust it, then extend through
`1.1.25` and `1.1.26`. Its research number `276` collides with the canonical
checkpoint; this lane writes Research 283.

Antigravity ships roughly daily. Under Contract 029's In-Run Latest Movement
rule (operator-accepted 2026-09-04), a stable published before card 071's
identity commit lands is added as a further hop and recomputed, not a stop.
The lane stops only if an added hop changes a selected surface, capability,
or authority, or latest moves after the identity commit, in which case the
later stable stays `UnverifiedNewer`.

## Runway

1. Card 071 freezes exact GitHub release, asset, tarball, and extracted
   binary identity for every published stable from `1.1.18` through
   `1.1.26`, recomputes the frozen `1.1.17` corpus, compares the selected
   print, catalogue, and continuation surfaces hop by hop, and names one
   outcome without executing a downloaded binary.
2. Stop if a selected surface changed without a deterministic mapping,
   identity disagrees, an authority boundary widened, or a new driver or
   facade revision is required.
3. Continue to card 072 only for an admitted Contract 029 segment.
4. Card 072 changes only the proved range and exact downstream truth, then
   stops for exact-head review.

## Boundary

One family only. No provider prompt, login, install, host update, live
session, execution of downloaded binaries, Gemini deferral lift, Claude Code
reopen, ACP-registry `antigravity-acp` flattening, or feature-specific
widening. Card 069's Contract 061 projection code in the Antigravity crate is
out of scope and must not be edited.

## Batch Cards

- [071 Antigravity 1.1.26 Identity](batch-cards/071-antigravity-1-1-26-identity.md) — complete; evidence stop at `1.1.22`; ceiling remains `1.1.17`
- [072 Antigravity 1.1.26 Claim](batch-cards/072-antigravity-1-1-26-claim.md) — planned; gated behind card 071

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.

| Field | Card 071 |
| --- | --- |
| Readiness | ready |
| Prerequisites | g05.026 closed at `43033f75`; frozen `antigravity-cli-1.1.17` fixtures on `main`; PR 182 branch reachable on origin as evidence input |
| Completion conditions | Research 283 committed with exact identity for every hop `1.1.18..=1.1.26`, recomputed `1.1.17` corpus, hop-by-hop selected-surface ledger, authority trace, and one named outcome; production claims byte-identical in that commit; card `## Result` filled; focused and Northstar gates green |
| Owned mutable paths | `docs/research/283-*.md`; `docs/research/README.md` (one index line); `crates/swallowtail-adapter-antigravity/tests/**` fixtures and delta-ledger tests only; `docs/roadmaps/g05/batch-cards/071-*.md` result and status; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, `docs/roadmaps/g05/027-*.md`, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/roadmaps/standing-lanes.md`, `docs/logs/README.md`; the coordinator edits these at closeout |
| Forbidden paths | `crates/swallowtail-adapter-antigravity/src/**` (including `selection.rs` and `consumer_route_projection.rs`), every other crate, route/feature matrices, guides, `CHANGELOG.md`, `docs/contracts/**`, `docs/architecture/**`, the PR 182 branch itself |
| Approved concurrent siblings | g05.009 cards 070 and 065 |
| Serial edges | card 072 follows card 071 only on an admitted segment |
| Worker capability class | evidence-first identity worker; artifact download and hashing; Rust fixture authoring; no provider credentials |
| Acceptance evidence | GitHub release, tag commit, tarball, and extracted-binary digests per hop; recomputed `1.1.17` corpus; in-binary version literals; selected-flag presence per hop; mutation-sensitive delta ledger |
| Review oracle | one commit contains identity evidence and zero claim edits; the smallest counterexample is a changed selection constant, a hop taken from PR 182 without recomputation, or a surface verdict without an artifact anchor |
| Stop conditions | an added hop changes a selected surface, capability, or authority (latest moving before the identity commit is a hop extension, not a stop); identity disagreement; a selected surface or authority change without deterministic mapping; new revision required |
| Escalation owner | operator (Tom) via Chatterbox for policy or authority questions; coordinator for mechanical blockers |

Card 072 enters the manifest only after the coordinator records card 071's
admitted segment at closeout; its owned paths add `selection.rs`, the
Antigravity prepared guide, the route and feature matrix cells,
`CHANGELOG.md` `[Unreleased]`, and the standing-lane claim paragraph.

## Acceptance

- [ ] exact official identity is reproducible for every hop from independent
      artifacts, not from PR 182 alone
- [ ] mapped and unmapped changes are classified per hop
- [ ] identity evidence lands before any production claim edit
- [ ] only an admitted segment reaches the claim, or the stop is recorded
- [ ] `1.1.8`, other families, and the Contract 061 projection code stay
      unchanged
- [ ] focused, package, API, route, docs, and Northstar gates pass
