# g05.026 Kimi Code Local Server 0.41.0 Useful Newer (retargeted from 0.40.1)

Status: ready; retargeted to official latest `0.41.0` on 2026-09-04; card 062 identity resumes on the same worker; card 063 claim is gated behind an admitted segment
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Depends on: Contract 029; Contracts 017 and 023; Research 270 and 276; tagged `v0.4.0`; completed g05.016 and g05.017
Vision tags: route currentness, Kimi Code, compatibility, process authority

## Purpose

Qualify or stop official stable `@moonshot-ai/kimi-code` `0.41.0` as the one
Contract 029 family `kimi-code.local-server`. Research 276 selected this
family alone as the first post-release candidate and changed no claim. The
family shares the npm package with the installed harness but is a separate
route, claim, corpus, and guide; g05.016 and g05.017 did not touch it.

Two facts make this run an authority investigation before a range question.
Research 270 recorded `kimi web` and `kap-server` protocol deltas from
`0.39.0` (remote-terminal flag removal, loopback-only PTY routes, changed auth
middleware, model-catalog and ws-control protocol files). The `0.40.0` release
notes remove the workspace restriction on the Bash tool's `cwd` parameter,
which is the same risk class that moved `kimi-code.acp` to `QualifiedOnly`
under the A2 gate. Card 062 must trace whether any Swallowtail control or
provider boundary actually contains that change before naming a segment
shape. A stop that keeps `0.38.0` as the ceiling is an acceptable outcome.

## Runway

1. Card 062 freezes exact npm/GitHub/tarball/archive identity for `0.41.0`
   plus publication adjacency through `0.39.0`, `0.39.1`, `0.40.0`, and
   `0.40.1`,
   revalidates the frozen `0.38.0` local-server corpus, compares the selected
   REST/WebSocket v2 surfaces, traces the Bash `cwd` change and the `0.39.0`
   web/server deltas as Contract 017/023 authority questions, and names one
   of: compatible extension, private milestone, new revision, or stop.
2. Stop if a selected mapped surface changed without a deterministic
   provider-neutral mapping, identity disagrees, the `cwd` change widens
   process authority with no contained boundary, or a new driver/facade
   revision is required.
3. Continue to card 063 only for an admitted Contract 029 segment.
4. Card 063 changes only the proved range and exact downstream truth, then
   stops for exact-head review.

## Boundary

One family only. No provider prompt, login, install, host update, live
server, live session, catalogue request, feature-facade, watcher, skill,
papercut, release, `kimi-code.acp` or `kimi-code.headless` change, g05.009
card 034, or feature-specific widening. Downloaded official binaries are
hashed and never executed.

## Batch Cards

- [062 Kimi Code Local Server 0.41.0 Identity](batch-cards/062-kimi-code-local-server-0-40-1-identity.md) — ready; retargeted from `0.40.1`; resume the same worker
- [063 Kimi Code Local Server 0.41.0 Claim](batch-cards/063-kimi-code-local-server-0-40-1-claim.md) — planned; gated behind card 062

## Retarget

The first card 062 run stopped when official latest moved from `0.40.1` to
`0.41.0` during the run. The operator chose on 2026-09-04 to retarget the
family to `0.41.0`; `0.40.1` becomes published adjacency and the collected
evidence is retained. The `0.40.0` Bash `cwd` authority question is unchanged.
A second latest move before push stops the lane and returns a policy question
(freeze the target at dispatch versus chase latest) to the operator.

## Dispatch Manifest

Promoted planning commit: the `main` commit that records the retarget.

| Field | Card 062 |
| --- | --- |
| Readiness | ready |
| Prerequisites | tagged `v0.4.0` at `56f3913a`; Research 276; frozen `0.38.0` local-server corpus present on `main`; the retarget to `0.41.0` recorded on `main`; the existing worker on branch `worker/g05-card062-kimi-local-server-0401-identity` resumes rather than a fresh launch |
| Completion conditions | Research 282 committed with exact identity, adjacency, corpus revalidation, surface delta ledger, authority trace, and one named outcome; production claims byte-identical in that commit; card `## Result` filled; focused and Northstar gates green |
| Owned mutable paths | `docs/research/282-*.md`; `docs/research/README.md` (one index line); `crates/swallowtail-adapter-kimi/**` test fixtures and corpus modules for local server only; `docs/roadmaps/g05/batch-cards/062-*.md` result and status; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, `docs/roadmaps/g05/026-*.md`, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/roadmaps/standing-lanes.md`, `docs/logs/README.md`; the coordinator edits these at closeout |
| Forbidden paths | `crates/swallowtail-adapter-kimi/src/local_server/selection.rs`, every other `selection.rs`, route/feature matrices, guides, `CHANGELOG.md`, `docs/contracts/**`, `docs/architecture/**` |
| Approved concurrent siblings | none; the frontier is exactly card 062 |
| Serial edges | card 063 follows card 062 only on an admitted segment; Antigravity `1.1.24` follows g05.026 closeout |
| Worker capability class | evidence-first identity worker; artifact download and hashing; Rust fixture authoring; no provider credentials |
| Acceptance evidence | independent official channels agree on identity; recomputed corpus digests; mutation-sensitive delta ledger; explicit authority trace naming the containing control or its absence |
| Review oracle | one commit contains identity evidence and zero claim edits; the smallest counterexample is a changed selection constant, a widened range, or an authority conclusion without a traced control |
| Stop conditions | official latest moves again during the run (return the freeze-at-dispatch policy question); identity disagreement; `cwd` authority change with no contained boundary; selected surface changed without deterministic mapping; new revision required |
| Escalation owner | operator (Tom) via Chatterbox for product-policy or authority decisions; coordinator for mechanical blockers |

Card 063 enters the manifest only after the coordinator records card 062's
admitted segment at closeout; its owned paths add `local_server/selection.rs`,
the local-server prepared guide, the route matrix cell, `CHANGELOG.md`
`[Unreleased]`, and the standing-lane claim paragraph.

## Acceptance

- [ ] exact official identity is reproducible from independent channels
- [ ] the `0.39.0` web/server deltas and the `0.40.0` Bash `cwd` change are
      classified as authority questions with a traced answer
- [ ] mapped and unmapped protocol changes are classified
- [ ] identity evidence lands before any production claim edit
- [ ] only an admitted segment reaches the claim, or the stop is recorded
- [ ] `kimi-code.acp`, `kimi-code.headless`, exact feature pins, and other
      families stay unchanged
- [ ] focused, package, API, route, docs, and Northstar gates pass
