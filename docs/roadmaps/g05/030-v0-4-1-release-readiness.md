# g05.030 v0.4.1 Release Readiness

Status: ready; card 090 accepted and merged; card 091 is ready under its one-shot prepare authorization; card 092 is serial behind it
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Depends on: Contract 036; immutable `v0.4.0` at `56f3913a`; completed g05.021 as precedent; completed g05.029 card 080 and card 089
Vision tags: source release, compatibility, consumer proof

## Purpose

Produce one reviewed, frozen `v0.4.1` source candidate and the evidence for
a later operator tag decision. The carrier content is the Claude SDK
read-write session and permission policy (g05.029 card 080), which Bovine
Desktop switches to the moment it is tagged. The candidate also carries the
OpenCode `1.18.28` qualification, the Contract 061 tranches for candidates
C, E, F, I, and J with their two shared baselines, and the bounded-profile
preflight scoping from card 089.

Contract 036 says compatible public API and guaranteed-behaviour changes
advance the patch. Every change since `v0.4.0` is believed additive or
widening; card 090 proves that rather than assumes it, and read-only release
status must infer `0.4.1`. If the audit finds a break, the lane stops and
returns to Chatterbox for a minor decision instead of bending the
classification.

This milestone authorizes no tag, tag push, crates.io publication, GitHub
Release object, binary, sidecar, installer, provider call, or consumer-repo
mutation.

## Runway

1. Card 090 audits exact `v0.4.0` to reviewed-current-source package,
   dependency, route, semantic API, and guaranteed-behaviour deltas; freezes
   the release census; classifies every change; and confirms the patch class.
2. Card 091, after separate operator authorization, runs one Effigy prepare
   transaction for coordinated `0.4.1` Cargo versions, changelog promotion,
   and workspace-only lock sync, reruns all local gates on the frozen tree,
   lands the accepted candidate on canonical `main`, and requires CI at that
   exact SHA.
3. Card 092 reruns the external source consumer against the exact candidate,
   performs one operator-authorized authenticated working-application smoke
   (the operator names the application; Chatterbox recommends a Bovine
   Desktop editing session on `claude-agent.sdk` because that is the
   requirement's own acceptance), compiles final evidence, and stops for the
   explicit operator tag decision.

The cards are serial. Card 091 consumes card 090's frozen classifications;
card 092 consumes card 091's immutable candidate SHA and the operator's
smoke authority packet.

## Release Boundary

No card creates or pushes a tag. Card 092 stops with the exact candidate
SHA and asks the operator to authorize source commit, canonical branch and
remote, exact tag name, annotated message, local tag creation, and tag push,
confirming that publication, a GitHub Release, binaries, sidecars, and
installers are absent. Any candidate change after the local gates,
exact-head review, canonical merge, or exact-SHA CI returns to card 091.

## Known Limits Required In Release Notes

- `claude-agent.sdk` write tools run under an explicit ambient-host posture
  with consumer-mediated per-call admission; `acceptEdits` auto-approves
  edits; no filesystem boundary is claimed
- Bash, mid-session model change, resume, and MCP on the SDK route remain
  later cards (081-084)
- the SDK route keeps its five exact qualified-only version pins until card
  087
- Contract 061 coverage is 570 of 767 rows; candidates B, K, and L remain
- Kimi local server and Antigravity ceilings stay at `0.38.0` and `1.1.17`
  with named reopen conditions; Gemini remains deferred; the watcher stays
  exact `2.1.251`

## Batch Cards

- [093 Claude SDK Sidecar Fixture Determinism](batch-cards/093-claude-sdk-sidecar-fixture-determinism.md) — ready; release-lane repair under the freeze; card 091 re-prepares on its merged base
- [090 v0.4.0 To Candidate Compatibility Audit](batch-cards/090-v0-4-0-to-candidate-compatibility-audit.md) — accepted and merged as `3dcf4f12`
- [091 v0.4.1 Candidate Preparation And Exact-SHA CI](batch-cards/091-v0-4-1-candidate-preparation-and-exact-sha-ci.md) — ready; one-shot prepare authorization granted 2026-09-05
- [092 v0.4.1 Consumer Proof And Operator Tag Gate](batch-cards/092-v0-4-1-consumer-proof-and-operator-tag-gate.md) — planned; serial after card 091; Bovine Desktop editing session accepted as the smoke; exact checkout, command, and retry budget still required

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.

| Field | Card 090 |
| --- | --- |
| Readiness | ready |
| Prerequisites | immutable `v0.4.0` at `56f3913a`; card 080's second PR merged at `23d3cd8d`; card 034 merged at `404fa068`; no open feature PR |
| Completion conditions | Research 286 with the complete `v0.4.0..HEAD` package, dependency, route, semantic API, and guaranteed-behaviour ledgers; every change classified compatible or breaking with evidence; immutable `v0.4.0` baselines byte-identical; patch class confirmed or a stop recorded; card result filled; docs and Northstar gates green |
| Owned mutable paths | `docs/research/286-*.md`; `docs/research/README.md` one index line; this card's `## Result`; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md` |
| Forbidden paths | every `crates/**` path; `Cargo.toml`, `Cargo.lock`; `CHANGELOG.md`; `release-baselines/**`; `docs/releases/**`; contracts; version claims |
| Approved concurrent siblings | none; the currentness and Contract 061 lanes are quiet and the release lane freezes feature merges until card 092 stops |
| Serial edges | card 091 follows an accepted card 090; card 092 follows card 091's candidate merge and exact-SHA CI |
| Worker capability class | evidence-first audit worker with `cargo-public-api` and Contract 036 toolchain discipline; no provider credentials |
| Acceptance evidence | ledgers keyed to the `v0.4.0` peel and the audited head; immutable-baseline diff; read-only release-status output inferring `0.4.1` |
| Review oracle | one exact tree supports every statement; the smallest counterexample is a public item or guaranteed value changed since `v0.4.0` but absent from the ledger, or a break classified compatible |
| Stop conditions | any breaking public API or guaranteed-behaviour change (return to Chatterbox for the minor decision); a modified immutable baseline; an open mergeable feature or currentness PR |
| Escalation owner | operator via Chatterbox for release classification; coordinator for mechanical blockers |

### Card 091 Manifest

Promoted planning commit: the `main` commit that introduces this section.
Card 090 was accepted at exact head `90c9c65a` and merged as `3dcf4f12`;
Research 286 confirms the patch class. The operator's one-shot prepare
authorization of 2026-09-05 applies.

| Field | Card 091 |
| --- | --- |
| Readiness | ready |
| Prerequisites | card 090 merged; Research 286 on `main`; clean canonical base with no open feature or currentness PR; `Cargo.lock` in sync before the first `--locked` gate (the `v0.4.0` lock-sync precedent) |
| Completion conditions | `docs/releases/0.4.1.md` and index entry authored from Research 286 before any mutation; read-only release status infers `0.4.1` with the three-mutation plan; exactly one prepare transaction applies coordinated `0.4.1` versions, promotes the changelog, and syncs the workspace-only lock; all 11 local gates pass on the frozen tree; the exact promoted changelog is extracted; distinct `0.4.1` semantic baseline, route inventory, and dependency graph are generated without touching any `0.4.0` file; one candidate PR; exact-head review; canonical merge; workflow-dispatch CI green at the merged SHA |
| Owned mutable paths | every workspace `Cargo.toml` version and internal requirement through the prepare transaction only; `Cargo.lock` workspace entries only; `CHANGELOG.md` promotion only; `docs/releases/0.4.1.md`; `docs/releases/README.md` current-release entry; `release-baselines/public-api-0.4.1/**`, `production-routes-0.4.1.txt`, `internal-dependencies-0.4.1.tsv`; `.release-prepared.json`; this card's `## Result`; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md`, the `docs/releases/README.md` historical lines |
| Forbidden paths | every `crates/**/src` and test path; every `release-baselines/*0.4.0*` and earlier file; contracts; architecture; guides; matrices; version claims; any feature change |
| Approved concurrent siblings | none; feature freeze |
| Serial edges | card 092 follows the merged candidate with green exact-SHA CI and the completed smoke packet |
| Worker capability class | release-preparation worker with Effigy release discipline; frontier-tier; no provider credentials; no tag authority |
| Acceptance evidence | read-only status output; prepare transaction log with the three mutations; 11 gate results on the frozen tree; extracted changelog; new `0.4.1` baseline files; PR head and merged SHA; workflow-dispatch run id at that SHA |
| Review oracle | one exact tree supports every candidate statement; the smallest counterexample is a post-gate commit presented as the candidate, a regenerated `0.4.0` baseline, or a changelog that disagrees with Research 286 |
| Stop conditions | a gate fails (roll back the three mutations; report to Chatterbox with the captured gate log; a transient, reproduced-green failure is renewed by Chatterbox under the operator's 2026-09-05 standing grant; a real defect stops the lane); release status infers anything but `0.4.1`; a feature or currentness PR is open; the lock is out of sync at the first `--locked` gate |
| Escalation owner | operator via Chatterbox for any further authorization; coordinator for mechanical blockers |
 Card 092 enters after card
091's candidate merges with green exact-SHA CI and the smoke packet is
complete: Bovine Desktop on `claude-agent.sdk` is accepted; the exact checkout,
command, and retry budget are still to be supplied.

## Feature Freeze

From this promotion until card 092 stops, no feature or currentness PR
merges to `main`. Cards 081-088 and the Contract 061 remainder stay queued.
Release-lane repairs that fix a gate defect without changing production
behaviour (card 093) are the one exception, on the `v0.4.0` precedent.

## Third Prepare And Fixture Repair

Card 091's third prepare succeeded on all 11 gates, but the required
frozen-tree rerun failed the floor on two sidecar-asset tests that time out
under host load. The operator ruled on 2026-09-05 that the fixture is the
defect and must be fixed, not timed around. Card 093 owns that repair. The
prepared tree in the worker workspace is discarded; card 091 re-prepares on
card 093's merged base under the standing grant, in `--json` mode.

### Card 093 Manifest

Promoted planning commit: the `main` commit that introduces this section.

| Field | Card 093 |
| --- | --- |
| Readiness | ready |
| Prerequisites | card 091's frozen-gates log naming the two tests; current `main` |
| Completion conditions | fixture reads observations after the wire record that guarantees them; no passing test depends on a bound; loop-under-load proof with zero failures recorded; production sidecar and adapter source unchanged; named validation green under stable and `1.95.0`; one PR |
| Owned mutable paths | `crates/swallowtail-adapter-claude-agent/tests/**` (including `sidecar_asset_support/fake-sdk.mjs` and `mod.rs`); this card's `## Result`; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md` |
| Forbidden paths | `crates/swallowtail-adapter-claude-agent/src/**` and `sidecar/**`; every other crate; Cargo files; `CHANGELOG.md`; `release-baselines/**`; contracts |
| Approved concurrent siblings | none |
| Serial edges | card 091's fourth prepare follows card 093's merge |
| Worker capability class | Rust and Node test-fixture worker with concurrency discipline; no provider credentials |
| Acceptance evidence | the loop-under-load run log; diff confined to tests; pinned-toolchain focused test run |
| Review oracle | the card's invariant |
| Stop conditions | ordering cannot be guaranteed without a production change (return to Chatterbox) |
| Escalation owner | operator via Chatterbox; coordinator for mechanical blockers |


## Acceptance

- [ ] every semantic API and guaranteed-behaviour delta from `v0.4.0` is
      inventoried and classified, and the patch class holds
- [ ] immutable `v0.4.0` baselines are unchanged; a distinct `0.4.1`
      baseline and route inventory are created only during preparation
- [ ] all local gates pass together on the frozen candidate, then exact-SHA CI
- [ ] the external source consumer and one authenticated working-application
      editing session pass against the exact candidate
- [ ] final evidence names the exact SHA and stops before tag creation
